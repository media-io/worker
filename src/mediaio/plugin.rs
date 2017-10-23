
use libc::*;
use std::ffi::{CStr, CString};

pub type ActionEntry = extern fn(action: *const c_char) -> *mut c_void;

#[derive(Debug)]
#[repr(C)]
struct MediaioPlugin
{
  api: *const c_char,
  api_version: c_int,
  identifier: *const c_char,
  label: *const c_char,
  description: *const c_char,
  version_major: c_uint,
  version_minor: c_uint,
  entry: ActionEntry,
}

#[derive(Debug)]
pub struct Plugin
{
  pub api: String,
  pub api_version: i32,
  pub identifier: String,
  pub label: String,
  pub description: String,
  pub version_major: u32,
  pub version_minor: u32
}

#[link(name = "mediaio-host-c")]
extern {
  fn get_plugin_count() -> i32;
  fn get_plugins() -> *mut *mut MediaioPlugin;
  fn search_plugin(searched_name: *const c_char, api: *const c_char) -> *mut MediaioPlugin;
}

fn to_string(c_data: *const c_char) -> String {
  let c_str: &CStr = unsafe {CStr::from_ptr(c_data)};
  c_str.to_str().unwrap().to_string()
}

fn to_plugin(plugin: *const MediaioPlugin) -> Plugin {
  let api = unsafe {to_string((*plugin).api)};
  let api_version = unsafe {(*plugin).api_version};
  let identifier = unsafe {to_string((*plugin).identifier)};
  let label = unsafe {to_string((*plugin).label)};
  let description = unsafe {to_string((*plugin).description)};
  let version_major = unsafe {(*plugin).version_major};
  let version_minor = unsafe {(*plugin).version_minor};

  Plugin{
    api: api,
    api_version: api_version,
    identifier: identifier,
    label: label,
    description: description,
    version_major: version_major,
    version_minor: version_minor
  }
}

pub fn get_all_plugins() -> Vec<Plugin> {
  let count = unsafe{get_plugin_count()};
  let plugins = unsafe{get_plugins()};
  let p = unsafe{ Vec::from_raw_parts(plugins, count as usize, count as usize)};

  let mut result: Vec<Plugin> = vec![];
  for plugin in p {
    result.push(to_plugin(plugin));
  }
  result
}

pub fn search(name: &str) -> Option<Plugin> {
  match CString::new(name) {
    Ok(c_name) => {
      let plugin = unsafe { search_plugin(c_name.as_ptr() as *const c_char, "MediaioDecoderPluginApi\0".as_ptr() as *const c_char) };
      if plugin.is_null() {
        None
      } else {
        Some(to_plugin(plugin))
      }
    },
    Err(_msg) => return None,
  }
}
