mod bitcoin_script;
mod derive_address;
mod tx_input;
mod tx_output;
mod transaction;
mod send_btc_to_address;
mod spend_from_previous;


fn main() {
  // Encode preimage
  let preimage = b"Btrust Builders";

  // Generate redeem script
  let redeem_script = bitcoin_script::generate_redeem_script(preimage);

  // Convert redeem script to hexadecimal string
  let redeem_script_hex = hex::encode(&redeem_script);
  println!("Redeem Script Hex: {}", redeem_script_hex);

  // Step 2: Derive address from redeem script
  let address = derive_address::generate_address(&redeem_script);

  // Convert address to hexadecimal string
  let address_hex = hex::encode(&address);
  println!("Derived Address: {}", address_hex);

  // Step 3: construct a transaction that send Bitcoins to the address
  // Create transaction to send Bitcoins to an address
  let amount = 100000;
  let tx_to_address = send_btc_to_address::create_transaction_to_address(address_hex.as_bytes(), amount);

  // Print the transaction to send Bitcoins to the address
 // println!("Transaction to Send Bitcoins to Address: {:?}", tx_to_address);

  // Create transaction spending from previous transaction
  let new_address = hex::encode(&address);
  let returned_tx_spending = spend_from_previous::create_transaction_spending(&tx_to_address, amount, &new_address.as_bytes());

  println!("Transaction Spending from Previous Transaction: {:?}", returned_tx_spending);




}


