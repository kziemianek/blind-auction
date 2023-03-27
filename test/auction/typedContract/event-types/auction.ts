import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/auction';

export interface AuctionCreated {
	auctionId: number;
	owner: ReturnTypes.AccountId;
	end: number;
	revealPhaseStart: number;
	startingPrice: ReturnNumber;
	requiredEscrow: ReturnNumber;
}

