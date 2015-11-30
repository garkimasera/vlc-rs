// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use ffi;

pub struct MediaList {
    pub ptr: *mut ffi::libvlc_media_list_t,
}

impl Drop for MediaList {
    fn drop(&mut self) {
        unsafe{ ffi::libvlc_media_list_release(self.ptr) };
    }
}
