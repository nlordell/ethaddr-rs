//! Stack-allocated formatting buffer.
//!
//! This enables efficient, and allocation-free formatting and serialization
//! implementations.

use core::{
    fmt::{self, Write},
    mem::MaybeUninit,
    ptr, slice, str,
};

/// Addresses are formated as 0x-prefixed hex strings. This means they are
/// always exactly 42 bytes long.
const N: usize = 42;

/// A stack-allocated buffer that can fit exactly one address.
pub struct FormatBuffer {
    offset: usize,
    buffer: [MaybeUninit<u8>; N],
}

impl FormatBuffer {
    /// Creates a new formatting buffer.
    pub fn new() -> Self {
        Self {
            offset: 0,
            buffer: [MaybeUninit::uninit(); N],
        }
    }

    /// Returns a `str` to the currently written data.
    pub fn as_str(&self) -> &str {
        // SAFETY: We only ever write valid UTF-8 strings to the buffer, so the
        // resulting string will always be valid.
        unsafe {
            let buffer = slice::from_raw_parts(self.buffer[0].as_ptr(), self.offset);
            str::from_utf8_unchecked(buffer)
        }
    }

    /// Returns a `str` to the currently written data.
    pub fn as_str_mut(&mut self) -> &mut str {
        // SAFETY: See :point_up:.
        unsafe {
            let buffer = slice::from_raw_parts_mut(self.buffer[0].as_mut_ptr(), self.offset);
            str::from_utf8_unchecked_mut(buffer)
        }
    }
}

impl Write for FormatBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.offset.checked_add(s.len()).ok_or(fmt::Error)?;

        // Make sure there is enough space in the buffer.
        if end > N {
            return Err(fmt::Error);
        }

        // SAFETY: We checked that there is enough space in the buffer to fit
        // the string `s` starting from `offset`, and the pointers cannot be
        // overlapping because of Rust ownership semantics (i.e. `s` cannot
        // overlap with `buffer` because we have a mutable reference to `self`
        // and by extension `buffer`).
        unsafe {
            let buffer = self.buffer[0].as_mut_ptr().add(self.offset);
            ptr::copy_nonoverlapping(s.as_ptr(), buffer, s.len());
        }
        self.offset = end;

        Ok(())
    }
}
