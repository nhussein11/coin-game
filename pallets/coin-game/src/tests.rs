use frame_system::Origin;
use crate::{mock::*, Event, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;

// pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);

#[test]
fn create_coin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(CoinGame::create_coin(RuntimeOrigin::signed(1u64)));
		let expected_event = RuntimeEvent::CoinGame(Event::CoinCreated(1));
		assert_eq!(last_event(), expected_event);
	});
}


#[test]
fn create_coin_fails() {
	new_test_ext().execute_with(|| {
		assert_ok!(CoinGame::create_coin(RuntimeOrigin::signed(1u64)));
		assert_eq!(
			CoinGame::create_coin(RuntimeOrigin::signed(1u64)),
			Err(DispatchError::from(Error::<Test>::CoinAlreadyExists))
		);

	});
}

