// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use sys;
use ::Instance;
use ::Media;
use ::EventManager;
use ::libc::{c_void, c_uint};
use ::enums::{State, Position};
use std::mem::transmute;

/// A LibVLC media player plays one media (usually in a custom drawable).
pub struct MediaPlayer {
    pub(crate) ptr: *mut sys::libvlc_media_player_t,
}

impl MediaPlayer {
    /// Create an empty Media Player object
    pub fn new(instance: &Instance) -> Option<MediaPlayer> {
        unsafe{
            let p = sys::libvlc_media_player_new(instance.ptr);

            if p.is_null() {
                return None;
            }
            Some(MediaPlayer{ptr: p})
        }
    }

    /// Set the media that will be used by the media_player. If any, previous md will be released.
    pub fn set_media(&self, md: &Media) {
        unsafe{ sys::libvlc_media_player_set_media(self.ptr, md.ptr) };
    }

    /// Get the media used by the media_player.
    pub fn get_media(&self) -> Option<Media> {
        let p = unsafe{ sys::libvlc_media_player_get_media(self.ptr) };
        if p.is_null() {
            None
        }else{
            Some(Media{ptr: p})
        }
    }

    /// Get the Event Manager from which the media player send event.
    pub fn event_manager<'a>(&'a self) -> EventManager<'a> {
        unsafe{
            let p = sys::libvlc_media_player_event_manager(self.ptr);
            assert!(!p.is_null());
            EventManager{ptr: p, _phantomdata: ::std::marker::PhantomData}
        }
    }

    /// is_playing
    pub fn is_playing(&self) -> bool {
        if unsafe{ sys::libvlc_media_player_is_playing(self.ptr) } == 0 {
            false
        }else{
            true
        }
    }

    /// Play
    pub fn play(&self) -> Result<(), ()> {
        if unsafe{ sys::libvlc_media_player_play(self.ptr) } == 0 {
            Ok(())
        }else{
            Err(())
        }
    }

    /// Pause or resume (no effect if there is no media)
    pub fn set_pause(&self, do_pause: bool) {
        unsafe{ sys::libvlc_media_player_set_pause(self.ptr, if do_pause {1} else {0}) };
    }

    /// Toggle pause (no effect if there is no media)
    pub fn pause(&self) {
        unsafe{ sys::libvlc_media_player_pause(self.ptr) };
    }

    /// Stop (no effect if there is no media)
    pub fn stop(&self) {
        unsafe{ sys::libvlc_media_player_stop(self.ptr) };
    }

    pub fn set_callbacks<F>(
        &self,
        play: F,
        pause: Option<Box<dyn Fn(i64) + Send + 'static>>,
        resume: Option<Box<dyn Fn(i64) + Send + 'static>>,
        flush: Option<Box<dyn Fn(i64) + Send + 'static>>,
        drain: Option<Box<dyn Fn() + Send + 'static>>)
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
            sys::libvlc_audio_set_callbacks(
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
        unsafe{ sys::libvlc_media_player_set_nsobject(self.ptr, drawable) };
    }

    /// Get the NSView handler previously set with set_nsobject(). 
    pub fn get_nsobject(&self) -> Option<*mut c_void> {
        let nso = unsafe{ sys::libvlc_media_player_get_nsobject(self.ptr) };
        if nso.is_null() { None }else{ Some(nso) }
    }

    /// Set an X Window System drawable where the media player should render its video output.
    pub fn set_xwindow(&self, drawable: u32) {
        unsafe{ sys::libvlc_media_player_set_xwindow(self.ptr, drawable) };
    }

    /// Get the X Window System window identifier previously set with set_xwindow(). 
    pub fn get_xwindow(&self) -> Option<u32> {
        let id = unsafe{ sys::libvlc_media_player_get_xwindow(self.ptr) };
        if id == 0 { None }else{ Some(id) }
    }

    /// Set a Win32/Win64 API window handle (HWND) where the media player should render its video output.
    /// If LibVLC was built without Win32/Win64 API output support, then this has no effects.
    pub fn set_hwnd(&self, drawable: *mut c_void) {
        unsafe{ sys::libvlc_media_player_set_hwnd(self.ptr, drawable) };
    }

    /// Get the Windows API window handle (HWND) previously set with set_hwnd().
    pub fn get_hwnd(&self) -> Option<*mut c_void> {
        let hwnd = unsafe{ sys::libvlc_media_player_get_hwnd(self.ptr) };
        if hwnd.is_null() { None }else{ Some(hwnd) }
    }

    /// Get the current movie time (in ms).
    pub fn get_time(&self) -> Option<i64> {
        unsafe{
            let t = sys::libvlc_media_player_get_time(self.ptr);
            if t == -1 { None }else{ Some(t) }
        }
    }

    /// Set the movie time (in ms).
    /// This has no effect if no media is being played. Not all formats and protocols support this.
    pub fn set_time(&self, time: i64) {
        unsafe{ sys::libvlc_media_player_set_time(self.ptr, time); }
    }

    /// Get movie position as percentage between 0.0 and 1.0.
    pub fn get_position(&self) -> Option<f32> {
        unsafe{
            let pos = sys::libvlc_media_player_get_position(self.ptr);
            if pos == -1f32 { None }else{ Some(pos) }
        }
    }

    /// Set movie position as percentage between 0.0 and 1.0.
    /// This has no effect if playback is not enabled. This might not work depending on the underlying input format and protocol.
    pub fn set_position(&self, pos: f32) {
        unsafe{ sys::libvlc_media_player_set_position(self.ptr, pos); }
    }

    /// Set movie chapter (if applicable).
    pub fn set_chapter(&self, chapter: i32) {
        unsafe{ sys::libvlc_media_player_set_chapter(self.ptr, chapter); }
    }

    /// Get movie chapter.
    pub fn get_chapter(&self) -> Option<i32> {
        unsafe{
            let c = sys::libvlc_media_player_get_chapter(self.ptr);
            if c == -1 { None }else{ Some(c) }
        }
    }

    /// Get movie chapter count.
    pub fn chapter_count(&self) -> Option<i32> {
        unsafe{
            let c = sys::libvlc_media_player_get_chapter_count(self.ptr);
            if c == -1 { None }else{ Some(c) }
        }
    }

    /// Is the player able to play.
    pub fn will_play(&self) -> bool {
        unsafe{
            let b = sys::libvlc_media_player_will_play(self.ptr);
            if b == 0 { false }else{ true }
        }
    }

    /// Get title chapter count.
    pub fn chapter_count_for_title(&self, title: i32) -> Option<i32> {
        unsafe{
            let c = sys::libvlc_media_player_get_chapter_count_for_title(self.ptr, title);
            if c == -1 { None }else{ Some(c) }
        }
    }

    /// Set movie title.
    pub fn set_title(&self, title: i32) {
        unsafe{ sys::libvlc_media_player_set_title(self.ptr, title); }
    }

    /// Get movie title.
    pub fn get_title(&self) -> Option<i32> {
        unsafe{
            let t = sys::libvlc_media_player_get_title(self.ptr);
            if t == -1 { None }else{ Some(t) }
        }
    }

    /// Get movie title count.
    pub fn title_count(&self) -> Option<i32> {
        unsafe{
            let t = sys::libvlc_media_player_get_title_count(self.ptr);
            if t == -1 { Some(t) } else { None }
        }
    }

    /// Set previous chapter (if applicable)
    pub fn previous_chapter(&self) {
        unsafe{ sys::libvlc_media_player_previous_chapter(self.ptr); }
    }

    /// Set next chapter (if applicable)
    pub fn next_chapter(&self) {
        unsafe{ sys::libvlc_media_player_next_chapter(self.ptr); }
    }

    /// Get the requested movie play rate.
    pub fn get_rate(&self) -> f32 {
        unsafe{ sys::libvlc_media_player_get_rate(self.ptr) }
    }

    /// Set movie play rate.
    pub fn set_rate(&self, rate: f32) -> Result<(),()> {
        unsafe{
            if sys::libvlc_media_player_set_rate(self.ptr, rate) == -1 {
                Err(())
            }else{
                Ok(())
            }
        }
    }

    /// Get current movie state.
    pub fn state(&self) -> State {
        unsafe{ sys::libvlc_media_player_get_state(self.ptr) }
    }

    /// How many video outputs does this media player have?
    pub fn has_vout(&self) -> u32 {
        unsafe{ sys::libvlc_media_player_has_vout(self.ptr) }
    }

    /// Is this media player seekable?
    pub fn is_seekable(&self) -> bool {
        unsafe{
            let b = sys::libvlc_media_player_is_seekable(self.ptr);
            if b == 0 { false }else{ true }
        }
    }

    /// Can this media player be paused?
    pub fn can_pause(&self) -> bool {
        unsafe{
            let b = sys::libvlc_media_player_can_pause(self.ptr);
            if b == 0 { false }else{ true }
        }
    }

    /// Check if the current program is scrambled.
    pub fn program_scrambled(&self) -> bool {
        unsafe{
            let b = sys::libvlc_media_player_program_scrambled(self.ptr);
            if b == 0 { false }else{ true }
        }
    }

    /// Display the next frame (if supported)
    pub fn next_frame(&self) {
        unsafe{ sys::libvlc_media_player_next_frame(self.ptr); }
    }

    /// Navigate through DVD Menu.
    pub fn navigate(&self, navigate: u32) {
        unsafe{ sys::libvlc_media_player_navigate(self.ptr, navigate); }
    }

    /// Set if, and how, the video title will be shown when media is played.
    pub fn set_video_title_display(&self, position: Position, timeout: u32) {
        unsafe{ sys::libvlc_media_player_set_video_title_display(self.ptr, position, timeout); }
    }

    /// Returns raw pointer
    pub fn raw(&self) -> *mut sys::libvlc_media_player_t {
        self.ptr
    }
}

impl Drop for MediaPlayer {
    fn drop(&mut self) {
        unsafe{ sys::libvlc_media_player_release(self.ptr) };
    }
}

// For audio_set_callbacks
struct AudioCallbacksData {
    play: Box<dyn Fn(*const c_void, u32, i64) + Send + 'static>,
    pause: Option<Box<dyn Fn(i64) + Send + 'static>>,
    resume: Option<Box<dyn Fn(i64) + Send + 'static>>,
    flush: Option<Box<dyn Fn(i64) + Send + 'static>>,
    drain: Option<Box<dyn Fn() + Send + 'static>>,
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TrackDescription {
    pub id: i32,
    pub name: Option<String>,
}

