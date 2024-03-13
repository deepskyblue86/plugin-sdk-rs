//! # Falco events
//!
//! This crate provides support for working with Falco events:
//!
//! ## Event header
//!
//! ## Raw (untyped) events
//!
//! ## Typed events
//!
//! ### Primitive data types
//!
//! ### Autogenerated enums, bitflags and dynamic value types
//!
//! ### Autogenerated event types
//!
//! ## Serialization and deserialization to/from raw byte buffers
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#[allow(missing_docs)]
pub mod events;
pub mod fields;
mod types;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(missing_docs)]
mod ffi;

// things for the derive macro to access under a well-known name
mod event_derive {
    pub use byteorder::NativeEndian;
    pub use byteorder::ReadBytesExt;
    pub use byteorder::WriteBytesExt;

    pub use crate::events::Event;
    pub use crate::events::EventMetadata;
    pub use crate::events::EventPayload;
    pub use crate::events::PayloadFromBytes;
    pub use crate::events::PayloadToBytes;
    pub use crate::events::RawEvent;
    pub use crate::fields::types as event_field_type;
    pub use crate::fields::FromBytes;
    pub use crate::fields::FromBytesError;
    pub use crate::fields::FromBytesResult;
    pub use crate::fields::NoDefault;
    pub use crate::fields::ToBytes;
}
