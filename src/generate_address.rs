use bitcoin::address::Address;
use bitcoin::network::Network;
use bitcoin::blockdata::script::Script;

pub fn create_address(redeem_script: &Script) -> Address {
    let address = Address::p2sh(redeem_script, Network::Bitcoin)
        .expect("Failed to generate P2SH address");

    // Print the derived address
    println!("Derived Address: {}", address.to_string());

    address
}
