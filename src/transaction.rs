use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;

use std::fmt;

// Define transaction structure
#[derive(Debug)]
pub struct Transaction {
    // Version number
    pub version: i32,
    // List of transaction inputs
    pub inputs: Vec<TxInput>,
    // List of transaction outputs
    pub outputs: Vec<TxOutput>,
    // Lock time
    pub lock_time: u32,
}

impl fmt::Display for Transaction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      // Customize formatting of Transaction fields
      write!(f, "Transaction: <details>")
  }
}

