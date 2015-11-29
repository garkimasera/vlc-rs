// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use ffi;
use ::{Instance, EventManager};
use ::enums::{State, Meta, TrackType};
use ::tools::{to_cstr, from_cstr};

pub struct Media {
    pub ptr: *mut ffi::libvlc_media_t,
}

impl Media {
    /// Create a media with a certain given media resource location, for instance a valid URL. 
    pub fn new_location(instance: &Instance, mrl: &str) -> Option<Media> {
        let cstr = to_cstr(mrl);
        
        unsafe{
            let p = ffi::libvlc_media_new_location(instance.ptr, cstr.as_ptr());
            if p.is_null() {
                return None;
            }
            
            Some(Media{ptr: p})
        }
    }

    /// Create a media for a certain file path. 
    pub fn new_path(instance: &Instance, path: &str) -> Option<Media> {
        let cstr = to_cstr(path);
        
        unsafe{
            let p = ffi::libvlc_media_new_path(instance.ptr, cstr.as_ptr());
            if p.is_null() {
                return None;
            }
            
            Some(Media{ptr: p})
        }
    }
    
    pub fn new_fd(instance: &Instance, fd: i32) -> Option<Media> {
        unsafe{
            let p = ffi::libvlc_media_new_fd(instance.ptr, fd);
            if p.is_null() {
                return None;
            }
            
            Some(Media{ptr: p})
        }
    }

    pub fn mrl(&self) -> Option<String> {
        unsafe{
            let p_str = ffi::libvlc_media_get_mrl(self.ptr);
            let s = from_cstr(p_str);
            ffi::libvlc_free(p_str as *mut ::libc::c_void);
            s
        }
    }

    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe{
            let p = ffi::libvlc_media_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager{ptr: p, _phantomdata: ::std::marker::PhantomData}
        }
    }

    /// Read the meta of the media.
    /// If the media has not yet been parsed this will return None.
    pub fn get_meta(&self, meta: Meta) -> Option<String> {
        unsafe{
            let p_str = ffi::libvlc_media_get_meta(self.ptr, meta);
            let s = from_cstr(p_str);
            ffi::libvlc_free(p_str as *mut ::libc::c_void);
            s
        }
    }

    /// Set the meta of the media.
    /// (This function will not save the meta, call save_meta in order to save the meta) 
    pub fn set_meta(&self, meta: Meta, value: &str) {
        unsafe{
            ffi::libvlc_media_set_meta(self.ptr, meta, to_cstr(value).as_ptr());
        }
    }

    /// Save the meta previously set.
    pub fn save_meta(&self) -> bool {
        if unsafe{ ffi::libvlc_media_save_meta(self.ptr) } == 0 { false }else{ true }
    }

    /// Get current state of media descriptor object.
    pub fn state(&self) -> State {
        unsafe{ ffi::libvlc_media_get_state(self.ptr) }
    }

    /// Get duration (in ms) of media descriptor object item.
    pub fn duration(&self) -> Option<i64> {
        let time = unsafe{
            ffi::libvlc_media_get_duration(self.ptr)
        };
        if time != -1 { Some(time) }else{ None }
    }

    /// Parse a media. 
    pub fn parse(&self) {
        unsafe{ ffi::libvlc_media_parse(self.ptr) };
    }

    /// Parse a media.
    pub fn parse_async(&self) {
        unsafe{ ffi::libvlc_media_parse_async(self.ptr) };
    }

    /// Get Parsed status for media descriptor object.
    pub fn is_parsed(&self) -> bool {
        if unsafe{ ffi::libvlc_media_is_parsed(self.ptr) } == 0 { false }else{ true }
    }
}

impl Drop for Media {
    fn drop(&mut self) {
        unsafe{ ffi::libvlc_media_release(self.ptr) };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MediaTrackInfo {
    pub codec: u32,
    pub id: i32,
    pub track_type: TrackType,
    
    pub profile: i32,
    pub level: i32,

    audio: MediaTrackInfoAudio,
    video: MediaTrackInfoVideo,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MediaTrackInfoAudio {
    pub channels: i32,
    pub rate: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MediaTrackInfoVideo {
    pub height: i32,
    pub width: i32,
}

impl MediaTrackInfo {
    pub fn audio(&self) -> Option<MediaTrackInfoAudio> {
        if self.track_type == TrackType::Audio {
            Some(self.audio)
        }else{
            None
        }
    }

    pub fn video(&self) -> Option<MediaTrackInfoVideo> {
        if self.track_type == TrackType::Video {
            Some(self.video)
        }else{
            None
        }
    }
}
