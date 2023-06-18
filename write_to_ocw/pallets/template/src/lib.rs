#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	pub type OffChainDataType = BoundedVec<u8, ConstU32<4>>;

	#[derive(Debug, Encode, Decode, Default)]
    struct OffChainData(OffChainDataType);

	const ONCHAIN_TX_KEY: &[u8] = b"my_pallet::indexing1";

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OffChainDataStored { data: OffChainDataType, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn save_data_to_off_chain(origin: OriginFor<T>, data:OffChainDataType ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let to_store_data = OffChainData(data.clone());
            sp_io::offchain_index::set(&ONCHAIN_TX_KEY, &to_store_data.encode());
            log::info!("data set:{:?}",&to_store_data);
            Self::deposit_event(Event::OffChainDataStored { data, who });
            Ok(())
		}

	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("OCW ==> Hello World from offchain workers!: {:?}", block_number);

            if let Some(data_stored) =
                sp_runtime::offchain::storage::StorageValue::persistent(ONCHAIN_TX_KEY)
                    .get::<OffChainData>()
                    .unwrap_or_else(|_| {
                        log::info!("OffChainWorker ==> Read Data Failed!");
                        None
                    }) {
                log::info!("OffChainWorker ==> Storage data is :{:?}",data_stored.0)
            }

            log::info!("OCW ==> Leave from offchain workers!: {:?}", block_number);
        }
    }
}

// ./target/release/node-template --enable-offchain-indexing true

