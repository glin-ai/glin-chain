#![cfg_attr(not(feature = "std"), no_std)]

use codec::DecodeWithMemTracking;
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency, Imbalance},
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_runtime::{
    traits::{Saturating, Zero},
    Percent,
};

pub use pallet::*;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency type for staking
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Minimum stake required to become a provider
        #[pallet::constant]
        type MinimumStake: Get<BalanceOf<Self>>;

        /// Maximum number of providers
        #[pallet::constant]
        type MaxProviders: Get<u32>;

        /// Slash percentage for malicious behavior
        #[pallet::constant]
        type SlashPercentage: Get<Percent>;

        /// Cooldown period for unstaking (in blocks)
        #[pallet::constant]
        type UnstakingPeriod: Get<BlockNumberFor<Self>>;
    }

    #[derive(Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ProviderStatus {
        Active,
        Idle,
        Busy,
        Offline,
        Suspended,
        Unbonding,
    }

    #[derive(Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum GpuTier {
        Consumer,  // RTX 3070, 3080, etc.
        Prosumer,  // RTX 4080, 4090
        Professional, // A100, H100
    }

    #[derive(Encode, Decode, DecodeWithMemTracking, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Provider<T: Config> {
        pub stake: BalanceOf<T>,
        pub status: ProviderStatus,
        pub hardware_info: HardwareInfo,
        pub reputation_score: u32, // 0-1000, where 1000 is perfect
        pub total_tasks_completed: u32,
        pub total_gradients_computed: u64,
        pub total_tokens_earned: BalanceOf<T>,
        pub registered_at: BlockNumberFor<T>,
        pub last_active: BlockNumberFor<T>,
        pub unbonding_at: Option<BlockNumberFor<T>>,
    }

    #[derive(Encode, Decode, DecodeWithMemTracking, Clone, RuntimeDebug, TypeInfo, Default, PartialEq, MaxEncodedLen)]
    pub struct HardwareInfo {
        pub gpu_model: BoundedVec<u8, ConstU32<100>>,
        pub gpu_tier: GpuTier,
        pub vram_gb: u32,
        pub compute_capability: u32, // Stored as u32, divide by 10 for float
        pub bandwidth_mbps: u32,
        pub cpu_cores: u32,
        pub ram_gb: u32,
    }

    impl Default for GpuTier {
        fn default() -> Self {
            GpuTier::Consumer
        }
    }

    #[derive(Encode, Decode, DecodeWithMemTracking, Clone, RuntimeDebug, TypeInfo, PartialEq, MaxEncodedLen)]
    pub enum SlashReason {
        MaliciousGradient,
        FalseHardwareClaim,
        Downtime,
        ValidationFailure,
    }

    #[pallet::storage]
    #[pallet::getter(fn providers)]
    pub type Providers<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Provider<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn provider_count)]
    pub type ProviderCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn slash_history)]
    pub type SlashHistory<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BlockNumberFor<T>,
        (SlashReason, BalanceOf<T>),
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Provider registered [provider, stake]
        ProviderRegistered {
            provider: T::AccountId,
            stake: BalanceOf<T>,
        },

        /// Provider updated hardware info [provider]
        HardwareUpdated {
            provider: T::AccountId,
        },

        /// Provider slashed [provider, amount, reason]
        ProviderSlashed {
            provider: T::AccountId,
            amount: BalanceOf<T>,
            reason: SlashReason,
        },

        /// Provider started unbonding [provider, unbonding_at]
        UnbondingStarted {
            provider: T::AccountId,
            unbonding_at: BlockNumberFor<T>,
        },

        /// Provider withdrew stake [provider, amount]
        StakeWithdrawn {
            provider: T::AccountId,
            amount: BalanceOf<T>,
        },

        /// Provider reputation updated [provider, new_score]
        ReputationUpdated {
            provider: T::AccountId,
            new_score: u32,
        },

        /// Provider status changed [provider, new_status]
        StatusChanged {
            provider: T::AccountId,
            new_status: ProviderStatus,
        },
    }

    #[pallet::error]
    #[derive(PartialEq)]
    pub enum Error<T> {
        /// Provider not found
        ProviderNotFound,
        /// Provider already registered
        ProviderAlreadyRegistered,
        /// Stake below minimum
        StakeBelowMinimum,
        /// Too many providers
        TooManyProviders,
        /// Invalid hardware specification
        InvalidHardwareSpec,
        /// Provider not active
        ProviderNotActive,
        /// Still in unbonding period
        StillUnbonding,
        /// Nothing to withdraw
        NothingToWithdraw,
        /// Invalid GPU model
        InvalidGpuModel,
        /// Insufficient stake for operation
        InsufficientStake,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register as a compute provider with stake
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn register_provider(
            origin: OriginFor<T>,
            stake_amount: BalanceOf<T>,
            hardware_info: HardwareInfo,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            // Validate inputs
            ensure!(stake_amount >= T::MinimumStake::get(), Error::<T>::StakeBelowMinimum);
            ensure!(!Providers::<T>::contains_key(&provider), Error::<T>::ProviderAlreadyRegistered);
            ensure!(ProviderCount::<T>::get() < T::MaxProviders::get(), Error::<T>::TooManyProviders);
            ensure!(hardware_info.vram_gb > 0, Error::<T>::InvalidHardwareSpec);
            // GPU model validation is handled by BoundedVec

            // Reserve the stake
            T::Currency::reserve(&provider, stake_amount)?;

            // Validate GPU model length
            let gpu_model_bounded = hardware_info.gpu_model.try_into().map_err(|_| Error::<T>::InvalidGpuModel)?;
            let hardware_info_bounded = HardwareInfo {
                gpu_model: gpu_model_bounded,
                gpu_tier: hardware_info.gpu_tier,
                vram_gb: hardware_info.vram_gb,
                compute_capability: hardware_info.compute_capability,
                bandwidth_mbps: hardware_info.bandwidth_mbps,
                cpu_cores: hardware_info.cpu_cores,
                ram_gb: hardware_info.ram_gb,
            };

            // Create provider entry
            let provider_info = Provider {
                stake: stake_amount,
                status: ProviderStatus::Active,
                hardware_info: hardware_info_bounded,
                reputation_score: 500, // Start with neutral reputation
                total_tasks_completed: 0,
                total_gradients_computed: 0,
                total_tokens_earned: Zero::zero(),
                registered_at: frame_system::Pallet::<T>::block_number(),
                last_active: frame_system::Pallet::<T>::block_number(),
                unbonding_at: None,
            };

            // Store provider
            Providers::<T>::insert(&provider, &provider_info);
            ProviderCount::<T>::mutate(|count| *count = count.saturating_add(1));

            // Emit event
            Self::deposit_event(Event::ProviderRegistered {
                provider,
                stake: stake_amount,
            });

            Ok(())
        }

        /// Update hardware information
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(5_000, 0))]
        pub fn update_hardware(
            origin: OriginFor<T>,
            hardware_info: HardwareInfo,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            Providers::<T>::try_mutate(&provider, |maybe_provider| {
                let provider_info = maybe_provider.as_mut().ok_or(Error::<T>::ProviderNotFound)?;

                ensure!(hardware_info.vram_gb > 0, Error::<T>::InvalidHardwareSpec);

                // Validate GPU model length
                let gpu_model_bounded = hardware_info.gpu_model.try_into().map_err(|_| Error::<T>::InvalidGpuModel)?;
                let hardware_info_bounded = HardwareInfo {
                    gpu_model: gpu_model_bounded,
                    gpu_tier: hardware_info.gpu_tier,
                    vram_gb: hardware_info.vram_gb,
                    compute_capability: hardware_info.compute_capability,
                    bandwidth_mbps: hardware_info.bandwidth_mbps,
                    cpu_cores: hardware_info.cpu_cores,
                    ram_gb: hardware_info.ram_gb,
                };

                provider_info.hardware_info = hardware_info_bounded;

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::HardwareUpdated { provider });

            Ok(())
        }

        /// Start unbonding process
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(6_000, 0))]
        pub fn start_unbonding(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            Providers::<T>::try_mutate(&provider, |maybe_provider| {
                let provider_info = maybe_provider.as_mut().ok_or(Error::<T>::ProviderNotFound)?;

                ensure!(provider_info.status != ProviderStatus::Unbonding, Error::<T>::StillUnbonding);

                let unbonding_at = frame_system::Pallet::<T>::block_number()
                    .saturating_add(T::UnstakingPeriod::get());

                provider_info.status = ProviderStatus::Unbonding;
                provider_info.unbonding_at = Some(unbonding_at);

                Ok::<_, DispatchError>(unbonding_at)
            }).map(|unbonding_at| {
                Self::deposit_event(Event::UnbondingStarted {
                    provider,
                    unbonding_at,
                });
            })?;

            Ok(())
        }

        /// Withdraw stake after unbonding period
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(8_000, 0))]
        pub fn withdraw_stake(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            let provider_info = Providers::<T>::get(&provider).ok_or(Error::<T>::ProviderNotFound)?;

            ensure!(provider_info.status == ProviderStatus::Unbonding, Error::<T>::ProviderNotActive);

            let unbonding_at = provider_info.unbonding_at.ok_or(Error::<T>::NothingToWithdraw)?;
            let current_block = frame_system::Pallet::<T>::block_number();
            ensure!(current_block >= unbonding_at, Error::<T>::StillUnbonding);

            // Unreserve stake
            T::Currency::unreserve(&provider, provider_info.stake);

            // Remove provider
            Providers::<T>::remove(&provider);
            ProviderCount::<T>::mutate(|count| *count = count.saturating_sub(1));

            Self::deposit_event(Event::StakeWithdrawn {
                provider,
                amount: provider_info.stake,
            });

            Ok(())
        }

        /// Slash a provider for malicious behavior (governance/sudo only)
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn slash_provider(
            origin: OriginFor<T>,
            provider: T::AccountId,
            reason: SlashReason,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Providers::<T>::try_mutate(&provider, |maybe_provider| {
                let provider_info = maybe_provider.as_mut().ok_or(Error::<T>::ProviderNotFound)?;

                let slash_amount = T::SlashPercentage::get() * provider_info.stake;

                // Reduce stake
                let remaining = provider_info.stake.saturating_sub(slash_amount);

                // Slash from reserved balance
                let (actual_slash, _) = T::Currency::slash_reserved(&provider, slash_amount);
                let actual_slash_amount = actual_slash.peek();

                provider_info.stake = remaining;

                // Update reputation
                provider_info.reputation_score = provider_info.reputation_score.saturating_sub(100);

                // Suspend if reputation too low or stake below minimum
                if provider_info.reputation_score < 200 || remaining < T::MinimumStake::get() {
                    provider_info.status = ProviderStatus::Suspended;
                }

                // Record slash
                let block = frame_system::Pallet::<T>::block_number();
                SlashHistory::<T>::insert(&provider, block, (reason.clone(), actual_slash_amount));

                Ok::<_, DispatchError>(actual_slash_amount)
            }).map(|slash_amount| {
                Self::deposit_event(Event::ProviderSlashed {
                    provider,
                    amount: slash_amount,
                    reason,
                });
            })?;

            Ok(())
        }

        /// Update provider reputation (governance/reward pallet only)
        #[pallet::call_index(5)]
        #[pallet::weight(Weight::from_parts(5_000, 0))]
        pub fn update_reputation(
            origin: OriginFor<T>,
            provider: T::AccountId,
            new_score: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(new_score <= 1000, Error::<T>::InvalidHardwareSpec);

            Providers::<T>::try_mutate(&provider, |maybe_provider| {
                let provider_info = maybe_provider.as_mut().ok_or(Error::<T>::ProviderNotFound)?;

                provider_info.reputation_score = new_score;

                // Auto-suspend if reputation too low
                if new_score < 200 {
                    provider_info.status = ProviderStatus::Suspended;
                }

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::ReputationUpdated {
                provider,
                new_score,
            });

            Ok(())
        }

        /// Update provider status (internal use)
        #[pallet::call_index(6)]
        #[pallet::weight(Weight::from_parts(5_000, 0))]
        pub fn update_status(
            origin: OriginFor<T>,
            provider: T::AccountId,
            new_status: ProviderStatus,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Providers::<T>::try_mutate(&provider, |maybe_provider| {
                let provider_info = maybe_provider.as_mut().ok_or(Error::<T>::ProviderNotFound)?;

                provider_info.status = new_status.clone();
                provider_info.last_active = frame_system::Pallet::<T>::block_number();

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::StatusChanged {
                provider,
                new_status,
            });

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Check if an account is a registered provider
        pub fn is_provider(account: &T::AccountId) -> bool {
            Providers::<T>::contains_key(account)
        }

        /// Get provider's stake amount
        pub fn get_stake(account: &T::AccountId) -> Option<BalanceOf<T>> {
            Providers::<T>::get(account).map(|p| p.stake)
        }

        /// Get provider's reputation score
        pub fn get_reputation(account: &T::AccountId) -> Option<u32> {
            Providers::<T>::get(account).map(|p| p.reputation_score)
        }
    }
}