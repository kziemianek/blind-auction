/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/auction';
import type * as ReturnTypes from '../types-returns/auction';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";


export default class Methods {
	private __nativeContract : ContractPromise;
	private __keyringPair : KeyringPair;
	private __callerAddress : string;
	private __apiPromise: ApiPromise;

	constructor(
		apiPromise : ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
		this.__callerAddress = keyringPair.address;
	}

	/**
	* createAuction
	*
	* @param { (number | string | BN) } end,
	* @param { (string | number | BN) } startingPrice,
	* @param { (string | number | BN) } requiredEscrow,
	* @returns { void }
	*/
	"createAuction" (
		end: (number | string | BN),
		startingPrice: (string | number | BN),
		requiredEscrow: (string | number | BN),
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "createAuction", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "auction");
		}, [end, startingPrice, requiredEscrow], __options);
	}

	/**
	* join
	*
	* @param { (number | string | BN) } auctionId,
	* @returns { void }
	*/
	"join" (
		auctionId: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "join", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "auction");
		}, [auctionId], __options);
	}

	/**
	* bid
	*
	* @param { (number | string | BN) } auctionId,
	* @param { Array<(number | string | BN)> } bidSignature,
	* @returns { void }
	*/
	"bid" (
		auctionId: (number | string | BN),
		bidSignature: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "bid", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "auction");
		}, [auctionId, bidSignature], __options);
	}

	/**
	* revealBid
	*
	* @param { (number | string | BN) } auctionId,
	* @param { (string | number | BN) } bid,
	* @returns { void }
	*/
	"revealBid" (
		auctionId: (number | string | BN),
		bid: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "revealBid", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "auction");
		}, [auctionId, bid], __options);
	}

	/**
	* close
	*
	* @param { (number | string | BN) } auctionId,
	* @returns { void }
	*/
	"close" (
		auctionId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "close", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "auction");
		}, [auctionId], __options);
	}

}