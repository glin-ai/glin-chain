#![cfg_attr(not(feature = "std"), no_std)]

//! # Testnet Points Pallet
//!
//! Track user activities on testnet for mainnet airdrop calculation.
//! This pallet records various activities and assigns points that will
//! determine the user's share of the mainnet airdrop.

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, Time},
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::{Saturating, Zero};
use sp_std::vec::Vec;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency for potential staking requirements
        type Currency: Currency<Self::AccountId>;

        /// Time provider for timestamps
        type Time: Time;

        /// Maximum activities to store per user
        #[pallet::constant]
        type MaxActivitiesPerUser: Get<u32>;

        /// Multiplier for early participants (in basis points, 10000 = 1x)
        #[pallet::constant]
        type EarlyBirdMultiplier: Get<u32>;

        /// Multiplier for GPU providers (in basis points)
        #[pallet::constant]
        type GPUProviderMultiplier: Get<u32>;

        /// Multiplier for bug reporters (in basis points)
        #[pallet::constant]
        type BugReporterMultiplier: Get<u32>;
    }

    /// Activity types that earn points
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ActivityType {
        /// User claimed from faucet
        FaucetClaim,
        /// Created an ML training task
        TaskCreated,
        /// Successfully completed a task
        TaskCompleted,
        /// Provided GPU for training (per hour)
        GPUProvided,
        /// Performed validation
        ValidationPerformed,
        /// Participated in governance
        GovernanceVoted,
        /// Reported and confirmed bug
        BugReported,
        /// Successfully referred new user
        ReferralSuccess,
        /// Provided liquidity (if DEX enabled)
        LiquidityProvided,
        /// Participated in testing specific features
        FeatureTested,
    }

    /// Record of a single activity
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ActivityRecord<Moment> {
        pub activity_type: ActivityType,
        pub points: u32,
        pub timestamp: Moment,
        pub metadata: BoundedVec<u8, ConstU32<256>>, // Additional data if needed
    }

    /// User statistics for airdrop calculation
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct UserStats<Moment> {
        pub total_points: u32,
        pub tasks_created: u32,
        pub tasks_completed: u32,
        pub gpu_hours_provided: u32,
        pub governance_participations: u32,
        pub bugs_reported: u32,
        pub referrals: u32,
        pub first_activity: Option<Moment>,
        pub last_activity: Option<Moment>,
        pub is_verified: bool,
    }

    /// Storage: User points balance
    #[pallet::storage]
    #[pallet::getter(fn user_points)]
    pub type UserPoints<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        u32,
        ValueQuery,
    >;

    /// Storage: User activity records
    #[pallet::storage]
    #[pallet::getter(fn user_activities)]
    pub type UserActivities<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<ActivityRecord<T::Moment>, T::MaxActivitiesPerUser>,
        ValueQuery,
    >;

    /// Storage: User statistics
    #[pallet::storage]
    #[pallet::getter(fn user_stats)]
    pub type UserStats<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        UserStats<T::Moment>,
        ValueQuery,
    >;

    /// Storage: Total points distributed
    #[pallet::storage]
    #[pallet::getter(fn total_points)]
    pub type TotalPoints<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Storage: Number of unique participants
    #[pallet::storage]
    #[pallet::getter(fn participant_count)]
    pub type ParticipantCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Storage: Referral relationships
    #[pallet::storage]
    #[pallet::getter(fn referrer)]
    pub type Referrer<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        T::AccountId,
    >;

    /// Storage: Verified users (Twitter/GitHub/Discord verified)
    #[pallet::storage]
    #[pallet::getter(fn verified_users)]
    pub type VerifiedUsers<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery,
    >;

    /// Storage: Early bird cutoff timestamp
    #[pallet::storage]
    #[pallet::getter(fn early_bird_cutoff)]
    pub type EarlyBirdCutoff<T: Config> = StorageValue<_, T::Moment>;

    /// Storage: Leaderboard (top users by points)
    #[pallet::storage]
    #[pallet::getter(fn leaderboard)]
    pub type Leaderboard<T: Config> = StorageValue<
        _,
        BoundedVec<(T::AccountId, u32), ConstU32<100>>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Points earned by user
        PointsEarned {
            user: T::AccountId,
            activity: ActivityType,
            points: u32,
        },
        /// User verified their account
        UserVerified {
            user: T::AccountId,
        },
        /// Referral recorded
        ReferralRecorded {
            referrer: T::AccountId,
            referred: T::AccountId,
        },
        /// Milestone reached
        MilestoneReached {
            user: T::AccountId,
            milestone: u32,
        },
        /// Early bird period ended
        EarlyBirdEnded {
            timestamp: T::Moment,
        },
        /// Leaderboard updated
        LeaderboardUpdated,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// User already claimed faucet
        AlreadyClaimedFaucet,
        /// User already verified
        AlreadyVerified,
        /// Invalid referral (self-referral or circular)
        InvalidReferral,
        /// Maximum activities reached for user
        TooManyActivities,
        /// Activity not allowed
        ActivityNotAllowed,
        /// User not verified for this action
        NotVerified,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Record points for an activity (called by other pallets or sudo)
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn record_activity(
            origin: OriginFor<T>,
            user: T::AccountId,
            activity: ActivityType,
            metadata: Vec<u8>,
        ) -> DispatchResult {
            // This should be called by other pallets or governance
            ensure_root(origin)?;

            let points = Self::calculate_points(&activity, &user);
            let now = T::Time::now();

            // Update user activities
            UserActivities::<T>::try_mutate(&user, |activities| {
                let activity_record = ActivityRecord {
                    activity_type: activity.clone(),
                    points,
                    timestamp: now,
                    metadata: BoundedVec::try_from(metadata).map_err(|_| Error::<T>::TooManyActivities)?,
                };

                activities.try_push(activity_record).map_err(|_| Error::<T>::TooManyActivities)?;
                Ok::<(), Error<T>>(())
            })?;

            // Update user stats
            UserStats::<T>::mutate(&user, |stats| {
                stats.total_points = stats.total_points.saturating_add(points);

                match activity {
                    ActivityType::TaskCreated => stats.tasks_created = stats.tasks_created.saturating_add(1),
                    ActivityType::TaskCompleted => stats.tasks_completed = stats.tasks_completed.saturating_add(1),
                    ActivityType::GPUProvided => stats.gpu_hours_provided = stats.gpu_hours_provided.saturating_add(1),
                    ActivityType::GovernanceVoted => stats.governance_participations = stats.governance_participations.saturating_add(1),
                    ActivityType::BugReported => stats.bugs_reported = stats.bugs_reported.saturating_add(1),
                    ActivityType::ReferralSuccess => stats.referrals = stats.referrals.saturating_add(1),
                    _ => {},
                }

                if stats.first_activity.is_none() {
                    stats.first_activity = Some(now);
                    ParticipantCount::<T>::mutate(|count| *count = count.saturating_add(1));
                }
                stats.last_activity = Some(now);
            });

            // Update user points
            UserPoints::<T>::mutate(&user, |user_points| {
                *user_points = user_points.saturating_add(points);
            });

            // Update total points
            TotalPoints::<T>::mutate(|total| *total = total.saturating_add(points));

            // Update leaderboard if needed
            Self::update_leaderboard(user.clone(), UserPoints::<T>::get(&user));

            Self::deposit_event(Event::PointsEarned {
                user,
                activity,
                points,
            });

            Ok(())
        }

        /// Verify a user account (admin only for now)
        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn verify_user(
            origin: OriginFor<T>,
            user: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            ensure!(!VerifiedUsers::<T>::get(&user), Error::<T>::AlreadyVerified);

            VerifiedUsers::<T>::insert(&user, true);
            UserStats::<T>::mutate(&user, |stats| {
                stats.is_verified = true;
            });

            Self::deposit_event(Event::UserVerified { user });
            Ok(())
        }

        /// Record a referral
        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn record_referral(
            origin: OriginFor<T>,
            referred: T::AccountId,
            referrer: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Prevent self-referral
            ensure!(referred != referrer, Error::<T>::InvalidReferral);

            // Check if already has referrer
            ensure!(!Referrer::<T>::contains_key(&referred), Error::<T>::InvalidReferral);

            Referrer::<T>::insert(&referred, &referrer);

            // Give points to referrer
            Self::record_activity(
                frame_system::RawOrigin::Root.into(),
                referrer.clone(),
                ActivityType::ReferralSuccess,
                Vec::new(),
            )?;

            Self::deposit_event(Event::ReferralRecorded { referrer, referred });
            Ok(())
        }

        /// Set early bird cutoff (admin only)
        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn set_early_bird_cutoff(
            origin: OriginFor<T>,
            timestamp: T::Moment,
        ) -> DispatchResult {
            ensure_root(origin)?;

            EarlyBirdCutoff::<T>::put(timestamp);

            Self::deposit_event(Event::EarlyBirdEnded { timestamp });
            Ok(())
        }

        /// Claim faucet (can be called by faucet service)
        #[pallet::weight(10_000)]
        #[pallet::call_index(4)]
        pub fn claim_faucet(
            origin: OriginFor<T>,
            user: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Check if already claimed
            let stats = UserStats::<T>::get(&user);
            let activities = UserActivities::<T>::get(&user);

            // Check if user already has faucet claim
            let has_faucet_claim = activities.iter().any(|a| a.activity_type == ActivityType::FaucetClaim);
            ensure!(!has_faucet_claim, Error::<T>::AlreadyClaimedFaucet);

            // Record faucet claim
            Self::record_activity(
                frame_system::RawOrigin::Root.into(),
                user,
                ActivityType::FaucetClaim,
                Vec::new(),
            )?;

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// Calculate points for an activity
        fn calculate_points(activity: &ActivityType, user: &T::AccountId) -> u32 {
            let base_points = match activity {
                ActivityType::FaucetClaim => 10,
                ActivityType::TaskCreated => 100,
                ActivityType::TaskCompleted => 500,
                ActivityType::GPUProvided => 50,
                ActivityType::ValidationPerformed => 200,
                ActivityType::GovernanceVoted => 50,
                ActivityType::BugReported => 1000,
                ActivityType::ReferralSuccess => 100,
                ActivityType::LiquidityProvided => 300,
                ActivityType::FeatureTested => 150,
            };

            // Apply multipliers
            let mut multiplier = 10000u32; // Base 1x in basis points

            // Early bird bonus
            if let Some(cutoff) = EarlyBirdCutoff::<T>::get() {
                let stats = UserStats::<T>::get(user);
                if let Some(first_activity) = stats.first_activity {
                    if first_activity < cutoff {
                        multiplier = multiplier.saturating_add(T::EarlyBirdMultiplier::get());
                    }
                }
            }

            // GPU provider bonus (for any activity if they're a GPU provider)
            let stats = UserStats::<T>::get(user);
            if stats.gpu_hours_provided > 0 {
                multiplier = multiplier.saturating_add(T::GPUProviderMultiplier::get());
            }

            // Bug reporter bonus
            if stats.bugs_reported > 0 {
                multiplier = multiplier.saturating_add(T::BugReporterMultiplier::get());
            }

            // Apply multiplier (divide by 10000 to get actual multiplier)
            base_points.saturating_mul(multiplier) / 10000
        }

        /// Update the leaderboard
        fn update_leaderboard(user: T::AccountId, points: u32) {
            Leaderboard::<T>::mutate(|board| {
                // Check if user is already in leaderboard
                if let Some(pos) = board.iter().position(|(u, _)| u == &user) {
                    board[pos].1 = points;
                } else if board.len() < 100 {
                    // Add to leaderboard if space
                    let _ = board.try_push((user, points));
                } else {
                    // Replace lowest score if higher
                    if let Some(min_pos) = board.iter().position(|(_, p)| *p < points) {
                        board[min_pos] = (user, points);
                    }
                }

                // Sort leaderboard
                board.sort_by(|a, b| b.1.cmp(&a.1));
            });
        }
    }
}