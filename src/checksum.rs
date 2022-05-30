//! Checksummed formatting for Ethereum public addresses.

use crate::buffer::{self, Alphabet, FormattingBuffer};
use core::str;
use sha3::{Digest as _, Keccak256};

/// Format address bytes with EIP-55 checksum.
pub fn fmt(bytes: &[u8; 20]) -> FormattingBuffer {
    let mut buffer = buffer::fmt(bytes, Alphabet::Lower);

    // SAFETY: We only ever change lowercase ASCII characters to upper case
    // characters, so the buffer remains valid UTF-8 bytes.
    let addr = unsafe { &mut buffer.as_bytes_mut()[2..] };
    let digest = keccak256(addr);
    for i in 0..addr.len() {
        let byte = digest[i / 2];
        let nibble = 0xf & if i % 2 == 0 { byte >> 4 } else { byte };
        if nibble >= 8 {
            addr[i] = addr[i].to_ascii_uppercase();
        }
    }

    buffer
}

/// Verifies an address checksum.
pub fn verify(bytes: &[u8; 20], checksum: &str) -> Result<(), FormattingBuffer> {
    let expected = fmt(bytes);
    if checksum.strip_prefix("0x").unwrap_or(checksum) != expected.as_bytes_str() {
        return Err(expected);
    }
    Ok(())
}

/// Perform Keccak-256 hash over some input bytes.
fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}
