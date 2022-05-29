//! Internal module used for hex-string parsing.

use core::{
    fmt::{self, Display, Formatter},
    mem::{self, MaybeUninit},
};

/// Decode a hex string into address bytes.
pub fn decode(s: &str) -> Result<[u8; 20], ParseAddressError> {
    let s = strip_0x_prefix(s);
    if s.len() != 40 {
        return Err(ParseAddressError::InvalidLength { len: s.len() });
    }

    let mut bytes = [MaybeUninit::<u8>::uninit(); 20];
    let nibble = |c| match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 0xa),
        b'a'..=b'f' => Some(c - b'a' + 0xa),
        _ => None,
    };

    for (i, ch) in s.as_bytes().chunks(2).enumerate() {
        let (hi, lo) = (ch[0], ch[1]);

        let hi = nibble(hi).ok_or(ParseAddressError::InvalidHexCharacter {
            c: hi,
            index: i * 2,
        })?;
        let lo = nibble(lo).ok_or(ParseAddressError::InvalidHexCharacter {
            c: lo,
            index: i * 2 + 1,
        })?;

        bytes[i].write((hi << 4) + lo);
    }

    let bytes = unsafe { mem::transmute(bytes) };
    Ok(bytes)
}

/// Represents an error parsing an address from a string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseAddressError {
    /// The hex string does not match
    InvalidLength { len: usize },
    /// An invalid character was found.
    InvalidHexCharacter { c: u8, index: usize },
    /// The checksum encoded in the hex string's case does not match the
    /// address.
    ChecksumMismatch,
}

impl Display for ParseAddressError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidLength { .. } => write!(f, "invalid hex string length"),
            Self::InvalidHexCharacter { c, index } => {
                write!(f, "invalid character \\x{c:02x} at position {index}")
            }
            Self::ChecksumMismatch => write!(f, "address checksum does not match"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseAddressError {}

/// Utility for striping leading `0x-` prefix from strings.
pub fn strip_0x_prefix(s: &str) -> &str {
    s.strip_prefix("0x").unwrap_or(s)
}
