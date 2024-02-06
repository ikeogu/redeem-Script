use bitcoin::blockdata::transaction::{Transaction, TxIn, TxOut,OutPoint, Sequence};
use bitcoin::absolute::LockTime;
use bitcoin::address::Address;
use bitcoin::network::Network;
use bitcoin::blockdata::script::ScriptBuf;
use bitcoin::Txid;
use bitcoin::blockdata::witness::Witness;
use bitcoin::amount::Amount;


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

    // Create a new transaction builder
    let mut tx_builder = Transaction{
      version:bitcoin::transaction::Version(1),
      lock_time: ,
      input:Vec::new(),
      output:Vec::new()
      
    };


    // Set the network
    let network = Network::Bitcoin; // Change to appropriate network if needed

    // Build the transaction
   // let transaction = tx_builder.build().expect("Failed to build transaction");

    // Serialize the transaction to its raw bytes
    tx_builder.to_bytes()
}
