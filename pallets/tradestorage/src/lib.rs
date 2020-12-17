#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		Trades get(fn trade): Option<u32>;
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		TradeStored(u32, AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn store_trade(origin, trade: u32) -> dispatch::DispatchResult {
			let caller = ensure_signed(origin)?;
			Trades::put(trade);
			Self::deposit_event(RawEvent::TradeStored(trade, caller));
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _caller = ensure_signed(origin)?;

			// Read a value from storage.
			match Trades::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(_value) => {
					Ok(())
				},
			}
		}
	}
}
