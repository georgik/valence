// Replace `std` usage with `core` or custom implementations for `no_std`.
use core::fmt;
use core::fmt::Write;

// Use alloc for heap-allocated collections in no_std contexts.
extern crate alloc;
use crate::Encode;
use crate::Decode;

use anyhow::bail;
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VarIntDecodeError {
    Incomplete,
    TooLarge,
}

impl fmt::Display for VarIntDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarIntDecodeError::Incomplete => write!(f, "incomplete VarInt decode"),
            VarIntDecodeError::TooLarge => write!(f, "VarInt is too large"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VarIntDecodeError {}

/// Define a minimal `Read` trait for no_std compatibility.
pub trait Read {
    fn read_u8(&mut self) -> Result<u8, VarIntDecodeError>;
}

/// Implement `Read` for `&[u8]` slices.
impl<'a> Read for &'a [u8] {
    fn read_u8(&mut self) -> Result<u8, VarIntDecodeError> {
        if self.is_empty() {
            Err(VarIntDecodeError::Incomplete)
        } else {
            let byte = self[0];
            *self = &self[1..];
            Ok(byte)
        }
    }
}

/// An `i32` encoded with variable length.
#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Deref,
    DerefMut,
    From,
    Into,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct VarInt(pub i32);

impl VarInt {
    /// The maximum number of bytes a `VarInt` could occupy when read from and
    /// written to the Minecraft protocol.
    pub const MAX_SIZE: usize = 5;

    /// Returns the exact number of bytes this varint will write when
    /// [`Encode::encode`] is called, assuming no error occurs.
    pub const fn written_size(self) -> usize {
        match self.0 {
            0 => 1,
            n => (31 - n.leading_zeros() as usize) / 7 + 1,
        }
    }

    /// Decodes a `VarInt` with partial input support.
    pub fn decode_partial<R: Read>(mut r: R) -> Result<i32, VarIntDecodeError> {
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            let byte = r.read_u8()?;
            val |= (i32::from(byte) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(val);
            }
        }
        Err(VarIntDecodeError::TooLarge)
    }
}

impl Encode for VarInt {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        let x = self.0 as u64;
        let mut buffer = [0u8; Self::MAX_SIZE];
        let mut index = 0;

        while x >> (index * 7) != 0 || index == 0 {
            buffer[index] = ((x >> (index * 7)) & 0b01111111) as u8;
            if x >> ((index + 1) * 7) != 0 {
                buffer[index] |= 0b10000000;
            }
            index += 1;
        }

        w.write_all(&buffer[..index])?;
        Ok(())
    }
}

impl Decode<'_> for VarInt {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            let byte = r.read_u8()?;
            val |= (i32::from(byte) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(VarInt(val));
            }
        }
        bail!("VarInt is too large")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn varint_written_size() {
        let test_cases = [
            (0, 1),
            (127, 1),
            (128, 2),
            (16383, 2),
            (16384, 3),
            (2097151, 3),
            (2097152, 4),
            (268435455, 4),
            (268435456, 5),
        ];

        for (value, expected_size) in test_cases.iter() {
            let varint = VarInt(*value);
            assert_eq!(varint.written_size(), *expected_size);
        }
    }

    #[test]
    fn varint_round_trip() {
        let mut rng = rand::thread_rng();
        let mut buf = vec![];

        for n in (0..1_000_000).map(|_| rng.gen()).chain([0, i32::MIN, i32::MAX]) {
            VarInt(n).encode(&mut buf).unwrap();

            let mut slice = buf.as_slice();
            assert!(slice.len() <= VarInt::MAX_SIZE);

            assert_eq!(n, VarInt::decode(&mut slice).unwrap().0);

            assert!(slice.is_empty());
            buf.clear();
        }
    }
}
