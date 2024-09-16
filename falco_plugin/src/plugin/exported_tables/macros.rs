#[doc(hidden)]
#[macro_export]
macro_rules! impl_export_table_get {
    (
        $self:ident,
        static: $($i:literal: $field_name:ident,)*
    ) => {
        fn get(
            &$self,
            key: $crate::internals::tables::export::FieldId,
            type_id: $crate::internals::tables::export::FieldTypeId,
            out: &mut $crate::api::ss_plugin_state_data,
        ) -> Result<(), $crate::anyhow::Error> {
            use $crate::internals::tables::export::FieldValue;
            use $crate::internals::tables::export::FieldId;
            match key {
                $(FieldId::Static($i) => $self.$field_name.to_data(out, type_id),)*
                _ => $crate::anyhow::bail!("Unknown field")
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_export_table_set {
    (
        $self:ident,
        static: $($i:literal: $field_name:ident,)*
    ) => {
        fn set(
            &mut $self,
            key: $crate::internals::tables::export::FieldId,
            value: $crate::internals::tables::export::DynamicFieldValue)
            -> std::result::Result<(), $crate::anyhow::Error> {
            use $crate::internals::tables::export::FieldId;
            match key {
                $(FieldId::Static($i) => Ok($self.$field_name = value.try_into()?),)*
                _ => $crate::anyhow::bail!("Unknown field")
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_export_table {
    (for $name:ident {
        $([$i:literal] $field_tag:literal ($field_name_bstr:literal) as $field_name:ident: $field_type:ty; readonly = $readonly:literal)*
    }) => {
        const _: () = {
            use $crate::internals::tables::FieldTypeId;
            use $crate::internals::tables::export::FieldDescriptor;
            use $crate::internals::tables::export::FieldId;
            use $crate::internals::tables::export::FieldRef;
            use $crate::internals::tables::export::HasMetadata;
            use $crate::internals::tables::export::Metadata;
            use $crate::internals::tables::export::StaticField;
            use $crate::internals::tables::export::TableMetadata;
            use $crate::api::ss_plugin_table_fieldinfo;
            use $crate::phf;

            static STATIC_FIELDS: $crate::phf::Map<&'static [u8], FieldDescriptor> = $crate::phf::phf_map! {
                $($field_name_bstr => FieldDescriptor::new(
                    FieldId::Static($i),
                    <$field_type as StaticField>::TYPE_ID,
                    $readonly,
                ),)*
            };

            pub struct EntryMetadata {
                $(pub $field_name: <$field_type as HasMetadata>::Metadata,)*
            }

            impl Metadata for EntryMetadata {
                fn new() -> $crate::anyhow::Result<Self> {
                    Ok(Self {
                        $($field_name: Metadata::new()?,)*
                    })
                }
            }

            impl TableMetadata for EntryMetadata {
                fn get_field(&self, name: &::std::ffi::CStr) ->
                    std::option::Option<FieldRef>
                {
                    let field = STATIC_FIELDS.get(name.to_bytes_with_nul())?;
                    Some(FieldRef::Static(field))
                }

                fn add_field(
                    &mut self,
                    name: &std::ffi::CStr,
                    field_type: FieldTypeId,
                    read_only: bool,
                ) ->
                    std::option::Option<FieldRef>
                {
                    None
                }

                fn list_fields(&self) -> std::vec::Vec<ss_plugin_table_fieldinfo> {
                    STATIC_FIELDS
                        .entries()
                        .map(|(name, field)| field.to_raw(name))
                        .collect()
                }
            }

            impl HasMetadata for $name {
                type Metadata = ::std::rc::Rc<::std::cell::RefCell<EntryMetadata>>;

                fn new_with_metadata(tag: &'static std::ffi::CStr, meta: &Self::Metadata) -> ::std::result::Result<Self, $crate::anyhow::Error> {
                    Ok(Self {
                       $($field_name: HasMetadata::new_with_metadata($field_tag, &meta.borrow().$field_name)?,)*
                        .. std::default::Default::default()
                    })
                }
            }

            impl $crate::internals::tables::export::Entry for $name {
                $crate::impl_export_table_get!(
                    self,
                    static: $($i: $field_name,)*
                );
                $crate::impl_export_table_set!(
                    self,
                    static: $($i: $field_name,)*
                );
            }
        };
    };
}
