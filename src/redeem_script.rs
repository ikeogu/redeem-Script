
use bitcoin::ScriptBuf;
use sha2::{Digest, Sha256};
use bitcoin::blockdata::opcodes::all::OP_SHA256;
use bitcoin::blockdata::script::Builder;

pub fn redeem_script()-> ScriptBuf 
{
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

     redeem_script 
 
}