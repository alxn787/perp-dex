# Perp-DEX

A decentralized perpetual futures exchange built on Solana using Anchor framework. This system enables users to trade perpetual futures with leverage, featuring an order book and AMM-based liquidity.

## Overview

Perp-DEX is a sophisticated perpetual futures trading platform that combines traditional order book mechanics with automated market maker (AMM) liquidity. The system supports leveraged trading, multiple markets, and real-time price feeds through oracles.

## Architecture

The system consists of several key components:
- **State Management**: Global state and configuration
- **Market Management**: Individual perpetual futures markets
- **User Management**: User accounts and positions
- **Order Management**: Order placement, matching, and execution
- **AMM Integration**: Automated market maker for liquidity provision

## Order Flow

### 1. User Registration & Setup
```
User → Initialize User Account → Deposit Collateral → Ready to Trade
```

### 2. Order Placement Flow
```
User → Place Order → Order Validation → Order Storage → Order Book Update
```

**Order Types Supported:**
- **Market Orders**: Execute immediately at best available price
- **Limit Orders**: Execute only at specified price or better
- **Direction**: Long (buy) or Short (sell)
- **Leverage**: Configurable leverage up to maximum allowed

### 3. Order Execution Flow
```
Order Matching → Fill Order → Position Update → Collateral Adjustment → Order Status Update
```

**Execution Methods:**
1. **Order Matching**: Direct matching with opposite orders
2. **AMM Liquidity**: Filling remaining orders through AMM
3. **Hybrid**: Combination of both methods for optimal execution

### 4. Position Management
```
Position Creation → Collateral Management → Leverage Control → Risk Management
```

## Main Functions

### Core System Functions

#### `initialize_state(perp_fee: u64)`
- **Purpose**: Initializes the global state of the system
- **Parameters**: 
  - `perp_fee`: Fee percentage for perpetual trading
- **What it does**: Sets up the foundational system parameters and fee structure

#### `initialize_market(market_params: InitializeMarketParams)`
- **Purpose**: Creates a new perpetual futures market
- **Parameters**: Market configuration including reserves, leverage limits, and margin requirements
- **What it does**: 
  - Creates market account with AMM configuration
  - Sets up market vault for token storage
  - Configures maximum leverage and margin ratios
  - Initializes AMM with base and quote asset reserves

#### `initialize_user(account_id: u16)`
- **Purpose**: Creates a new user account for trading
- **Parameters**: 
  - `account_id`: Unique identifier for the user account
- **What it does**: 
  - Creates user account with order management capabilities
  - Initializes order tracking and position management
  - Sets up user authority and account structure

### Trading Functions

#### `place_order(order_params: OrderParams)`
- **Purpose**: Places a new trading order
- **Parameters**: Order details including price, amount, direction, leverage, and market
- **What it does**:
  - Validates order parameters (amount, leverage, price limits)
  - Creates order record in user's order book
  - Updates user's position tracking
  - Manages order ID generation and status

#### `fill_order(order_id: Option<u64>)`
- **Purpose**: Executes order matching and filling
- **Parameters**: 
  - `order_id`: Specific order to fill (optional, defaults to last order)
- **What it does**:
  - Identifies matching orders across users
  - Executes trades through order matching or AMM
  - Updates positions and collateral balances
  - Manages order status and fill amounts

### Liquidity Management

#### `deposit(market_index: u16, amount: u64)`
- **Purpose**: Deposits collateral into a specific market
- **Parameters**: 
  - `market_index`: Target market identifier
  - `amount`: Token amount to deposit
- **What it does**:
  - Transfers tokens to market vault
  - Updates user's collateral balance
  - Creates or updates market position

#### `withdraw(market_index: u16, amount: u64)`
- **Purpose**: Withdraws collateral from a market position
- **Parameters**: 
  - `market_index`: Source market identifier
  - `amount`: Token amount to withdraw
- **What it does**:
  - Validates withdrawal eligibility
  - Transfers tokens from vault to user
  - Updates collateral balances and positions

### Oracle Management

#### `initialize_oracle(market_index, initial_price, confidence_interval, max_deviation)`
- **Purpose**: Sets up price oracle for a market
- **What it does**: Configures price feed with confidence intervals and deviation limits

#### `update_oracle_price(market_index, new_price)`
- **Purpose**: Updates current market price from oracle
- **What it does**: Refreshes price data and triggers AMM price updates

## Order Execution Logic

### Fill Method Selection
The system intelligently chooses between:
1. **Direct Order Matching**: When better prices exist in the order book
2. **AMM Liquidity**: For remaining unfilled amounts or when AMM provides better pricing
3. **Hybrid Approach**: Optimal combination of both methods

### Price Priority
- Orders are filled based on price priority (best price first)
- Market orders execute immediately at best available prices
- Limit orders only execute when price conditions are met

### Leverage and Risk Management
- Maximum leverage limits per market
- Initial and maintenance margin requirements
- Automatic position monitoring and risk controls

## Technical Features

- **Solana Native**: Built on Solana blockchain for high performance
- **Anchor Framework**: Leverages Anchor for secure smart contract development
- **Token Standards**: SPL token integration for asset management
- **Account Management**: PDA-based account structure for security
- **Error Handling**: Comprehensive error management and validation

## Getting Started

1. **Setup Environment**: Install Solana CLI and Anchor
2. **Build**: `anchor build`
3. **Deploy**: `anchor deploy`
4. **Initialize**: Set up state, markets, and oracles
5. **Trade**: Start placing and filling orders

## Security Features

- **Authority Validation**: Strict user authority checks
- **Parameter Validation**: Comprehensive input validation
- **Arithmetic Safety**: Overflow protection and safe math operations
- **Access Control**: Restricted access to administrative functions

This system provides a robust foundation for decentralized perpetual futures trading with advanced order management, risk controls, and liquidity provision mechanisms.
