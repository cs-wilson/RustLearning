use frame_support::{
	pallet_prelude::*
};

pub const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

pub type KittyName = [u8; 8];

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct Kitty {
	pub dna: [u8; 16],
	pub name: KittyName,
}
