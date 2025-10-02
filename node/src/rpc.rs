//! A collection of node-specific RPC methods.

use std::sync::Arc;

use glin_runtime::{opaque::Block, AccountId, Balance, Index};
use jsonrpsee::RpcModule;
use polkadot_sdk::{
    sc_transaction_pool_api::{self, TransactionPool},
    sp_api::{self, ProvideRuntimeApi},
    sp_block_builder::{self, BlockBuilder},
    sp_blockchain::{self, Error as BlockChainError, HeaderBackend, HeaderMetadata},
    sc_rpc_api::{self},
    substrate_frame_rpc_system::{self},
    pallet_transaction_payment_rpc::{self},
};

pub use polkadot_sdk::sc_rpc_api::DenyUnsafe;

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcModule::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe: _,
    } = deps;

    module.merge(System::new(client.clone(), pool.clone()).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    Ok(module)
}