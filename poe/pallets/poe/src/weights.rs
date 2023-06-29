
//! Autogenerated weights for pallet_poe
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-29, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro-From-Wilson.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_poe
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// ./pallets/poe/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_poe.
pub trait WeightInfo {
	fn create_claim(d: u32, ) -> Weight;
	fn revoke_claim(d: u32, ) -> Weight;
	fn transfer_claim(d: u32, ) -> Weight;
}

/// Weights for pallet_poe using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: PoeModule Proofs (r:1 w:1)
	/// Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 512]`.
	fn create_claim(_d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `4031`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(45_405_061, 4031)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: PoeModule Proofs (r:1 w:1)
	/// Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 512]`.
	fn revoke_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86 + d * (1 ±0)`
		//  Estimated: `4031`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(47_154_902, 4031)
			// Standard Error: 9_932
			.saturating_add(Weight::from_parts(24_763, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: PoeModule Proofs (r:1 w:1)
	/// Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 512]`.
	fn transfer_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86 + d * (1 ±0)`
		//  Estimated: `4031`
		// Minimum execution time: 32_000_000 picoseconds.
		Weight::from_parts(43_045_673, 4031)
			// Standard Error: 7_498
			.saturating_add(Weight::from_parts(11_984, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: PoeModule Proofs (r:1 w:1)
	/// Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 512]`.
	fn create_claim(_d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `4031`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(45_405_061, 4031)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: PoeModule Proofs (r:1 w:1)
	/// Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 512]`.
	fn revoke_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86 + d * (1 ±0)`
		//  Estimated: `4031`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(47_154_902, 4031)
			// Standard Error: 9_932
			.saturating_add(Weight::from_parts(24_763, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: PoeModule Proofs (r:1 w:1)
	/// Proof: PoeModule Proofs (max_values: None, max_size: Some(566), added: 3041, mode: MaxEncodedLen)
	/// The range of component `d` is `[0, 512]`.
	fn transfer_claim(d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86 + d * (1 ±0)`
		//  Estimated: `4031`
		// Minimum execution time: 32_000_000 picoseconds.
		Weight::from_parts(43_045_673, 4031)
			// Standard Error: 7_498
			.saturating_add(Weight::from_parts(11_984, 0).saturating_mul(d.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
