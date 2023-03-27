/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/auction';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
	}
	/**
	 * createAuction
	 *
	 * @param { (number | string | BN) } end,
	 * @param { (string | number | BN) } startingPrice,
	 * @param { (string | number | BN) } requiredEscrow,
	*/
	"createAuction" (
		end: (number | string | BN),
		startingPrice: (string | number | BN),
		requiredEscrow: (string | number | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "createAuction", [end, startingPrice, requiredEscrow], __options);
	}

	/**
	 * join
	 *
	 * @param { (number | string | BN) } auctionId,
	*/
	"join" (
		auctionId: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "join", [auctionId], __options);
	}

	/**
	 * bid
	 *
	 * @param { (number | string | BN) } auctionId,
	 * @param { Array<(number | string | BN)> } bidSignature,
	*/
	"bid" (
		auctionId: (number | string | BN),
		bidSignature: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "bid", [auctionId, bidSignature], __options);
	}

	/**
	 * revealBid
	 *
	 * @param { (number | string | BN) } auctionId,
	 * @param { (string | number | BN) } bid,
	*/
	"revealBid" (
		auctionId: (number | string | BN),
		bid: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "revealBid", [auctionId, bid], __options);
	}

	/**
	 * close
	 *
	 * @param { (number | string | BN) } auctionId,
	*/
	"close" (
		auctionId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "close", [auctionId], __options);
	}

}