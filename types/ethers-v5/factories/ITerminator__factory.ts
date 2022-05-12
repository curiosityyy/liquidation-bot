/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Signer, utils } from "ethers";
import { Provider } from "@ethersproject/providers";
import type { ITerminator, ITerminatorInterface } from "../ITerminator";

const _abi = [
  {
    inputs: [
      {
        internalType: "address",
        name: "_executor",
        type: "address",
      },
    ],
    name: "allowExecutor",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "",
        type: "address",
      },
    ],
    name: "executors",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "_executor",
        type: "address",
      },
    ],
    name: "forbidExecutor",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address",
        name: "_creditManager",
        type: "address",
      },
      {
        internalType: "address",
        name: "_borrower",
        type: "address",
      },
      {
        components: [
          {
            internalType: "uint256",
            name: "amountIn",
            type: "uint256",
          },
          {
            internalType: "address[]",
            name: "path",
            type: "address[]",
          },
          {
            internalType: "uint256",
            name: "amountOutMin",
            type: "uint256",
          },
        ],
        internalType: "struct ITerminator.UniV2Params[]",
        name: "_routes",
        type: "tuple[]",
      },
      {
        internalType: "address[]",
        name: "_yearnTokens",
        type: "address[]",
      },
    ],
    name: "liquidate",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "address[]",
        name: "creditManagers",
        type: "address[]",
      },
      {
        internalType: "address[]",
        name: "tokens",
        type: "address[]",
      },
    ],
    name: "provideAllowance",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
];

export class ITerminator__factory {
  static readonly abi = _abi;
  static createInterface(): ITerminatorInterface {
    return new utils.Interface(_abi) as ITerminatorInterface;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): ITerminator {
    return new Contract(address, _abi, signerOrProvider) as ITerminator;
  }
}