use std::cell::RefCell;
use std::ffi::{c_char, CStr};
use std::rc::Rc;

use num_traits::FromPrimitive;

use falco_plugin_api::{
    ss_plugin_bool, ss_plugin_rc, ss_plugin_rc_SS_PLUGIN_FAILURE, ss_plugin_rc_SS_PLUGIN_SUCCESS,
    ss_plugin_state_data, ss_plugin_state_type, ss_plugin_table_entry_t, ss_plugin_table_field_t,
    ss_plugin_table_fieldinfo, ss_plugin_table_fields_vtable_ext, ss_plugin_table_iterator_func_t,
    ss_plugin_table_iterator_state_t, ss_plugin_table_reader_vtable_ext, ss_plugin_table_t,
    ss_plugin_table_writer_vtable_ext,
};

use crate::plugin::error::ffi_result::FfiResult;
use crate::plugin::tables::data::{FieldTypeId, Key};
use crate::tables::export::{DynamicField, DynamicTable, Entry};

// SAFETY: `table` must be a valid pointer to Table<K,E>
unsafe extern "C" fn get_table_name<K, E>(table: *mut ss_plugin_table_t) -> *const c_char
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return std::ptr::null_mut();
        };
        table.name().as_ptr()
    }
}

// SAFETY: `table` must be a valid pointer to Table<K,E>
unsafe extern "C" fn get_table_size<K, E>(table: *mut ss_plugin_table_t) -> u64
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return 0;
        };
        table.size() as u64
    }
}

// SAFETY: `table` must be a valid pointer to Table<K,E>
// SAFETY: `key` must be a valid pointer to ss_plugin_state_data
unsafe extern "C" fn get_table_entry<K, E>(
    table: *mut ss_plugin_table_t,
    key: *const ss_plugin_state_data,
) -> *mut ss_plugin_table_entry_t
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return std::ptr::null_mut();
        };
        let Some(key) = key.as_ref() else {
            return std::ptr::null_mut();
        };

        let key = K::from_data(key);
        match table.lookup(key) {
            Some(entry) => Box::into_raw(Box::new(entry)) as *mut _,
            None => std::ptr::null_mut(),
        }
    }
}

// SAFETY: all pointers must be valid
unsafe extern "C" fn read_entry_field<K, E>(
    table: *mut ss_plugin_table_t,
    entry: *mut ss_plugin_table_entry_t,
    field: *const ss_plugin_table_field_t,
    out: *mut ss_plugin_state_data,
) -> ss_plugin_rc
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let Some(entry) = (entry as *mut Rc<RefCell<E>>).as_mut() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let Some(field) = (field as *const Rc<DynamicField>).as_ref() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let Some(out) = out.as_mut() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };

        table.get_field_value(entry, field, out).status_code()
    }
}

// SAFETY: all pointers must be valid
unsafe extern "C" fn release_table_entry<E>(
    _table: *mut ss_plugin_table_t,
    entry: *mut ss_plugin_table_entry_t,
) where
    E: Entry,
{
    if !entry.is_null() {
        unsafe {
            drop(Box::from_raw(entry as *mut Rc<RefCell<E>>));
        }
    }
}

// SAFETY: all pointers must be valid
unsafe extern "C" fn iterate_entries<K, E>(
    table: *mut ss_plugin_table_t,
    func: ss_plugin_table_iterator_func_t,
    state: *mut ss_plugin_table_iterator_state_t,
) -> ss_plugin_bool
where
    K: Key + Ord + Clone,
    E: Entry,
{
    let Some(func) = func else {
        return 0;
    };
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return 0;
        };

        table.iterate_entries(|e| {
            let entry = e as *mut _ as *mut ss_plugin_table_entry_t;
            func(state, entry) != 0
        });
    }

    1
}

// SAFETY: `table` must be a valid pointer to Table<K,E>
unsafe extern "C" fn clear_table<K, E>(table: *mut ss_plugin_table_t) -> ss_plugin_rc
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        table.clear();
    }
    ss_plugin_rc_SS_PLUGIN_SUCCESS
}

// TODO(spec) is removing a nonexistent entry an error?
// SAFETY: all pointers must be valid
unsafe extern "C" fn erase_table_entry<K, E>(
    table: *mut ss_plugin_table_t,
    key: *const ss_plugin_state_data,
) -> ss_plugin_rc
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let Some(key) = key.as_ref() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let key = K::from_data(key);
        table.erase(key);
    }
    ss_plugin_rc_SS_PLUGIN_SUCCESS
}

// SAFETY: `table` must be a valid pointer to Table<K,E>
extern "C" fn create_table_entry<K, E>(
    _table: *mut ss_plugin_table_t,
) -> *mut ss_plugin_table_entry_t
where
    K: Key + Ord + Clone,
    E: Entry,
{
    Box::into_raw(Box::new(DynamicTable::<K, E>::create_entry())).cast()
}

