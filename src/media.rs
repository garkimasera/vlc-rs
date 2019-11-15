// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use crate::sys;
use crate::{Instance, EventManager};
use crate::enums::{State, Meta, TrackType};
use crate::tools::{to_cstr, from_cstr, path_to_cstr};
use std::path::Path;

pub struct Media {
    pub(crate) ptr: *mut sys::libvlc_media_t,
}

impl Media {
    /// Create a media with a certain given media resource location, for instance a valid URL.
    pub fn new_location(instance: &Instance, mrl: &str) -> Option<Media> {
        let cstr = to_cstr(mrl);

        unsafe{
            let p = sys::libvlc_media_new_location(instance.ptr, cstr.as_ptr());
            if p.is_null() {
                return None;
            }

            Some(Media{ptr: p})
        }
    }

    /// Create a media for a certain file path.
    pub fn new_path<T: AsRef<Path>>(instance: &Instance, path: T) -> Option<Media> {
        let cstr = match path_to_cstr(path.as_ref()) {
            Ok(s) => s,
            Err(_) => { return None; },
        };

        unsafe{
            let p = sys::libvlc_media_new_path(instance.ptr, cstr.as_ptr());
            if p.is_null() {
                return None;
            }

            Some(Media{ptr: p})
        }
    }

    pub fn new_fd(instance: &Instance, fd: i32) -> Option<Media> {
        unsafe{
            let p = sys::libvlc_media_new_fd(instance.ptr, fd);
            if p.is_null() {
                return None;
            }

            Some(Media{ptr: p})
        }
    }

    pub fn mrl(&self) -> Option<String> {
        unsafe{
            let p_str = sys::libvlc_media_get_mrl(self.ptr);
            let s = from_cstr(p_str);
            sys::libvlc_free(p_str as *mut ::libc::c_void);
            s
        }
    }

    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe{
            let p = sys::libvlc_media_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager{ptr: p, _phantomdata: ::std::marker::PhantomData}
        }
    }

    /// Read the meta of the media.
    /// If the media has not yet been parsed this will return None.
    pub fn get_meta(&self, meta: Meta) -> Option<String> {
        unsafe{
            let p_str = sys::libvlc_media_get_meta(self.ptr, meta);
            let s = from_cstr(p_str);
            sys::libvlc_free(p_str as *mut ::libc::c_void);
            s
        }
    }

    /// Set the meta of the media.
    /// (This function will not save the meta, call save_meta in order to save the meta)
    pub fn set_meta(&self, meta: Meta, value: &str) {
        unsafe{
            sys::libvlc_media_set_meta(self.ptr, meta, to_cstr(value).as_ptr());
        }
    }

    /// Save the meta previously set.
    pub fn save_meta(&self) -> bool {
        if unsafe{ sys::libvlc_media_save_meta(self.ptr) } == 0 { false }else{ true }
    }

    /// Get current state of media descriptor object.
    pub fn state(&self) -> State {
        unsafe{ sys::libvlc_media_get_state(self.ptr) }
    }

    /// Get duration (in ms) of media descriptor object item.
    pub fn duration(&self) -> Option<i64> {
        let time = unsafe{
            sys::libvlc_media_get_duration(self.ptr)
        };
        if time != -1 { Some(time) }else{ None }
    }

    /// Parse a media.
    pub fn parse(&self) {
        unsafe{ sys::libvlc_media_parse(self.ptr) };
    }

    /// Parse a media.
    pub fn parse_async(&self) {
        unsafe{ sys::libvlc_media_parse_async(self.ptr) };
    }

    /// Get Parsed status for media descriptor object.
    pub fn is_parsed(&self) -> bool {
        if unsafe{ sys::libvlc_media_is_parsed(self.ptr) } == 0 { false }else{ true }
    }

    pub fn tracks(&self) -> Option<Vec<MediaTrack>> {
        unsafe{
            let mut p_track: *mut *mut sys::libvlc_media_track_t = ::std::ptr::null_mut();
            let n = sys::libvlc_media_tracks_get(self.ptr, &mut p_track);
            if n == 0 {
                return None;
            }

            let mut track = Vec::new();

            for i in 0..n {
                let p = p_track.offset(i as isize);
                let type_specific_data = match (**p).i_type {
                    TrackType::Audio => {
                        let audio = (**p).audio();
                        MediaTrackUnion::Audio(AudioTrack{
                            channels: (*audio).i_channels,
                            rate:     (*audio).i_rate,
                        })
                    },
                    TrackType::Video => {
                        let video = (**p).video();
                        MediaTrackUnion::Video(VideoTrack{
                            height:         (*video).i_height,
                            width:          (*video).i_width,
                            sar_num:        (*video).i_sar_num,
                            sar_den:        (*video).i_sar_den,
                            frame_rate_num: (*video).i_frame_rate_num,
                            frame_rate_den: (*video).i_frame_rate_den,
                        })
                    },
                    TrackType::Text => {
                        let subtitle = (**p).subtitle();
                        MediaTrackUnion::Subtitle(SubtitleTrack{
                            encoding: from_cstr((*subtitle).psz_encoding)
                        })
                    },
                    TrackType::Unknown => MediaTrackUnion::None,
                };
                track.push(MediaTrack{
                    codec:              (**p).i_codec,
                    original_fourcc:    (**p).i_original_fourcc,
                    id:                 (**p).i_id,
                    track_type:         (**p).i_type,
                    profile:            (**p).i_profile,
                    level:              (**p).i_level,
                    bitrate:            (**p).i_bitrate,
                    language:           from_cstr((**p).psz_language),
                    description:        from_cstr((**p).psz_description),
                    type_specific_data: type_specific_data,
                });
            }

            sys::libvlc_media_tracks_release(p_track, n);
            Some(track)
        }
    }

    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_media_t {
        self.ptr
    }
}

impl Drop for Media {
    fn drop(&mut self) {
        unsafe{ sys::libvlc_media_release(self.ptr) };
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct MediaTrack {
    pub codec: u32,
    pub original_fourcc: u32,
    pub id: i32,
    pub track_type: TrackType,
    pub profile: i32,
    pub level: i32,
    pub bitrate: u32,
    pub language: Option<String>,
    pub description: Option<String>,
    pub type_specific_data: MediaTrackUnion,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum MediaTrackUnion {
    Audio(AudioTrack), Video(VideoTrack), Subtitle(SubtitleTrack), None,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AudioTrack {
    pub channels: u32,
    pub rate: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VideoTrack {
    pub height: u32,
    pub width: u32,
    pub sar_num: u32,
    pub sar_den: u32,
    pub frame_rate_num: u32,
    pub frame_rate_den: u32,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SubtitleTrack {
    pub encoding: Option<String>,
}

