use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use vlc_sys as sys;
use crate::Instance;
use crate::tools::{from_cstr, to_cstr};

pub trait Vlm {
    fn add_broadcast(&self, name: &str, input: &str, output: &str, options: Option<Vec<String>>, enabled: bool, loop_broadcast: bool, ) -> Result<(), ()>;

    fn add_vod(&self, name: &str, input: &str, mux: &str, options: Option<Vec<String>>, enabled: bool) -> Result<(), ()>;

    fn play_media(&self, name: &str) -> Result<(), ()>;

    fn pause_media(&self, name: &str) -> Result<(), ()>;

    fn stop_media(&self, name: &str) -> Result<(), ()>;

    fn get_media_instance_position(&self, name: &str, instance: i32) -> Result<f32, ()>;

    fn get_media_instance_length(&self, name: &str, instance: i32) -> Result<i32, ()>;

    fn get_media_instance_time(&self, name: &str, instance: i32) -> Result<i32, ()>;

    fn get_media_instance_rate(&self, name: &str, instance: i32) -> Result<i32, ()>;

    fn show_media(&self, name: &str) -> Result<String, ()>;
}

impl Vlm for Instance {
    fn add_broadcast(&self, name: &str, input: &str, output: &str, options: Option<Vec<String>>, enabled: bool, loop_broadcast: bool, ) -> Result<(), ()> {
        let name = to_cstr(name);
        let input = to_cstr(input);
        let output = to_cstr(output);
        let opts_c_ptr: Vec<*const c_char>;
        let opts_c: Vec<CString>;
        let enabled = if enabled { 1 } else { 0 };
        let loop_broadcast = if loop_broadcast { 1 } else { 0 };
        if let Some(vec) = options {
            opts_c = vec.into_iter()
                .map(|x| CString::new(x).expect("Error: Unexpected null byte")).collect();
            opts_c_ptr = opts_c.iter().map(|x| x.as_ptr()).collect();
        } else {
            opts_c_ptr = Vec::new();
        }
        let result = unsafe {
            if opts_c_ptr.is_empty() {
                sys::libvlc_vlm_add_broadcast(self.ptr, name.as_ptr(), input.as_ptr(), output.as_ptr(), 0, ptr::null(), enabled, loop_broadcast)
            } else {
                sys::libvlc_vlm_add_broadcast(self.ptr, name.as_ptr(), input.as_ptr(), output.as_ptr(), opts_c_ptr.len() as i32, opts_c_ptr.as_ptr(), enabled, loop_broadcast)
            }
        };
        if result == 0 { Ok(()) } else { Err(()) }
    }

    fn add_vod(&self, name: &str, input: &str, mux: &str, options: Option<Vec<String>>, enabled: bool) -> Result<(), ()> {
        let name = to_cstr(name);
        let input = to_cstr(input);
        let mux = to_cstr(mux);
        let opts_c_ptr: Vec<*const c_char>;
        let opts_c: Vec<CString>;
        let enabled = if enabled { 1 } else { 0 };
        if let Some(vec) = options {
            opts_c = vec.into_iter()
                .map(|x| CString::new(x).expect("Error: Unexpected null byte")).collect();
            opts_c_ptr = opts_c.iter().map(|x| x.as_ptr()).collect();
        } else {
            opts_c_ptr = Vec::new();
        }
        let result = unsafe {
            if opts_c_ptr.is_empty() {
                sys::libvlc_vlm_add_vod(self.ptr, name.as_ptr(), input.as_ptr(), 0, ptr::null(), enabled, mux.as_ptr())
            } else {
                sys::libvlc_vlm_add_vod(self.ptr, name.as_ptr(), input.as_ptr(), opts_c_ptr.len() as i32, opts_c_ptr.as_ptr(), enabled, mux.as_ptr())
            }
        };
        if result == 0 { Ok(()) } else { Err(()) }
    }

    fn play_media(&self, name: &str) -> Result<(), ()> {
        let name = to_cstr(name);
        let result = unsafe {
            sys::libvlc_vlm_play_media(self.ptr, name.as_ptr())
        };
        if result == 0 { Ok(()) } else { Err(()) }
    }

    fn pause_media(&self, name: &str) -> Result<(), ()> {
        let name = to_cstr(name);
        let result = unsafe {
            sys::libvlc_vlm_pause_media(self.ptr, name.as_ptr())
        };
        if result == 0 { Ok(()) } else { Err(()) }
    }

    fn stop_media(&self, name: &str) -> Result<(), ()> {
        let name = to_cstr(name);
        let result = unsafe {
            sys::libvlc_vlm_stop_media(self.ptr, name.as_ptr())
        };
        if result == 0 { Ok(()) } else { Err(()) }
    }

    fn get_media_instance_position(&self, name: &str, instance: i32) -> Result<f32, ()> {
        let name = to_cstr(name);
        let result = unsafe {
            sys::libvlc_vlm_get_media_instance_position(self.ptr, name.as_ptr(), instance)
        };
        if result != -1f32 { Ok(result) } else { Err(()) }
    }

    fn get_media_instance_length(&self, name: &str, instance: i32) -> Result<i32, ()> {
        let name = to_cstr(name);
        let result = unsafe {
            sys::libvlc_vlm_get_media_instance_length(self.ptr, name.as_ptr(), instance)
        };
        if result != -1 { Ok(result) } else { Err(()) }
    }

    fn get_media_instance_time(&self, name: &str, instance: i32) -> Result<i32, ()> {
        let name = to_cstr(name);
        let result = unsafe {
            sys::libvlc_vlm_get_media_instance_time(self.ptr, name.as_ptr(), instance)
        };
        if result != -1 { Ok(result) } else { Err(()) }
    }

    fn get_media_instance_rate(&self, name: &str, instance: i32) -> Result<i32, ()> {
        let name = to_cstr(name);
        let result = unsafe {
            sys::libvlc_vlm_get_media_instance_rate(self.ptr, name.as_ptr(), instance)
        };
        if result != -1 { Ok(result) } else { Err(()) }
    }

    fn show_media(&self, name: &str) -> Result<String, ()> {
        let name = to_cstr(name);
        let result = unsafe {
            from_cstr(sys::libvlc_vlm_show_media(self.ptr, name.as_ptr()))
        };
        if let Some(data) = result {
            Ok(data.to_string())
        } else {
            Err(())
        }
    }
}
