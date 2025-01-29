use anyhow::ensure;
use crate::Write;
use core::slice;
use alloc::format;

use crate::{Decode, Encode};
use crate::var_int::Read;
use core::fmt;
// use embedded_io::Read;

#[derive(Debug)]
pub struct VarIntDecodeError {
    details: &'static str,
}

impl VarIntDecodeError {
    pub fn new(details: &'static str) -> Self {
        Self { details }
    }
}

impl fmt::Display for VarIntDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VarIntDecodeError: {}", self.details)
    }
}

// Ensure that `VarIntDecodeError` can be converted to `anyhow::Error`
impl From<VarIntDecodeError> for anyhow::Error {
    fn from(err: VarIntDecodeError) -> Self {
        anyhow::anyhow!("{}", err)
    }
}

impl Encode for bool {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_u8(u8::from(*self)).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }

    fn encode_slice(slice: &[bool], mut w: impl Write) -> anyhow::Result<()> {
        // SAFETY: Bools have the same layout as u8.
        let bytes = unsafe { slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
        w.write_all(bytes).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}




impl Decode<'_> for bool {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let n = r
            .read_u8()
            .map_err(|_| anyhow::Error::from(VarIntDecodeError::new("failed to decode VarInt")))?;
        ensure!(n <= 1, "decoded boolean byte is not 0 or 1 (got {n})");
        Ok(n == 1)
    }
}


impl Encode for u8 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_u8(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }

    fn encode_slice(slice: &[u8], mut w: impl Write) -> anyhow::Result<()> {
        w.write_all(slice).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for u8 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        r.read_u8().map_err(|_| VarIntDecodeError::new("Failed to decode u8").into())
    }
}

impl Encode for i8 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_i8(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }

    fn encode_slice(slice: &[i8], mut w: impl Write) -> anyhow::Result<()> {
        // SAFETY: The layout of `i8` is the same as `u8`, so we can safely cast.
        let bytes = unsafe { slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
        w.write_all(bytes).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for i8 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        r.read_i8().map_err(|e| anyhow::anyhow!(e))
    }
}


impl Encode for u16 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_u16(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for u16 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        r.read_u16().map_err(|e| anyhow::anyhow!(e))
    }
}

impl Encode for i16 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_i16(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for i16 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        r.read_i16().map_err(|e| anyhow::anyhow!(e))
    }
}

impl Encode for u32 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_u32(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for u32 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        r.read_u32().map_err(|e| anyhow::anyhow!(e))
    }
}

impl Encode for i32 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_i32(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for i32 {
    fn decode(r: &mut &'_ [u8]) -> anyhow::Result<Self> {
        r.read_i32().map_err(|e| anyhow::anyhow!(e))
    }
}

impl Encode for u64 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_u64(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for u64 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        r.read_u64().map_err(|e| anyhow::anyhow!(e))
    }
}

impl Encode for i64 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_i64(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for i64 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        r.read_i64().map_err(|e| anyhow::anyhow!(e))
    }
}

impl Encode for u128 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        w.write_u128(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

use embedded_io::Read as EmbeddedIoRead;
impl Decode<'_> for u128 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let mut buf = [0u8; 16];
        r.read_exact(&mut buf).map_err(|e| anyhow::anyhow!(e))?;
        Ok(u128::from_be_bytes(buf))
    }
}

impl Encode for f32 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        ensure!(
            self.is_finite(),
            "attempt to encode non-finite f32 ({})",
            self
        );
        w.write_f32(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for f32 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let f = r.read_f32().map_err(|e| anyhow::anyhow!(e))?;
        ensure!(f.is_finite(), "attempt to decode non-finite f32 ({f})");
        Ok(f)
    }
}

impl Encode for f64 {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        ensure!(
            self.is_finite(),
            "attempt to encode non-finite f64 ({})",
            self
        );
        w.write_f64(*self).map_err(|e| anyhow::Error::msg(format!("{:?}", e)))?;
        Ok(())
    }
}

impl Decode<'_> for f64 {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let f = r.read_f64().map_err(|e| anyhow::anyhow!(e))?;
        ensure!(f.is_finite(), "attempt to decode non-finite f64 ({f})");
        Ok(f)
    }
}
