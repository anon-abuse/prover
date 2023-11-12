// #![no_std] // std support is experimental

use std::hash;

use risc0_zkvm::guest::env;
// use utils::transactions;
use alloy_primitives::{
    address, b256, bytes, Address, Bytes, ChainId, FixedBytes, Sign, TxNumber, B256, U256,
};
use alloy_rlp::Encodable;
use json::parse;
use utils::outputs::Outputs;
use utils::{
    self,
    keccak::keccak,
    signature::TxSignature,
    transactions::{
        transactions::{EthereumTxEssence, TransactionKind, TxEssenceEip1559},
        Transaction,
    },
};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let (transaction, target_address, merkle_proof, transactions_root): (
        String,
        String,
        String,
        String,
    ) = env::read();

    let transaction_json = parse(&transaction).unwrap();

    let nonce_hex: &str = transaction_json["result"]["nonce"].as_str().unwrap();
    let max_priority_fee_per_gas_hex = transaction_json["result"]["maxPriorityFeePerGas"]
        .as_str()
        .unwrap();
    let max_fee_per_gas_hex = transaction_json["result"]["maxFeePerGas"].as_str().unwrap();
    let value_hex: &str = transaction_json["result"]["value"].as_str().unwrap();
    let gas_limit_hex = transaction_json["result"]["gas"].as_str().unwrap();
    let to_hex = transaction_json["result"]["to"].as_str().unwrap();
    let from_hex = transaction_json["result"]["from"].as_str().unwrap();
    let chain_id_hex = transaction_json["result"]["chainId"].as_str().unwrap();
    let v_hex = transaction_json["result"]["v"].as_str().unwrap();
    let r_hex = transaction_json["result"]["r"].as_str().unwrap();
    let s_hex = transaction_json["result"]["s"].as_str().unwrap();

    let hash_hex = transaction_json["result"]["hash"].as_str().unwrap();

    let merkle_proof_json = parse(&merkle_proof).unwrap();
    let proof_len = merkle_proof_json["proof"].len();

    // let mut resultant: Bytes = merkle_proof_json["proof"][0]
    //     .as_str()
    //     .unwrap()
    //     .parse::<Bytes>()
    //     .unwrap();

    // if proof_len > 1 {
    //     for n in 1..proof_len {
    //         let el = merkle_proof_json["proof"][n]
    //             .as_str()
    //             .unwrap()
    //             .parse::<Bytes>()
    //             .unwrap();
    //         let concat_byte = [resultant.clone(), el].concat();

    //         let keccak_res: [u8; 32] = keccak(concat_byte);
    //         resultant = Bytes::from(keccak_res);
    //     }
    // }

    // let tx_root_hex: String = hex::encode(resultant);
    // println!("tx_root_hex: {:?}", tx_root_hex);
    // println!("tx_root_expected: {:?}", transactions_root);

    let nonce: u64 = hex_to_u64(nonce_hex);
    let max_priority_fee_per_gas: U256 = max_priority_fee_per_gas_hex.parse().unwrap();
    let max_fee_per_gas: U256 = max_fee_per_gas_hex.parse().unwrap();
    let value: U256 = value_hex.parse().unwrap();
    let gas_limit: U256 = gas_limit_hex.parse().unwrap();
    let data_hex = transaction_json["result"]["input"].as_str().unwrap();
    let data_bytes = data_hex.parse::<Bytes>().unwrap();
    let chain_id: u64 = hex_to_u64(chain_id_hex);

    let to: Address = to_hex.trim_start_matches("0x").parse().unwrap();
    let from: Address = from_hex.trim_start_matches("0x").parse().unwrap();
    let target_address: Address = target_address.trim_start_matches("0x").parse().unwrap();
    let transaction_hash: String = hash_hex.trim_start_matches("0x").parse().unwrap();

    assert_eq!(
        target_address, to,
        "Transaction not sent to phishing address"
    );

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

    let tx_signature: TxSignature = TxSignature { v: v, r: r, s: s };

    let tx = Transaction {
        essence: EthereumTxEssence::Eip1559(tx_essense.clone()),
        signature: tx_signature,
    };

    let tx_hash: FixedBytes<32> = tx.hash();

    let eth_tx_essence = EthereumTxEssence::Eip1559(tx_essense);
    let mut rlp_buf: Vec<u8> = Vec::new();
    eth_tx_essence.encode(&mut rlp_buf);

    let tx_hash: String = hex::encode(tx_hash);

    assert_eq!(
      tx_hash, transaction_hash,
      "Transaction not sent to phishing address"
    );


    let outputs = Outputs {
        phishing_address: to.to_checksum(None),
        phished_address: from.to_checksum(None),
        transaction_hash: transactions_root,
    };

    env::commit(&outputs);
}

pub fn hex_to_u64(hex_str: &str) -> u64 {
    let hex_str = hex_str.trim_start_matches("0x");
    u64::from_str_radix(hex_str, 16).unwrap()
}
