use std::ffi::{CStr, CString};
use std::ptr;

use lazy_static::lazy_static;
use libc::{c_char, size_t};

use super::two_touch_input::Converter;

#[repr(C)]
pub struct TwoTouchStringResult {
    len: size_t,
    data: *const *const c_char,
}

#[no_mangle]
pub extern "C" fn convert_to_two_touch_string(val: *const c_char) -> TwoTouchStringResult {
    let c_str = unsafe { CStr::from_ptr(val) };
    let s = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return TwoTouchStringResult {
                len: 0,
                data: ptr::null(),
            }
        }
    };
    let results = CONVERTER.convert_to_two_touch_string(s.to_string());
    let results = match results {
        Ok(r) => r,
        Err(_) => {
            return TwoTouchStringResult {
                len: 0,
                data: ptr::null(),
            }
        }
    };
    let mut data: Vec<*const c_char> = Vec::with_capacity(results.len());
    for r in results {
        let s = match CString::new(r.as_str()) {
            Ok(s) => s,
            Err(_) => {
                return TwoTouchStringResult {
                    len: 0,
                    data: ptr::null(),
                }
            }
        };
        data.push(s.into_raw());
    }
    let two_touch_string_result = TwoTouchStringResult {
        len: data.len(),
        data: data.as_ptr() as *const *const c_char,
    };
    std::mem::forget(data);
    two_touch_string_result
}

#[no_mangle]
pub extern "C" fn convert_from_two_touch_string(val: *const c_char) -> *const c_char {
    let c_str = unsafe { CStr::from_ptr(val) };
    let s = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null(),
    };
    let result = CONVERTER.convert_from_two_touch_string(s.to_string());
    let result = match result {
        Ok(r) => r,
        Err(_) => return ptr::null(),
    };
    let result = match CString::new(result.as_str()) {
        Ok(r) => r,
        Err(_) => return ptr::null(),
    };
    result.into_raw()
}

lazy_static! {
    static ref CONVERTER: Converter = Converter::new();
}
