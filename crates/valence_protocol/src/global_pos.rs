use alloc::borrow::Cow;

use valence_ident::Ident;

use crate::BlockPos;
use crate::{Decode, Encode};

#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
pub struct GlobalPos<'a> {
    pub dimension_name: Ident<Cow<'a, str>>,
    pub position: BlockPos,
}
