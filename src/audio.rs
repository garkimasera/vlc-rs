// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use ffi;
use ::MediaPlayer;

pub trait MediaPlayerAudioEx {
    fn get_mute(&self) -> Option<bool>;
    fn set_mute(&self, bool);
    fn get_volume(&self) -> i32;
    fn set_volume(&self, volume: i32) -> Result<(), ()>;
}

impl MediaPlayerAudioEx for MediaPlayer {
    fn get_mute(&self) -> Option<bool> {
        let r = unsafe{ ffi::libvlc_audio_get_mute(self.ptr) };

        if r == 0 {
            Some(false)
        }else if r == -1 {
            None
        }else{
            Some(true)
        }
    }

    fn set_mute(&self, status: bool) {
        unsafe{ ffi::libvlc_audio_set_mute(self.ptr, if status { 1 }else{ 0 }) };
    }

    fn get_volume(&self) -> i32 {
        unsafe{ ffi::libvlc_audio_get_volume(self.ptr) }
    }
    fn set_volume(&self, volume: i32) -> Result<(), ()> {
        unsafe{
            if ffi::libvlc_audio_set_volume(self.ptr, volume) == 0 { Ok(()) }else{ Err(()) }
        }
    }
}
