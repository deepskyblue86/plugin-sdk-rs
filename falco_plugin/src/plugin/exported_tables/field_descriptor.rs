use crate::plugin::tables::data::FieldTypeId;
use falco_plugin_api::{ss_plugin_field_type, ss_plugin_table_fieldinfo};
use std::rc::Rc;

/// An opaque id describing a particular field
///
/// It's effectively an index describing the order in which the fields were added,
/// separately for static and dynamic fields
#[derive(Clone, Copy, Eq, PartialEq, Debug, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum FieldId {
    Static(usize),
    Dynamic(usize),
}

/// A reference to a field descriptor
///
/// For static fields, it's a static reference to the descriptor (stored in a static global
/// somewhere). For dynamic fields, it's a reference-counted pointer to the descriptor living
/// in a runtime-managed map
#[allow(missing_docs)]
pub enum FieldRef {
    Static(&'static FieldDescriptor),
    Dynamic(Rc<FieldDescriptor>),
}

impl AsRef<FieldDescriptor> for FieldRef {
    fn as_ref(&self) -> &FieldDescriptor {
        match self {
            FieldRef::Static(s) => s,
            FieldRef::Dynamic(d) => d.as_ref(),
        }
    }
}

/// # A descriptor for a dynamically added field
///
/// It knows its sequential ID (to look up fields by numbers, not by strings all the time)
/// and the type of stored data.
///
/// **Note**: the data is stored as an enum capable of holding values of any type, but the table enforces
/// the defined type on all incoming data.
pub struct FieldDescriptor {
    pub(in crate::plugin::exported_tables) index: FieldId,
    pub(in crate::plugin::exported_tables) type_id: FieldTypeId,
    pub(in crate::plugin::exported_tables) read_only: bool,
}

impl FieldDescriptor {
    /// Create a field
    ///
    /// This gets called by the derive macro for visible entry fields (excluding private ones)
    /// and returns a field descriptor
    pub const fn new(index: FieldId, type_id: FieldTypeId, read_only: bool) -> Self {
        Self {
            index,
            type_id,
            read_only,
        }
    }

    /// Get the raw API representation of a field descriptor
    ///
    /// This is used to list table fields
    pub fn to_raw(&self, name: &'static [u8]) -> ss_plugin_table_fieldinfo {
        ss_plugin_table_fieldinfo {
            name: name.as_ptr().cast(),
            field_type: self.type_id as ss_plugin_field_type,
            read_only: self.read_only.into(),
        }
    }
}
