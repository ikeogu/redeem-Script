// Define transaction input structure
#[derive(Debug)]
pub struct TxInput {
  // Previous transaction hash
  pub prev_tx_hash: [u8; 32],
  // Index of the previous transaction output
  pub prev_tx_output_index: u32,
  // ScriptSig (unlocking script)
  pub script_sig: Vec<u8>,
  // Sequence number
  pub sequence: u32,
}
