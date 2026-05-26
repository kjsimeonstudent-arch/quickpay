# QuickPay SME
Instant USDC point-of-sale settlements on Stellar for small business merchants.

## Problem & Solution
**Problem**: Café owners and small merchants in urban Vietnam increasingly face customers who want to pay digitally. Unfortunately, settlement through local e-wallets takes 2-3 days, leaving the merchant short on the cash needed to buy fresh ingredients daily.

**Solution**: QuickPay SME solves this by enabling merchants to accept instant USDC payments via Stellar. Funds settle on-chain in seconds into the merchant's wallet, ensuring they maintain the liquidity necessary for daily operations and inventory survival.

## Timeline
* **Phase 1**: Core MVP (Soroban contract, USDC payments, state recording).
* **Phase 2**: Front-end Merchant Dashboard & QR generation integration.
* **Phase 3**: Built-in DEX/Anchor integration for auto-swapping USDC to local currency (VND).

## Stellar Features Used
* **USDC Transfers**: Low-cost, fast stablecoin routing.
* **Soroban Smart Contracts**: Trustless settlement and on-chain merchant sales tracking.
* **Trustlines**: Connecting the merchant's wallet to standard network assets.
* **Built-in DEX** (Upcoming): For immediate conversion to local currency.

## Vision and Purpose
To empower small, local merchants with enterprise-grade cash flow velocity, closing the gap between a customer's digital payment and a business owner's working capital.

## Prerequisites
* [Rust](https://rustup.rs/) (latest stable version)
* [Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) (`stellar-cli`, replacing the older `soroban-cli`)
* Target set to wasm32: `rustup target add wasm32-unknown-unknown`

## How to Build
To compile the contract into a WebAssembly (.wasm) file:
```bash
stellar contract build