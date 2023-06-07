pub use v2 as current_version;
pub mod v2;

use frame_support::{
	pallet_prelude::*,
	traits::GetStorageVersion,
	weights::Weight,
	StoragePrefixedMap
};

use crate::{Config, Kitties, Pallet, KittyIndex};
use frame_support::{migration::storage_key_iter, Blake2_128Concat};

mod v0;
mod v1;

pub fn select_migration<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version: StorageVersion = Pallet::<T>::current_storage_version();

	// if on_chain_version == 0 && current_version == 1 {
	// 	return v0_migrate_to_v1::<T>();
	// }

	if on_chain_version == 0 && current_version == 2 {
		return v0_migrate_to_v2::<T>();
	}

	if on_chain_version == 1 && current_version == 2 {
		return v1_migrate_to_v2::<T>();
	}

	Weight::zero()
}


// pub fn v0_migrate_to_v1<T: Config>() -> Weight {

// 	let module = Kitties::<T>::module_prefix();
// 	let item = Kitties::<T>::storage_prefix();

// 	for (index, kitty) in storage_key_iter::<KittyIndex, v0::Kitty, Blake2_128Concat>(module, item).drain() {
// 		let  new_kitty = v1::Kitty {
// 			dna: kitty.0,
// 			name: *b"v0v1",
// 		};
// 		Kitties::<T>::insert(index, new_kitty);
// 	}

// 	Weight::zero()

// }

pub fn v0_migrate_to_v2<T: Config>() -> Weight {

	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in storage_key_iter::<KittyIndex, v0::Kitty, Blake2_128Concat>(module, item).drain() {
		let  new_kitty = v2::Kitty {
			dna: kitty.0,
			name: *b"v0_to_v2",
		};
		Kitties::<T>::insert(index, new_kitty);
	}

	Weight::zero()

}


pub fn v1_migrate_to_v2<T: Config>() -> Weight {

	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in storage_key_iter::<KittyIndex, v1::Kitty, Blake2_128Concat>(module, item).drain() {
		let  new_kitty = v2::Kitty {
			dna: kitty.dna,
			name: v1_to_v2_name_concat(kitty.name, *b"tov2")
		};
		Kitties::<T>::insert(index, new_kitty);
	}

	Weight::zero()

}

fn v1_to_v2_name_concat(name: [u8; 4], concat: [u8; 4]) -> [u8; 8] {
	let mut new_name = [0u8; 8];
	new_name[0..4].copy_from_slice(&name);
	new_name[4..8].copy_from_slice(&concat);
	new_name
}


