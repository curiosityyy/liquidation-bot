/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumber,
  BigNumberish,
  BytesLike,
  CallOverrides,
  PopulatedTransaction,
  Signer,
  utils,
} from "ethers";
import type {
  FunctionFragment,
  Result,
  EventFragment,
} from "@ethersproject/abi";
import type { Listener, Provider } from "@ethersproject/providers";
import type {
  TypedEventFilter,
  TypedEvent,
  TypedListener,
  OnEvent,
} from "../../../common";

export interface IPriceOracleV2Interface extends utils.Interface {
  functions: {
    "convert(uint256,address,address)": FunctionFragment;
    "convertFromUSD(uint256,address)": FunctionFragment;
    "convertToUSD(uint256,address)": FunctionFragment;
    "fastCheck(uint256,address,uint256,address)": FunctionFragment;
    "priceFeeds(address)": FunctionFragment;
    "version()": FunctionFragment;
  };

  getFunction(
    nameOrSignatureOrTopic:
      | "convert"
      | "convertFromUSD"
      | "convertToUSD"
      | "fastCheck"
      | "priceFeeds"
      | "version"
  ): FunctionFragment;

  encodeFunctionData(
    functionFragment: "convert",
    values: [BigNumberish, string, string]
  ): string;
  encodeFunctionData(
    functionFragment: "convertFromUSD",
    values: [BigNumberish, string]
  ): string;
  encodeFunctionData(
    functionFragment: "convertToUSD",
    values: [BigNumberish, string]
  ): string;
  encodeFunctionData(
    functionFragment: "fastCheck",
    values: [BigNumberish, string, BigNumberish, string]
  ): string;
  encodeFunctionData(functionFragment: "priceFeeds", values: [string]): string;
  encodeFunctionData(functionFragment: "version", values?: undefined): string;

  decodeFunctionResult(functionFragment: "convert", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "convertFromUSD",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "convertToUSD",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "fastCheck", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "priceFeeds", data: BytesLike): Result;
  decodeFunctionResult(functionFragment: "version", data: BytesLike): Result;

  events: {
    "NewPriceFeed(address,address)": EventFragment;
  };

  getEvent(nameOrSignatureOrTopic: "NewPriceFeed"): EventFragment;
}

export interface NewPriceFeedEventObject {
  token: string;
  priceFeed: string;
}
export type NewPriceFeedEvent = TypedEvent<
  [string, string],
  NewPriceFeedEventObject
>;

export type NewPriceFeedEventFilter = TypedEventFilter<NewPriceFeedEvent>;

export interface IPriceOracleV2 extends BaseContract {
  connect(signerOrProvider: Signer | Provider | string): this;
  attach(addressOrName: string): this;
  deployed(): Promise<this>;

  interface: IPriceOracleV2Interface;

  queryFilter<TEvent extends TypedEvent>(
    event: TypedEventFilter<TEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TEvent>>;

  listeners<TEvent extends TypedEvent>(
    eventFilter?: TypedEventFilter<TEvent>
  ): Array<TypedListener<TEvent>>;
  listeners(eventName?: string): Array<Listener>;
  removeAllListeners<TEvent extends TypedEvent>(
    eventFilter: TypedEventFilter<TEvent>
  ): this;
  removeAllListeners(eventName?: string): this;
  off: OnEvent<this>;
  on: OnEvent<this>;
  once: OnEvent<this>;
  removeListener: OnEvent<this>;

  functions: {
    convert(
      amount: BigNumberish,
      tokenFrom: string,
      tokenTo: string,
      overrides?: CallOverrides
    ): Promise<[BigNumber]>;

    convertFromUSD(
      amount: BigNumberish,
      token: string,
      overrides?: CallOverrides
    ): Promise<[BigNumber]>;

    convertToUSD(
      amount: BigNumberish,
      token: string,
      overrides?: CallOverrides
    ): Promise<[BigNumber]>;

    fastCheck(
      amountFrom: BigNumberish,
      tokenFrom: string,
      amountTo: BigNumberish,
      tokenTo: string,
      overrides?: CallOverrides
    ): Promise<
      [BigNumber, BigNumber] & {
        collateralFrom: BigNumber;
        collateralTo: BigNumber;
      }
    >;

    priceFeeds(token: string, overrides?: CallOverrides): Promise<[string]>;

    version(overrides?: CallOverrides): Promise<[BigNumber]>;
  };

  convert(
    amount: BigNumberish,
    tokenFrom: string,
    tokenTo: string,
    overrides?: CallOverrides
  ): Promise<BigNumber>;

  convertFromUSD(
    amount: BigNumberish,
    token: string,
    overrides?: CallOverrides
  ): Promise<BigNumber>;

  convertToUSD(
    amount: BigNumberish,
    token: string,
    overrides?: CallOverrides
  ): Promise<BigNumber>;

  fastCheck(
    amountFrom: BigNumberish,
    tokenFrom: string,
    amountTo: BigNumberish,
    tokenTo: string,
    overrides?: CallOverrides
  ): Promise<
    [BigNumber, BigNumber] & {
      collateralFrom: BigNumber;
      collateralTo: BigNumber;
    }
  >;

  priceFeeds(token: string, overrides?: CallOverrides): Promise<string>;

  version(overrides?: CallOverrides): Promise<BigNumber>;

  callStatic: {
    convert(
      amount: BigNumberish,
      tokenFrom: string,
      tokenTo: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    convertFromUSD(
      amount: BigNumberish,
      token: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    convertToUSD(
      amount: BigNumberish,
      token: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    fastCheck(
      amountFrom: BigNumberish,
      tokenFrom: string,
      amountTo: BigNumberish,
      tokenTo: string,
      overrides?: CallOverrides
    ): Promise<
      [BigNumber, BigNumber] & {
        collateralFrom: BigNumber;
        collateralTo: BigNumber;
      }
    >;

    priceFeeds(token: string, overrides?: CallOverrides): Promise<string>;

    version(overrides?: CallOverrides): Promise<BigNumber>;
  };

  filters: {
    "NewPriceFeed(address,address)"(
      token?: string | null,
      priceFeed?: string | null
    ): NewPriceFeedEventFilter;
    NewPriceFeed(
      token?: string | null,
      priceFeed?: string | null
    ): NewPriceFeedEventFilter;
  };

  estimateGas: {
    convert(
      amount: BigNumberish,
      tokenFrom: string,
      tokenTo: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    convertFromUSD(
      amount: BigNumberish,
      token: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    convertToUSD(
      amount: BigNumberish,
      token: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    fastCheck(
      amountFrom: BigNumberish,
      tokenFrom: string,
      amountTo: BigNumberish,
      tokenTo: string,
      overrides?: CallOverrides
    ): Promise<BigNumber>;

    priceFeeds(token: string, overrides?: CallOverrides): Promise<BigNumber>;

    version(overrides?: CallOverrides): Promise<BigNumber>;
  };

  populateTransaction: {
    convert(
      amount: BigNumberish,
      tokenFrom: string,
      tokenTo: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    convertFromUSD(
      amount: BigNumberish,
      token: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    convertToUSD(
      amount: BigNumberish,
      token: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    fastCheck(
      amountFrom: BigNumberish,
      tokenFrom: string,
      amountTo: BigNumberish,
      tokenTo: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    priceFeeds(
      token: string,
      overrides?: CallOverrides
    ): Promise<PopulatedTransaction>;

    version(overrides?: CallOverrides): Promise<PopulatedTransaction>;
  };
}