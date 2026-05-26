#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contract]
pub struct QuickPaySmeContract;

/// Storage keys to maintain contract state
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Merchant,     // Address of the SME merchant receiving funds
    UsdcToken,    // Address of the accepted USDC token contract
    PaymentCount, // Number of payments processed
    TotalVolume,  // Total amount of USDC processed
}

#[contractimpl]
impl QuickPaySmeContract {
    /// Initializes the contract.
    /// Sets the merchant address and the specific USDC token accepted.
    pub fn initialize(env: Env, merchant: Address, usdc_token: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Merchant),
            "Already initialized"
        );
        env.storage().instance().set(&DataKey::Merchant, &merchant);
        env.storage().instance().set(&DataKey::UsdcToken, &usdc_token);
        env.storage().instance().set(&DataKey::PaymentCount, &0u32);
        env.storage().instance().set(&DataKey::TotalVolume, &0i128);
    }

    /// Processes a payment from the customer to the merchant.
    /// Maps directly to: Customer scans QR -> Sends USDC -> Contract records -> Merchant receives
    pub fn pay_merchant(env: Env, customer: Address, amount: i128) {
        // Ensure the customer actually signed this transaction
        customer.require_auth();
        
        // Validation: Ensure the payment amount is positive
        assert!(amount > 0, "Payment amount must be greater than zero");

        // Retrieve merchant and token details from state
        let merchant: Address = env.storage().instance().get(&DataKey::Merchant).expect("Not initialized");
        let usdc_token: Address = env.storage().instance().get(&DataKey::UsdcToken).expect("Not initialized");

        // Transfer funds directly from the customer to the merchant
        let client = token::Client::new(&env, &usdc_token);
        client.transfer(&customer, &merchant, &amount);

        // Update state: Increment transaction count and add to total volume
        let mut count: u32 = env.storage().instance().get(&DataKey::PaymentCount).unwrap();
        let mut volume: i128 = env.storage().instance().get(&DataKey::TotalVolume).unwrap();
        
        count += 1;
        volume += amount;

        env.storage().instance().set(&DataKey::PaymentCount, &count);
        env.storage().instance().set(&DataKey::TotalVolume, &volume);
    }

    /// Retrieves current payment statistics (can be used for an external dashboard)
    pub fn get_stats(env: Env) -> (u32, i128) {
        let count: u32 = env.storage().instance().get(&DataKey::PaymentCount).unwrap_or(0);
        let volume: i128 = env.storage().instance().get(&DataKey::TotalVolume).unwrap_or(0);
        (count, volume)
    }
}