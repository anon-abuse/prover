// #![no_std] // std support is experimental

use risc0_zkvm::guest::env;
// use utils::transactions;
use alloy_primitives::{
    address, b256, bytes, Address, Bytes, ChainId, FixedBytes, TxNumber, B256, U256, Sign,
};
use alloy_rlp::Encodable;
use json::parse;
use utils::{
    self,
    keccak::keccak,
    transactions::{Transaction, transactions::{EthereumTxEssence, TransactionKind, TxEssenceEip1559}},
    signature::TxSignature,
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // // TODO: Implement your guest code here

    // // read the input
    // let input: u32 = env::read();

    // // TODO: do something with the input

    // // write public output to the journal
    // env::commit(&input);

    // // transactions::test_function();
    // utils::add();

    let data: String = env::read();
    // let sha = *Impl::hash_bytes(&data.as_bytes());
    let data1 = parse(&data).unwrap();
    let nonce_hex: &str = data1["result"]["nonce"].as_str().unwrap();
    let max_priority_fee_per_gas_hex = data1["result"]["maxPriorityFeePerGas"].as_str().unwrap();
    let max_fee_per_gas_hex = data1["result"]["maxFeePerGas"].as_str().unwrap();
    let value_hex: &str = data1["result"]["value"].as_str().unwrap();
    let gas_limit_hex = data1["result"]["gas"].as_str().unwrap();
    let to_hex = data1["result"]["to"].as_str().unwrap();
    let chain_id_hex = data1["result"]["chainId"].as_str().unwrap();
    let v_hex = data1["result"]["v"].as_str().unwrap();
    let r_hex = data1["result"]["r"].as_str().unwrap();
    let s_hex = data1["result"]["s"].as_str().unwrap();


    let nonce: u64 = hex_to_u64(nonce_hex);


    let max_priority_fee_per_gas: U256 = max_priority_fee_per_gas_hex.parse().unwrap();
    let max_fee_per_gas: U256 = max_fee_per_gas_hex.parse().unwrap();
    println!("2");
    let value: U256 = value_hex.parse().unwrap();
    println!("3");
    let to: Address = to_hex.trim_start_matches("0x").parse().unwrap();
    println!("4");
    let gas_limit: U256 = gas_limit_hex.parse().unwrap();
    println!("5");
    let data_hex = data1["result"]["input"].as_str().unwrap();
    println!("6");
    // let data_bytes: Bytes = Bytes::new();
    let data_bytes = data_hex.parse::<Bytes>().unwrap();

    println!("data_bytes is {:?}", data_bytes);

    let chain_id: u64 = hex_to_u64(chain_id_hex);

    println!("r is {:?}", r_hex);
    println!("s is {:?}", s_hex);
    println!("v is {:?}", v_hex);

    let r: U256 = r_hex.parse().unwrap();
    let s: U256 = s_hex.parse().unwrap();
    let v: u64 = hex_to_u64(v_hex);

    let tx_essense: TxEssenceEip1559 = TxEssenceEip1559 {
        chain_id,
        nonce,
        max_priority_fee_per_gas,
        max_fee_per_gas,
        gas_limit,
        to: TransactionKind::Call(to),
        value,
        data: data_bytes,
        access_list: utils::access_list::AccessList(Vec::new()),
    };


    let tx_signature: TxSignature = TxSignature {
        v: v,
        r: r,
        s: s,
    };

    let tx = Transaction {
      essence: EthereumTxEssence::Eip1559(tx_essense.clone()) ,
      signature: tx_signature
    };

    let tx_hash = tx.hash();
    let tx_hash_string = hex::encode(tx_hash);
    println!("tx_hash_string: ${:?}", tx_hash_string);

    let eth_tx_essence = EthereumTxEssence::Eip1559(tx_essense);
    let mut rlp_buf: Vec<u8> = Vec::new();
    eth_tx_essence.encode(&mut rlp_buf);

    println!("rlp_buf: ${:?}", rlp_buf);

    let keccak_res: [u8; 32] = keccak(rlp_buf);
    let keccak_hex_string = hex::encode(keccak_res);

    println!("keccak_res: ${:?}", keccak_hex_string);

    // env::commit(&proven_val);
}

pub fn hex_to_u64(hex_str: &str) -> u64 {
    let hex_str = hex_str.trim_start_matches("0x");
    u64::from_str_radix(hex_str, 16).unwrap()
}
