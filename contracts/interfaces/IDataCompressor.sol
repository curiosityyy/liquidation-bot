// SPDX-License-Identifier: GPL-2.0-or-later
// Gearbox Protocol. Generalized leverage for DeFi protocols
// (c) Gearbox Holdings, 2021
pragma solidity ^0.8.10;

import {CreditAccountData, CreditManagerData, PoolData, TokenInfo} from "../libraries/Types.sol";
import {IVersion} from "./IVersion.sol";

interface IDataCompressorExceptions {
    /// @dev throws if provided address is not registered CreditManager
    error NotCreditManagerException();

    /// @dev throws if provided address is not registered Pool
    error NotPoolException();
}

interface IDataCompressor is IDataCompressorExceptions, IVersion {
    /// @dev Returns CreditAccountData for all opened account for particluar borrower
    /// @param borrower Borrower address
    function getCreditAccountList(address borrower)
        external
        view
        returns (CreditAccountData[] memory);

    function hasOpenedCreditAccount(address creditManager, address borrower)
        external
        view
        returns (bool);

    /// @dev Returns CreditAccountData for particular account for creditManager and borrower
    /// @param _creditManager Credit manager address
    /// @param borrower Borrower address
    function getCreditAccountData(address _creditManager, address borrower)
        external
        view
        returns (CreditAccountData memory);

    /// @dev Returns all credit managers data
    function getCreditManagersList()
        external
        view
        returns (CreditManagerData[] memory);

    /// @dev Returns CreditManagerData for particular _creditManager and
    /// set flg hasOpenedCreditAccount for provided borrower
    /// @param _creditManager CreditManager address
    function getCreditManagerData(address _creditManager)
        external
        view
        returns (CreditManagerData memory);

    /// @dev Returns PoolData for particulr pool
    /// @param _pool Pool address
    function getPoolData(address _pool) external view returns (PoolData memory);

    /// @dev Returns PoolData for all registered pools
    function getPoolsList() external view returns (PoolData[] memory);

    /// @dev Returns compressed token data for particular token.
    /// Be careful, it can be reverted for non-standart tokens which has no "symbol" method for example
    function getTokenData(address[] memory addr)
        external
        view
        returns (TokenInfo[] memory);
}
