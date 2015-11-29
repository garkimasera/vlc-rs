// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use std::ffi::{CString, CStr};
use std::borrow::Cow;
use libc::c_char;

// Convert String to CString.
// Panic if the string includes null bytes.
pub fn to_cstr(s: &str) -> CString {
    CString::new(s.to_owned()).expect("Error: Unexpected null byte")
}

// Convert *const c_char to String
pub unsafe fn from_cstr(p: *const c_char) -> Option<String> {
    if p.is_null() {
            None
    }else{
        let cstr = CStr::from_ptr(p);

        Some(cstr.to_string_lossy().into_owned())
    }
}

// Convert *const c_char to &str
pub unsafe fn from_cstr_ref<'a>(p: *const c_char) -> Option<Cow<'a, str>> {
    if p.is_null() {
            None
    }else{
        let cstr = CStr::from_ptr(p);

        Some(cstr.to_string_lossy())
    }
}
