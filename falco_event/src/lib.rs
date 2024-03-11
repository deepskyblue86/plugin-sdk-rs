#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use crate::events::EventType;
use crate::payload::PayloadToBytes;

pub mod dynamic_params;
pub mod event_field_type;
#[allow(missing_docs)]
pub mod event_flags;
#[allow(missing_docs)]
pub mod events;
pub mod from_bytes;
pub mod payload;
pub mod raw_event;
pub mod to_bytes;
pub mod type_id;
pub mod types;

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(missing_docs)]
mod ffi;

#[derive(Clone)]
pub struct EventMetadata {
    pub ts: u64,
    pub tid: i64,
}

impl EventMetadata {
    pub fn timestamp(&self) -> Option<SystemTime> {
        if self.ts == u64::MAX {
            None
        } else {
            Some(UNIX_EPOCH + Duration::from_nanos(self.ts))
        }
    }
}

impl Debug for EventMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventMetadata")
            .field("ts", &self.timestamp())
            .field("tid", &self.tid)
            .finish()
    }
}

impl Default for EventMetadata {
    fn default() -> Self {
        Self {
            ts: u64::MAX,
            tid: -1,
        }
    }
}

#[derive(Debug)]
pub struct Event<T> {
    pub metadata: EventMetadata,
    pub params: T,
}

pub trait EventToBytes {
    fn write<W: Write>(&self, writer: W) -> std::io::Result<()>;
}

impl<T: PayloadToBytes> EventToBytes for Event<T> {
    fn write<W: Write>(&self, writer: W) -> std::io::Result<()> {
        self.params.write(&self.metadata, writer)
    }
}

impl<'a> EventToBytes for &'a [u8] {
    fn write<W: Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(self)
    }
}

#[doc(hidden)]
// things for the derive macro to access under a well-known name
pub mod event_derive {
    pub use crate::event_field_type;
    pub use crate::from_bytes::FromBytes;
    pub use crate::from_bytes::FromBytesError;
    pub use crate::from_bytes::FromBytesResult;
    pub use crate::payload::EventPayload;
    pub use crate::payload::PayloadFromBytes;
    pub use crate::payload::PayloadToBytes;
    pub use crate::to_bytes::NoDefault;
    pub use crate::to_bytes::ToBytes;
    pub use crate::EventMetadata;
    pub use byteorder::NativeEndian;
    pub use byteorder::ReadBytesExt;
    pub use byteorder::WriteBytesExt;
}
