use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(vec![0u8; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller.clone()), claim.to_vec())
	verify {
		assert_eq!(Proofs::<T>::get(&claim), Some((caller, frame_system::Pallet::<T>::block_number())));
	}

	revoke_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(vec![0u8; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		Proofs::<T>::insert(&claim, (caller.clone(), frame_system::Pallet::<T>::block_number()));
	}: _(RawOrigin::Signed(caller.clone()), claim.to_vec())
	verify {
		assert_eq!(Proofs::<T>::get(&claim), None);
	}

	transfer_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(vec![0u8; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let dest: T::AccountId = whitelisted_caller();
		Proofs::<T>::insert(&claim, (caller.clone(), frame_system::Pallet::<T>::block_number()));
	}: _(RawOrigin::Signed(caller.clone()), claim.to_vec(), dest.clone())
	verify {
		assert_eq!(Proofs::<T>::get(&claim), Some((dest, frame_system::Pallet::<T>::block_number())));
	}

	impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);

}
