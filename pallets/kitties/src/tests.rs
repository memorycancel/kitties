use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::Currency};

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 10000);
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
	});
}

#[test]
fn create_fails_when_token_not_enough() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 100);
		System::set_block_number(1);
		assert_noop!(KittiesModule::create(Origin::signed(1)), Error::<Test>::TokenNotEnough);
	});
}

#[test]
fn create_fails_when_kitties_count_overflow() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 1000000);
		System::set_block_number(1);
		for _ in 0..8 {
			_ = KittiesModule::create(Origin::signed(1));
		}
		assert_noop!(KittiesModule::create(Origin::signed(1)), Error::<Test>::KittiesCountOverflow);
	});
}


#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 10000);
		let _ = Balances::deposit_creating(&2, 10000);
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 0, 2));
	});
}

#[test]
fn transfer_fails_when_kitty_id_invalid() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 10000);
		let _ = Balances::deposit_creating(&2, 10000);
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
		assert_noop!(KittiesModule::transfer(Origin::signed(1), 10, 2), Error::<Test>::InvalidKittyId);
	});
}

#[test]
fn transfer_fails_when_token_not_enough() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 10000);
		let _ = Balances::deposit_creating(&2, 100);
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
		assert_noop!(KittiesModule::transfer(Origin::signed(1), 0, 2), Error::<Test>::TokenNotEnough);
	});
}

#[test]
fn transfer_fails_when_exceed_max_kitty_owned() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 100000);
		let _ = Balances::deposit_creating(&2, 100000);
		_ = KittiesModule::create(Origin::signed(1));
		System::set_block_number(1);
		for _ in 0..8 {
			_ = KittiesModule::create(Origin::signed(2));
		}
		assert_noop!(KittiesModule::transfer(Origin::signed(1), 0, 2), Error::<Test>::ExceedMaxKittyOwned);
	});
}

#[test]
fn breed_works() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 10000);
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 2);
		assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
		assert_eq!(KittiesModule::next_kitty_id(), 3);
	});
}

#[test]
fn breed_fails_when_same_kitty_id() {
	new_test_ext().execute_with(|| {
		let _ = Balances::deposit_creating(&1, 10000);
		System::set_block_number(1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(KittiesModule::next_kitty_id(), 1);
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 0), Error::<Test>::SameKittyId);
	});
}
