use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, pallet_prelude::DispatchResultWithPostInfo};

fn init_balance(account: AccountId, balance: Balance) -> DispatchResultWithPostInfo {
	Balances::force_set_balance(RuntimeOrigin::root(), account, balance)
}

#[test]
//test create kitty
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		//deposit to account_id
		assert_ok!(init_balance(account_id, 1000_000));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		let kitty = KittiesModule::kitties(kitty_id).expect("Kitty Created");
		System::assert_last_event(Event::KittyCreated{
			who: account_id,
			kitty_id: kitty_id,
			kitty: kitty,
		}.into());

		crate::NextKittyId::<Test>::set(crate::KittyIndex::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
//test breed kitty
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		//deposit to account_id
		assert_ok!(init_balance(account_id, 1000_000));

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittiesModule::breed(
			RuntimeOrigin::signed(account_id),
			kitty_id,
			kitty_id + 1
		));

		let breed_kitty_id = kitty_id + 2;
		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));

		assert_eq!(
			KittiesModule::kitty_parents(breed_kitty_id),
			Some((kitty_id, kitty_id + 1))
		);

		let breed_kitty = KittiesModule::kitties(breed_kitty_id).expect("Breed Kitty Created");
		System::assert_last_event(
			Event::KittyBred{
				who: account_id,
				kitty_id: breed_kitty_id,
				kitty: breed_kitty,
			}.into()
		);


	});
}


#[test]
//test transfer kitty
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient_id = 2;

		assert_ok!(init_balance(account_id, 1000_000));

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(
				RuntimeOrigin::signed(recipient_id),
				account_id,
				kitty_id
			),
			Error::<Test>::NotOwner
		);

		assert_noop!(
			KittiesModule::transfer(
				RuntimeOrigin::signed(account_id),
				account_id,
				kitty_id),
			Error::<Test>::TransferToSelf
		);

		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(account_id),
			recipient_id,
			kitty_id
		));

		System::assert_last_event(
			Event::KittyTransferred{
				who: account_id,
				recipient: recipient_id,
				kitty_id: kitty_id,
			}.into()
		);

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient_id));

		assert_ok!(KittiesModule::transfer(
			RuntimeOrigin::signed(recipient_id),
			account_id,
			kitty_id
		));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		System::assert_last_event(
			Event::KittyTransferred{
				who: recipient_id,
				recipient: account_id,
				kitty_id: kitty_id,
			}.into()
		);


	});
}

#[test]
//test sale kitty
fn it_works_for_sale(){
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_ok!(init_balance(account_id, 1000_000));

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_on_sale(kitty_id), None);

		assert_ok!(KittiesModule::sale(
			RuntimeOrigin::signed(account_id),
			kitty_id
		));

		assert_eq!(KittiesModule::kitty_on_sale(kitty_id), Some(()));

		System::assert_last_event(
			Event::KittyOnSale{
				who: account_id,
				kitty_id: kitty_id,
			}.into()
		);
	});
}

#[test]
//test buy kitty
fn it_works_for_buy(){
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id1 = 1;
		let account_id2 = 2;

		assert_ok!(init_balance(account_id1, 1000_000));

		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id1), kitty_id),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id1)));
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id1), kitty_id),
			Error::<Test>::TransferToSelf
		);

		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id2), kitty_id),
			Error::<Test>::NotOnSale
		);

		assert_ok!(KittiesModule::sale(
			RuntimeOrigin::signed(account_id1),
			kitty_id
		));

		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id2), kitty_id),
			Error::<Test>::NotEnoughBalance
		);
		assert_ok!(init_balance(account_id2, 1000_000));
		assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(account_id2), kitty_id));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id2));

		System::assert_last_event(
			Event::KittyBought {
				 who: (account_id2),
				 owner: (account_id1),
				 kitty_id: (kitty_id)
			}.into()
		);

	});
}


