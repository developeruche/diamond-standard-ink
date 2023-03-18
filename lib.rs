#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod diamond {
    use ink_storage::traits::SpreadAllocate;
    use openbrush::{
        contracts::diamond::extensions::diamond_loupe::*,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct Contract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        diamond: diamond::Data<Loupe>,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            let mut _instance = Self::default();
            _instance._init_with_owner(owner);
            _instance
        }

        /// this function would be forwarding the the calls to the modules that has the implementation of the functions 
        #[ink(message, payable, selector = _)]
        pub fn forward(&self) {
            self._fallback()
        }
    }

    impl Ownable for Contract {}

    impl Diamond for Contract {}

    impl DiamondLoupe for Contract {}
}