#![allow(unused_doc_comments)]
extern crate whatlang;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::slice::from_raw_parts;
use whatlang::Lang;

mod lang;
mod script;
mod traits;

use crate::lang::lang_from_int;
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
pub extern "C" fn detect_lang_whitelist(ptr: *const c_char, list: *const c_int, len: c_int) -> i32 {
    let cs = unsafe { CStr::from_ptr(ptr) };
    let lang_ints = unsafe { from_raw_parts(list, len as usize) };
    let whitelist = lang_ints.iter().map(lang_from_int).collect::<Vec<Lang>>();

    let detector = whatlang::Detector::with_whitelist(whitelist);

    match cs.to_str() {
        Ok(s) => {
            let res = detector.detect_lang(s);
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
