#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;
extern crate hyper;

use jsonrpc_client_http::HttpTransport;
use hyper::header::{Authorization, Basic};

#[derive(Deserialize)]
pub struct SerializedBlock {
    pub result: String,
}

#[derive(Deserialize)]
pub struct Block {
    pub hash: String,
    pub confirmations: i64,
    pub strippedsize: i64,
    pub size: i64,
    pub weight: i64,
    pub height: i64,
    pub version: i64,
    pub version_hex: String,
    pub merkleroot: String,
    pub tx: Vec<String>,
    pub time: i64,
    pub mediantime: i64,
    pub nonce: i64,
    pub bits: String,
    pub chainwork: String,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

#[derive(Deserialize)]
pub struct FullBlock {
    pub hash: String,
    pub confirmations: i64,
    pub strippedsize: i64,
    pub size: i64,
    pub weight: i64,
    pub height: i64,
    pub version: i64,
    pub version_hex: String,
    pub merkleroot: String,
    pub tx: Vec<Transaction>,
    pub time: i64,
    pub mediantime: i64,
    pub nonce: i64,
    pub bits: String,
    pub chainwork: String,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

#[derive(Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub hash: String,
    pub version: i64,
    pub size: i64,
    pub vsize: i64,
    pub locktime: i64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub hex: String,
}

#[derive(Deserialize)]
pub enum Vin {
    Tx(VinTx),
    Coinbase(VinCoinbase),
}

#[derive(Deserialize)]
pub struct VinTx {

}

#[derive(Deserialize)]
pub struct VinCoinbase {
    pub coinbase: String,
    pub sequence: i64
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: String,
    pub n: i64,
    pub script_pub_key: ScriptPubKey,
}

#[derive(Deserialize)]
pub enum GetBlockReplyOld {
    True(Block),
    False(SerializedBlock),
}

#[derive(Deserialize)]
pub enum GetBlockReply {
    Zero(SerializedBlock),
    One(Block),
    Two(FullBlock)
}

#[derive(Deserialize)]
pub struct Enforce {
    pub status: bool,
    pub found: i64,
    pub required: i64,
    pub window: i64,
}

#[derive(Deserialize)]
pub struct Reject {
    pub status: bool,
    pub found: i64,
    pub required: i64,
    pub window: i64,
}

#[derive(Deserialize)]
pub struct Softfork {
    pub id: String,
    pub version: i64,
    pub enforce: Enforce,
    pub reject: Reject,
}

#[derive(Deserialize)]
pub struct BlockChainInfo {
    pub chain: String,
    pub blocks: i64,
    pub headers: i64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub mediantime: i64,
    pub verificationprogress: f64,
    pub chainwork: String,
    pub pruned: bool,
    pub softforks: Vec<Softfork>,
}

#[derive(Deserialize)]
pub struct Tip {
    pub height: u64,
    pub hash: String,
    pub branchlen: u64,
    pub status: String,
}

#[derive(Deserialize)]
pub struct MemPoolInfo {
    pub size: i64,
    pub bytes: i64,
    pub usage: i64,
    pub maxmempool: i64,
    pub mempoolminfee: f64,
}

#[derive(Deserialize)]
pub struct TxDescription {
    pub txid: String,
    pub size: i64,
    pub fee: f64,
    pub time: i64,
    pub height: i64,
    pub startingpriority: i64,
    pub currentpriority: i64,
    pub depends: Vec<String>,
}

#[derive(Deserialize)]
pub struct TXIDS {
    pub result: Vec<String>,
}

#[derive(Deserialize)]
pub enum RawMemPool {
    True(TxDescription),
    False(TXIDS),
}

#[derive(Deserialize)]
pub struct ScriptPubKey {
    pub asm: String,
    pub hex: String,
    #[serde(rename = "reqSigs")]  pub req_sigs: i64,
    #[serde(rename = "type")] pub script_type: String,
    pub addresses: Vec<String>,
}

#[derive(Deserialize)]
pub struct TxOut {
    pub bestblock: String,
    pub confirmations: i64,
    pub value: f64,
    pub scriptpubkey: ScriptPubKey,
    pub coinbase: bool,
}

#[derive(Deserialize)]
pub struct TxOutSetInfo {
    pub height: i64,
    pub bestblock: String,
    pub transactions: i64,
    pub txouts: i64,
    pub bytes_serialized: i64,
    pub hash_serialized: String,
    pub total_amount: f64,
}

jsonrpc_client!(pub struct BitcoinRpcClient {
    pub fn getblock(&mut self, header_hash: String, verbosity: i32) -> RpcRequest<GetBlockReply>;
    pub fn getblockchaininfo(&mut self) -> RpcRequest<BlockChainInfo>;
    pub fn getblockcount(&mut self) -> RpcRequest<i64>;
    pub fn getblockhash(&mut self, block_height: i64) -> RpcRequest<String>;
    pub fn getrawmempool(&mut self, format: bool) -> RpcRequest<RawMemPool>;
    pub fn gettxout(&mut self, txid: String, vout: i64, unconfirmed: bool) -> RpcRequest<TxOut>;
});

pub type BitcoinRpc = BitcoinRpcClient<jsonrpc_client_http::HttpHandle>;

/// Creates a connection to a bitcoin rpc server
pub fn new_client(url: &str, user: Option<String>, pass: Option<String>) -> BitcoinRpcClient<jsonrpc_client_http::HttpHandle> {
    // Check that if we have a password, we have a username; other way around is ok
    debug_assert!(pass.is_none() || user.is_some());

    let transport = HttpTransport::new().standalone().unwrap();
    let mut transport_handle = transport.handle(url).unwrap();
    if let Some(ref user) = user {
        transport_handle.set_header(Authorization(Basic {
            username: user.clone(),
            password: pass.clone()
        }));
    }
    return BitcoinRpcClient::new(transport_handle);
}
