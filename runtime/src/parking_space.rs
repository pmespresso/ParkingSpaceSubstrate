use srml_support::{StorageValue, StorageMap, dispatch::Result};
use {balances, system::ensure_signed};

extern crate sr_primitives as primitives;
extern crate substrate_primitives;

pub trait Trait: balances::Trait {}

#[derive(Encode, Eq, PartialEq, Clone, Copy, Decode, Default)]
pub struct Space <AccountId, Balance> {
    index: u32,
    renter: AccountId,
    space_balance: Balance,
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn rent_space(_origin, payment: T::Balance, space_index: u32) -> Result {
            let renter = ensure_signed(_origin)?;

            // check if balance is zero (then it's empty)
            ensure!(Self::spaceAt(space_index).space_balance < payment, "Sorry, someone's already paid more for this space.");

            // decrease the renter's balance by 1
            <balances::Module<T>>::decrease_free_balance(&renter, payment)?;

            // construct the space struct
            let my_space = Space {
                index: space_index.clone(),
                renter: renter.clone(),
                space_balance: payment.clone()
            };

            // add that balance to rent the space
            <SpaceAt<T>>::insert(space_index, my_space.clone());

            // insert the mapping of renter accound id to the space index and its current balance
            <SpaceOf<T>>::insert(&renter, my_space.clone());

            Ok(())
        }
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as ParkingSpaceStorage {
        pub SpaceAt get(spaceAt): map u32 => Space<T::AccountId, T::Balance>;
        pub SpaceOf get(spaceOf): map T::AccountId => Space<T::AccountId, T::Balance>;
        pub OwnerOf get(ownerOf): map u32 => T::AccountId;
    }
}
