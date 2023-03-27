/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/auction';
import type * as ReturnTypes from '../types-returns/auction';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		nativeApi : ApiPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
		this.__apiPromise = nativeApi;
	}

	/**
	* createAuction
	*
	* @param { (number | string | BN) } end,
	* @param { (string | number | BN) } startingPrice,
	* @param { (string | number | BN) } requiredEscrow,
	* @returns { Result<Result<number, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"createAuction" (
		end: (number | string | BN),
		startingPrice: (string | number | BN),
		requiredEscrow: (string | number | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<number, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "createAuction", [end, startingPrice, requiredEscrow], __options , (result) => { return handleReturnType(result, getTypeDescription(9, 'auction')); });
	}

	/**
	* join
	*
	* @param { (number | string | BN) } auctionId,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"join" (
		auctionId: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "join", [auctionId], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'auction')); });
	}

	/**
	* bid
	*
	* @param { (number | string | BN) } auctionId,
	* @param { Array<(number | string | BN)> } bidSignature,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"bid" (
		auctionId: (number | string | BN),
		bidSignature: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "bid", [auctionId, bidSignature], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'auction')); });
	}

	/**
	* revealBid
	*
	* @param { (number | string | BN) } auctionId,
	* @param { (string | number | BN) } bid,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"revealBid" (
		auctionId: (number | string | BN),
		bid: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "revealBid", [auctionId, bid], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'auction')); });
	}

	/**
	* close
	*
	* @param { (number | string | BN) } auctionId,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"close" (
		auctionId: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "close", [auctionId], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'auction')); });
	}

}