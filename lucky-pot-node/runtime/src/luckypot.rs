// Encoding library
use parity_codec::Encode;

// Enables access to the runtime storage
// Imports the `Result` type that is returned from runtime functions
// Imports the `decl_module!` and `decl_storage!` macros
use support::{decl_module, decl_storage, StorageValue, dispatch::Result};
// use decl_event;

// Enables us to do hashing
use runtime_primitives::traits::Hash;

// Enables access to account balances and interacting with signed messages
use balances;
use system::ensure_signed;

/// The module's configuration trait.
// pub trait Trait: system::Trait {
// 	// TODO: Add other types and constants required configure this module.

// 	/// The overarching event type.
// 	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
// }


pub trait Trait: balances::Trait {}
    /// The overarching event type.
	// type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
// }

/// This module's storage items.
decl_storage! {
  trait Store for Module<T: Trait> as LuckyPot {
    Payment get(payment): Option<T::Balance>;
    Pot get(pot): T::Balance;
  }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		// Initializing events
		// this is needed only if you are using events in your module
		// fn deposit_event<T>() = default;

        fn play(origin) -> Result {
            // Ensure we have a signed message, and derive the sender's account id from the signature
            let sender = ensure_signed(origin)?;

            // Here we grab the payment, and put it into a local variable.
            // We are able to use Self::payment() because we defined it in our decl_storage! macro below
            // If there is no payment, exit with an error message
            let payment = Self::payment().ok_or("Must have payment amount set")?;

            // First, we decrease the balance of the sender by the payment amount using the balances module
            <balances::Module<T>>::decrease_free_balance(&sender, payment)?;

            // Then we flip a coin by generating a random seed
            // We pass the seed with our sender's account id into a hash algorithm
            // Then we check if the first byte of the hash is less than 128
            if (<system::Module<T>>::random_seed(), &sender)
                .using_encoded(<T as system::Trait>::Hashing::hash)
                .using_encoded(|e| e[0] < 128)
            {
                // If the sender wins the coin flip, we increase the sender's balance by the pot amount
                // `::take()` will also remove the pot amount from storage, which by default will give it a value of 0
                <balances::Module<T>>::increase_free_balance_creating(&sender, <Pot<T>>::take());
    
    			// Self::deposit_event(RawEvent::SomethingStored(something, who));
            }

            // No matter the outcome, we will add the original sender's payment back into the pot
            <Pot<T>>::mutate(|pot| *pot += payment);
	
    		// Self::deposit_event(RawEvent::SomethingStored(something, who));

            Ok(())
        }

        fn set_payment(_origin, value: T::Balance) -> Result {
            //If the payment has not been set...
            if Self::payment().is_none() {
                // ... we will set it to the value we passed in.
                <Payment<T>>::put(value);
                
                // We will also put that initial value into the pot for someone to win
                <Pot<T>>::put(value);
            }
            
            Ok(())
        }
        
    }
}


// decl_event!(
// 	/// An event in this module.
// 	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
// 		// Just a dummy event.
// 		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
// 		// To emit this event, we call the deposit funtion, from our runtime funtions
// 		SomethingStored(u32, AccountId),
// 	}
// );
