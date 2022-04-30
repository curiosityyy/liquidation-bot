// SPDX-License-Identifier: MIT
pragma solidity ^0.8.10;
pragma abicoder v2;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {SafeMath} from "@openzeppelin/contracts/utils/math/SafeMath.sol";

import {Constants} from "./Constants.sol";
import {IUniswapV2Router02} from "./integrations/IUniswapV2Router02.sol";
import {IYVault} from "./integrations/IYVault.sol";

import {ICreditFacade, MultiCall} from "./interfaces/ICreditFacade.sol";
import {ICreditManagerV2} from "./interfaces/ICreditManagerV2.sol";

/// @dev Terminator is an example of smartcontract which is used for liquidation bots
/// Current version gets information from the bot, withdraw all YEARN tokens and then
/// convert everything on Uniswap V2 protocol into underlying asset.
///
/// This is just an example contract, do not use it in productuion.
/// This constract is not audited, use it on your own risk.

contract Terminator is Ownable {
    using SafeMath for uint256;
    using SafeERC20 for IERC20;

    // List of yearn vaults connected
    address[] public yearn;

    // WETH token address
    address public wethToken;

    // Set or executros. Executors could call liquidate function.
    // This role is design to make parallel liquidations from different liquidaiton
    // without waiting confirmations for your address
    mapping(address => bool) public executors;

    // Parameters for Uniswap V2 which provides information
    struct UniV2Params {
        uint256 amountIn;
        address[] path;
        uint256 amountOutMin;
    }

    modifier executorOnly() {
        require(executors[msg.sender], "For executors only");
        _;
    }

    constructor(address _wethToken) {
        wethToken = _wethToken;
    }

    // Adds an executor to the set
    function allowExecutor(address _executor) external onlyOwner {
        executors[_executor] = true;
    }

    // Removes executor from the set
    function forbidExecutor(address _executor) external onlyOwner {
        executors[_executor] = false;
    }

    // Adds yVault to the list
    function addYearn(address _yearn) external onlyOwner {
        yearn.push(_yearn);
    }

    /// @dev Liquidates account and sell all assets in Uniswap V2
    /// @param _creditFacade address of credit facade
    /// @param _borrower address of borrower
    /// (the system finds credit manager by these 2 params, finds credit account by credit manager and _borrower)
    /// @param skipTokenMask Tokenmask contains 1 for tokens which needed to be skipped for sending
    /// @param convertWETH It true, it converts WETH token into ETH when sends it to "to" address
    /// @param calls Multicall structure for calls. Basic usage is to place addCollateral calls to provide collateral in
    /// @param _router Address of UniV2 router
    /// @param _paths Data for uniswap - how to swap all assets into underlying one
    function liquidateAndSellOnV2(
        address _creditFacade,
        address _borrower,
        uint256 skipTokenMask,
        bool convertWETH,
        MultiCall[] calldata calls,
        address _router,
        UniV2Params[] calldata _paths
    ) external executorOnly {
        // Getting creditFacade, creditManager and underlyingToken
        ICreditFacade creditFacade = ICreditFacade(_creditFacade);
        ICreditManagerV2 creditManager = ICreditManagerV2(
            creditFacade.creditManager()
        );

        uint256[] memory balancesBefore = getBalanceBefore(creditManager, _borrower, skipTokenMask, _paths.length);

        // Providing allowance for creditManager to withdraw liquidation amount
        _provideAllowance(
            address(creditManager),
            creditManager.underlying()
        );

        // @dev Run a bunch of transactions (multicall) and then liquidate credit account
        // - Wraps ETH to WETH and sends it msg.sender (liquidator) is value > 0
        // - It checks that hf < 1, otherwise it reverts
        // - It computes the amount which should be paid back: borrowed amount + interest + fees
        // - Executes multicall functions for it (the main function is to swap all assets into undelying one)
        // - Close credit account:
        //    + It checks underlying token balance, if it > than funds need to be paid to pool, the debt is paid
        //      by funds from creditAccount
        //    + if there is no enough funds in credit Account, it withdraws all funds from credit account, and then
        //      transfers the diff from msg.sender address
        //    + Then, if sendAllAssets is false, it transfers all non-zero balances from credit account to address "to".
        //      Otherwise no transfers would be made. If liquidator is confident that all assets were transffered
        //      During multicall, this option could save gas costs.
        //    + If convertWETH is true, the function converts WETH into ETH on the fly
        // - Emits LiquidateCreditAccount event
        //
        // @param to Address to send funds during closing contract operation
        // @param skipTokenMask Tokenmask contains 1 for tokens which needed to be skipped for sending
        // @param convertWETH It true, it converts WETH token into ETH when sends it to "to" address
        // @param calls Multicall structure for calls. Basic usage is to place addCollateral calls to provide collateral in
        //   assets that differ than undelyring one 
        creditFacade.liquidateCreditAccount(_borrower, address(this), skipTokenMask, convertWETH, calls);

        sellAllTokensOnV2(creditManager, _borrower, skipTokenMask, balancesBefore, _router, _paths);
    }

    function getBalanceBefore(
        ICreditManagerV2 creditManager,
        address _borrower,
        uint256 skipTokenMask,
        uint256 path_length
    ) internal view returns (uint256[] memory) {
        // Provides address of credit account of reverts. It'll revert if someone already liquidate it
        address creditAccount = creditManager.getCreditAccountOrRevert(
            _borrower
        );

        // Gets the quantity of allowed tokens
        uint256 allowedTokenQty = creditManager.allowedTokensCount();

        // Storing balances, they will not be available after liquidation
        uint256[] memory balancesBefore = new uint256[](allowedTokenQty);

        // Getting enabledTokens - token mask which represents non-zero balances
        uint256 enabledTokens = creditManager.enabledTokensMap(creditAccount);
        enabledTokens &= ~skipTokenMask;

        uint256 tokenMask;

        // Checks that parameters array which provide params for swaping assets has the same size
        require(path_length == allowedTokenQty, "Incorrect path length");
        
               // Before executing liquidation method, we keep all balances of this smart contract to be able to
        // compute how many tokens of each asset we get during liquidation.
        // Terminator keeps different tokens to be able to liquidate different pools, it's why
        // it's crucial to distinguish balance we've already have and how many tokens we get.
        for (uint256 i = 1; i < allowedTokenQty;) {
            // Tokenmaks is a but mask which is used for gas saving during iteration for allowed tokens list
            tokenMask = 1 << i;
            if (enabledTokens & tokenMask > 0) {
                address token = creditManager.allowedTokens(i);
                balancesBefore[i] = IERC20(token).balanceOf(address(this));
            }
            unchecked {
                ++i;
            }
        }

        return balancesBefore;
    }

    function sellAllTokensOnV2(
        ICreditManagerV2 creditManager,
        address _borrower,
        uint256 skipTokenMask,
        uint256[] memory balancesBefore,
        address _router,
        UniV2Params[] calldata _paths
    ) internal {
        // Provides address of credit account of reverts. It'll revert if someone already liquidate it
        address creditAccount = creditManager.getCreditAccountOrRevert(
            _borrower
        );

        // Gets the quantity of allowed tokens
        uint256 allowedTokenQty = creditManager.allowedTokensCount();

        // Getting enabledTokens - token mask which represents non-zero balances
        uint256 enabledTokens = creditManager.enabledTokensMap(creditAccount);
        enabledTokens &= ~skipTokenMask;

        uint256 tokenMask;


        // At first stage, we just withdraw all yearn assets we got during liquidation
        for (uint256 i = 0; i < yearn.length; i++) {
            if (IERC20(yearn[i]).balanceOf(address(this)) > 1) {
                IYVault(yearn[i]).withdraw();
            }
        }

        // The last thing is to swap all assets into underlying one
        // We compare current balace with stored one, and if it has difference
        // swap it into underlying account
        for (uint256 i = 1; i < allowedTokenQty;) {
            tokenMask = 1 << i;
            if (enabledTokens & tokenMask > 0) {
                address token = creditManager.allowedTokens(i);
                uint256 balance = IERC20(token).balanceOf(address(this));

                if (balance > balancesBefore[i] + 1) {
                    balance = balance - balancesBefore[i] - 1;
                    require(token == _paths[i].path[0], "incorrect path");
                    sellTokensOnV2(_router, balance, _paths[i]);
                }
            }
            unchecked {
                ++i;
            }
        }
    }

    /// @dev Internal function which facilitate swap assets on Uniswap V2
    function sellTokensOnV2(
        address _router,
        uint256 amount,
        UniV2Params calldata paths
    ) internal {
        _provideAllowance(_router, paths.path[0]);

        // Computes amountOut based on rate provided as parameter
        uint256 amountOut = paths.amountOutMin.mul(amount).div(paths.amountIn);

        // By design, gearbox transfers eth if credit account has WETH position
        // So, we swap ETH using swapExactETHForTokens
        if (paths.path[0] == wethToken) {
            IUniswapV2Router02(_router).swapExactETHForTokens{value: amount}(
                amountOut,
                paths.path,
                address(this),
                block.timestamp
            );
        } else {
            IUniswapV2Router02(_router).swapExactTokensForTokens(
                amount,
                amountOut,
                paths.path,
                address(this),
                block.timestamp
            );
        }
    }

    /// @dev Sends tokens back to owner. Helps to take the profits back
    function transferToOwner(address token, uint256 amount) external onlyOwner {
        IERC20(token).safeTransfer(msg.sender, amount);
    }

    /// @dev Provides needed allowance
    function _provideAllowance(address externalContract, address token)
        internal
    {
        if (
            ERC20(token).allowance(address(this), externalContract) <
            Constants.MAX_INT_4
        ) {
            ERC20(token).approve(externalContract, Constants.MAX_INT);
        }
    }

    /// @dev Allows contract to get ETH during liquidation
    receive() external payable {}
}
