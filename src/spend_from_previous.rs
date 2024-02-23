use crate::transaction::Transaction;
use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;

// Function to create a transaction spending from the previous transaction
pub fn create_transaction_spending(previous_tx: &Transaction, amount: u64, new_address: &[u8]) -> Transaction {
    // Example: Assuming we have the previous transaction hash and output index
    let prev_tx_hash = hash_transaction(previous_tx); // Use the hash of the previous transaction as prev_tx_hash
    let prev_tx_output_index = 0; // Previous transaction output index

    // Create an example transaction input spending from the previous transaction output
    let tx_input = TxInput {
        prev_tx_hash, // Previous transaction hash
        prev_tx_output_index, // Previous transaction output index
        script_sig: vec![], // Empty unlocking script (to be filled)
        sequence: 0xFFFFFFFF, // Max sequence number
    };

    // Create an example transaction output directing Bitcoins to a new address
    let tx_output = TxOutput {
        value: amount,
        script_pubkey: new_address.to_vec(), // Assuming new_address is a P2PKH address script pubkey
    };

    // Create a transaction with the input and output
    let _tx_spending = Transaction {
      version: 1, // Transaction version
      inputs: vec![tx_input], // Add input
      outputs: vec![tx_output], // Add output
      lock_time: 0, // Lock time (0 for immediate)
  };

  // Return the transaction
  _tx_spending


}

// Function to hash transaction data
fn hash_transaction(_tx: &Transaction) -> [u8; 32] {
    // Simplified hash calculation for demonstration
    // Actual Bitcoin transactions use double SHA256 hashing
    // Here, we just return an all-zero hash for simplicity
    [0; 32]
}
