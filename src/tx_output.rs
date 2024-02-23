// Define transaction output structure
#[derive(Debug)]
pub struct TxOutput {
  // Value in Satoshis
  pub value: u64,
  // ScriptPubKey (locking script)
  pub script_pubkey: Vec<u8>,
}
