#![cfg(test)]

use crate::{QuickPaySmeContract, QuickPaySmeContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::token::{Client as TokenClient, StellarAssetClient as TokenAdminClient};

/// Helper function to setup the environment with a mock USDC token
fn setup_env() -> (Env, QuickPaySmeContractClient, Address, Address, TokenClient) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, QuickPaySmeContract);
    let client = QuickPaySmeContractClient::new(&env, &contract_id);

    let merchant = Address::generate(&env);
    let customer = Address::generate(&env);

    // Setup mock USDC token
    let token_admin = Address::generate(&env);
    let token_contract_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = TokenClient::new(&env, &token_contract_id.address());
    let token_admin_client = TokenAdminClient::new(&env, &token_contract_id.address());

    // Mint 1000 "USDC" to the customer for testing
    token_admin_client.mint(&customer, &1000);

    (env, client, merchant, customer, token_client)
}

// TEST 1 (Happy path): The MVP transaction executes successfully end-to-end
#[test]
fn test_happy_path_payment() {
    let (_, client, merchant, customer, token_client) = setup_env();
    client.initialize(&merchant, &token_client.address);

    // Customer pays 150 USDC
    client.pay_merchant(&customer, &150);

    // Assert merchant received funds and customer balance decreased
    assert_eq!(token_client.balance(&merchant), 150);
    assert_eq!(token_client.balance(&customer), 850);
}

// TEST 2 (Edge case): Invalid amount failure scenario
#[test]
#[should_panic(expected = "Payment amount must be greater than zero")]
fn test_invalid_amount_fails() {
    let (_, client, merchant, customer, token_client) = setup_env();
    client.initialize(&merchant, &token_client.address);

    // Trying to pay 0 should fail
    client.pay_merchant(&customer, &0);
}

// TEST 3 (State verification): Assert that contract storage reflects correct state
#[test]
fn test_state_verification() {
    let (_, client, merchant, customer, token_client) = setup_env();
    client.initialize(&merchant, &token_client.address);

    // Process two separate payments
    client.pay_merchant(&customer, &50);
    client.pay_merchant(&customer, &75);

    // Check state verification via get_stats
    let (count, volume) = client.get_stats();
    assert_eq!(count, 2, "Payment count should be 2");
    assert_eq!(volume, 125, "Total volume should be 125");
}

// TEST 4 (Edge case): Initialization failure scenario (Duplicate entry)
#[test]
#[should_panic(expected = "Already initialized")]
fn test_already_initialized_fails() {
    let (_, client, merchant, _, token_client) = setup_env();
    
    client.initialize(&merchant, &token_client.address);
    // Calling initialize a second time should fail
    client.initialize(&merchant, &token_client.address);
}

// TEST 5 (Edge case): Uninitialized contract scenario
#[test]
#[should_panic(expected = "Not initialized")]
fn test_uninitialized_payment_fails() {
    let (_, client, _, customer, _) = setup_env();
    
    // Attempting to pay before initialize() was called
    client.pay_merchant(&customer, &100);
}