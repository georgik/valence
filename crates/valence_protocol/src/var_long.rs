

use anyhow::bail;
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use core::fmt::Write;

use crate::{Decode, Encode};

/// An `i64` encoded with variable length.
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
    From,
    Into,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct VarLong(pub i64);

impl VarLong {
    /// The maximum number of bytes a `VarLong` can occupy when read from and
    /// written to the Minecraft protocol.
    pub const MAX_SIZE: usize = 10;

    /// Returns the exact number of bytes this varlong will write when
    /// [`Encode::encode`] is called, assuming no error occurs.
    pub fn written_size(self) -> usize {
        match self.0 {
            0 => 1,
            n => (63 - n.leading_zeros() as usize) / 7 + 1,
        }
    }
}
impl Encode for VarLong {
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        not(target_os = "macos"),
        feature = "std"
    ))]
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        #[cfg(target_arch = "x86")]
        use std::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;

        let mut res = [0_u64; 2];
        {
            let x = self.0 as u64;

            res[0] = unsafe { _pdep_u64(x, 0x7f7f7f7f7f7f7f7f) };
            res[1] = unsafe { _pdep_u64(x >> 56, 0x000000000000017f) };
        }
        let stage1: __m128i = unsafe { std::mem::transmute(res) };

        let minimum = unsafe { _mm_set_epi8(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff_u8 as i8) };
        let exists = unsafe { _mm_or_si128(_mm_cmpgt_epi8(stage1, _mm_setzero_si128()), minimum) };
        let bits = unsafe { _mm_movemask_epi8(exists) };

        let bytes_needed = 32 - bits.leading_zeros() as u8;

        let ascend = unsafe { _mm_setr_epi8(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15) };
        let mask = unsafe { _mm_cmplt_epi8(ascend, _mm_set1_epi8(bytes_needed as i8)) };

        let shift = unsafe { _mm_bsrli_si128(mask, 1) };
        let msbmask = unsafe { _mm_and_si128(shift, _mm_set1_epi8(128_u8 as i8)) };

        let merged = unsafe { _mm_or_si128(stage1, msbmask) };
        let bytes = unsafe { std::mem::transmute::<__m128i, [u8; 16]>(merged) };

        w.write_all(unsafe { bytes.get_unchecked(..bytes_needed as usize) })?;

        Ok(())
    }

    #[cfg(any(
        not(any(target_arch = "x86", target_arch = "x86_64")),
        target_os = "macos",
        not(feature = "std")
    ))]
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        #[cfg(feature = "std")]
        use byteorder::WriteBytesExt;

        let mut val = self.0 as u64;
        loop {
            if val & 0b1111111111111111111111111111111111111111111111111111111110000000 == 0 {
                w.write_all(&[val as u8])?;
                return Ok(());
            }
            w.write_all(&[(val as u8 & 0b01111111) | 0b10000000])?;
            val >>= 7;
        }
    }

    #[cfg(all(not(feature = "std"), feature = "no_std"))]
    fn encode(&self, mut w: &mut [u8]) -> anyhow::Result<()> {
        let mut val = self.0 as u64;
        let mut index = 0;
        loop {
            if index >= w.len() {
                return Err(anyhow::anyhow!("Buffer overflow"));
            }
            if val & 0b1111111111111111111111111111111111111111111111111111111110000000 == 0 {
                w[index] = val as u8;
                return Ok(());
            }
            w[index] = (val as u8 & 0b01111111) | 0b10000000;
            val >>= 7;
            index += 1;
        }
    }
}

impl Decode<'_> for VarLong {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            let byte = r.read_u8()?;
            val |= (i64::from(byte) & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(VarLong(val));
            }
        }
        bail!("VarInt is too large")
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::*;

    #[test]
    fn encode_decode() {
        let mut rng = thread_rng();
        let mut buf = vec![];

        for n in (0..1_000_000)
            .map(|_| rng.gen())
            .chain([0, i64::MIN, i64::MAX])
        {
            VarLong(n).encode(&mut buf).unwrap();

            let mut slice = buf.as_slice();
            assert!(slice.len() <= VarLong::MAX_SIZE);

            assert_eq!(n, VarLong::decode(&mut slice).unwrap().0);
            assert!(slice.is_empty());
            buf.clear();
        }
    }
}
