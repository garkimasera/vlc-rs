// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use ffi;
use ::{Instance, MediaList};

pub struct MediaLibrary {
    pub ptr: *mut ffi::libvlc_media_library_t,
}

impl MediaLibrary {
    /// Create an new Media Library object.
    pub fn new(instance: &Instance) -> Option<MediaLibrary> {
        unsafe{
            let p = ffi::libvlc_media_library_new(instance.ptr);
            if p.is_null() { None }else{ Some(MediaLibrary{ptr: p}) }
        }
    }

    /// Load media library.
    pub fn load(&self) -> Result<(), ()> {
        unsafe{
            if ffi::libvlc_media_library_load(self.ptr) == 0 { Ok(()) }else{ Err(()) }
        }
    }

    /// Get media library subitems.
    pub fn media_list(&self) -> Option<MediaList> {
        unsafe{
            let p = ffi::libvlc_media_library_media_list(self.ptr);
            if p.is_null() { None }else{ Some(MediaList{ptr: p}) }
        }
    }
}

impl Drop for MediaLibrary {
    fn drop(&mut self) {
        unsafe{ ffi::libvlc_media_library_release(self.ptr) };
    }
}
