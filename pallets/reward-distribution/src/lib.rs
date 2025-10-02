#![cfg_attr(not(feature = "std"), no_std)]

use codec::DecodeWithMemTracking;
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement},
    PalletId,
};
use frame_system::pallet_prelude::*;
use scale_info::prelude::vec::Vec;
use sp_std;
use sp_runtime::{
    traits::{AccountIdConversion, Saturating, Zero, Hash as HashT},
    Permill,
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

        /// Currency type for rewards
        type Currency: Currency<Self::AccountId>;

        /// Maximum providers in a single batch
        #[pallet::constant]
        type MaxProvidersPerBatch: Get<u32>;

        /// Minimum reward amount
        #[pallet::constant]
        type MinimumReward: Get<BalanceOf<Self>>;

        /// The pallet's ID for deriving account
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Settlement period in blocks
        #[pallet::constant]
        type SettlementPeriod: Get<BlockNumberFor<Self>>;

        /// Platform fee percentage
        #[pallet::constant]
        type PlatformFeePercentage: Get<Permill>;
    }

    #[derive(Encode, Decode, DecodeWithMemTracking, Clone, TypeInfo, PartialEq, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct RewardBatch<T: Config> {
        pub task_id: T::Hash,
        pub total_bounty: BalanceOf<T>,
        pub coordinator: T::AccountId,
        pub created_at: BlockNumberFor<T>,
        pub settled: bool,
        pub merkle_root: T::Hash, // For efficient verification
    }

    #[derive(Encode, Decode, DecodeWithMemTracking, Clone, TypeInfo, PartialEq, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct ProviderReward<T: Config> {
        pub provider: T::AccountId,
        pub amount: BalanceOf<T>,
        pub gradients_contributed: u64,
        pub quality_score: u32, // 0-1000
        pub hardware_multiplier: u32, // Stored as u32, divide by 100 for decimal
    }

    impl<T: Config> sp_std::fmt::Debug for ProviderReward<T> {
        fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
            f.debug_struct("ProviderReward")
                .field("gradients_contributed", &self.gradients_contributed)
                .field("quality_score", &self.quality_score)
                .field("hardware_multiplier", &self.hardware_multiplier)
                .finish()
        }
    }

    #[derive(Encode, Decode, DecodeWithMemTracking, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct RewardMetrics {
        pub total_gradients: u64,
        pub avg_quality_score: u32,
        pub participants: u32,
    }

    #[pallet::storage]
    #[pallet::getter(fn reward_batches)]
    pub type RewardBatches<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash, // Batch ID
        RewardBatch<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn batch_rewards)]
    pub type BatchRewards<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::Hash, // Batch ID
        Blake2_128Concat,
        T::AccountId, // Provider
        ProviderReward<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn pending_rewards)]
    pub type PendingRewards<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn total_rewards_distributed)]
    pub type TotalRewardsDistributed<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_settlement_block)]
    pub type LastSettlementBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn platform_fees_collected)]
    pub type PlatformFeesCollected<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Reward batch created [batch_id, task_id, total_bounty]
        BatchCreated {
            batch_id: T::Hash,
            task_id: T::Hash,
            total_bounty: BalanceOf<T>,
        },

        /// Rewards distributed [batch_id, provider_count, total_amount]
        RewardsDistributed {
            batch_id: T::Hash,
            provider_count: u32,
            total_amount: BalanceOf<T>,
        },

        /// Individual reward allocated [provider, amount]
        RewardAllocated {
            provider: T::AccountId,
            amount: BalanceOf<T>,
        },

        /// Rewards claimed [provider, amount]
        RewardsClaimed {
            provider: T::AccountId,
            amount: BalanceOf<T>,
        },

        /// Settlement executed [block_number, batches_settled]
        SettlementExecuted {
            block_number: BlockNumberFor<T>,
            batches_settled: u32,
        },

        /// Platform fee collected [amount]
        PlatformFeeCollected {
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    #[derive(PartialEq)]
    pub enum Error<T> {
        /// Batch not found
        BatchNotFound,
        /// Batch already settled
        BatchAlreadySettled,
        /// No rewards to claim
        NoRewardsToClaim,
        /// Too many providers in batch
        TooManyProviders,
        /// Invalid reward amount
        InvalidRewardAmount,
        /// Insufficient escrow balance
        InsufficientEscrowBalance,
        /// Not authorized
        NotAuthorized,
        /// Batch already exists
        BatchAlreadyExists,
        /// Provider not in batch
        ProviderNotInBatch,
        /// Invalid quality score
        InvalidQualityScore,
        /// Settlement too early
        SettlementTooEarly,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a reward batch for a completed task
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_batch(
            origin: OriginFor<T>,
            task_id: T::Hash,
            total_bounty: BalanceOf<T>,
            merkle_root: T::Hash,
        ) -> DispatchResult {
            let coordinator = ensure_signed(origin)?;

            // Generate batch ID
            let batch_id = T::Hashing::hash_of(&(task_id, coordinator.clone(), frame_system::Pallet::<T>::block_number()));

            ensure!(!RewardBatches::<T>::contains_key(&batch_id), Error::<T>::BatchAlreadyExists);

            // Create batch
            let batch = RewardBatch {
                task_id,
                total_bounty,
                coordinator,
                created_at: frame_system::Pallet::<T>::block_number(),
                settled: false,
                merkle_root,
            };

            RewardBatches::<T>::insert(&batch_id, &batch);

            Self::deposit_event(Event::BatchCreated {
                batch_id,
                task_id,
                total_bounty,
            });

            Ok(())
        }

        /// Submit provider rewards for a batch
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(15_000, 0))]
        pub fn submit_rewards(
            origin: OriginFor<T>,
            batch_id: T::Hash,
            rewards: Vec<ProviderReward<T>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let batch = RewardBatches::<T>::get(&batch_id).ok_or(Error::<T>::BatchNotFound)?;

            ensure!(!batch.settled, Error::<T>::BatchAlreadySettled);
            ensure!(batch.coordinator == who, Error::<T>::NotAuthorized);
            ensure!(rewards.len() <= T::MaxProvidersPerBatch::get() as usize, Error::<T>::TooManyProviders);

            // Calculate total rewards
            let mut total_rewards: BalanceOf<T> = Zero::zero();

            for reward in rewards.iter() {
                ensure!(reward.amount >= T::MinimumReward::get(), Error::<T>::InvalidRewardAmount);
                ensure!(reward.quality_score <= 1000, Error::<T>::InvalidQualityScore);
                total_rewards = total_rewards.saturating_add(reward.amount);

                // Store individual rewards
                BatchRewards::<T>::insert(&batch_id, &reward.provider, reward.clone());

                // Add to pending rewards
                PendingRewards::<T>::mutate(&reward.provider, |pending| {
                    *pending = pending.saturating_add(reward.amount);
                });

                Self::deposit_event(Event::RewardAllocated {
                    provider: reward.provider.clone(),
                    amount: reward.amount,
                });
            }

            // Ensure total doesn't exceed bounty
            ensure!(total_rewards <= batch.total_bounty, Error::<T>::InvalidRewardAmount);

            Self::deposit_event(Event::RewardsDistributed {
                batch_id,
                provider_count: rewards.len() as u32,
                total_amount: total_rewards,
            });

            Ok(())
        }

        /// Settle a batch and transfer rewards
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(20_000, 0))]
        pub fn settle_batch(
            origin: OriginFor<T>,
            batch_id: T::Hash,
        ) -> DispatchResult {
            ensure_root(origin)?;

            RewardBatches::<T>::try_mutate(&batch_id, |maybe_batch| {
                let batch = maybe_batch.as_mut().ok_or(Error::<T>::BatchNotFound)?;

                ensure!(!batch.settled, Error::<T>::BatchAlreadySettled);

                // Check if settlement period has passed
                let current_block = frame_system::Pallet::<T>::block_number();
                let settlement_due = batch.created_at.saturating_add(T::SettlementPeriod::get());
                ensure!(current_block >= settlement_due, Error::<T>::SettlementTooEarly);

                batch.settled = true;

                Ok::<(), DispatchError>(())
            })?;

            // Process all rewards in batch
            let mut settled_count = 0u32;
            let mut total_settled: BalanceOf<T> = Zero::zero();
            let mut total_fee: BalanceOf<T> = Zero::zero();

            for (provider, reward) in BatchRewards::<T>::iter_prefix(&batch_id) {
                // Calculate platform fee
                let fee = T::PlatformFeePercentage::get() * reward.amount;
                let net_reward = reward.amount.saturating_sub(fee);
                total_fee = total_fee.saturating_add(fee);

                // Transfer from escrow to provider
                let escrow_account = Self::account_id();
                T::Currency::transfer(
                    &escrow_account,
                    &provider,
                    net_reward,
                    ExistenceRequirement::KeepAlive,
                )?;

                // Update platform fees
                PlatformFeesCollected::<T>::mutate(|total| *total = total.saturating_add(fee));

                // Update statistics
                TotalRewardsDistributed::<T>::mutate(|total| *total = total.saturating_add(net_reward));

                // Clear pending reward
                PendingRewards::<T>::mutate(&provider, |pending| {
                    *pending = pending.saturating_sub(reward.amount);
                });

                settled_count += 1;
                total_settled = total_settled.saturating_add(net_reward);
            }

            // Update last settlement block
            LastSettlementBlock::<T>::put(frame_system::Pallet::<T>::block_number());

            Self::deposit_event(Event::SettlementExecuted {
                block_number: frame_system::Pallet::<T>::block_number(),
                batches_settled: settled_count,
            });

            if !total_fee.is_zero() {
                Self::deposit_event(Event::PlatformFeeCollected {
                    amount: total_fee,
                });
            }

            Ok(())
        }

        /// Claim accumulated rewards
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(8_000, 0))]
        pub fn claim_rewards(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            let pending = PendingRewards::<T>::get(&provider);
            ensure!(!pending.is_zero(), Error::<T>::NoRewardsToClaim);

            // Transfer from escrow to provider
            let escrow_account = Self::account_id();
            T::Currency::transfer(
                &escrow_account,
                &provider,
                pending,
                ExistenceRequirement::KeepAlive,
            )?;

            // Clear pending rewards
            PendingRewards::<T>::remove(&provider);

            // Update total distributed
            TotalRewardsDistributed::<T>::mutate(|total| *total = total.saturating_add(pending));

            Self::deposit_event(Event::RewardsClaimed {
                provider,
                amount: pending,
            });

            Ok(())
        }

        /// Execute periodic settlement (can be called by anyone)
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(25_000, 0))]
        pub fn periodic_settlement(
            origin: OriginFor<T>,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let current_block = frame_system::Pallet::<T>::block_number();
            let last_settlement = LastSettlementBlock::<T>::get();
            let next_settlement = last_settlement.saturating_add(T::SettlementPeriod::get());

            ensure!(current_block >= next_settlement, Error::<T>::SettlementTooEarly);

            let mut batches_settled = 0u32;

            // Settle all eligible batches
            for (batch_id, mut batch) in RewardBatches::<T>::iter() {
                if !batch.settled {
                    let settlement_due = batch.created_at.saturating_add(T::SettlementPeriod::get());
                    if current_block >= settlement_due {
                        batch.settled = true;
                        RewardBatches::<T>::insert(&batch_id, &batch);
                        batches_settled += 1;

                        // Process rewards for this batch
                        for (provider, reward) in BatchRewards::<T>::iter_prefix(&batch_id) {
                            let fee = T::PlatformFeePercentage::get() * reward.amount;
                            let net_reward = reward.amount.saturating_sub(fee);

                            // Transfer rewards
                            let escrow_account = Self::account_id();
                            let _ = T::Currency::transfer(
                                &escrow_account,
                                &provider,
                                net_reward,
                                ExistenceRequirement::KeepAlive,
                            );

                            PendingRewards::<T>::mutate(&provider, |pending| {
                                *pending = pending.saturating_sub(reward.amount);
                            });
                        }
                    }
                }
            }

            LastSettlementBlock::<T>::put(current_block);

            Self::deposit_event(Event::SettlementExecuted {
                block_number: current_block,
                batches_settled,
            });

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Get the account ID of the pallet's escrow account
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        /// Calculate reward for a provider based on contribution
        pub fn calculate_reward(
            base_bounty: BalanceOf<T>,
            gradients: u64,
            total_gradients: u64,
            quality_score: u32,
            hardware_multiplier: u32,
        ) -> BalanceOf<T> {
            if total_gradients == 0 {
                return Zero::zero();
            }

            // Base reward proportional to contribution
            let contribution_ratio = Permill::from_rational(gradients, total_gradients);
            let base_reward = contribution_ratio * base_bounty;

            // Apply quality multiplier (0-1000 score maps to 0-100%)
            let quality_multiplier = Permill::from_rational(quality_score, 1000u32);
            let quality_adjusted = quality_multiplier * base_reward;

            // Apply hardware multiplier (stored as u32, 100 = 1.0x)
            let hardware_adjusted = quality_adjusted
                .saturating_mul(hardware_multiplier.into())
                / 100u32.into();

            hardware_adjusted
        }
    }
}