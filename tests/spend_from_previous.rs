// spend_from_previous.rs

// Import the necessary modules and structs
use crate::transaction::Transaction;
use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;
use crate::spend_from_previous::create_transaction_spending;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction_spending() {
        // Test data
        let previous_tx = Transaction {
            // Define previous transaction data
            version: 1,
            inputs: vec![],
            outputs: vec![],
            lock_time: 0,
        };
        let amount = 100_000;
        let new_address = "NEW_ADDRESS".to_string();

       
        let tx_spending = create_transaction_spending(&previous_tx, amount, new_address.as_bytes());
        assert_eq!(tx_spending.version, 1);
       
    }
}
