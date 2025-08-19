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

### AMM Integration

The Automated Market Maker (AMM) provides continuous liquidity for the perpetual futures markets:

**Key Features**:
- **Constant Product Formula**: Uses x * y = k formula for price discovery
- **Oracle Price Weighting**: Combines AMM pricing with oracle feeds for stability
- **Dynamic Reserves**: Automatically adjusts base and quote asset reserves
- **Price Impact Calculation**: Determines price impact based on trade size
- **Funding Rate Mechanism**: Implements funding rate calculations for perpetual markets

**AMM Functions**:
- `calculate_quote_for_base_with_limit`: Calculates quote amount with price limits
- `calculate_quote_for_base_no_limit`: Calculates quote amount without price limits
- `execute_trade`: Executes trades and updates reserves
- `get_bid_price` / `get_ask_price`: Provides current bid/ask prices

## Order Execution Logic

### Fill Method Selection
The system intelligently chooses between:
1. **Direct Order Matching**: When better prices exist in the order book
2. **AMM Liquidity**: For remaining unfilled amounts or when AMM provides better pricing
3. **Hybrid Approach**: Optimal combination of both methods

### Fulfillment Methods (`get_types_of_filling`)

The `get_types_of_filling` function determines the optimal execution strategy for each order by analyzing available liquidity sources:

**Function Purpose**: Analyzes order book and AMM to determine the best execution path
**Parameters**:
- `order`: The order to be filled
- `maker_id_index_price`: Available maker orders with prices
- `amm`: Current AMM state and pricing
- `limit_price`: Optional price limit for the taker

**Execution Strategy**:
1. **Maker Order Analysis**: Evaluates each available maker order for price improvement
2. **Price Comparison**: Compares maker prices against AMM pricing
3. **Crossing Logic**: Determines if orders can cross (taker willing to pay maker's price)
4. **Priority Ranking**: Orders execution methods by price priority (best first)
5. **AMM Fallback**: Uses AMM liquidity for remaining unfilled amounts

**Return Value**: Vector of `FullfillmentMethod` enum values:
- `AMM(Option<u64>)`: AMM execution with optional price limit
- `Match(Pubkey, u16, u64)`: Direct order matching with maker details

### Fill Functions

#### `fill_with_amm(user, order_index, limit_price, market)`

**Purpose**: Executes order filling through the Automated Market Maker
**Parameters**:
- `user`: User account with the order
- `order_index`: Index of the order to fill
- `limit_price`: Optional maximum price for the fill
- `market`: Market account with AMM configuration

**What it does**:
1. **Amount Calculation**: Determines base asset amount to fill
2. **Quote Calculation**: Computes required quote asset amount based on AMM pricing
3. **Price Validation**: Ensures fill price meets limit requirements
4. **AMM Execution**: Executes the trade through the AMM algorithm
5. **Order Update**: Updates order status and fill amounts
6. **Position Tracking**: Maintains accurate position records

**Return Value**: Tuple of (base_asset_amount_filled, quote_asset_amount_filled)

#### `fill_with_match(taker, taker_order_index, taker_limit_price, maker, maker_order_index, maker_price, maker_fill_map)`

**Purpose**: Executes direct order matching between taker and maker orders
**Parameters**:
- `taker`: User placing the order to be filled
- `taker_order_index`: Index of taker's order
- `taker_limit_price`: Maximum price taker is willing to pay
- `maker`: User with the opposite order
- `maker_order_index`: Index of maker's order
- `maker_price`: Price of maker's order
- `maker_fill_map`: Tracking map for maker fills

**What it does**:
1. **Direction Validation**: Ensures orders are in opposite directions (long vs short)
2. **Crossing Check**: Verifies if orders can cross at maker's price
3. **Fill Calculation**: Determines fillable amounts for both orders
4. **Order Updates**: Updates both taker and maker order status
5. **Fill Tracking**: Maintains maker fill map for position calculations
6. **Status Management**: Updates order status to filled when complete

**Return Value**: Tuple of (base_asset_amount_filled, quote_asset_amount_filled)

### Helper Functions

#### `update_order_after_filling(order, base_asset_amount, quote_asset_amount)`
**Purpose**: Updates order status and fill amounts after execution
**What it does**:
- Increments filled amounts for base and quote assets
- Updates order status to "Filled" when completely filled
- Maintains accurate order tracking

#### `calculate_fill_by_match(maker_base_asset_amount, maker_price, taker_base_asset_amount)`
**Purpose**: Calculates the actual fillable amounts between two orders
**What it does**:
- Determines the minimum fillable amount between maker and taker
- Calculates corresponding quote asset amount based on maker's price
- Ensures no over-filling occurs

#### `does_order_cross(maker_direction, maker_order_price, limit_price)`
**Purpose**: Determines if an order can cross at the given price
**Logic**:
- **Long Orders**: Cross when taker's limit price > maker's ask price
- **Short Orders**: Cross when taker's limit price < maker's bid price

### Execution Flow Summary

1. **Order Analysis**: `get_types_of_filling` analyzes available liquidity
2. **Method Selection**: Chooses optimal execution path (matching vs AMM)
3. **Order Execution**: 
   - Direct matching via `fill_with_match`
   - AMM liquidity via `fill_with_amm`
4. **Status Updates**: Updates order status and fill amounts
5. **Position Tracking**: Maintains accurate position records

### Price Priority
- Orders are filled based on price priority (best price first)
- Market orders execute immediately at best available prices
- Limit orders only execute when price conditions are met

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
