//! Serde serialization implementation for Ethereum public addresses.

//#[cfg(test)]
#[cfg(feature = "false")]
mod tests {
    use super::*;

    #[test]
    fn deserialize_address() {
        assert_eq!(
            serde_json::from_value::<Address>(json!("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"))
                .unwrap(),
            Address([0xee; 20]),
        )
    }

    #[test]
    fn deserialize_address_requires_0x_prefix() {
        let without_prefix = "EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
        assert!(without_prefix.parse::<Address>().is_ok());
        assert!(serde_json::from_value::<Address>(json!(without_prefix)).is_err());
    }
}
