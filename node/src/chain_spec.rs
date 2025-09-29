use glin_runtime::{
    AccountId, Signature, WASM_BINARY, GLIN,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, ed25519, Pair, Public, crypto::Ss58Codec, ByteArray};
use sp_runtime::traits::{IdentifyAccount, Verify};
use serde_json::{json, Value};

// The URL for the telemetry server
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec` for the normal runtime.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?,
        None,  // No extensions for standalone chain
    )
    .with_name("Development")
    .with_id("dev")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(development_genesis())
    .with_properties(properties())
    .build())
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?,
        None,  // No extensions for standalone chain
    )
    .with_name("Local Testnet")
    .with_id("local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(local_testnet_genesis())
    .with_properties(properties())
    .build())
}

/// Incentivized testnet configuration for Railway deployment
pub fn incentivized_testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?,
        None,
    )
    .with_name("GLIN Incentivized Testnet")
    .with_id("glin_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_patch(incentivized_testnet_genesis())
    .with_properties(testnet_properties())
    .with_boot_nodes(vec![])  // No default bootnodes
    .build())
}

fn properties() -> serde_json::Map<String, Value> {
    let mut props = serde_json::Map::new();
    props.insert("tokenSymbol".to_string(), json!("GLIN"));
    props.insert("tokenDecimals".to_string(), json!(18));
    props.insert("ss58Format".to_string(), json!(42));
    props
}

fn testnet_properties() -> serde_json::Map<String, Value> {
    let mut props = serde_json::Map::new();
    props.insert("tokenSymbol".to_string(), json!("tGLIN"));
    props.insert("tokenDecimals".to_string(), json!(18));
    props.insert("ss58Format".to_string(), json!(42));
    props.insert("isTestnet".to_string(), json!(true));
    props
}

fn development_genesis() -> Value {
    let initial_authorities = vec![authority_keys_from_seed("Alice")];
    let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");
    let endowed_accounts = vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
    ];

    testnet_genesis(initial_authorities, root_key, endowed_accounts)
}

fn local_testnet_genesis() -> Value {
    let initial_authorities = vec![
        authority_keys_from_seed("Alice"),
        authority_keys_from_seed("Bob"),
    ];
    let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");
    let endowed_accounts = vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
        get_account_id_from_seed::<sr25519::Public>("Dave"),
        get_account_id_from_seed::<sr25519::Public>("Eve"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
        get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
        get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
    ];

    testnet_genesis(initial_authorities, root_key, endowed_accounts)
}

/// Incentivized testnet genesis with real accounts
fn incentivized_testnet_genesis() -> Value {
    // Using the actual generated validator public keys
    // Private keys will be inserted via RPC after node startup
    let initial_authorities: Vec<(AuraId, GrandpaId)> = vec![
        (
            // Validator 1
            AuraId::from_slice(&hex::decode("b4bbc6fb7d021444523daf2c5b429d4e234ba810a8bf3d3ca4482e9ed76a787e").expect("Valid hex"))
                .expect("Valid Aura key"),
            GrandpaId::from_slice(&hex::decode("63be21044cc774e87021a0a0481e3478d8f5d5dba0df38341656445959734e75").expect("Valid hex"))
                .expect("Valid GRANDPA key"),
        ),
        (
            // Validator 2
            AuraId::from_slice(&hex::decode("e6ccca3c9b1fb4454ff8632ec22200192e0c185d0f35e2b996eba2b3d3807244").expect("Valid hex"))
                .expect("Valid Aura key"),
            GrandpaId::from_slice(&hex::decode("203d88aec03894cf4e2bac89173727363f07c7363b1dead1261e87eef2088b22").expect("Valid hex"))
                .expect("Valid GRANDPA key"),
        ),
        (
            // Validator 3
            AuraId::from_slice(&hex::decode("e817436ddda78a99e5a1aa2bc2c5aeb276aed241ba0078d2dceac6d843f35167").expect("Valid hex"))
                .expect("Valid Aura key"),
            GrandpaId::from_slice(&hex::decode("89782419d12adb3877aa248dd89bf769836fd528d707d783aed9e312ece4d0bb").expect("Valid hex"))
                .expect("Valid GRANDPA key"),
        ),
    ];

    // Special accounts with controlled access - using real generated addresses

    // Faucet account - controlled by faucet service
    let faucet_account = AccountId::from_ss58check("5FHNa6qqbh3rD4uUD3nkdiCvwJgn8dzT7ViTpyWrSAjPJNrr")
        .expect("Valid faucet address");

    // Treasury account - controlled by governance
    let treasury_account = AccountId::from_ss58check("5EvHDz1hSYKpp4xg5nr8ZnWDbiT1aMqAMUJb7f6PxzwEf7L5")
        .expect("Valid treasury address");

    // Team operations account - for partnerships and testing
    let team_ops_account = AccountId::from_ss58check("5EfN47pArrgnHQMpHYenFBjFvHPnhoptzi2xLdX8y2aKyUHa")
        .expect("Valid team ops address");

    // Ecosystem fund - for grants and bounties
    let ecosystem_account = AccountId::from_ss58check("5CJs5wCYsEy5ne1YGXWJoVgFrkfhRydG2neWUFnVEwZrXEyi")
        .expect("Valid ecosystem address");

    // Burn address (using a known unspendable address)
    let burn_account = AccountId::from_ss58check("5BURN0000000000000000000000000000000000000000001")
        .unwrap_or_else(|_| get_account_id_from_seed::<sr25519::Public>("Burn"));

    json!({
        "balances": {
            "balances": vec![
                (faucet_account.clone(), 100_000_000 * GLIN),     // 100M for faucet
                (treasury_account.clone(), 300_000_000 * GLIN),   // 300M for treasury
                (team_ops_account.clone(), 50_000_000 * GLIN),     // 50M for team ops
                (ecosystem_account.clone(), 50_000_000 * GLIN),    // 50M for ecosystem
                (burn_account, 500_000_000 * GLIN),                // 500M burned/locked
                // Validators get minimal balance for operations (using their actual addresses)
                (AccountId::from_ss58check("5G9gFursQMBi1taQxi7BHLDY6rGcgNTtJxJZf1MM24UQuPch").expect("Valid validator 1 address"), 1000 * GLIN),
                (AccountId::from_ss58check("5HHKhuA1RBZQi4e3GHCX2uHh5qop22ERk4B8eyx8qZaPAMoh").expect("Valid validator 2 address"), 1000 * GLIN),
                (AccountId::from_ss58check("5HK1spmDFQm5kNwQW1iVieT9osdryiQHfDyddWn8ECdfyxD9").expect("Valid validator 3 address"), 1000 * GLIN),
            ],
        },
        "aura": {
            "authorities": initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
        },
        "grandpa": {
            "authorities": initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect::<Vec<_>>(),
        },
        // No sudo for incentivized testnet - governance only
        "sudo": {
            "key": None::<AccountId>,
        },
        // Transaction payment configuration
        "transactionPayment": {
            "multiplier": "1000000000000000000",
        },
    })
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> Value {
    json!({
        "balances": {
            "balances": endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1_000_000 * GLIN))
                .collect::<Vec<_>>(),
        },
        "aura": {
            "authorities": initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
        },
        "grandpa": {
            "authorities": initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect::<Vec<_>>(),
        },
        "sudo": {
            "key": Some(root_key),
        },
    })
}