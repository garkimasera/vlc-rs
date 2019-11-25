// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use vlc_sys as sys;
use crate::{Instance, MediaList};

pub struct MediaLibrary {
    pub(crate) ptr: *mut sys::libvlc_media_library_t,
}

impl MediaLibrary {
    /// Create an new Media Library object.
    pub fn new(instance: &Instance) -> Option<MediaLibrary> {
        unsafe{
            let p = sys::libvlc_media_library_new(instance.ptr);
            if p.is_null() { None }else{ Some(MediaLibrary{ptr: p}) }
        }
    }

    /// Load media library.
    pub fn load(&self) -> Result<(), ()> {
        unsafe{
            if sys::libvlc_media_library_load(self.ptr) == 0 { Ok(()) }else{ Err(()) }
        }
    }

    /// Get media library subitems.
    pub fn media_list(&self) -> Option<MediaList> {
        unsafe{
            let p = sys::libvlc_media_library_media_list(self.ptr);
            if p.is_null() { None }else{ Some(MediaList{ptr: p}) }
        }
    }

    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_media_library_t {
        self.ptr
    }
}

impl Drop for MediaLibrary {
    fn drop(&mut self) {
        unsafe{ sys::libvlc_media_library_release(self.ptr) };
    }
}
