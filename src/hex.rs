//! Internal module used for hex-string parsing.

use core::{
    fmt::{self, Display, Formatter},
    mem::{self, MaybeUninit},
};

/// Decode a hex string into address bytes.
pub fn decode(s: &str) -> Result<[u8; 20], ParseAddressError> {
    let (s, ch_offset) = match s.strip_prefix("0x") {
        Some(s) => (s, 2),
        None => (s, 0),
    };
    if s.len() != 40 {
        return Err(ParseAddressError::InvalidLength);
    }

    let mut bytes = [MaybeUninit::<u8>::uninit(); 20];
    let nibble = |c| match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 0xa),
        b'a'..=b'f' => Some(c - b'a' + 0xa),
        _ => None,
    };
    let invalid_char = |i: usize| ParseAddressError::InvalidHexCharacter {
        c: s[i..].chars().next().unwrap(),
        index: i + ch_offset,
    };

    for (i, ch) in s.as_bytes().chunks(2).enumerate() {
        let hi = nibble(ch[0]).ok_or_else(|| invalid_char(i * 2))?;
        let lo = nibble(ch[1]).ok_or_else(|| invalid_char(i * 2 + 1))?;
        bytes[i].write((hi << 4) + lo);
    }

    let bytes = unsafe { mem::transmute(bytes) };
    Ok(bytes)
}

/// Represents an error parsing an address from a string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseAddressError {
    /// The hex string does not have the correct length.
    InvalidLength,
    /// An invalid character was found.
    InvalidHexCharacter { c: char, index: usize },
    /// The checksum encoded in the hex string's case does not match the
    /// address.
    #[allow(dead_code)]
    ChecksumMismatch,
}

impl Display for ParseAddressError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength { .. } => write!(f, "invalid hex string length"),
            Self::InvalidHexCharacter { c, index } => {
                write!(f, "invalid character `{c}` at position {index}")
            }
            Self::ChecksumMismatch => write!(f, "address checksum does not match"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseAddressError {}
