mod transaction_builder;
mod redeem_script;
mod generate_address;
mod second_transaction_builder;

pub extern crate hex;
use std::str::FromStr;

use chrono::Local;
use rand::Rng;



fn main() {

    //genraate reedem script
   let redeem_script = redeem_script::redeem_script();
    /*
      Derive an address with from the above (1) redeem script
    */
    let address = generate_address::create_address(&redeem_script);

    let previous_txid = bitcoin::Txid::from_str(&generate_transaction_id()).unwrap();
    let previous_output_index = 0;

    // Build the transaction using the function from the imported module
    let transaction_bytes = transaction_builder::build_transaction(previous_txid, previous_output_index, address);

    
    // second transaction 
    let change_transaction =  
      second_transaction_builder::create_transaction(
        previous_txid, 
        previous_output_index, 
        unlock_script, 
        locking_script, 
        main_output_address, 
        change_output_address)

}


fn generate_transaction_id()-> String {
    let now = Local::now();
    let timestamp = now.timestamp_millis();
    let random_number = rand::thread_rng().gen_range(0..=u64::MAX);
    let txid_str = format!("{:0>16x}-{:x}", timestamp, random_number);
    txid_str
}


