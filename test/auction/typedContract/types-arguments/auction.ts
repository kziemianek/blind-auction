import type BN from 'bn.js';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export enum Error {
	notEnoughEscrow = 'NotEnoughEscrow',
	auctionHasEnded = 'AuctionHasEnded',
	revealPhaseHasStarted = 'RevealPhaseHasStarted',
	cannotBidTwice = 'CannotBidTwice',
	revealPhaseHasntStartedYet = 'RevealPhaseHasntStartedYet',
	invalidBid = 'InvalidBid',
	cannotCloseAuction = 'CannotCloseAuction',
	unknownAuction = 'UnknownAuction'
}

