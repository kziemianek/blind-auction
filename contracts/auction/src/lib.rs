#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod auction {
    use ink::prelude::vec::Vec;
    use ink::{env::DefaultEnvironment, storage::Mapping};

    pub type AuctionId = u32;

    #[derive(Clone, Debug, PartialEq, scale::Decode, scale::Encode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        NotEnoughEscrow,
        AuctionHasEnded,
        RevealPhaseHasStarted,
        CannotBidTwice,
        RevealPhaseHasntStartedYet,
        InvalidBid,
        CannotCloseAuction,
        UnknownAuction,
        NotBidded,
    }

    #[ink(event)]
    pub struct AuctionCreated {
        auction_id: AuctionId,
        owner: AccountId,
        end: BlockNumber,
        reveal_phase_start: BlockNumber,
        starting_price: Balance,
        required_escrow: Balance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Clone, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    )]
    pub struct Auction {
        pub id: AuctionId,
        pub owner: AccountId,
        pub end: BlockNumber,
        pub reveal_phase_start: BlockNumber,
        pub starting_price: Balance,
        pub required_escrow: Balance,
    }

    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
    )]
    pub struct CommitedBid {
        pub signature: [[u8; 32]; 2],
        pub public_key: [u8; 32],
    }

    impl Auction {
        pub fn new(
            id: AuctionId,
            owner: AccountId,
            end: BlockNumber,
            reveal_phase_start: BlockNumber,
            starting_price: Balance,
            required_escrow: Balance,
        ) -> Self {
            Self {
                id,
                owner,
                end,
                reveal_phase_start,
                starting_price,
                required_escrow,
            }
        }

        pub fn assert_not_ended(&self, block: BlockNumber) -> Result<()> {
            if block >= self.end {
                return Err(Error::AuctionHasEnded);
            }
            Ok(())
        }

        pub fn assert_reveal_phase_not_started(&self, block: BlockNumber) -> Result<()> {
            if block >= self.reveal_phase_start {
                return Err(Error::RevealPhaseHasStarted);
            }
            Ok(())
        }
    }

    #[ink(storage)]
    pub struct AuctionHouse {
        last_auction_id: AuctionId,
        auctions: Mapping<AuctionId, Auction>,
        participants: Mapping<AuctionId, Vec<AccountId>>,
        commited_bids: Mapping<(AuctionId, AccountId), CommitedBid>,
        revealed_bids: Mapping<(AuctionId, AccountId), Balance>,
    }

    impl AuctionHouse {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                last_auction_id: AuctionId::default(),
                auctions: Mapping::default(),
                participants: Mapping::default(),
                commited_bids: Mapping::default(),
                revealed_bids: Mapping::default(),
            }
        }

        /// Creates auction with given end block number, starting_price and required escrow. Escrow should be deposited by owner and bidders to make sure all sides follows the rules.
        #[ink(message, payable)]
        pub fn create_auction(
            &mut self,
            end: BlockNumber,
            starting_price: Balance,
            required_escrow: Balance,
        ) -> Result<AuctionId> {
            let owner = ink::env::caller::<DefaultEnvironment>();
            let transfered_value = ink::env::transferred_value::<DefaultEnvironment>();

            if transfered_value < required_escrow {
                return Err(Error::NotEnoughEscrow);
            }

            self.last_auction_id += 1;
            self.auctions.insert(
                self.last_auction_id,
                &Auction::new(
                    self.last_auction_id,
                    owner,
                    end,
                    end - 10,
                    starting_price,
                    required_escrow,
                ),
            );

            self.env().emit_event(AuctionCreated {
                auction_id: self.last_auction_id,
                owner,
                end,
                reveal_phase_start: end + 10,
                starting_price,
                required_escrow,
            });

            Ok(self.last_auction_id)
        }

        #[ink(message, payable)]
        pub fn join(&mut self, auction_id: AuctionId) -> Result<()> {
            let auction = self.get_auction(auction_id)?;
            let transfered_value = ink::env::transferred_value::<DefaultEnvironment>();

            if transfered_value < auction.required_escrow {
                return Err(Error::NotEnoughEscrow);
            }

            let participant = ink::env::caller::<DefaultEnvironment>();
            if let Some(mut participants) = self.participants.get(&auction_id) {
                participants.push(participant);
            } else {
                let mut participants = Vec::default();
                participants.push(participant);
                self.participants.insert(&auction_id, &participants);
            }
            Ok(())
        }

        #[ink(message)]
        pub fn bid(
            &mut self,
            auction_id: AuctionId,
            bid_signature: [u8; 64],
            public_key: [u8; 32],
        ) -> Result<()> {
            let bidder = ink::env::caller::<DefaultEnvironment>();
            let current_block = ink::env::block_number::<DefaultEnvironment>();
            let auction: Auction = self.get_auction(auction_id)?;
            auction.assert_not_ended(current_block)?;
            auction.assert_reveal_phase_not_started(current_block)?;
            let mut first_part: [u8; 32] = [0; 32];
            first_part.copy_from_slice(&bid_signature[..32]);
            let mut second_part: [u8; 32] = [0; 32];
            second_part.copy_from_slice(&bid_signature[32..64]);

            if let None = self.commited_bids.get((auction_id, bidder)) {
                self.commited_bids.insert(
                    (auction_id, bidder),
                    &CommitedBid {
                        signature: [first_part, second_part],
                        public_key,
                    },
                );
            } else {
                return Err(Error::CannotBidTwice);
            }

            Ok(())
        }

        #[ink(message)]
        pub fn reveal_bid(&mut self, auction_id: AuctionId, bid: Balance) -> Result<()> {
            let current_block: u32 = ink::env::block_number::<DefaultEnvironment>();
            let auction = self.get_auction(auction_id)?;
            if current_block < auction.reveal_phase_start {
                return Err(Error::RevealPhaseHasntStartedYet);
            }
            auction.assert_not_ended(current_block)?;
            let bidder = ink::env::caller::<DefaultEnvironment>();
            let commited_bid = self
                .commited_bids
                .get((auction_id, bidder))
                .ok_or(Error::NotBidded)?;
            self.verify_bid(bid, commited_bid)?;
            self.revealed_bids.insert((auction_id, bidder), &bid);
            Ok(())
        }

        #[ink(message)]
        pub fn close(&mut self, auction_id: AuctionId) -> Result<()> {
            //only owner/bidders can close action, user who triggered closing auction should get reward.
            //owner should close auction withing defined time of blocks, if he doesn't bidders can close
            let auction = self.get_auction(auction_id)?;
            let current_block = ink::env::block_number::<DefaultEnvironment>();

            let is_owner = ink::env::caller::<DefaultEnvironment>() == auction.owner;

            if is_owner {
                if current_block > auction.end + 10 {
                    return Err(Error::CannotCloseAuction);
                }
                let participants = self.participants.get(auction_id).unwrap();

                for participant in participants {
                    if let Some(_) = self.revealed_bids.get((auction_id, participant)) {
                        // return escrow to participant
                        ink::env::transfer::<DefaultEnvironment>(
                            participant,
                            auction.required_escrow,
                        )
                        .unwrap();
                    } else {
                        // transfer participant escrow to auction owner
                        ink::env::transfer::<DefaultEnvironment>(
                            auction.owner,
                            auction.required_escrow,
                        )
                        .unwrap();
                    }
                }
            }

            let caller = ink::env::caller::<DefaultEnvironment>();
            // if caller has revealed his bid he can also close auction
            if let Some(_) = self.revealed_bids.get((auction_id, caller)) {
                if current_block <= auction.end + 10 {
                    return Err(Error::CannotCloseAuction);
                }
                let participants = self.participants.get(auction_id).unwrap();

                for participant in participants {
                    if let Some(_) = self.revealed_bids.get((auction_id, participant)) {
                        // return escrow to participant
                        ink::env::transfer::<DefaultEnvironment>(
                            participant,
                            auction.required_escrow,
                        )
                        .unwrap();
                    } else {
                        // transfer participant escrow to auction owner
                        ink::env::transfer::<DefaultEnvironment>(caller, auction.required_escrow)
                            .unwrap();
                    }
                }
            }
            return Ok(());
        }

        fn verify_bid(&mut self, bid: Balance, comitted_bid: CommitedBid) -> Result<()> {
            let mut signature: [u8; 64] = [0; 64];
            signature[..32].copy_from_slice(&comitted_bid.signature[0]);
            signature[32..64].copy_from_slice(&comitted_bid.signature[1]);
            let bid: u128 = bid as u128;
            let message: &[u8] = &bid.to_le_bytes();
            ink::env::sr25519_verify(&signature, &message, &comitted_bid.public_key)
                .map_err(|_| Error::InvalidBid)
        }

        fn get_auction(&self, auction_id: AuctionId) -> Result<Auction> {
            self.auctions.get(auction_id).ok_or(Error::UnknownAuction)
        }
    }

    #[cfg(test)]
    mod test {
        use ink::env::DefaultEnvironment;

        use super::*;
        use ink_test_utils::assert_event;

        type Event = <AuctionHouse as ::ink::reflect::ContractEventBase>::Type;

        #[ink::test]
        fn create_auction_works() {
            let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
            let mut auction_house = AuctionHouse::default();

            let auction_owner = accounts.alice;
            let auction_end = ink::env::block_number::<DefaultEnvironment>() + 10;
            let auction_starting_price = 1000;
            let auction_required_escrow = 100;

            ink::env::test::set_caller::<DefaultEnvironment>(auction_owner);
            ink::env::test::set_value_transferred::<DefaultEnvironment>(auction_required_escrow);
            let created_auction_id = auction_house
                .create_auction(auction_end, auction_starting_price, auction_required_escrow)
                .expect("could not create auction");

            assert_eq!(created_auction_id, 1);

            assert_event! {
                0: AuctionCreated (auction_id, owner, end, reveal_phase_start, starting_price, required_escrow) [
                    assert_eq!(auction_id, 1),
                    assert_eq!(owner, auction_owner),
                    assert_eq!(end, auction_end),
                    assert_eq!(reveal_phase_start, auction_end + 10),
                    assert_eq!(starting_price, auction_starting_price),
                    assert_eq!(required_escrow, auction_required_escrow)
                ]
            }
        }

        #[ink::test]
        fn create_auction_requires_escrow() {
            let accounts = ink::env::test::default_accounts::<DefaultEnvironment>();
            let mut auction_house = AuctionHouse::default();

            let auction_owner = accounts.alice;
            let end = ink::env::block_number::<DefaultEnvironment>() + 10;
            let starting_price = 1000;
            let required_escrow = 100;

            ink::env::test::set_caller::<DefaultEnvironment>(auction_owner);
            ink::env::test::set_value_transferred::<DefaultEnvironment>(required_escrow - 2);
            let result = auction_house.create_auction(end, starting_price, required_escrow);

            assert_eq!(result, Err(Error::NotEnoughEscrow));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn create_auction_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = AuctionHouseRef::default();
            let contract_acc_id = client
                .instantiate("auction", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // when
            let create = build_message::<AuctionHouseRef>(contract_acc_id.clone())
                .call(|ah| ah.create_auction(10, 10, 10));
            let create_res = client
                .call(&ink_e2e::bob(), create, 15, None)
                .await
                .expect("create failed");

            // then
            assert_eq!(create_res.return_value(), Ok(1));
            Ok(())
        }
    }
}
