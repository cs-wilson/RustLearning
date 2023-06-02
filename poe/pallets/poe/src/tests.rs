use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use crate::pallet::Config;


#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
            .map_err(|_| Error::<Test>::ProofTooLong)
            .unwrap_or_else(|e| panic!("unexpected error: {:?}", e));

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        assert_eq!(
            Proofs::<Test>::get(&bounded_claim),
            Some((1, frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn create_claim_failed_when_claim_already_existed() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
			.map_err(|_| Error::<Test>::ProofTooLong)
			.unwrap_or_else(|e| panic!("unexpected error: {:?}", e));

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
			.map_err(|_| Error::<Test>::ProofTooLong)
			.unwrap_or_else(|e| panic!("unexpected error: {:?}", e));

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_eq!(Proofs::<Test>::get(&bounded_claim), None);
	});
}

#[test]
fn revoke_claim_failed_when_claim_not_existed() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn revoke_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	});
}

#[test]
fn revoke_claim_failed_when_claim_is_revoked() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone())
			.map_err(|_| Error::<Test>::ProofTooLong)
			.unwrap_or_else(|e| panic!("unexpected error: {:?}", e));

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2));

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn transfer_claim_failed_when_claim_not_existed() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	});
}

#[test]
fn transfer_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	});
}

#[test]
fn transfer_claim_failed_when_new_owner_is_same_as_old_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 1),
			Error::<Test>::NewOwnerIsSameAsOldOwner
		);
	});
}

#[test]
fn transfer_claim_failed_when_claim_is_revoked() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	});
}
