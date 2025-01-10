use core::fmt;
use core::fmt::Write;
use crate::Encode;
use crate::Decode;

use bitfield_struct::bitfield;
// use anyhow::bail;
// use valence_math::{DVec3, IVec3};

// Custom error struct
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Error(pub BlockPos);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block position of {:?} is out of range", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

// BlockPos implementation
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn packed(self) -> Result<PackedBlockPos, Error> {
        match (self.x, self.y, self.z) {
            (-0x2000000..=0x1ffffff, -0x800..=0x7ff, -0x2000000..=0x1ffffff) => {
                Ok(PackedBlockPos::new()
                    .with_x(self.x)
                    .with_y(self.y)
                    .with_z(self.z))
            }
            _ => Err(Error(self)),
        }
    }
}

#[bitfield(u64)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PackedBlockPos {
    #[bits(12)]
    pub y: i32,
    #[bits(26)]
    pub z: i32,
    #[bits(26)]
    pub x: i32,
}

impl Encode for PackedBlockPos {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&self.x.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.y.to_le_bytes());
        bytes[6..8].copy_from_slice(&self.z.to_le_bytes());
        w.write_all(&bytes)?;
        Ok(())
    }
}

impl Decode<'_> for PackedBlockPos {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let x = i32::from_le_bytes(r[0..4].try_into()?);
        let y = i32::from_le_bytes(r[4..6].try_into()?);
        let z = i32::from_le_bytes(r[6..8].try_into()?);
        Ok(Self { x, y, z })
    }
}
