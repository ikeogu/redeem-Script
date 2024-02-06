
use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut,OutPoint, Sequence};
use bitcoin::address::Address;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::Txid;
use bitcoin::blockdata::witness::Witness;
use bitcoin::amount::Amount;
use std::time::{SystemTime, UNIX_EPOCH};
use bitcoin::consensus::encode::serialize;


pub fn build_transaction(previous_txid: Txid , previous_output_index: u32, address: Address) -> Vec<u8> {
    // Construct the transaction input
    let input = TxIn {
        previous_output: OutPoint {
            txid: previous_txid,
            vout: previous_output_index,
        },
        script_sig: ScriptBuf::new(), // Replace with the script sig
        sequence: Sequence::MAX,
        witness: Witness::new(), // Replace with the witness
    };

    // Construct the transaction output sending bitcoins to the derived address
    let output = TxOut {
        value: Amount::from_sat(50000), // Satoshis
        script_pubkey: address.script_pubkey(),
    };


  // Get the current timestamp
  let now = SystemTime::now();
  let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();


    // Create a new transaction builder
    let tx_builder = Transaction{
      version:bitcoin::transaction::Version(1),
      lock_time:bitcoin::absolute::LockTime::from_time(timestamp as u32).expect("Failed to create lock time from timestamp"),
      input:vec![input],
      output:vec![output]
      
    };


    // Serialize the transaction to its raw bytes
    let transaction_bytes = serialize(&tx_builder);
    // Print the derived address
    println!("Transaction Bytes (Hex): {}", hex::encode(&transaction_bytes));

    // Return the transaction bytes
    transaction_bytes

}
