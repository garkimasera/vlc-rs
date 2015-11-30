// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use ffi;

pub struct MediaLibrary {
    pub ptr: *mut ffi::libvlc_media_library_t,
}

impl Drop for MediaLibrary {
    fn drop(&mut self) {
        unsafe{ ffi::libvlc_media_library_release(self.ptr) };
    }
}
