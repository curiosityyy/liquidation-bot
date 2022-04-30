/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Signer, utils } from "ethers";
import type { Provider } from "@ethersproject/providers";
import type {
  ICreditFacadeExceptions,
  ICreditFacadeExceptionsInterface,
} from "../../../../contracts/interfaces/ICreditFacade.sol/ICreditFacadeExceptions";

const _abi = [
  {
    inputs: [],
    name: "AccountTransferNotAllowedException",
    type: "error",
  },
  {
    inputs: [],
    name: "CantLiquidateWithSuchHealthFactorException",
    type: "error",
  },
  {
    inputs: [],
    name: "CantTransferLiquidatableAccountException",
    type: "error",
  },
  {
    inputs: [],
    name: "ContractNotAllowedException",
    type: "error",
  },
  {
    inputs: [],
    name: "CreditConfiguratorOnlyException",
    type: "error",
  },
  {
    inputs: [],
    name: "CreditManagerCallsForbiddenException",
    type: "error",
  },
  {
    inputs: [],
    name: "HasAlreadyOpenedCreditAccountInDegenMode",
    type: "error",
  },
  {
    inputs: [],
    name: "IncorrectCallDataLengthException",
    type: "error",
  },
  {
    inputs: [],
    name: "IncorrectOpenCreditAccountAmountException",
    type: "error",
  },
  {
    inputs: [],
    name: "IncreaseAndDecreaseForbiddenInOneCallException",
    type: "error",
  },
  {
    inputs: [],
    name: "IncreaseDebtForbiddenException",
    type: "error",
  },
  {
    inputs: [],
    name: "IntlCallsDuringClosureForbiddenException",
    type: "error",
  },
  {
    inputs: [],
    name: "NoDegenNFTInDegenModeException",
    type: "error",
  },
  {
    inputs: [],
    name: "TargetIsNotAdapterException",
    type: "error",
  },
  {
    inputs: [],
    name: "TokenNotAllowedException",
    type: "error",
  },
  {
    inputs: [],
    name: "UnknownMethodException",
    type: "error",
  },
];

export class ICreditFacadeExceptions__factory {
  static readonly abi = _abi;
  static createInterface(): ICreditFacadeExceptionsInterface {
    return new utils.Interface(_abi) as ICreditFacadeExceptionsInterface;
  }
  static connect(
    address: string,
    signerOrProvider: Signer | Provider
  ): ICreditFacadeExceptions {
    return new Contract(
      address,
      _abi,
      signerOrProvider
    ) as ICreditFacadeExceptions;
  }
}
