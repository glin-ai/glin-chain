//! GLIN Substrate Node

#![warn(missing_docs)]

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

use polkadot_sdk::sc_cli;

fn main() -> sc_cli::Result<()> {
    command::run()
}