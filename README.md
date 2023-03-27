# blind auction

blind auction in ink!

Users can create blind auction which lasts until given block. It is required to despoit escrow in order to start auction, the same escrow deposit is required from bidders. The dishonest users will lose their escrow!

Auction is splitted into 3 phases:
* bidding phase
* revealing phase
* ending phase

## Bidding phase
In this phase users can join auction and place theirs bids. To keep things secret, users do not submit their real bids but signatures. To generate signature users need to take bid value and sign it with their private key.

## Revealing phase
In this phase users reveal their bids. They submit real bid values and smart contract check whether bid was used to generate previously submitted signature. If they match the bid is accepted.

## Ending phase
In this phase escrow deposits are returned to honest participants, dishonest participants escrows are splitted across the rest. The user triggering end phase gets the biggest portion of it. This phase is splitted into two subphases:

* owner ending - time window in which only auction owner is allowed to end it
* bidders ending  - time window in which any participant who revealed bid is allowed to end auction


