#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::PalletId;
	use frame_support::traits::Randomness;

	#[pallet::pallet]
	pub struct Pallet<T>(_);


	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		/// Type representing the Pallet ID
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// Type representing the random number generator
		type MyRandomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo, PartialOrd, Default)]
	pub enum CoinSide {
		#[default]
		Head,
		Tail,
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo, PartialOrd, Default)]
	pub struct Coin {
		side: CoinSide,
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type CoinStorage<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Coin, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new coin has been created
		CoinCreated(AccountIdOf<T>),
		/// Coin has been Flipped
		CoinFlipped(AccountIdOf<T>, CoinSide),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Coin already exists
		CoinAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::default_weight())]
		pub fn create_coin(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_coin(&who)?;
			Self::deposit_event(Event::CoinCreated(who));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn do_create_coin(who: &T::AccountId) -> DispatchResult {
			let coin = Coin {
				side: Self::random_coin_side(),
			};

			if CoinStorage::<T>::contains_key(who) {
				return Err(Error::<T>::CoinAlreadyExists.into());
			}

			CoinStorage::<T>::insert(who, coin);


			Ok(())
		}

		pub fn random_coin_side() -> CoinSide {
			let block_number = <frame_system::Pallet<T>>::block_number();
			let seed = block_number.try_into().unwrap_or_else(|_| 0u32);

			if Self::generate_insecure_random_boolean(seed) {
				CoinSide::Head
			} else {
				CoinSide::Tail
			}
		}


		// TODO: check for safer alternatives to generate random numbers in this env
		// You should call this function with different seed values, in this case I'm are using the block number as seed
		pub fn generate_insecure_random_boolean(seed: u32) -> bool {
			let pallet_id = T::PalletId::get();
			let (random_seed, _) = T::MyRandomness::random(&(pallet_id, seed).encode());
			let random_number = <u32>::decode(&mut random_seed.as_ref())
				.expect("secure hashes should always be bigger than u32; qed");
			random_number % 2 == 0
		}
	}
}
