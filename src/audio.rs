// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use sys;
use ::MediaPlayer;
use ::TrackDescription;
use ::tools::from_cstr;

pub trait MediaPlayerAudioEx {
    fn get_mute(&self) -> Option<bool>;
    fn set_mute(&self, bool);
    fn get_volume(&self) -> i32;
    fn set_volume(&self, volume: i32) -> Result<(), ()>;
    fn get_audio_track_description(&self) -> Option<Vec<TrackDescription>>;
}

impl MediaPlayerAudioEx for MediaPlayer {
    fn get_mute(&self) -> Option<bool> {
        let r = unsafe{ sys::libvlc_audio_get_mute(self.ptr) };

        if r == 0 {
            Some(false)
        }else if r == -1 {
            None
        }else{
            Some(true)
        }
    }

    fn set_mute(&self, status: bool) {
        unsafe{ sys::libvlc_audio_set_mute(self.ptr, if status { 1 }else{ 0 }) };
    }

    fn get_volume(&self) -> i32 {
        unsafe{ sys::libvlc_audio_get_volume(self.ptr) }
    }
    fn set_volume(&self, volume: i32) -> Result<(), ()> {
        unsafe{
            if sys::libvlc_audio_set_volume(self.ptr, volume) == 0 { Ok(()) }else{ Err(()) }
        }
    }
    fn get_audio_track_description(&self) -> Option<Vec<TrackDescription>> {
        unsafe{
            let p0 = sys::libvlc_audio_get_track_description(self.ptr);
            if p0.is_null() { return None; }
            let mut td = Vec::new();
            let mut p = p0;

            while !(*p).p_next.is_null() {
                td.push(TrackDescription{ id: (*p).i_id, name: from_cstr((*p).psz_name) });
                p = (*p).p_next;
            }
            sys::libvlc_track_description_list_release(p0);
            Some(td)
        }
    }

}
