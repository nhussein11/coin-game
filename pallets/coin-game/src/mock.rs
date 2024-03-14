use crate as pallet_coin_game;
use frame_support::{construct_runtime, parameter_types, traits::{ConstU16, ConstU32, ConstU64, Everything}, PalletId};
use frame_system::Config;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use frame_support_test::TestRandomness;

type Block = frame_system::mocking::MockBlock<Test>;


construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		CoinGame: pallet_coin_game,
	}
);


parameter_types! {
	pub const CoinFlipperPalletId: PalletId = PalletId(*b"coinflip");
}

impl Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}


impl pallet_coin_game::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type PalletId = CoinFlipperPalletId;
	type Randomness = TestRandomness<Self>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

// Helper to get the last events
pub fn last_event() -> RuntimeEvent {
	System::events().pop().expect("Event expected").event
}
