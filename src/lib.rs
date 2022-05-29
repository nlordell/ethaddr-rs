//! TODO(nlordell)

#[cfg(any(feature = "sha3", feature = "tiny-keccak"))]
mod checksum;
#[cfg(feature = "serde")]
mod serde;

use core::{
    array::{IntoIter, TryFromSliceError},
    fmt::{self, Debug, Display, Formatter, LowerHex, UpperHex},
    ops::{Deref, DerefMut},
    slice::Iter,
    str::{self, FromStr},
};

/// An Ethereum public address.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Address(pub [u8; 20]);

impl Address {
    /// Creates an address from a slice.
    ///
    /// # Panics
    ///
    /// This method panics if the length of the slice is not 20 bytes.
    pub fn from_slice(slice: &[u8]) -> Self {
        slice.try_into().unwrap()
    }

    /// Creates a reference to an address from a reference to a 20-byte array.
    pub fn from_ref<'a>(array: &'a [u8; 20]) -> &'a Self {
        // SAFETY: `Address` and `[u8; 20]` have the same memory layout.
        unsafe { &*(array as *const [u8; 20]).cast::<Self>() }
    }

    /// Creates a mutable reference to an address from a mutable reference to a
    /// 20-byte array.
    pub fn from_mut<'a>(array: &'a mut [u8; 20]) -> &'a mut Self {
        // SAFETY: `Address` and `[u8; 20]` have the same memory layout.
        unsafe { &mut *(array as *mut [u8; 20]).cast::<Self>() }
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Address")
            .field(&format_args!("{self}"))
            .finish()
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        #[cfg(any(feature = "sha3", feature = "tiny-keccak"))]
        {
            f.write_str(checksum::fmt(self).as_str())
        }
        #[cfg(not(any(feature = "sha3", feature = "tiny-keccak")))]
        {
            write!(f, "{self:#x}")
        }
    }
}

impl LowerHex for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for byte in self {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl UpperHex for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for byte in self {
            write!(f, "{byte:02X}")?;
        }
        Ok(())
    }
}

impl AsRef<[u8; 20]> for Address {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8; 20]> for Address {
    fn as_mut(&mut self) -> &mut [u8; 20] {
        &mut self.0
    }
}

impl AsMut<[u8]> for Address {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Deref for Address {
    type Target = [u8; 20];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Address {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Address {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut address = Self::default();
        let s = s.strip_prefix("0x").unwrap_or(s);

        hex::decode_to_slice(s, address.as_mut())?;
        Ok(address)
    }
}

/// Represents an error parsing an address from a string.
pub enum ParseAddressError {
    /// An invalid character was found. Valid ones are: `0...9`, `a...f`
    /// or `A...F`.
    InvalidHexCharacter { c: char, index: usize },

    /// A hex string's length needs to be even, as two digits correspond to
    /// one byte.
    OddLength,
    /// If the hex string is decoded into a fixed sized container, such as an
    /// array, the hex string's length * 2 has to match the container's
    /// length.
    InvalidLength,
}

impl IntoIterator for Address {
    type Item = u8;
    type IntoIter = IntoIter<u8, 20>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Address {
    type Item = &'a u8;
    type IntoIter = Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl PartialEq<[u8; 20]> for Address {
    fn eq(&self, other: &'_ [u8; 20]) -> bool {
        **self == *other
    }
}

impl PartialEq<[u8]> for Address {
    fn eq(&self, other: &'_ [u8]) -> bool {
        **self == *other
    }
}

impl PartialEq<&'_ [u8]> for Address {
    fn eq(&self, other: &&'_ [u8]) -> bool {
        **self == **other
    }
}

impl PartialEq<&'_ mut [u8]> for Address {
    fn eq(&self, other: &&'_ mut [u8]) -> bool {
        **self == **other
    }
}

impl PartialEq<Vec<u8>> for Address {
    fn eq(&self, other: &Vec<u8>) -> bool {
        **self == **other
    }
}

impl TryFrom<&'_ [u8]> for Address {
    type Error = TryFromSliceError;

    fn try_from(value: &'_ [u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl TryFrom<&'_ mut [u8]> for Address {
    type Error = TryFromSliceError;

    fn try_from(value: &'_ mut [u8]) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl<'a> TryFrom<&'a [u8]> for &'a Address {
    type Error = TryFromSliceError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Address::from_ref(value.try_into()?))
    }
}

impl<'a> TryFrom<&'a mut [u8]> for &'a mut Address {
    type Error = TryFromSliceError;

    fn try_from(value: &'a mut [u8]) -> Result<Self, Self::Error> {
        Ok(Address::from_mut(value.try_into()?))
    }
}

impl<'a> TryFrom<Vec<u8>> for Address {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_address() {
        for s in &[
            "0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1",
            "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE",
        ] {
            let address = s.parse::<Address>().unwrap();
            assert_eq!(address.to_string(), *s);
        }
    }

    #[test]
    fn without_prefix_and_checksum() {
        assert_eq!(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Address>()
                .unwrap(),
            Address([0xee; 20]),
        );
    }
}
