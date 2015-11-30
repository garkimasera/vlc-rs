// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use ffi;
use ::{Instance, Media, EventManager};

pub struct MediaList {
    pub ptr: *mut ffi::libvlc_media_list_t,
}

impl MediaList {
    /// Create an empty media list.
    pub fn new(instance: &Instance) -> Option<MediaList> {
        unsafe{
            let p = ffi::libvlc_media_list_new(instance.ptr);
            if p.is_null() { None }else{ Some(MediaList{ptr: p}) }
        }
    }

    /// Associate media instance with this media list instance.
    /// If another media instance was present it will be released. The libvlc_media_list_lock should NOT be held upon entering this function.
    pub fn set_media(&self, md: &Media) {
        unsafe{ ffi::libvlc_media_list_set_media(self.ptr, md.ptr); }
    }

    /// Get media instance from this media list instance.
    /// The MediaList::lock should NOT be held upon entering this function.
    pub fn media(&self) -> Option<Media> {
        unsafe{
            let p = ffi::libvlc_media_list_media(self.ptr);
            if p.is_null() { None }else{ Some(Media{ptr: p}) }
        }
    }

    /// Add media instance to media list.
    /// The MediaList::lock should be held upon entering this function.
    pub fn add_media(&self, md: &Media) -> Result<(), ()> {
        unsafe{
            if ffi::libvlc_media_list_add_media(self.ptr, md.ptr) == 0 { Ok(()) }else{ Err(()) }
        }
    }

    /// Insert media instance in media list on a position.
    /// The MediaList::lock should be held upon entering this function.
    pub fn insert_media(&self, md: &Media, pos: i32) -> Result<(), ()> {
        unsafe{
            if ffi::libvlc_media_list_insert_media(self.ptr, md.ptr, pos) == 0 { Ok(()) }else{ Err(()) }
        }
    }

    /// Remove media instance from media list on a position.
    /// The MediaList::lock should be held upon entering this function.
    pub fn remove_index(&self, pos: i32) -> Result<(), ()> {
        unsafe{
            if ffi::libvlc_media_list_remove_index(self.ptr, pos) == 0 { Ok(()) }else{ Err(()) }
        }
    }

    /// Get count on media list items.
    /// The MediaList::lock should be held upon entering this function.
    pub fn count(&self) -> i32 {
        unsafe{ ffi::libvlc_media_list_count(self.ptr) }
    }

    /// List media instance in media list at a position.
    /// The MediaList::lock should be held upon entering this function.
    pub fn item_at_index(&self, pos: i32) -> Option<Media> {
        unsafe{
            let p = ffi::libvlc_media_list_item_at_index(self.ptr, pos);
            if p.is_null() { None }else{ Some(Media{ptr: p}) }
        }
    }

    /// Find index position of List media instance in media list.
    pub fn index_of_item(&self, md: &Media) -> Option<i32> {
        unsafe{
            let i = ffi::libvlc_media_list_index_of_item(self.ptr, md.ptr);
            if i == -1 { None }else{ Some(i) }
        }
    }

    /// This indicates if this media list is read-only from a user point of view.
    pub fn is_readonly(&self) -> bool {
        unsafe{ if ffi::libvlc_media_list_is_readonly(self.ptr) == 0 { false }else{ true } }
    }

    /// Get lock on media list items
    pub fn lock(&self) {
        unsafe{ ffi::libvlc_media_list_lock(self.ptr); }
    }

    /// Release lock on media list items
    /// The libvlc_media_list_lock should be held upon entering this function.
    pub fn unlock(&self) {
        unsafe{ ffi::libvlc_media_list_unlock(self.ptr); }
    }

    /// Get EventManager from this media list instance.
    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe{
            let p = ffi::libvlc_media_list_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager{ptr: p, _phantomdata: ::std::marker::PhantomData}
        }
    }
}

impl Drop for MediaList {
    fn drop(&mut self) {
        unsafe{ ffi::libvlc_media_list_release(self.ptr) };
    }
}
