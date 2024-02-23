use crate::transaction::Transaction;
use crate::tx_output::TxOutput;

// Function to create a transaction to send Bitcoins to an address
pub fn create_transaction_to_address(address: &[u8], amount: u64) -> Transaction {
    // Create an example transaction output sending Bitcoins to the address
    let tx_output = TxOutput {
        value: amount,
        script_pubkey: address.to_vec(), // Assuming address is a P2PKH address script pubkey
    };

    // Create a transaction with the output
    Transaction {
        version: 1, // Transaction version
        inputs: vec![], // No inputs for this transaction
        outputs: vec![tx_output], // Add output
        lock_time: 0, // Lock time (0 for immediate)
    }
}
