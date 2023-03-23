#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod auction {

    #[ink(storage)]
    pub struct Auction {}

    impl Auction {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn void(&self) {}
    }
}
