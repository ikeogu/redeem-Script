mod transaction_builder;

pub extern crate hex;

use std::str::FromStr;

use bitcoin::address::Address;
use bitcoin::network::Network;
use bitcoin_hashes::hex::ToHex;
use sha2::{Digest, Sha256};
use bitcoin::blockdata::opcodes::all::OP_SHA256;
use bitcoin::blockdata::script::Builder;
use chrono::Local;
use rand::Rng;






fn main() {

    /*
     Given the string "Btrust Builders", whose bytes encoding is 427472757374204275696c64657273, write a script/program that does the following:
     Generate the redeem script in hex format for the given pre-image. Note: redeem_script => OP_SHA256 <lock_hex> OP_EQUAL.
    */

     // Given pre-image in hexadecimal format
    let preimage_hex = "427472757374204275696c64657273";

    // Convert the hexadecimal pre-image to bytes
    let preimage_bytes = hex::decode(preimage_hex).expect("Invalid hex string");

    // Hash the pre-image using SHA-256
    let lock_hash = Sha256::digest(&preimage_bytes);

    // Convert the lock hash to a byte array
    let lock_hash_bytes: [u8; 32] = lock_hash.into();

    // Construct the redeem script
    let redeem_script = Builder::new()
        .push_opcode(OP_SHA256)
        .push_slice(&lock_hash_bytes)
        .push_opcode(OP_SHA256)
        .into_script();

      // Print the redeem script in hex format
    println!("Redeem Script (Hex): {}", redeem_script);

    /*
      Derive an address with from the above (1) redeem script
    */
    let address = Address::p2sh(&redeem_script, Network::Bitcoin).expect("Failed to generate P2SH address");

    // Print the derived address
    println!("Derived Address: {}", address.to_string());

    let previous_txid = bitcoin::Txid::from_str(&generate_transaction_id()).unwrap();
    let previous_output_index = 0;


    // Build the transaction using the function from the imported module
    let transaction_bytes = transaction_builder::build_transaction(previous_txid, previous_output_index, address);

    // Print the transaction in hex format
    println!("Transaction (Hex): {}", transaction_bytes.to_hex());


}


fn generate_transaction_id()-> String {
    let now = Local::now();
    let timestamp = now.timestamp_millis();
    let random_number = rand::thread_rng().gen_range(0..=u64::MAX);
    let txid_str = format!("{:0>16x}-{:x}", timestamp, random_number);
    txid_str
}


