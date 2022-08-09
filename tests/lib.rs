#![allow(clippy::type_complexity)]

use {
  self::{state::State, test::Test, transaction_options::TransactionOptions},
  bdk::{
    blockchain::{
      rpc::{RpcBlockchain, RpcConfig},
      ConfigurableBlockchain,
    },
    database::MemoryDatabase,
    keys::bip39::Mnemonic,
    template::Bip84,
    wallet::{signer::SignOptions, AddressIndex, SyncOptions, Wallet},
    KeychainKind,
  },
  bitcoin::{hash_types::Txid, network::constants::Network, Address, Block, OutPoint},
  bitcoincore_rpc::{Client, RawTx, RpcApi},
  executable_path::executable_path,
  log::LevelFilter,
  regex::Regex,
  std::{
    collections::BTreeMap,
    fs,
    net::TcpListener,
    process::{Child, Command, Stdio},
    str::{self, FromStr},
    sync::Once,
    thread::sleep,
    time::Duration,
  },
  tempfile::TempDir,
  unindent::Unindent,
};

mod epochs;
mod find;
mod index;
mod info;
mod list;
mod name;
mod nft;
mod range;
mod server;
mod state;
mod supply;
mod test;
mod traits;
mod transaction_options;
mod version;
mod wallet;