use crate::parse::EventInput;
use crate::plugin::base::PluginWrapper;
use crate::plugin::error::ffi_result::FfiResult;
use crate::plugin::parse::ParsePlugin;
use falco_plugin_api::plugin_api__bindgen_ty_3 as parse_plugin_api;
use falco_plugin_api::{
    ss_plugin_event_input, ss_plugin_event_parse_input, ss_plugin_rc,
    ss_plugin_rc_SS_PLUGIN_FAILURE, ss_plugin_t,
};
use std::any::TypeId;
use std::collections::BTreeMap;
use std::ffi::{c_char, CString};
use std::sync::Mutex;

pub trait ParsePluginFallbackApi {
    const PARSE_API: parse_plugin_api = parse_plugin_api {
        get_parse_event_types: None,
        get_parse_event_sources: None,
        parse_event: None,
    };
}
impl<T> ParsePluginFallbackApi for T {}

pub struct ParsePluginApi<T>(std::marker::PhantomData<T>);
impl<T: ParsePlugin + 'static> ParsePluginApi<T> {
    pub const PARSE_API: parse_plugin_api = parse_plugin_api {
        get_parse_event_types: Some(plugin_get_parse_event_types::<T>),
        get_parse_event_sources: Some(plugin_get_parse_event_sources::<T>),
        parse_event: Some(plugin_parse_event::<T>),
    };
}

/// # Safety
///
/// All pointers must be valid
pub unsafe extern "C" fn plugin_get_parse_event_types<T: ParsePlugin>(
    numtypes: *mut u32,
    _plugin: *mut ss_plugin_t,
) -> *mut u16 {
    let types = T::EVENT_TYPES;
    if let Some(numtypes) = numtypes.as_mut() {
        *numtypes = types.len() as u32;
        types.as_ptr() as *const u16 as *mut u16 // this should ****really**** be const
    } else {
        std::ptr::null_mut()
    }
}

//noinspection DuplicatedCode
pub extern "C" fn plugin_get_parse_event_sources<T: ParsePlugin + 'static>() -> *const c_char {
    static SOURCES: Mutex<BTreeMap<TypeId, CString>> = Mutex::new(BTreeMap::new());

    let ty = TypeId::of::<T>();
    let mut sources_map = SOURCES.lock().unwrap();
    // we only generate the string once and never change or delete it
    // so the pointer should remain valid for the static lifetime
    sources_map
        .entry(ty)
        .or_insert_with(|| {
            let sources = serde_json::to_string(T::EVENT_SOURCES)
                .expect("failed to serialize event source array");
            CString::new(sources.into_bytes()).expect("failed to add NUL to event source array")
        })
        .as_ptr()
}

/// # Safety
///
/// All pointers must be valid
pub unsafe extern "C" fn plugin_parse_event<T: ParsePlugin>(
    plugin: *mut ss_plugin_t,
    event: *const ss_plugin_event_input,
    parse_input: *const ss_plugin_event_parse_input,
) -> ss_plugin_rc {
    let plugin = plugin as *mut PluginWrapper<T>;
    unsafe {
        let Some(plugin) = plugin.as_mut() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let Some(ref mut actual_plugin) = &mut plugin.plugin else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };

        let Some(event) = event.as_ref() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };
        let event = EventInput(*event);

        let Some(parse_input) = parse_input.as_ref() else {
            return ss_plugin_rc_SS_PLUGIN_FAILURE;
        };

        actual_plugin
            .plugin
            .parse_event(&event, &parse_input)
            .rc(&mut plugin.error_buf)
    }
}

/// # Register an event parsing plugin
///
/// This macro must be called at most once in a crate (it generates public functions)
/// with a type implementing [`ParsePlugin`] as the sole parameter.
#[macro_export]
macro_rules! parse_plugin {
    ($ty:ty) => {
        $crate::wrap_ffi! {
            #[no_mangle]
            use $crate::internals::parse::wrappers: <$ty>;

            unsafe fn plugin_get_parse_event_types(
                numtypes: *mut u32,
                plugin: *mut falco_plugin::api::ss_plugin_t,
            ) -> *mut u16;
            unsafe fn plugin_get_parse_event_sources() -> *const ::std::ffi::c_char;
            unsafe fn plugin_parse_event(
                plugin: *mut falco_plugin::api::ss_plugin_t,
                event_input: *const falco_plugin::api::ss_plugin_event_input,
                parse_input: *const falco_plugin::api::ss_plugin_event_parse_input,
            ) -> i32;
        }

        #[allow(dead_code)]
        fn __typecheck_plugin_parse_api() -> falco_plugin::api::plugin_api__bindgen_ty_3 {
            falco_plugin::api::plugin_api__bindgen_ty_3 {
                get_parse_event_types: Some(plugin_get_parse_event_types),
                get_parse_event_sources: Some(plugin_get_parse_event_sources),
                parse_event: Some(plugin_parse_event),
            }
        }
    };
}
