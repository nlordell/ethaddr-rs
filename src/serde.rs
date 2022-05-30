//! Serde serialization implementation for Ethereum public addresses.

use crate::Address;
use core::fmt::{self, Formatter};
use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AddressVisitor)
    }
}

struct AddressVisitor;

impl<'de> Visitor<'de> for AddressVisitor {
    type Value = Address;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("a `0x`-prefixed 20-byte hex string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        s.strip_prefix("0x")
            .ok_or_else(|| de::Error::custom("missing `0x`-prefix"))?
            .parse()
            .map_err(de::Error::custom)
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let buffer = Address::fmt(self);
        serializer.serialize_str(buffer.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{self, BorrowedStrDeserializer};

    #[test]
    fn deserialize_address() {
        for s in [
            "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE",
            "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
            "0xEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE",
        ] {
            let deserializer = BorrowedStrDeserializer::<value::Error>::new(s);
            assert_eq!(
                Address::deserialize(deserializer).unwrap(),
                Address([0xee; 20]),
            )
        }
    }

    #[test]
    fn deserialize_address_requires_0x_prefix() {
        let without_prefix = "EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
        let deserializer = BorrowedStrDeserializer::<value::Error>::new(without_prefix);
        assert!(Address::deserialize(deserializer).is_err());
    }
}
