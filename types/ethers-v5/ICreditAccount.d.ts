/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import {
  ethers,
  EventFilter,
  Signer,
  BigNumber,
  BigNumberish,
  PopulatedTransaction,
  BaseContract,
  ContractTransaction,
  Overrides,
  CallOverrides,
} from "ethers";
import { BytesLike } from "@ethersproject/bytes";
import { Listener, Provider } from "@ethersproject/providers";
import { FunctionFragment, EventFragment, Result } from "@ethersproject/abi";
import type { TypedEventFilter, TypedEvent, TypedListener } from "./common";

interface ICreditAccountInterface extends ethers.utils.Interface {
  functions: {
    "approveToken(address,address)": FunctionFragment;
    "borrowedAmount()": FunctionFragment;
    "cancelAllowance(address,address)": FunctionFragment;
    "connectTo(address)": FunctionFragment;
    "creditManager()": FunctionFragment;
    "cumulativeIndexAtOpen()": FunctionFragment;
    "execute(address,bytes)": FunctionFragment;
    "factory()": FunctionFragment;
    "initialize()": FunctionFragment;
    "safeTransfer(address,address,uint256)": FunctionFragment;
    "setGenericParameters(uint256,uint256)": FunctionFragment;
    "since()": FunctionFragment;
    "updateBorrowedAmount(uint256)": FunctionFragment;
  };

  encodeFunctionData(
    functionFragment: "approveToken",
    values: [string, string]
  ): string;
  encodeFunctionData(
    functionFragment: "borrowedAmount",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "cancelAllowance",
    values: [string, string]
  ): string;
  encodeFunctionData(functionFragment: "connectTo", values: [string]): string;
  encodeFunctionData(
    functionFragment: "creditManager",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "cumulativeIndexAtOpen",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "execute",
    values: [string, BytesLike]
  ): string;
  encodeFunctionData(functionFragment: "factory", values?: undefined): string;
  encodeFunctionData(
    functionFragment: "initialize",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "safeTransfer",
    values: [string, string, BigNumberish]
  ): string;
  encodeFunctionData(
    functionFragment: "setGenericParameters",
    values: [BigNumberish, BigNumberish]
  ): string;
  encodeFunctionData(functionFragment: "since", values?: undefined): string;
  encodeFunctionData(
    functionFragment: "updateBorrowedAmount",
    values: [BigNumberish]
  ): string;

  decodeFunctionResult(
    functionFragment: "approveToken",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "borrowedAmount",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "cancelAllowance",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "connectTo", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "creditManager",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "cumulativeIndexAtOpen",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "execute", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "factory", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "initialize", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "safeTransfer",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "setGenericParameters",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "since", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "updateBorrowedAmount",
    data: BytesLike
  ): Result;

  events: {};
}

export class ICreditAccount extends BaseContract {
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;

