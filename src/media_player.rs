// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use ffi;
use ::Instance;
use ::Media;
use ::EventManager;
use ::libc::{c_void, c_uint};
use std::mem::transmute;

/// A LibVLC media player plays one media (usually in a custom drawable).
pub struct MediaPlayer {
    pub ptr: *mut ffi::libvlc_media_player_t,
}

impl MediaPlayer {
    /// Create an empty Media Player object
    pub fn new(instance: &Instance) -> Option<MediaPlayer> {
        unsafe{
            let p = ffi::libvlc_media_player_new(instance.ptr);

            if p.is_null() {
                return None;
            }
            Some(MediaPlayer{ptr: p})
        }
    }

    /// Set the media that will be used by the media_player. If any, previous md will be released.
    pub fn set_media(&self, md: &Media) {
        unsafe{ ffi::libvlc_media_player_set_media(self.ptr, md.ptr) };
    }

    /// Get the media used by the media_player.
    pub fn get_media(&self) -> Option<Media> {
        let p = unsafe{ ffi::libvlc_media_player_get_media(self.ptr) };
        if p.is_null() {
            None
        }else{
            Some(Media{ptr: p})
        }
    }

    /// Get the Event Manager from which the media player send event.
    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe{
            let p = ffi::libvlc_media_player_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager{ptr: p, _phantomdata: ::std::marker::PhantomData}
        }
    }

    /// is_playing
    pub fn is_playing(&self) -> bool {
        if unsafe{ ffi::libvlc_media_player_is_playing(self.ptr) } == 0 {
            false
        }else{
            true
        }
    }

    /// Play
    pub fn play(&self) -> Result<(), ()> {
        if unsafe{ ffi::libvlc_media_player_play(self.ptr) } == 0 {
            Ok(())
        }else{
            Err(())
        }
    }

    /// Pause or resume (no effect if there is no media)
    pub fn set_pause(&self, do_pause: bool) {
        unsafe{ ffi::libvlc_media_player_set_pause(self.ptr, if do_pause {1} else {0}) };
    }

    /// Toggle pause (no effect if there is no media)
    pub fn pause(&self) {
        unsafe{ ffi::libvlc_media_player_pause(self.ptr) };
    }

    /// Stop (no effect if there is no media)
    pub fn stop(&self) {
        unsafe{ ffi::libvlc_media_player_stop(self.ptr) };
    }

    pub fn set_callbacks<F>(
        &self,
        play: F,
        pause: Option<Box<Fn(i64) + Send + 'static>>,
        resume: Option<Box<Fn(i64) + Send + 'static>>,
        flush: Option<Box<Fn(i64) + Send + 'static>>,
        drain: Option<Box<Fn() + Send + 'static>>)
        where F: Fn(*const c_void, u32, i64) + Send + 'static,
    {
        let flag_pause = pause.is_some();
        let flag_resume = resume.is_some();
        let flag_flush = flush.is_some();
        let flag_drain = drain.is_some();
        
        let data = AudioCallbacksData {
            play: Box::new(play), pause: pause, resume: resume,
            flush: flush, drain: drain,
        };
        let data = Box::into_raw(Box::new(data));

        unsafe{
            ffi::libvlc_audio_set_callbacks(
                self.ptr,
                Some(audio_cb_play),
                if flag_pause {Some(audio_cb_pause)} else {None},
                if flag_resume {Some(audio_cb_resume)} else {None},
                if flag_flush {Some(audio_cb_flush)} else {None},
                if flag_drain {Some(audio_cb_drain)} else {None},
                data as *mut c_void);
        }
    }

    /// Set the NSView handler where the media player should render its video output. 
    pub fn set_nsobject(&self, drawable: *mut c_void) {
        unsafe{ ffi::libvlc_media_player_set_nsobject(self.ptr, drawable) };
    }

    /// Get the NSView handler previously set with set_nsobject(). 
    pub fn get_nsobject(&self) -> Option<*mut c_void> {
        let nso = unsafe{ ffi::libvlc_media_player_get_nsobject(self.ptr) };
        if nso.is_null() { None }else{ Some(nso) }
    }

    /// Set an X Window System drawable where the media player should render its video output.
    pub fn set_xwindow(&self, drawable: u32) {
        unsafe{ ffi::libvlc_media_player_set_xwindow(self.ptr, drawable) };
    }

    /// Get the X Window System window identifier previously set with set_xwindow(). 
    pub fn get_xwindow(&self) -> Option<u32> {
        let id = unsafe{ ffi::libvlc_media_player_get_xwindow(self.ptr) };
        if id == 0 { None }else{ Some(id) }
    }

    /// Set a Win32/Win64 API window handle (HWND) where the media player should render its video output.
    /// If LibVLC was built without Win32/Win64 API output support, then this has no effects.
    pub fn set_hwnd(&self, drawable: *mut c_void) {
        unsafe{ ffi::libvlc_media_player_set_hwnd(self.ptr, drawable) };
    }

    /// Get the Windows API window handle (HWND) previously set with set_hwnd().
    pub fn get_hwnd(&self) -> Option<*mut c_void> {
        let hwnd = unsafe{ ffi::libvlc_media_player_get_hwnd(self.ptr) };
        if hwnd.is_null() { None }else{ Some(hwnd) }
    }
}

impl Drop for MediaPlayer {
    fn drop(&mut self) {
        unsafe{ ffi::libvlc_media_player_release(self.ptr) };
    }
}

// For audio_set_callbacks
struct AudioCallbacksData {
    play: Box<Fn(*const c_void, u32, i64) + Send + 'static>,
    pause: Option<Box<Fn(i64) + Send + 'static>>,
    resume: Option<Box<Fn(i64) + Send + 'static>>,
    flush: Option<Box<Fn(i64) + Send + 'static>>,
    drain: Option<Box<Fn() + Send + 'static>>,
}

unsafe extern "C" fn audio_cb_play(
    data: *mut c_void, samples: *const c_void, count: c_uint, pts: i64) {
    let data: &AudioCallbacksData = transmute(data as *mut AudioCallbacksData);
    (data.play)(samples, count, pts);
    
}

unsafe extern "C" fn audio_cb_pause(data: *mut c_void, pts: i64) {
    let data: &AudioCallbacksData = transmute(data as *mut AudioCallbacksData);
    (data.pause.as_ref().unwrap())(pts);
}

unsafe extern "C" fn audio_cb_resume(data: *mut c_void, pts: i64) {
    let data: &AudioCallbacksData = transmute(data as *mut AudioCallbacksData);
    (data.resume.as_ref().unwrap())(pts);
}

unsafe extern "C" fn audio_cb_flush(data: *mut c_void, pts: i64) {
    let data: &AudioCallbacksData = transmute(data as *mut AudioCallbacksData);
    (data.flush.as_ref().unwrap())(pts);
}

unsafe extern "C" fn audio_cb_drain(data: *mut c_void) {
    let data: &AudioCallbacksData = transmute(data as *mut AudioCallbacksData);
    (data.drain.as_ref().unwrap())();
}

