#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency, ExistenceRequirement},
    PalletId, BoundedVec,
};
use frame_system::pallet_prelude::*;
use scale_info::prelude::vec::Vec;
use sp_runtime::{traits::{AccountIdConversion, Hash}, DispatchError};

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

        /// Currency type for token operations
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Minimum bounty for a task
        #[pallet::constant]
        type MinimumBounty: Get<BalanceOf<Self>>;

        /// Maximum providers per task
        #[pallet::constant]
        type MaxProvidersPerTask: Get<u32>;

        /// The pallet's ID, used for deriving its sovereign account
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum TaskStatus {
        Pending,
        Recruiting,
        Running,
        Validating,
        Completed,
        Failed,
        Cancelled,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ModelType {
        ResNet,
        Bert,
        Gpt,
        Custom,
        LoraFineTune,
    }

    #[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Task<T: Config> {
        pub creator: T::AccountId,
        pub name: BoundedVec<u8, ConstU32<255>>,
        pub model_type: ModelType,
        pub bounty: BalanceOf<T>,
        pub min_providers: u32,
        pub max_providers: u32,
        pub status: TaskStatus,
        pub created_at: BlockNumberFor<T>,
        pub completed_at: Option<BlockNumberFor<T>>,
        pub ipfs_hash: BoundedVec<u8, ConstU32<64>>, // Model/dataset IPFS hash
        pub hardware_requirements: HardwareRequirements,
    }

    #[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo, Default, PartialEq, MaxEncodedLen)]
    pub struct HardwareRequirements {
        pub min_vram_gb: u32,
        pub min_compute_capability: u32, // Stored as u32, divide by 10 for float
        pub min_bandwidth_mbps: u32,
    }

    #[pallet::storage]
    #[pallet::getter(fn tasks)]
    pub type Tasks<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Task<T>>;

    #[pallet::storage]
    #[pallet::getter(fn task_count)]
    pub type TaskCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn task_providers)]
    pub type TaskProviders<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::Hash, // Task ID
        Blake2_128Concat,
        T::AccountId, // Provider
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Task created [task_id, creator, bounty]
        TaskCreated {
            task_id: T::Hash,
            creator: T::AccountId,
            bounty: BalanceOf<T>
        },

        /// Task funded [task_id, amount]
        TaskFunded {
            task_id: T::Hash,
            amount: BalanceOf<T>
        },

        /// Task started [task_id]
        TaskStarted {
            task_id: T::Hash
        },

        /// Task completed [task_id]
        TaskCompleted {
            task_id: T::Hash
        },

        /// Task cancelled [task_id, refunded_amount]
        TaskCancelled {
            task_id: T::Hash,
            refunded_amount: BalanceOf<T>
        },

        /// Provider joined task [task_id, provider]
        ProviderJoined {
            task_id: T::Hash,
            provider: T::AccountId
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Task not found
        TaskNotFound,
        /// Bounty too low
        BountyTooLow,
        /// Invalid hardware requirements
        InvalidHardwareRequirements,
        /// Task already exists
        TaskAlreadyExists,
        /// Task not in correct status
        InvalidTaskStatus,
        /// Too many providers
        TooManyProviders,
        /// Provider already joined
        ProviderAlreadyJoined,
        /// Not task creator
        NotTaskCreator,
        /// Task already started
        TaskAlreadyStarted,
        /// Insufficient balance
        InsufficientBalance,
        /// Name too long
        NameTooLong,
        /// IPFS hash too long
        IpfsHashTooLong,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new federated learning task
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_task(
            origin: OriginFor<T>,
            name: Vec<u8>,
            model_type: ModelType,
            bounty: BalanceOf<T>,
            min_providers: u32,
            max_providers: u32,
            ipfs_hash: Vec<u8>,
            hardware_requirements: HardwareRequirements,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;

            // Validate inputs
            ensure!(bounty >= T::MinimumBounty::get(), Error::<T>::BountyTooLow);
            // Length validation is handled by BoundedVec
            ensure!(max_providers <= T::MaxProvidersPerTask::get(), Error::<T>::TooManyProviders);
            ensure!(min_providers > 0 && min_providers <= max_providers,
                Error::<T>::InvalidHardwareRequirements);

            // Reserve the bounty amount
            T::Currency::reserve(&creator, bounty)?;

            // Generate task ID
            let task_count = TaskCount::<T>::get();
            let task_id = T::Hashing::hash_of(&(creator.clone(), task_count));

            // Create task
            let task = Task {
                creator: creator.clone(),
                name: name.try_into().map_err(|_| Error::<T>::NameTooLong)?,
                model_type,
                bounty,
                min_providers,
                max_providers,
                status: TaskStatus::Pending,
                created_at: frame_system::Pallet::<T>::block_number(),
                completed_at: None,
                ipfs_hash: ipfs_hash.try_into().map_err(|_| Error::<T>::IpfsHashTooLong)?,
                hardware_requirements,
            };

            // Store task
            Tasks::<T>::insert(&task_id, &task);
            TaskCount::<T>::mutate(|count| *count = count.saturating_add(1));

            // Emit event
            Self::deposit_event(Event::TaskCreated {
                task_id,
                creator,
                bounty
            });

            Ok(())
        }

        /// Start recruiting providers for a task
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(5_000, 0))]
        pub fn start_recruiting(
            origin: OriginFor<T>,
            task_id: T::Hash,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Tasks::<T>::try_mutate(&task_id, |maybe_task| {
                let task = maybe_task.as_mut().ok_or(Error::<T>::TaskNotFound)?;

                ensure!(task.creator == who, Error::<T>::NotTaskCreator);
                ensure!(task.status == TaskStatus::Pending, Error::<T>::InvalidTaskStatus);

                task.status = TaskStatus::Recruiting;

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::TaskStarted { task_id });

            Ok(())
        }

        /// Cancel a task and refund the bounty
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(8_000, 0))]
        pub fn cancel_task(
            origin: OriginFor<T>,
            task_id: T::Hash,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let task = Tasks::<T>::get(&task_id).ok_or(Error::<T>::TaskNotFound)?;

            ensure!(task.creator == who, Error::<T>::NotTaskCreator);
            ensure!(
                task.status == TaskStatus::Pending || task.status == TaskStatus::Recruiting,
                Error::<T>::TaskAlreadyStarted
            );

            // Unreserve the bounty
            T::Currency::unreserve(&task.creator, task.bounty);

            // Update task status
            Tasks::<T>::mutate(&task_id, |maybe_task| {
                if let Some(task) = maybe_task {
                    task.status = TaskStatus::Cancelled;
                }
            });

            Self::deposit_event(Event::TaskCancelled {
                task_id,
                refunded_amount: task.bounty
            });

            Ok(())
        }

        /// Complete a task and prepare for reward distribution
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn complete_task(
            origin: OriginFor<T>,
            task_id: T::Hash,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Tasks::<T>::try_mutate(&task_id, |maybe_task| {
                let task = maybe_task.as_mut().ok_or(Error::<T>::TaskNotFound)?;

                ensure!(task.creator == who, Error::<T>::NotTaskCreator);
                ensure!(task.status == TaskStatus::Validating, Error::<T>::InvalidTaskStatus);

                task.status = TaskStatus::Completed;
                task.completed_at = Some(frame_system::Pallet::<T>::block_number());

                // Transfer bounty from reserve to escrow account
                let escrow_account = Self::account_id();
                let bounty = task.bounty;
                T::Currency::unreserve(&task.creator, bounty);
                T::Currency::transfer(&task.creator, &escrow_account, bounty, ExistenceRequirement::KeepAlive)?;

                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::TaskCompleted { task_id });

            Ok(())
        }

        /// Join a task as a provider
        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(6_000, 0))]
        pub fn join_task(
            origin: OriginFor<T>,
            task_id: T::Hash,
        ) -> DispatchResult {
            let provider = ensure_signed(origin)?;

            let task = Tasks::<T>::get(&task_id).ok_or(Error::<T>::TaskNotFound)?;

            ensure!(task.status == TaskStatus::Recruiting, Error::<T>::InvalidTaskStatus);
            ensure!(!TaskProviders::<T>::get(&task_id, &provider), Error::<T>::ProviderAlreadyJoined);

            // Count current providers
            let provider_count = TaskProviders::<T>::iter_prefix(&task_id)
                .filter(|(_, joined)| *joined)
                .count() as u32;

            ensure!(provider_count < task.max_providers, Error::<T>::TooManyProviders);

            // Add provider
            TaskProviders::<T>::insert(&task_id, &provider, true);

            // Check if we have enough providers to start
            if provider_count + 1 >= task.min_providers {
                Tasks::<T>::mutate(&task_id, |maybe_task| {
                    if let Some(task) = maybe_task {
                        if task.status == TaskStatus::Recruiting {
                            task.status = TaskStatus::Running;
                        }
                    }
                });
            }

            Self::deposit_event(Event::ProviderJoined { task_id, provider });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get the account ID of the pallet's escrow account
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }
    }
}