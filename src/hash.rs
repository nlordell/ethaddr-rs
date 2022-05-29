//! Hashing front-end.
//!
//! This allows this crate to be backed by different hashing algoriths.

#[cfg(all(feature = "sha3", feature = "tiny-keccak"))]
compile_error!("Can not enable both feature \"sha3\" and \"tiny-keccak\".");

#[cfg(not(any(feature = "sha3", feature = "tiny-keccak")))]
compile_error!("Either feature \"sha3\" or \"tiny-keccak\" must be enabled for this crate.");

/// Perform Keccak-256 hash over some input bytes.
pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    #[cfg(feature = "sha3")]
    {
        use sha3::{Keccak256, Digest as _};

        let mut hasher = Keccak256::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }

    #[cfg(feature = "tiny-keccak")]
    {
        use tiny_keccak::{Keccak, Hasher as _};

        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(bytes);
        hasher.finalize(&mut output);
        output
    }
}
