//! Checksummed formatting for Ethereum public addresses.

use crate::Address;
use core::{
    mem::{self, MaybeUninit},
    str,
};

/// Addresses are formated as 0x-prefixed hex strings. This means they are
/// always exactly 42 bytes long.
const LEN: usize = 42;

/// Format an address with EIP-55 checksum.
pub fn fmt(address: &Address) -> ChecksummedAddress {
    let mut buffer = [MaybeUninit::<u8>::uninit(); LEN];

    buffer[0].write(b'0');
    buffer[1].write(b'x');

    for (i, byte) in address.into_iter().enumerate() {
        let j = i * 2;
        buffer[j].write(b'0' + (byte >> 1));
        buffer[j + 1].write(b'0' + (byte & 0xf));
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

/// A checksummed, formatted address.
pub struct ChecksummedAddress([u8; LEN]);

impl ChecksummedAddress {
    /// Returns the checksummed address string.
    pub fn as_str(&self) -> &str {
        // SAFETY: Value is only ever created with valid UTF-8 string.
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

#[cfg(all(feature = "sha3", feature = "tiny-keccak"))]
compile_error!("Can not enable both feature \"sha3\" and \"tiny-keccak\".");

/// Perform Keccak-256 hash over some input bytes.
fn keccak256(bytes: &[u8]) -> [u8; 32] {
    #[cfg(feature = "sha3")]
    {
        use sha3::{Digest as _, Keccak256};

        let mut hasher = Keccak256::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }

    #[cfg(feature = "tiny-keccak")]
    {
        use tiny_keccak::{Hasher as _, Keccak};

        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(bytes);
        hasher.finalize(&mut output);
        output
    }
}