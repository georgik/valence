// Replace `std` usage with `core` or custom implementations for `no_std`.
use core::fmt;
use crate::Write;
use alloc::format;

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
    fn read_i8(&mut self) -> Result<i8, VarIntDecodeError>;
    fn read_u16(&mut self) -> Result<u16, VarIntDecodeError>;
    fn read_i16(&mut self) -> Result<i16, VarIntDecodeError>;
    fn read_u32(&mut self) -> Result<u32, VarIntDecodeError>;
    fn read_i32(&mut self) -> Result<i32, VarIntDecodeError>;
    fn read_u64(&mut self) -> Result<u64, VarIntDecodeError>;
    fn read_i64(&mut self) -> Result<i64, VarIntDecodeError>;
    fn read_u128(&mut self) -> Result<u128, VarIntDecodeError>;
    fn read_i128(&mut self) -> Result<i128, VarIntDecodeError>;
    fn read_f32(&mut self) -> Result<f32, VarIntDecodeError>;
    fn read_f64(&mut self) -> Result<f64, VarIntDecodeError>;
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

    fn read_i8(&mut self) -> Result<i8, VarIntDecodeError> {
        self.read_u8().map(|byte| byte as i8)
    }

    fn read_u16(&mut self) -> Result<u16, VarIntDecodeError> {
        if self.len() < 2 {
            Err(VarIntDecodeError::Incomplete)
        } else {
            // Read the first two bytes
            let result = u16::from_be_bytes([self[0], self[1]]);
            *self = &self[2..]; // Advance the slice
            Ok(result)
        }
    }

    fn read_i16(&mut self) -> Result<i16, VarIntDecodeError> {
        self.read_u16().map(|val| val as i16)
    }

    fn read_u32(&mut self) -> Result<u32, VarIntDecodeError> {
        if self.len() < 4 {
            Err(VarIntDecodeError::Incomplete)
        } else {
            let result = u32::from_be_bytes([self[0], self[1], self[2], self[3]]);
            *self = &self[4..];
            Ok(result)
        }
    }

    fn read_i32(&mut self) -> Result<i32, VarIntDecodeError> {
        self.read_u32().map(|val| val as i32)
    }

    fn read_u64(&mut self) -> Result<u64, VarIntDecodeError> {
        if self.len() < 8 {
            Err(VarIntDecodeError::Incomplete)
        } else {
            let result = u64::from_be_bytes([
                self[0], self[1], self[2], self[3], self[4], self[5], self[6], self[7],
            ]);
            *self = &self[8..];
            Ok(result)
        }
    }

    fn read_i64(&mut self) -> Result<i64, VarIntDecodeError> {
        self.read_u64().map(|val| val as i64)
    }

    fn read_u128(&mut self) -> Result<u128, VarIntDecodeError> {
        if self.len() < 16 {
            Err(VarIntDecodeError::Incomplete)
        } else {
            let result = u128::from_be_bytes([
                self[0], self[1], self[2], self[3], self[4], self[5], self[6], self[7],
                self[8], self[9], self[10], self[11], self[12], self[13], self[14], self[15],
            ]);
            *self = &self[16..];
            Ok(result)
        }
    }

    fn read_i128(&mut self) -> Result<i128, VarIntDecodeError> {
        self.read_u128().map(|val| val as i128)
    }


    fn read_f32(&mut self) -> Result<f32, VarIntDecodeError> {
        self.read_u32().map(|val| f32::from_bits(val))
    }

    fn read_f64(&mut self) -> Result<f64, VarIntDecodeError> {
        self.read_u64().map(|val| f64::from_bits(val))
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
            let byte = r.read_u8().map_err(|_e| VarIntDecodeError::Incomplete)?;
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

        // Step 1: Create the stage1 variable
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        // Step 2: Calculate the number of bytes needed to represent the value
        let leading = stage1.leading_zeros();
        let unused_bytes = (leading - 1) >> 3; // Each byte uses 7 bits
        let bytes_needed = 8 - unused_bytes; // Total bytes required

        // Step 3: Add the continuation bits (MSBs) for all but the last byte
        let msbs = 0x8080808080808080; // MSBs to indicate continuation
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);
        let merged = stage1 | (msbs & msbmask);

        // Step 4: Write the bytes to the writer
        let bytes = merged.to_le_bytes();
        w.write_all(&bytes[..bytes_needed as usize])
            .map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;

        Ok(())
    }
}
use esp_println::println;

impl Decode<'_> for VarInt {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        println!("r: {:?}", r);
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            let byte = r.read_u8().map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
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
