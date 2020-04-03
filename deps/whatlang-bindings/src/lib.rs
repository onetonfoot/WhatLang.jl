#![allow(unused_doc_comments)]
extern crate whatlang;

use std::ffi::CStr;
use std::os::raw::c_char;

mod lang;
mod script;
mod traits;

use crate::traits::ToInt;

#[repr(C)]
#[derive(Debug)]
pub struct CInfo {
    lang: i32,
    script: i32,
    confidence: f64,
}

#[no_mangle]
pub extern "C" fn detect(ptr: *const c_char, cinfo: &mut CInfo) -> i32 {
    let cs = unsafe { CStr::from_ptr(ptr) };

    match cs.to_str() {
        Ok(s) => {
            let res = whatlang::detect(s);
            match res {
                Some(info) => {
                    cinfo.lang = info.lang().to_int();
                    cinfo.script = info.script().to_int();
                    cinfo.confidence = info.confidence();
                    return 0;
                }
                None => {
                    // Could not detect language
                    return -1;
                }
            }
        }
        Err(_) => {
            // Bad string pointer
            return -2;
        }
    }
}

#[no_mangle]
pub extern "C" fn detect_lang(ptr: *const c_char) -> i32 {
    let cs = unsafe { CStr::from_ptr(ptr) };

    match cs.to_str() {
        Ok(s) => {
            let res = whatlang::detect_lang(s);
            match res {
                Some(lang) => {
                    return lang.to_int();
                }
                None => {
                    // Could not detect language
                    return -1;
                }
            }
        }
        Err(_) => {
            // Bad string pointer
            return -2;
        }
    }
}

#[no_mangle]
pub extern "C" fn detect_script(ptr: *const c_char) -> i32 {
    let cs = unsafe { CStr::from_ptr(ptr) };

    match cs.to_str() {
        Ok(s) => {
            let res = whatlang::detect_script(s);
            match res {
                Some(script) => {
                    return script.to_int();
                }
                None => {
                    // Could not detect language
                    return -1;
                }
            }
        }
        Err(_) => {
            // Bad string pointer
            return -2;
        }
    }
}