  listeners<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter?: TypedEventFilter<EventArgsArray, EventArgsObject>
  ): Array<TypedListener<EventArgsArray, EventArgsObject>>;
  off<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>,
    listener: TypedListener<EventArgsArray, EventArgsObject>
  ): this;
  on<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>,
    listener: TypedListener<EventArgsArray, EventArgsObject>
  ): this;
  once<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>,
    listener: TypedListener<EventArgsArray, EventArgsObject>
  ): this;
  removeListener<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>,
    listener: TypedListener<EventArgsArray, EventArgsObject>
  ): this;
  removeAllListeners<EventArgsArray extends Array<any>, EventArgsObject>(
    eventFilter: TypedEventFilter<EventArgsArray, EventArgsObject>
  ): this;

  listeners(eventName?: string): Array<Listener>;
  off(eventName: string, listener: Listener): this;
  on(eventName: string, listener: Listener): this;
  once(eventName: string, listener: Listener): this;
  removeListener(eventName: string, listener: Listener): this;
  removeAllListeners(eventName?: string): this;

  queryFilter<EventArgsArray extends Array<any>, EventArgsObject>(
    event: TypedEventFilter<EventArgsArray, EventArgsObject>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEvent<EventArgsArray & EventArgsObject>>>;

  interface: ICreditAccountInterface;

  functions: {
    approveToken(
      token: string,
      swapContract: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    borrowedAmount(overrides?: CallOverrides): Promise<[BigNumber]>;

    cancelAllowance(
      token: string,
      targetContract: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    connectTo(
      _creditManager: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    creditManager(overrides?: CallOverrides): Promise<[string]>;

    cumulativeIndexAtOpen(overrides?: CallOverrides): Promise<[BigNumber]>;

    execute(
      destination: string,
      data: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    factory(overrides?: CallOverrides): Promise<[string]>;

    initialize(
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    safeTransfer(
      token: string,
      to: string,
      amount: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    setGenericParameters(
      _borrowedAmount: BigNumberish,
      _cumulativeIndexAtOpen: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;

    since(overrides?: CallOverrides): Promise<[BigNumber]>;

    updateBorrowedAmount(
      _borrowedAmount: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<ContractTransaction>;
  };

  approveToken(
    token: string,
    swapContract: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  borrowedAmount(overrides?: CallOverrides): Promise<BigNumber>;

  cancelAllowance(
    token: string,
    targetContract: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  connectTo(
    _creditManager: string,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  creditManager(overrides?: CallOverrides): Promise<string>;

  cumulativeIndexAtOpen(overrides?: CallOverrides): Promise<BigNumber>;

  execute(
    destination: string,
    data: BytesLike,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  factory(overrides?: CallOverrides): Promise<string>;

  initialize(
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  safeTransfer(
    token: string,
    to: string,
    amount: BigNumberish,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  setGenericParameters(
    _borrowedAmount: BigNumberish,
    _cumulativeIndexAtOpen: BigNumberish,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  since(overrides?: CallOverrides): Promise<BigNumber>;

  updateBorrowedAmount(
    _borrowedAmount: BigNumberish,
    overrides?: Overrides & { from?: string | Promise<string> }
  ): Promise<ContractTransaction>;

  callStatic: {
    approveToken(
      token: string,
      swapContract: string,
      overrides?: CallOverrides
    ): Promise<void>;

    borrowedAmount(overrides?: CallOverrides): Promise<BigNumber>;

    cancelAllowance(
      token: string,
      targetContract: string,
      overrides?: CallOverrides
    ): Promise<void>;

    connectTo(_creditManager: string, overrides?: CallOverrides): Promise<void>;

    creditManager(overrides?: CallOverrides): Promise<string>;

    cumulativeIndexAtOpen(overrides?: CallOverrides): Promise<BigNumber>;

    execute(
      destination: string,
      data: BytesLike,
      overrides?: CallOverrides
    ): Promise<string>;

    factory(overrides?: CallOverrides): Promise<string>;

    initialize(overrides?: CallOverrides): Promise<void>;

    safeTransfer(
      token: string,
      to: string,
      amount: BigNumberish,
      overrides?: CallOverrides
    ): Promise<void>;

    setGenericParameters(
      _borrowedAmount: BigNumberish,
      _cumulativeIndexAtOpen: BigNumberish,
      overrides?: CallOverrides
    ): Promise<void>;

    since(overrides?: CallOverrides): Promise<BigNumber>;

    updateBorrowedAmount(
      _borrowedAmount: BigNumberish,
      overrides?: CallOverrides
    ): Promise<void>;
  };

  filters: {};

  estimateGas: {
    approveToken(
      token: string,
      swapContract: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    borrowedAmount(overrides?: CallOverrides): Promise<BigNumber>;

    cancelAllowance(
      token: string,
      targetContract: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    connectTo(
      _creditManager: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    creditManager(overrides?: CallOverrides): Promise<BigNumber>;

    cumulativeIndexAtOpen(overrides?: CallOverrides): Promise<BigNumber>;

    execute(
      destination: string,
      data: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    factory(overrides?: CallOverrides): Promise<BigNumber>;

    initialize(
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    safeTransfer(
      token: string,
      to: string,
      amount: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    setGenericParameters(
      _borrowedAmount: BigNumberish,
      _cumulativeIndexAtOpen: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;

    since(overrides?: CallOverrides): Promise<BigNumber>;

    updateBorrowedAmount(
      _borrowedAmount: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<BigNumber>;
  };

  populateTransaction: {
    approveToken(
      token: string,
      swapContract: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    borrowedAmount(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    cancelAllowance(
      token: string,
      targetContract: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    connectTo(
      _creditManager: string,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    creditManager(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    cumulativeIndexAtOpen(
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    execute(
      destination: string,
      data: BytesLike,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    factory(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    initialize(
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    safeTransfer(
      token: string,
      to: string,
      amount: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    setGenericParameters(
      _borrowedAmount: BigNumberish,
      _cumulativeIndexAtOpen: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;

    since(overrides?: CallOverrides): Promise<PopulatedTransaction>;

    updateBorrowedAmount(
      _borrowedAmount: BigNumberish,
      overrides?: Overrides & { from?: string | Promise<string> }
    ): Promise<PopulatedTransaction>;
  };
}