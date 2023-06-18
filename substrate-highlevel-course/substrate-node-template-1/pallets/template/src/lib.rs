#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_io::offchain_index;
	use sp_runtime::offchain::StorageValueRef;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Debug, Deserialize, Encode, Decode, Default)]
	struct IndexingData(Vec<u8>, u64);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { something: u32, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(100)]
		pub fn extrinsic(origin: OriginFor<T>, number: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let key = Self::derived_key(frame_system::Module::<T>::block_number());
			let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
			offchain_index::set(&key, &data.encode());
			Ok(())
		}

		fn derived_key(block_number: T::BlockNumber) -> Vec<u8> {
			block_number.using_encoded(|encoded_bn| {
				ONCHAIN_TX_KEY.clone().into_iter()
					.chain(b"/".into_iter())
					.chain(encoded_bn)
					.copied()
					.collect::<Vec<u8>>()
			})
		}

	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("OCW ==> Hello World from offchain workers!: {:?}", block_number);
			let key = Self::derived_key(block_number);
			let storage_ref = StorageValueRef::persistent(&key);

			if let Ok(Some(data)) = storage_ref.get::<IndexingData>() {
				debug::info!("local storage data: {:?}, {:?}",
					str::from_utf8(&data.0).unwrap_or("error"), data.1);
			} else {
				debug::info!("Error reading from local storage.");
			}
        }
    }
}
