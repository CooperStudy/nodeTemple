#![cfg_attr(not(feature = "std"),no_std)]


///poe cooper


pub use pallet::*;










#[frame_support::pallet]
pub mod pallet{
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
	#[pallet_config]
	pub trait Config: frame_system::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	pub type Proofs<T: Config> = StoreMap<_,Blake2_128Concat,Vec<u8>,(T::AccountId,T::Blocknumber)>;



	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId,Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		///创建存证
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyExist);
			Proofs::<T>::insert(&claim,(sender.clone(),frame_system::Pallet::<T>::block_number()));
			Self::deposit_event(Event::ClaimCreated(sender,claim));
			OK(().into())
		}

		///吊销存证
		#[pallet::weight(0)]
		pub fn revoke_claim(origin:OriginFor<T>,claim: Vec<u8>) -> DispatchResultWithPostInfo{
			let sender = ensure_signed(origin)?;
			let (owner,_) = Proofs::<T>::get(&claim.ok_or(Error::<T>::ClaimNotExist))?;
            ensure!(owner == sender,Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&claim)
		}


		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}