// TODO(spec) what if the entry already exists?
// SAFETY: all pointers must be valid
unsafe extern "C" fn add_table_entry<K, E>(
    table: *mut ss_plugin_table_t,
    key: *const ss_plugin_state_data,
    entry: *mut ss_plugin_table_entry_t,
) -> *mut ss_plugin_table_entry_t
where
    K: Key + Ord + Clone,
    E: Entry,
{
    if entry.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return std::ptr::null_mut();
        };
        let Some(key) = key.as_ref() else {
            return std::ptr::null_mut();
        };
        let key = K::from_data(key);
        let entry = Box::from_raw(entry as *mut Rc<RefCell<E>>);

        match table.add(key, *entry) {
            Some(entry) => Box::into_raw(Box::new(entry)) as *mut _,
            None => std::ptr::null_mut(),
        }
    }
}

// SAFETY: all pointers must be valid
unsafe extern "C" fn write_entry_field<K, E>(
    table: *mut ss_plugin_table_t,
    entry: *mut ss_plugin_table_entry_t,
    field: *const ss_plugin_table_field_t,
    value: *const ss_plugin_state_data,
) -> ss_plugin_rc
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let Some(entry) = (entry as *mut Rc<RefCell<E>>).as_mut() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let Some(field) = (field as *const Rc<DynamicField>).as_ref() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let Some(value) = value.as_ref() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        table.write(entry, field, value).status_code()
    }
}

// SAFETY: all pointers must be valid
unsafe extern "C" fn list_table_fields<K, E>(
    table: *mut ss_plugin_table_t,
    nfields: *mut u32,
) -> *const ss_plugin_table_fieldinfo
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return std::ptr::null_mut();
        };
        let fields = table.list_fields();
        *nfields = fields.len() as u32;
        fields.as_ptr()
    }
}

// SAFETY: all pointers must be valid
unsafe extern "C" fn get_table_field<K, E>(
    table: *mut ss_plugin_table_t,
    name: *const c_char,
    data_type: ss_plugin_state_type,
) -> *mut ss_plugin_table_field_t
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return std::ptr::null_mut();
        };
        let Some(data_type) = FieldTypeId::from_usize(data_type as usize) else {
            return std::ptr::null_mut();
        };
        let name = if name.is_null() {
            return std::ptr::null_mut();
        } else {
            CStr::from_ptr(name)
        };
        match table.get_field(name, data_type) {
            Some(field) => Box::into_raw(Box::new(field)) as *mut _,
            None => std::ptr::null_mut(),
        }
    }
}

// SAFETY: all pointers must be valid
unsafe extern "C" fn add_table_field<K, E>(
    table: *mut ss_plugin_table_t,
    name: *const c_char,
    data_type: ss_plugin_state_type,
) -> *mut ss_plugin_table_field_t
where
    K: Key + Ord + Clone,
    E: Entry,
{
    unsafe {
        let Some(table) = (table as *mut DynamicTable<K, E>).as_mut() else {
            return std::ptr::null_mut();
        };
        let Some(data_type) = FieldTypeId::from_usize(data_type as usize) else {
            return std::ptr::null_mut();
        };
        let name = if name.is_null() {
            return std::ptr::null_mut();
        } else {
            CStr::from_ptr(name)
        };
        match table.add_field(name, data_type, false) {
            Some(field) => Box::into_raw(Box::new(field)) as *mut _,
            None => std::ptr::null_mut(),
        }
    }
}

pub(crate) fn reader_vtable<K, E>() -> ss_plugin_table_reader_vtable_ext
where
    K: Key + Ord + Clone,
    E: Entry,
{
    ss_plugin_table_reader_vtable_ext {
        get_table_name: Some(get_table_name::<K, E>),
        get_table_size: Some(get_table_size::<K, E>),
        get_table_entry: Some(get_table_entry::<K, E>),
        read_entry_field: Some(read_entry_field::<K, E>),
        release_table_entry: Some(release_table_entry::<E>),
        iterate_entries: Some(iterate_entries::<K, E>),
    }
}

pub(crate) fn writer_vtable<K, E>() -> ss_plugin_table_writer_vtable_ext
where
    K: Key + Ord + Clone,
    E: Entry,
{
    ss_plugin_table_writer_vtable_ext {
        clear_table: Some(clear_table::<K, E>),
        erase_table_entry: Some(erase_table_entry::<K, E>),
        create_table_entry: Some(create_table_entry::<K, E>),
        destroy_table_entry: Some(release_table_entry::<E>), // same as release_table_entry
        add_table_entry: Some(add_table_entry::<K, E>),
        write_entry_field: Some(write_entry_field::<K, E>),
    }
}

pub(crate) fn fields_vtable<K, E>() -> ss_plugin_table_fields_vtable_ext
where
    K: Key + Ord + Clone,
    E: Entry,
{
    ss_plugin_table_fields_vtable_ext {
        list_table_fields: Some(list_table_fields::<K, E>),
        get_table_field: Some(get_table_field::<K, E>),
        add_table_field: Some(add_table_field::<K, E>),
    }
}
