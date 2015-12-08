// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use ffi;
use ::MediaPlayer;
use ::enums::VideoAdjustOption;

pub trait MediaPlayerVideoEx {
    fn toggle_fullscreen(&self);
    fn set_fullscreen(&self, fullscreen: bool);
    fn get_fullscreen(&self) -> bool;
    fn set_key_input(&self, on: bool);
    fn set_mouse_input(&self, on: bool);
    fn get_size(&self, num: u32) -> Option<(u32, u32)>;
    fn get_video_track(&self) -> Option<i32>;
    fn set_video_track(&self, track: i32);
    fn get_cursor(&self, num: u32) -> Option<(i32, i32)>;
    fn get_scale(&self) -> f32;
    fn set_scale(&self, factor: f32);
    fn get_adjust_int(&self, option: VideoAdjustOption) -> i32;
    fn set_adjust_int(&self, option: VideoAdjustOption, value: i32);
    fn get_adjust_float(&self, option: VideoAdjustOption) -> f32;
    fn set_adjust_float(&self, option: VideoAdjustOption, value: f32);
}

impl MediaPlayerVideoEx for MediaPlayer {
    fn toggle_fullscreen(&self) {
        unsafe{ ffi::libvlc_toggle_fullscreen(self.ptr); }
    }
    fn set_fullscreen(&self, fullscreen: bool) {
        unsafe{ ffi::libvlc_set_fullscreen(self.ptr, if fullscreen { 1 }else{ 0 }); }
    }
    fn get_fullscreen(&self) -> bool {
        unsafe{ if ffi::libvlc_get_fullscreen(self.ptr) == 0 { false }else{ true } }
    }
    fn set_key_input(&self, on: bool) {
        unsafe{ ffi::libvlc_video_set_key_input(self.ptr, if on { 1 }else{ 0 }); }
    }
    fn set_mouse_input(&self, on: bool) {
        unsafe{ ffi::libvlc_video_set_mouse_input(self.ptr, if on { 1 }else{ 0 }); }
    }
    fn get_size(&self, num: u32) -> Option<(u32, u32)> {
        unsafe{
            let mut x = 0;
            let mut y = 0;
            let res = ffi::libvlc_video_get_size(self.ptr, num, &mut x, &mut y);
            if res == -1 { None }else{ Some((x, y)) }
        }
    }
    fn get_cursor(&self, num: u32) -> Option<(i32, i32)> {
        unsafe{
            let mut x = 0;
            let mut y = 0;
            let res = ffi::libvlc_video_get_cursor(self.ptr, num, &mut x, &mut y);
            if res == -1 { None }else{ Some((x, y)) }
        }
    }
    fn get_scale(&self) -> f32 {
        unsafe{ ffi::libvlc_video_get_scale(self.ptr) }
    }
    fn set_scale(&self, factor: f32) {
        unsafe{ ffi::libvlc_video_set_scale(self.ptr, factor); }
    }
    fn get_video_track(&self) -> Option<i32> {
        unsafe{
            let track = ffi::libvlc_video_get_track(self.ptr);
            if track == -1 { None }else{ Some(track) }
        }
    }
    fn set_video_track(&self, track: i32) {
        unsafe{ ffi::libvlc_video_set_track(self.ptr, track); }
    }
    fn get_adjust_int(&self, option: VideoAdjustOption) -> i32 {
        unsafe{ ffi::libvlc_video_get_adjust_int(self.ptr, option as u32) }
    }
    fn set_adjust_int(&self, option: VideoAdjustOption, value: i32) {
        unsafe{ ffi::libvlc_video_set_adjust_int(self.ptr, option as u32, value); }
    }
    fn get_adjust_float(&self, option: VideoAdjustOption) -> f32 {
        unsafe{ ffi::libvlc_video_get_adjust_float(self.ptr, option as u32) }
    }
    fn set_adjust_float(&self, option: VideoAdjustOption, value: f32) {
        unsafe{ ffi::libvlc_video_set_adjust_float(self.ptr, option as u32, value); }
    }
}
