use sha2::{Digest, Sha256};


pub fn generate_redeem_script(preimage: &[u8]) -> Vec<u8> {
  // Calculate SHA256 hash of preimage
  let hash = Sha256::digest(preimage);

  // Construct redeem script
  let mut redeem_script = Vec::new();
  redeem_script.push(0x20); // Push 32-byte hash
  redeem_script.extend_from_slice(&hash);
  redeem_script.push(0x21); // Push OP_EQUAL
  redeem_script.push(0x9b); // OP_EQUALVERIFY

  redeem_script
}