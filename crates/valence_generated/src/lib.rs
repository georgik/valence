#![allow(clippy::unseparated_literal_suffix)]
#![no_std]
pub mod block;

pub mod attributes {
    use core::include;
    use core::concat;
    use core::env;
    include!(concat!(env!("OUT_DIR"), "/attributes.rs"));
}

pub mod item {
    use core::include;
    use core::concat;
    use core::env;
    include!(concat!(env!("OUT_DIR"), "/item.rs"));
}

pub mod sound {
    use core::include;
    use core::concat;
    use core::env;
    include!(concat!(env!("OUT_DIR"), "/sound.rs"));
}

/// Contains constants for every vanilla packet ID.
pub mod packet_id {
    use core::include;
    use core::concat;
    use core::env;
    include!(concat!(env!("OUT_DIR"), "/packet_id.rs"));
}

pub mod chunk_view {
    use core::include;
    use core::concat;
    use core::env;
    include!(concat!(env!("OUT_DIR"), "/chunk_view.rs"));
}

pub mod status_effects {
    use core::include;
    use core::concat;
    use core::env;
    include!(concat!(env!("OUT_DIR"), "/status_effects.rs"));
}
