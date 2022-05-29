//! Checksummed formatting for Ethereum public addresses.

use crate::hex::{self, ParseAddressError};
use core::{
    mem::{self, MaybeUninit},
    str,
};
use sha3::{Digest as _, Keccak256};

/// Addresses are formated as 0x-prefixed hex strings. This means they are
/// always exactly 42 bytes long.
const LEN: usize = 42;

/// Format address bytes with EIP-55 checksum.
pub fn fmt(bytes: &[u8; 20]) -> ChecksummedAddress {
    let mut buffer = [MaybeUninit::<u8>::uninit(); LEN];

    buffer[0].write(b'0');
    buffer[1].write(b'x');

    let nibble = |c: u8| b"0123456789abcdef"[c as usize];
    for (i, byte) in bytes.iter().enumerate() {
        let j = i * 2 + 2;
        buffer[j].write(nibble(byte >> 4));
        buffer[j + 1].write(nibble(byte & 0xf));
    }

    // SAFETY: We are guaranteed to written to every uninitilized byte.
    let mut buffer = unsafe { mem::transmute::<_, [u8; LEN]>(buffer) };

    let addr = &mut buffer[2..];
    let digest = keccak256(addr);
    for i in 0..addr.len() {
        let byte = digest[i / 2];
        let nibble = 0xf & if i % 2 == 0 { byte >> 4 } else { byte };
        if nibble >= 8 {
            addr[i] = addr[i].to_ascii_uppercase();
        }
    }

    ChecksummedAddress(buffer)
}

/// Parses address bytes and verifies checksum.
pub fn parse(s: &str) -> Result<[u8; 20], ParseAddressError> {
    let address = hex::decode(s)?;
    let checksum = fmt(&address);
    if hex::strip_0x_prefix(checksum.as_str()) != hex::strip_0x_prefix(s) {
        return Err(ParseAddressError::ChecksumMismatch);
    }

    Ok(address)
}

/// A checksummed, formatted address.
pub struct ChecksummedAddress([u8; LEN]);

impl ChecksummedAddress {
    /// Returns the checksummed address string.
    pub fn as_str(&self) -> &str {
        // SAFETY: Value is only ever created with valid UTF-8 string.
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

/// Perform Keccak-256 hash over some input bytes.
fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}
