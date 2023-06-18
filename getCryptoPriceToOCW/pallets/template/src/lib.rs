#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_system::{
	offchain::{
		AppCrypto, CreateSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes,
	},
};
use sp_runtime::{
    transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
	RuntimeDebug,
	offchain::{
        http, Duration,
    },
};
use codec::{Decode, Encode};

use serde::{Deserialize, Deserializer};

use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"btc!");
pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// #[derive(Deserialize, Encode, Decode)]
    // struct GithubInfo {
    //     #[serde(deserialize_with = "de_string_to_bytes")]
    //     login: Vec<u8>,
    //     #[serde(deserialize_with = "de_string_to_bytes")]
    //     blog: Vec<u8>,
    //     public_repos: u32,
    // }

	#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, scale_info::TypeInfo)]
	struct BTCInfo {
		mins: u8,
		#[serde(deserialize_with = "de_string_to_bytes")]
		price: Vec<u8>,
	}


    pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
        where
        D: Deserializer<'de>,
        {
            let s: &str = Deserialize::deserialize(de)?;
            Ok(s.as_bytes().to_vec())
        }

    // use core::{convert::TryInto, fmt};
    // impl fmt::Debug for GithubInfo {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         write!(
    //             f,
    //             "{{ login: {}, blog: {}, public_repos: {} }}",
    //             sp_std::str::from_utf8(&self.login).map_err(|_| fmt::Error)?,
    //             sp_std::str::from_utf8(&self.blog).map_err(|_| fmt::Error)?,
    //             &self.public_repos
    //             )
    //     }
    // }

	use core::{convert::TryInto, fmt};
	impl fmt::Debug for BTCInfo {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(
				f,
				"{{ mins: {}, price: {} }}",
				&self.mins,
				sp_std::str::from_utf8(&self.price).map_err(|_| fmt::Error)?,
			)
		}
	}


	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct Payload<Public> {
		btc_price: Vec<u8>,
		public: Public,
	}

	impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	// pub trait Config: frame_system::Config + frame_system::offchain::SendTransactionTypes<Call<Self>> {
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
    #[pallet::generate_deposit(pub (super))]
    pub enum Event<T: Config> {
    }

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn unsigned_extrinsic_with_signed_payload(origin: OriginFor<T>, payload: Payload<T::Public>, _signature: T::Signature,) -> DispatchResult {
			ensure_none(origin)?;

            log::info!("OCW ==> in call unsigned_extrinsic_with_signed_payload: {:?}", payload.btc_price);
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	}

	#[pallet::validate_unsigned]
		impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			const UNSIGNED_TXS_PRIORITY: u64 = 100;
			let valid_tx = |provide| ValidTransaction::with_tag_prefix("my-pallet")
				.priority(UNSIGNED_TXS_PRIORITY) // please define `UNSIGNED_TXS_PRIORITY` before this line
				.and_provides([&provide])
				.longevity(3)
				.propagate(true)
				.build();

			match call {
				Call::unsigned_extrinsic_with_signed_payload {
					ref payload,
					ref signature
				} => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into();
					}
					valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Offchain worker entry point.
		fn offchain_worker(block_number: T::BlockNumber) {


			log::info!("OCW ==> Hello World from offchain workers!: {:?}", block_number);

            if let Ok(info) = Self::fetch_github_info() {
                log::info!("OCW ==> BTC Info: {:?}", info.price);

				// let number: u64 = 42;
				let signer = Signer::<T, T::AuthorityId>::any_account();

				if let Some((_, res)) = signer.send_unsigned_transaction(
					// this line is to prepare and return payload
					|acct| Payload { btc_price: info.price.clone(), public: acct.public.clone() },
					|payload, signature| Call::unsigned_extrinsic_with_signed_payload { payload, signature },
				) {
					match res {
						Ok(()) => {log::info!("OCW ==> unsigned tx with signed payload successfully sent.");}
						Err(()) => {log::error!("OCW ==> sending unsigned tx with signed payload failed.");}
					};
				} else {
					// The case of `None`: no account is available for sending
					log::error!("OCW ==> No local account available");
				}
            } else {
                log::info!("OCW ==> Error while fetch github info!");
            }
         	log::info!("OCW ==> Leave from offchain workers!: {:?}", block_number);

		}
	}

	impl<T: Config> Pallet<T> {
        fn fetch_github_info() -> Result<BTCInfo, http::Error> {
            // prepare for send request
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
            let request =
                http::Request::get("https://data.binance.com/api/v3/avgPrice?symbol=BTCUSDT");
            let pending = request
                .add_header("User-Agent", "Substrate-Offchain-Worker")
                .deadline(deadline).send().map_err(|_| http::Error::IoError)?;
            let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
            if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown)
            }
            let body = response.body().collect::<Vec<u8>>();
            let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
                log::warn!("No UTF8 body");
                http::Error::Unknown
            })?;

            // parse the response str
            let btc_info: BTCInfo =
                serde_json::from_str(body_str).map_err(|_| http::Error::Unknown)?;

            Ok(btc_info)
        }

    }
}
