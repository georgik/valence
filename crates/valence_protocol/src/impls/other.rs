
use crate::Write;
use anyhow::Context;
use uuid::Uuid;
use valence_generated::block::{BlockEntityKind, BlockKind, BlockState};
use valence_generated::item::ItemKind;
use valence_ident::{Ident, IdentError};
use valence_nbt::Compound;
use crate::alloc::string::ToString;
use crate::{Decode, Encode, VarInt};
use alloc::format;

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        match self {
            Some(t) => {
                true.encode(&mut w)?;
                t.encode(w)
            }
            None => false.encode(w),
        }
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Option<T> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(if bool::decode(r)? {
            Some(T::decode(r)?)
        } else {
            None
        })
    }
}

impl Encode for Uuid {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        self.as_u128().encode(w)
    }
}

impl<'a> Decode<'a> for Uuid {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        u128::decode(r).map(Uuid::from_u128)
    }
}

#[cfg(not(feature = "binary"))]
fn custom_encode_to_binary(compound: &Compound, mut w: impl Write) -> anyhow::Result<()> {
    // Example: Custom encoding logic for no_std environments
    // Replace this with actual encoding logic based on your data format.
    // TODO
    // for (key, value) in compound.iter() {
    //     // Write the key and value (you'll need to implement the actual encoding logic here).
    //     write!(w, "{}: {}\n", key, value).map_err(|e| anyhow::anyhow!(e))?;
    // }
    Ok(())
}

#[cfg(feature = "binary")]
use valence_nbt::to_binary;

impl Encode for Compound {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        #[cfg(feature = "binary")]
        {
            // Use `to_binary` if the feature is available
            valence_nbt::to_binary(self, w, "")
        }
        #[cfg(not(feature = "binary"))]
        {
            // Use the custom implementation for no_std
            custom_encode_to_binary(self, w)
        }
    }
}

impl Decode<'_> for Compound {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        // Check for null compound.
        if r.first() == Some(&0) {
            *r = &r[1..];
            return Ok(Compound::new());
        }

        #[cfg(feature = "binary")]
        {
            // Use `valence_nbt::from_binary` if the feature is enabled
            valence_nbt::from_binary(r).map(|result| result.0)
        }

        #[cfg(not(feature = "binary"))]
        {
            // Use a custom decoding implementation if the feature is not enabled
            custom_decode_from_binary(r)
        }
    }
}

#[cfg(not(feature = "binary"))]
fn custom_decode_from_binary(r: &mut &[u8]) -> anyhow::Result<Compound> {
    // Placeholder for your custom decoding logic. Adapt as necessary to match the format.
    let mut compound = Compound::new();

    while !r.is_empty() {
        // Example logic: Parse key-value pairs (adapt this to your NBT format)
        let key_length = r[0] as usize;
        *r = &r[1..];

        let key = core::str::from_utf8(&r[..key_length])
            .map_err(|e| anyhow::anyhow!("Invalid key: {}", e))?;
        *r = &r[key_length..];

        // Assuming values are stored as bytes; adapt based on actual data structure
        let value = r[0];
        *r = &r[1..];

        // TODO
        // compound.insert(key.to_string(), value.into());
    }

    Ok(compound)
}


impl<S: Encode> Encode for Ident<S> {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        self.as_ref().encode(w)
    }
}

impl<'a, S> Decode<'a> for Ident<S>
where
    S: Decode<'a>,
    Ident<S>: TryFrom<S, Error = IdentError>,
{
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(Ident::try_from(S::decode(r)?)?)
    }
}

impl Encode for BlockState {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        VarInt(i32::from(self.to_raw())).encode(w)
    }
}

impl Decode<'_> for BlockState {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let id = VarInt::decode(r)?.0;
        let errmsg = "invalid block state ID";

        BlockState::from_raw(id.try_into().context(errmsg)?).context(errmsg)
    }
}

impl Encode for BlockKind {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        VarInt(i32::from(self.to_raw())).encode(w)
    }
}

impl Decode<'_> for BlockKind {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let id = VarInt::decode(r)?.0;
        let errmsg = "invalid block kind ID";

        BlockKind::from_raw(id.try_into().context(errmsg)?).context(errmsg)
    }
}

impl Encode for BlockEntityKind {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        VarInt(self.id() as i32).encode(w)
    }
}

impl<'a> Decode<'a> for BlockEntityKind {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        let id = VarInt::decode(r)?;
        Self::from_id(id.0 as u32).with_context(|| format!("id {}", id.0))
    }
}

impl Encode for ItemKind {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        VarInt(i32::from(self.to_raw())).encode(w)
    }
}

impl Decode<'_> for ItemKind {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let id = VarInt::decode(r)?.0;
        let errmsg = "invalid item ID";

        ItemKind::from_raw(id.try_into().context(errmsg)?).context(errmsg)
    }
}
