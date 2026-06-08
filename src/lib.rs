//! Schema-derived Signal contract for the ordinary `upgrade` surface.
//!
//! The public API of this crate is the generated wire contract: typed
//! `Input` / `Output` roots, payload records, route enums, short-header
//! codecs, and `signal-frame` request/reply aliases. Runtime migration
//! logic lives in `upgrade`.

pub mod schema {
    #[rustfmt::skip]
    pub mod lib;
}

pub use schema::lib::*;
