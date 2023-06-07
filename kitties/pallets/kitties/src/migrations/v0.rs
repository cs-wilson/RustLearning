use frame_support::{
	pallet_prelude::*
};

pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);


#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct Kitty(pub [u8; 16]);
