use crate::{mock::*, Event, Error, Coin, CoinSide};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::DispatchError;

// pub const ALICE: AccountId32 = AccountId32::new([1u8; 32]);
const ALICE : u64 = 1;
const BOB : u64 = 2;


/// Call: create_coin
/// Happy path: test that the create_coin function works, it creates a coin and emits the correct event
#[test]
fn create_coin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(CoinGame::create_coin(RuntimeOrigin::signed(ALICE)));

        let coin = CoinGame::get_coin(&ALICE).unwrap();

		// Need to check side since it's random
        if coin.side == CoinSide::Head {
            assert_eq!(coin, Coin { side: CoinSide::Head });
        } else {
            assert_eq!(coin, Coin { side: CoinSide::Tail });
		}


		let expected_event = RuntimeEvent::CoinGame(Event::CoinCreated(ALICE));
		assert_eq!(last_event(), expected_event);
	});
}

/// Call: create_coin
/// Unhappy path: test that the create_coin function fails when the origin is none
#[test]
fn create_coin_fails_with_no_permission() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			CoinGame::create_coin(RuntimeOrigin::none()),
			DispatchError::BadOrigin
		);
	});
}

/// Call: create_coin
/// Unhappy path: test that the create_coin function fails when the coin already exists
#[test]
fn create_coin_fails_with_coin_already_exists() {
	new_test_ext().execute_with(|| {
		assert_ok!(CoinGame::create_coin(RuntimeOrigin::signed(ALICE)));
		assert_noop!(
			CoinGame::create_coin(RuntimeOrigin::signed(ALICE)),
			Error::<Test>::CoinAlreadyExists
		);
	});
}

/// Function: gets_coin
/// Happy path: test that the get_coin function works
#[test]
fn get_coin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(CoinGame::create_coin(RuntimeOrigin::signed(ALICE)));
		let coin = CoinGame::get_coin(&ALICE).unwrap();
		if coin.side == CoinSide::Head {
			assert_eq!(coin, Coin { side: CoinSide::Head });
		} else {
			assert_eq!(coin, Coin { side: CoinSide::Tail });
		}
	});
}

/// Function: get_coin
/// Unhappy path: test that the get_coin function fails when the coin does not exist
#[test]
fn get_coin_fails_with_coin_not_found() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			CoinGame::get_coin(&ALICE),
			Error::<Test>::CoinNotFound
		);
	});
}

