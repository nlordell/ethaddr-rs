//! TODO(nlordell)

#![cfg_attr(not(any(feature = "std", test)), no_std)]

#[cfg(feature = "checksum")]
mod checksum;
mod hex;
#[cfg(feature = "serde")]
mod serde;

pub use crate::hex::ParseAddressError;
use core::{
    array::{IntoIter, TryFromSliceError},
    fmt::{self, Debug, Display, Formatter, LowerHex, UpperHex},
    ops::{Deref, DerefMut},
    slice::Iter,
    str::{self, FromStr},
};

/// TODO(nlordell): ...
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use ethaddr::{address, Address};
/// for address in [
///     address!("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"),
///     address!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"),
/// ] {
///     assert_eq!(address, Address([0xee; 20]));
/// }
/// ```
///
/// Note that by default, the procedural macro will verify address checksums:
///
/// ```compile_fail
/// # use ethaddr::address;
/// let _ = address!("0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");
/// ```
///
/// However, this behaviour can be ignored by prefixing the address with a `~`:
///
/// ```
/// # use ethaddr::address;
/// let _ = address!(~"0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee");
/// ```
#[cfg(feature = "macros")]
pub use ethaddr_macros::address;

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
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethaddr::Address;
    /// let buffer = (0..255).collect::<Vec<_>>();
    /// assert_eq!(
    ///     Address::from_slice(&buffer[0..20]),
    ///     Address([
    ///         0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09,
    ///         0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13
    ///     ]),
    /// );
    /// ```
    pub fn from_slice(slice: &[u8]) -> Self {
        slice.try_into().unwrap()
    }

    /// Creates a reference to an address from a reference to a 20-byte array.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethaddr::Address;
    /// let arrays = [[0; 20], [1; 20]];
    /// for address in arrays.iter().map(Address::from_ref) {
    ///     println!("{address}");
    /// }
    /// ```
    pub fn from_ref(array: &[u8; 20]) -> &'_ Self {
        // SAFETY: `Address` and `[u8; 20]` have the same memory layout.
        unsafe { &*(array as *const [u8; 20]).cast::<Self>() }
    }

    /// Creates a mutable reference to an address from a mutable reference to a
    /// 20-byte array.
    pub fn from_mut(array: &mut [u8; 20]) -> &'_ mut Self {
        // SAFETY: `Address` and `[u8; 20]` have the same memory layout.
        unsafe { &mut *(array as *mut [u8; 20]).cast::<Self>() }
    }

    /// Parses a checksummed `Address` from a string.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use ethaddr::Address;
    /// assert!(Address::from_str_checksum("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE",).is_ok());
    /// assert!(Address::from_str_checksum("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE",).is_ok());
    /// assert!(Address::from_str_checksum("0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",).is_err());
    /// ```
    #[cfg(feature = "checksum")]
    pub fn from_str_checksum(s: &str) -> Result<Self, ParseAddressError> {
        checksum::parse(s).map(Self)
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
        #[cfg(feature = "checksum")]
        {
            f.write_str(checksum::fmt(self).as_str())
        }
        #[cfg(not(feature = "checksum"))]
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
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s).map(Self)
    }
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

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
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
