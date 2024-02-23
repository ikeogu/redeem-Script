


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction_to_address() {
        // Test data
        let address_hex = "ADDRESS_HEX".to_string();
        let amount = 100_000;

        let tx_to_address = create_transaction_to_address(address_hex.as_bytes(), amount);

        assert_eq!(tx_to_address.version, 1);
    }
}
