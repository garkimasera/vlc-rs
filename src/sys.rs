// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

#![allow(non_camel_case_types, non_upper_case_globals)]

#[link(name = "vlc")]
extern "C" {}

use libc::{c_void, c_int, c_uint, c_char, c_float, uintptr_t, FILE};

pub type c_bool = u8;

pub type libvlc_event_type_t = c_int;

// From libvlc_structures.h
pub enum libvlc_instance_t {}
pub enum libvlc_log_iterator_t {}

pub type libvlc_time_t = i64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_log_message_t {
    pub i_severity: c_int,
    pub psz_type: *const c_char,
    pub psz_name: *const c_char,
    pub psz_header: *const c_char,
    pub psz_message: *const c_char,
}

// From libvlc.h
pub enum libvlc_event_manager_t {}
pub enum libvlc_log_t {}
pub enum vlc_log_t {}

pub type libvlc_callback_t = unsafe extern "C" fn(*const libvlc_event_t, *mut c_void);
pub type va_list = *mut c_void;
pub type libvlc_log_cb = unsafe extern "C" fn(*mut c_void, c_int, *const libvlc_log_t, *const c_char, va_list);

pub use crate::enums::LogLevel as libvlc_log_level;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_module_description_t
{
    pub psz_name: *const c_char,
    pub psz_shortname: *const c_char,
    pub psz_longname: *const c_char,
    pub psz_help: *const c_char,
    pub p_next: *mut libvlc_module_description_t,
}

extern "C" {
    pub fn libvlc_errmsg() -> *const c_char;
    pub fn libvlc_clearerr();
    pub fn libvlc_new(argc: c_int, argv: *const *const c_char) -> *mut libvlc_instance_t;
    pub fn libvlc_release(p_instance: *mut libvlc_instance_t);
    pub fn libvlc_retain(p_instance: *mut libvlc_instance_t);
    pub fn libvlc_add_intf(p_instance: *mut libvlc_instance_t, name: *const c_char) -> c_int;
    pub fn libvlc_set_exit_handler(
        p_instance: *mut libvlc_instance_t,
        cb: extern "C" fn(*mut c_void), opaque: *mut c_void);
    pub fn libvlc_wait(p_instance: *mut libvlc_instance_t);
    pub fn libvlc_set_user_agent(
        p_instance: *mut libvlc_instance_t, name: *const c_char, http: *const c_char);
    pub fn libvlc_set_app_id(
        p_instance: *mut libvlc_instance_t, id: *const c_char, version: *const c_char,
        icon: *const c_char);
    pub fn libvlc_get_version() -> *const c_char;
    pub fn libvlc_get_compiler() -> *const c_char;
    pub fn libvlc_get_changeset() -> *const c_char;
    pub fn libvlc_free(ptr: *mut c_void);
    pub fn libvlc_event_attach(
        p_event_manager: *mut libvlc_event_manager_t, i_event_type: libvlc_event_type_t,
        f_callback: libvlc_callback_t, user_data: *mut c_void) -> c_int;
    pub fn libvlc_event_type_name(event_type: libvlc_event_type_t) -> *const c_char;
    pub fn libvlc_log_get_context(
        ctx: *const libvlc_log_t, module: *const *const c_char, file: *const *const c_char,
        line: *mut c_uint);
    pub fn libvlc_log_get_object(
        ctx: *const libvlc_log_t, name: *const *const c_char,
        header: *const *const c_char, id: *mut uintptr_t);
    pub fn libvlc_log_unset(_: *mut libvlc_instance_t);
    pub fn libvlc_log_set(instance: *mut libvlc_instance_t, cb: libvlc_log_cb, data: *mut c_void);
    pub fn libvlc_log_set_file(_: *mut libvlc_instance_t, stream: *mut FILE);
    pub fn libvlc_module_description_list_release(p_list: *mut libvlc_module_description_t);
    pub fn libvlc_audio_filter_list_get(
        p_instance: *mut libvlc_instance_t) -> *mut libvlc_module_description_t;
    pub fn libvlc_video_filter_list_get(
        p_instance: *mut libvlc_instance_t) -> *mut libvlc_module_description_t;
    pub fn libvlc_clock() -> i64;
}

pub unsafe fn libvlc_delay(pts: i64) -> i64 {
    pts - libvlc_clock()
}

// From libvlc_media.h
pub enum libvlc_media_t {}

pub use crate::enums::Meta as libvlc_meta_t;
pub use crate::enums::State as libvlc_state_t;

pub const libvlc_media_option_trusted: u32 = 0x2;
pub const libvlc_media_option_unique: u32 = 0x100;

pub use crate::enums::TrackType as libvlc_track_type_t;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_media_stats_t {
    /* Input */
    pub i_read_bytes: c_int,
    pub f_input_bitrate: c_float,
    /* Demux */
    pub i_demux_read_bytes: c_int,
    pub f_demux_bitrate: c_float,
    pub i_demux_corrupted: c_int,
    pub i_demux_discontinuity: c_int,
    /* Decoders */
    pub i_decoded_video: c_int,
    pub i_decoded_audio: c_int,
    /* Video Output */
    pub i_displayed_pictures: c_int,
    pub i_lost_pictures: c_int,
    /* Audio output */
    pub i_played_abuffers: c_int,
    pub i_lost_abuffers: c_int,
    /* Stream output */
    pub i_sent_packets: c_int,
    pub i_sent_bytes: c_int,
    pub f_send_bitrate: c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_media_track_info_t {
    /* Codec fourcc */
    pub i_codec: u32,
    pub i_id: c_int,
    pub i_type: libvlc_track_type_t,
    /* Codec specific */
    pub i_profile: c_int,
    pub i_level: c_int,
    
    pub u: libvlc_media_track_info_t_types::u,
}

pub mod libvlc_media_track_info_t_types {
    use libc::c_uint;
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union u {
        pub audio: audio,
        pub video: video,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct audio {
        pub i_channels: c_uint,
        pub i_rate: c_uint,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct video {
        pub i_height: c_uint,
        pub i_width: c_uint,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_audio_track_t
{
    pub i_channels: c_uint,
    pub i_rate: c_uint,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct libvlc_video_track_t
{
    pub i_height: c_uint,
    pub i_width: c_uint,
    pub i_sar_num: c_uint,
    pub i_sar_den: c_uint,
    pub i_frame_rate_num: c_uint,
    pub i_frame_rate_den: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_subtitle_track_t {
    pub psz_encoding: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_media_track_t {
    pub i_codec: u32,
    pub i_original_fourcc: u32,
    pub i_id: c_int,
    pub i_type: libvlc_track_type_t,
    pub i_profile: c_int,
    pub i_level: c_int,
    pub u: libvlc_media_track_t_types::u,
    pub i_bitrate: c_uint,
    pub psz_language: *mut c_char,
    pub psz_description: *mut c_char,
}

pub mod libvlc_media_track_t_types {
    use super::*;
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union u {
        pub audio: *mut libvlc_audio_track_t,
        pub video: *mut libvlc_video_track_t,
        pub subtitle: *mut libvlc_subtitle_track_t,
    }
}

impl libvlc_media_track_t {
    pub unsafe fn audio(&self) -> *mut libvlc_audio_track_t {
        self.u.audio
    }
    pub unsafe fn video(&self) -> *mut libvlc_video_track_t {
        self.u.video
    }
    pub unsafe fn subtitle(&self) -> *mut libvlc_subtitle_track_t {
        self.u.subtitle
    }
}

extern "C" {
    pub fn libvlc_media_new_location(p_instance: *mut libvlc_instance_t, psz_mrl: *const c_char)
                                     -> *mut libvlc_media_t;
    pub fn libvlc_media_new_path(p_instance: *mut libvlc_instance_t, path: *const c_char)
                                 -> *mut libvlc_media_t;
    pub fn libvlc_media_new_fd(p_instance: *mut libvlc_instance_t, fd: c_int)
                               -> *mut libvlc_media_t;
    pub fn libvlc_media_as_node(p_instance: *mut libvlc_instance_t, psz_name: *const c_char)
                                -> *mut libvlc_media_t;
    pub fn libvlc_media_add_option(p_md: *mut libvlc_media_t, psz_options: *const c_char);
    pub fn libvlc_media_add_option_flag(
        p_md: *mut libvlc_media_t, psz_options: *const c_char, i_flags: c_uint);
    pub fn libvlc_media_retain(p_md: *mut libvlc_media_t);
    pub fn libvlc_media_release(p_md: *mut libvlc_media_t);
    pub fn libvlc_media_get_mrl(p_md: *mut libvlc_media_t) -> *mut c_char;
    pub fn libvlc_media_duplicate(p_md: *mut libvlc_media_t) -> *mut libvlc_media_t;
    pub fn libvlc_media_get_meta(p_md: *mut libvlc_media_t, e_meta: libvlc_meta_t) -> *mut c_char;
    pub fn libvlc_media_set_meta(p_md: *mut libvlc_media_t, e_meta: libvlc_meta_t,
                                 psz_value: *const c_char);
    pub fn libvlc_media_save_meta(p_md: *mut libvlc_media_t) -> c_int;
    pub fn libvlc_media_get_state(p_md: *mut libvlc_media_t) -> libvlc_state_t;
    pub fn libvlc_media_get_stats(p_md: *mut libvlc_media_t, p_stats: *mut libvlc_media_stats_t)
                                  -> c_int;
    pub fn libvlc_media_subitems(p_md: *mut libvlc_media_t)
                                 -> *mut libvlc_media_list_t;
    pub fn libvlc_media_event_manager(p_md: *mut libvlc_media_t)
                                      -> *mut libvlc_event_manager_t;
    pub fn libvlc_media_get_duration(p_md: *mut libvlc_media_t)
                                     -> libvlc_time_t;
    pub fn libvlc_media_parse(p_md: *mut libvlc_media_t);
    pub fn libvlc_media_parse_async(p_md: *mut libvlc_media_t);
    pub fn libvlc_media_is_parsed(p_md: *mut libvlc_media_t) -> c_int;
    pub fn libvlc_media_set_user_data(p_md: *mut libvlc_media_t,
                                      p_new_user_data: *mut c_void);
    pub fn libvlc_media_get_user_data(p_md: *mut libvlc_media_t) -> *mut c_void;
    pub fn libvlc_media_tracks_get(p_md: *mut libvlc_media_t,
                                   tracks: *mut *mut *mut libvlc_media_track_t) -> c_uint;
    pub fn libvlc_media_tracks_release(p_tracks: *mut *mut libvlc_media_track_t, i_count: c_uint);
}

// From libvlc_media_player.h

pub enum libvlc_media_player_t {}
pub enum libvlc_equalizer_t {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct libvlc_track_description_t {
    pub i_id: c_int,
    pub psz_name: *mut c_char,
    pub p_next: *mut libvlc_track_description_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_audio_output_t {
    pub psz_name: *mut c_char,
    pub psz_description: *mut c_char,
    pub p_next: *mut libvlc_audio_output_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_audio_output_device_t {
    pub p_next: *mut libvlc_audio_output_device_t,
    pub psz_device: *mut c_char,
    pub psz_description: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct libvlc_rectangle_t {
    pub top: c_int, pub left: c_int, pub bottom: c_int, pub right: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_video_marquee_option_t {
    libvlc_marquee_Enable = 0,
    libvlc_marquee_Text,
    libvlc_marquee_Color,
    libvlc_marquee_Opacity,
    libvlc_marquee_Position,
    libvlc_marquee_Refresh,
    libvlc_marquee_Size,
    libvlc_marquee_Timeout,
    libvlc_marquee_X,
    libvlc_marquee_Y,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_navigate_mode_t {
    libvlc_navigate_activate = 0,
    libvlc_navigate_up,
    libvlc_navigate_down,
    libvlc_navigate_left,
    libvlc_navigate_right,
}

pub use crate::enums::Position as libvlc_position_t;
pub use crate::enums::VideoAdjustOption as libvlc_video_adjust_option;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_video_logo_option_t {
    libvlc_logo_enable,
    libvlc_logo_file,
    libvlc_logo_x,
    libvlc_logo_y,
    libvlc_logo_delay,
    libvlc_logo_repeat,
    libvlc_logo_opacity,
    libvlc_logo_position
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_audio_output_device_types_t {
    libvlc_AudioOutputDevice_Error  = -1,
    libvlc_AudioOutputDevice_Mono   =  1,
    libvlc_AudioOutputDevice_Stereo =  2,
    libvlc_AudioOutputDevice_2F2R   =  4,
    libvlc_AudioOutputDevice_3F2R   =  5,
    libvlc_AudioOutputDevice_5_1    =  6,
    libvlc_AudioOutputDevice_6_1    =  7,
    libvlc_AudioOutputDevice_7_1    =  8,
    libvlc_AudioOutputDevice_SPDIF  = 10
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum libvlc_audio_output_channel_t {
    libvlc_AudioChannel_Error   = -1,
    libvlc_AudioChannel_Stereo  =  1,
    libvlc_AudioChannel_RStereo =  2,
    libvlc_AudioChannel_Left    =  3,
    libvlc_AudioChannel_Right   =  4,
    libvlc_AudioChannel_Dolbys  =  5
}

pub type libvlc_video_lock_cb = Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void>;
pub type libvlc_video_unlock_cb = Option<unsafe extern "C" fn(
    *mut c_void, *mut c_void, *const *mut c_void)>;
pub type libvlc_video_display_cb = Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>;
pub type libvlc_video_format_cb = Option<unsafe extern "C" fn(
    *mut *mut c_void, *mut c_char, *mut c_uint, *mut c_uint, *mut c_uint, *mut c_uint)>;
pub type libvlc_video_cleanup_cb = Option<unsafe extern "C" fn(*mut c_void)>;
pub type libvlc_audio_play_cb = Option<unsafe extern "C" fn(*mut c_void, *const c_void, c_uint, i64)>;
pub type libvlc_audio_pause_cb = Option<unsafe extern "C" fn(*mut c_void, i64)>;
pub type libvlc_audio_resume_cb = Option<unsafe extern "C" fn(*mut c_void, i64)>;
pub type libvlc_audio_flush_cb = Option<unsafe extern "C" fn(*mut c_void, i64)>;
pub type libvlc_audio_drain_cb = Option<unsafe extern "C" fn(*mut c_void)>;
pub type libvlc_audio_set_volume_cb = Option<unsafe extern "C" fn(*mut c_void, c_float, c_bool)>;
pub type libvlc_audio_setup_cb = Option<unsafe extern "C" fn(
    *mut *mut c_void, *mut c_char, *mut c_uint, *mut c_uint)>;
pub type libvlc_audio_cleanup_cb = Option<unsafe extern "C" fn(*mut c_void)>;

extern "C" {
    pub fn libvlc_media_player_new(p_libvlc_instance: *mut libvlc_instance_t)
                                   -> *mut libvlc_media_player_t;
    pub fn libvlc_media_player_new_from_media(p_md: *mut libvlc_media_t)
                                              -> *mut libvlc_media_player_t;
    pub fn libvlc_media_player_release(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_media_player_retain(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_media_player_set_media(p_mi: *mut libvlc_media_player_t,
                                         p_md: *mut libvlc_media_t);
    pub fn libvlc_media_player_get_media(p_mi: *mut libvlc_media_player_t) -> *mut libvlc_media_t;
    pub fn libvlc_media_player_event_manager(p_mi: *mut libvlc_media_player_t)
                                             -> *mut libvlc_event_manager_t;
    pub fn libvlc_media_player_is_playing(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_play(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_set_pause(mp: *mut libvlc_media_player_t, do_pause: c_int);
    pub fn libvlc_media_player_pause(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_media_player_stop(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_video_set_callbacks(
        mp: *mut libvlc_media_player_t, lock: libvlc_video_lock_cb,
        unlock: libvlc_video_unlock_cb, display: libvlc_video_display_cb,
        opaque: *mut c_void);
    pub fn libvlc_video_set_format(
        mp: *mut libvlc_media_player_t, chroma: *const c_char, width: c_uint, height: c_uint,
        pitch: c_uint);
    pub fn libvlc_video_set_format_callbacks(
        mp: *mut libvlc_media_player_t, setup: libvlc_video_format_cb,
        cleanup: libvlc_video_cleanup_cb);
    pub fn libvlc_media_player_set_nsobject(
        p_mi: *mut libvlc_media_player_t, drawable: *mut c_void);
    pub fn libvlc_media_player_get_nsobject(p_mi: *mut libvlc_media_player_t) -> *mut c_void;
    pub fn libvlc_media_player_set_xwindow(
        p_mi: *mut libvlc_media_player_t, drawable: u32);
    pub fn libvlc_media_player_get_xwindow(p_mi: *mut libvlc_media_player_t) -> u32;
    pub fn libvlc_media_player_set_hwnd(
        p_mi: *mut libvlc_media_player_t, drawable: *mut c_void);
    pub fn libvlc_media_player_get_hwnd(p_mi: *mut libvlc_media_player_t) -> *mut c_void;
    pub fn libvlc_audio_set_callbacks(
        mp: *mut libvlc_media_player_t, play: libvlc_audio_play_cb, pause: libvlc_audio_pause_cb,
        resume: libvlc_audio_resume_cb, flush: libvlc_audio_flush_cb,
        drain: libvlc_audio_drain_cb, opaque: *mut c_void);
    pub fn libvlc_audio_set_volume_callback(
        mp: *mut libvlc_media_player_t, set_volume: libvlc_audio_set_volume_cb);
    pub fn libvlc_audio_set_format_callbacks(
        mp: *mut libvlc_media_player_t, setup: libvlc_audio_setup_cb,
        cleanup: libvlc_audio_cleanup_cb);
    pub fn libvlc_audio_set_format(
        mp: *mut libvlc_media_player_t, format: *const c_char, rate: c_uint, channels: c_uint);
    pub fn libvlc_media_player_get_length(p_mi: *mut libvlc_media_player_t) -> libvlc_time_t;
    pub fn libvlc_media_player_get_time(p_mi: *mut libvlc_media_player_t) -> libvlc_time_t;
    pub fn libvlc_media_player_set_time(
        p_mi: *mut libvlc_media_player_t, i_time: libvlc_time_t);
    pub fn libvlc_media_player_get_position(p_mi: *mut libvlc_media_player_t) -> c_float;
    pub fn libvlc_media_player_set_position(p_mi: *mut libvlc_media_player_t, f_pos: c_float);
    pub fn libvlc_media_player_set_chapter(p_mi: *mut libvlc_media_player_t, i_chapter: c_int);
    pub fn libvlc_media_player_get_chapter(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_get_chapter_count(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_will_play(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_set_title(p_mi: *mut libvlc_media_player_t, i_title: c_int);
    pub fn libvlc_media_player_get_chapter_count_for_title(
        p_mi: *mut libvlc_media_player_t, i_title: c_int) -> c_int;
    pub fn libvlc_media_player_get_title(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_get_title_count(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_previous_chapter(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_media_player_next_chapter(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_media_player_get_rate(p_mi: *mut libvlc_media_player_t) -> c_float;
    pub fn libvlc_media_player_set_rate(p_mi: *mut libvlc_media_player_t, rate: c_float) -> c_int;
    pub fn libvlc_media_player_get_state(p_mi: *mut libvlc_media_player_t) -> libvlc_state_t;
    pub fn libvlc_media_player_get_fps(p_mi: *mut libvlc_media_player_t) -> c_float;
    pub fn libvlc_media_player_has_vout(p_mi: *mut libvlc_media_player_t) -> c_uint;
    pub fn libvlc_media_player_is_seekable(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_can_pause(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_program_scrambled(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_media_player_next_frame(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_media_player_navigate(p_mi: *mut libvlc_media_player_t, navigate: c_uint);
    pub fn libvlc_media_player_set_video_title_display(
        p_mi: *mut libvlc_media_player_t, position: libvlc_position_t, timeout: c_uint);
    pub fn libvlc_track_description_list_release(p_track_description: *mut libvlc_track_description_t);
    pub fn libvlc_toggle_fullscreen(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_set_fullscreen(p_mi: *mut libvlc_media_player_t, b_fullscreen: c_int);
    pub fn libvlc_get_fullscreen(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_video_set_key_input(p_mi: *mut libvlc_media_player_t, on: c_uint);
    pub fn libvlc_video_set_mouse_input(p_mi: *mut libvlc_media_player_t, on: c_uint);
    pub fn libvlc_video_get_size(
        p_mi: *mut libvlc_media_player_t, num: c_uint, px: *mut c_uint, py: *mut c_uint) -> c_int;
    pub fn libvlc_video_get_cursor(
        p_mi: *mut libvlc_media_player_t, num: c_uint, px: *mut c_int, py: *mut c_int) -> c_int;
    pub fn libvlc_video_get_scale(p_mi: *mut libvlc_media_player_t) -> c_float;
    pub fn libvlc_video_set_scale(p_mi: *mut libvlc_media_player_t, f_factor: c_float);
    pub fn libvlc_video_get_aspect_ratio(p_mi: *mut libvlc_media_player_t) -> *mut c_char;
    pub fn libvlc_video_set_aspect_ratio(p_mi: *mut libvlc_media_player_t, psz_aspect: *const c_char);
    pub fn libvlc_video_get_spu(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_video_get_spu_count(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_video_get_spu_description(p_mi: *mut libvlc_media_player_t)
     -> *mut libvlc_track_description_t;
    pub fn libvlc_video_set_spu(p_mi: *mut libvlc_media_player_t, i_spu: c_int) -> c_int;
    pub fn libvlc_video_set_subtitle_file(
        p_mi: *mut libvlc_media_player_t, psz_subtitle: *const c_char) -> c_int;
    pub fn libvlc_video_get_spu_delay(p_mi: *mut libvlc_media_player_t) -> i64;
    pub fn libvlc_video_set_spu_delay(
        p_mi: *mut libvlc_media_player_t, i_delay: i64) -> c_int;
    pub fn libvlc_video_get_title_description(
        p_mi: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t;
    pub fn libvlc_video_get_chapter_description(
        p_mi: *mut libvlc_media_player_t, i_title: c_int) -> *mut libvlc_track_description_t;
    pub fn libvlc_video_get_crop_geometry(p_mi: *mut libvlc_media_player_t) -> *mut c_char;
    pub fn libvlc_video_set_crop_geometry(
        p_mi: *mut libvlc_media_player_t, psz_geometry: *const c_char);
    pub fn libvlc_video_get_teletext(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_video_set_teletext(p_mi: *mut libvlc_media_player_t, i_page: c_int);
    pub fn libvlc_toggle_teletext(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_video_get_track_count(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_video_get_track_description(
        p_mi: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t;
    pub fn libvlc_video_get_track(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_video_set_track(p_mi: *mut libvlc_media_player_t, i_track: c_int) -> c_int;
    pub fn libvlc_video_take_snapshot(
        p_mi: *mut libvlc_media_player_t, num: c_uint, psz_filepath: *const c_char,
        i_width: c_uint, i_height: c_uint) -> c_int;
    pub fn libvlc_video_set_deinterlace(p_mi: *mut libvlc_media_player_t, psz_mode: *const c_char);
    pub fn libvlc_video_get_marquee_int(p_mi: *mut libvlc_media_player_t, option: c_uint) -> c_int;
    pub fn libvlc_video_get_marquee_string(
        p_mi: *mut libvlc_media_player_t, option: c_uint) -> *mut c_char;
    pub fn libvlc_video_set_marquee_int(
        p_mi: *mut libvlc_media_player_t, option: c_uint, i_val: c_int);
    pub fn libvlc_video_set_marquee_string(
        p_mi: *mut libvlc_media_player_t, option: c_uint, psz_text: *const c_char);
    pub fn libvlc_video_get_logo_int(p_mi: *mut libvlc_media_player_t, option: c_uint) -> c_int;
    pub fn libvlc_video_set_logo_int(p_mi: *mut libvlc_media_player_t, option: c_uint, value: c_int);
    pub fn libvlc_video_set_logo_string(
        p_mi: *mut libvlc_media_player_t, option: c_uint, psz_value: *const c_char);
    pub fn libvlc_video_get_adjust_int(
        p_mi: *mut libvlc_media_player_t, option: c_uint) -> c_int;
    pub fn libvlc_video_set_adjust_int(
        p_mi: *mut libvlc_media_player_t, option: c_uint, value: c_int);
    pub fn libvlc_video_get_adjust_float(
        p_mi: *mut libvlc_media_player_t, option: c_uint) -> c_float;
    pub fn libvlc_video_set_adjust_float(
        p_mi: *mut libvlc_media_player_t, option: c_uint, value: c_float);
    pub fn libvlc_audio_output_list_get(p_instance: *mut libvlc_instance_t)
     -> *mut libvlc_audio_output_t;
    pub fn libvlc_audio_output_list_release(p_list: *mut libvlc_audio_output_t);
    pub fn libvlc_audio_output_set(p_mi: *mut libvlc_media_player_t, psz_name: *const c_char) -> c_int;
    pub fn libvlc_audio_output_device_enum(
        mp: *mut libvlc_media_player_t) -> *mut libvlc_audio_output_device_t;
    pub fn libvlc_audio_output_device_list_get(
        p_instance: *mut libvlc_instance_t, aout: *const c_char) -> *mut libvlc_audio_output_device_t;
    pub fn libvlc_audio_output_device_list_release(p_list: *mut libvlc_audio_output_device_t);
    pub fn libvlc_audio_output_device_set(
        mp: *mut libvlc_media_player_t, module: *const c_char, device_id: *const c_char);
    pub fn libvlc_audio_toggle_mute(p_mi: *mut libvlc_media_player_t);
    pub fn libvlc_audio_get_mute(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_audio_set_mute(p_mi: *mut libvlc_media_player_t, status: c_int);
    pub fn libvlc_audio_get_volume(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_audio_set_volume(p_mi: *mut libvlc_media_player_t, i_volume: c_int) -> c_int;
    pub fn libvlc_audio_get_track_count(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_audio_get_track_description(
        p_mi: *mut libvlc_media_player_t) -> *mut libvlc_track_description_t;
    pub fn libvlc_audio_get_track(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_audio_set_track(p_mi: *mut libvlc_media_player_t, i_track: c_int) -> c_int;
    pub fn libvlc_audio_get_channel(p_mi: *mut libvlc_media_player_t) -> c_int;
    pub fn libvlc_audio_set_channel(p_mi: *mut libvlc_media_player_t, channel: c_int) -> c_int;
    pub fn libvlc_audio_get_delay(p_mi: *mut libvlc_media_player_t) -> i64;
    pub fn libvlc_audio_set_delay(p_mi: *mut libvlc_media_player_t, i_delay: i64) -> c_int;
    pub fn libvlc_audio_equalizer_get_preset_count() -> c_uint;
    pub fn libvlc_audio_equalizer_get_preset_name(u_index: c_uint) -> *const c_char;
    pub fn libvlc_audio_equalizer_get_band_count() -> c_uint;
    pub fn libvlc_audio_equalizer_get_band_frequency(u_index: c_uint) -> c_float;
    pub fn libvlc_audio_equalizer_new() -> *mut libvlc_equalizer_t;
    pub fn libvlc_audio_equalizer_new_from_preset(u_index: c_uint) -> *mut libvlc_equalizer_t;
    pub fn libvlc_audio_equalizer_release(p_equalizer: *mut libvlc_equalizer_t);
    pub fn libvlc_audio_equalizer_set_preamp(
        p_equalizer: *mut libvlc_equalizer_t, f_preamp: c_float) -> c_int;
    pub fn libvlc_audio_equalizer_get_preamp(p_equalizer: *mut libvlc_equalizer_t) -> c_float;
    pub fn libvlc_audio_equalizer_set_amp_at_index(
        p_equalizer: *mut libvlc_equalizer_t, f_amp: c_float, u_band: c_uint) -> c_int;
    pub fn libvlc_audio_equalizer_get_amp_at_index(
        p_equalizer: *mut libvlc_equalizer_t, u_band: c_uint) -> c_float;
    pub fn libvlc_media_player_set_equalizer(
        p_mi: *mut libvlc_media_player_t, p_equalizer: *mut libvlc_equalizer_t) -> c_int;
}

// From libvlc_events.h
pub use crate::enums::EventType as libvlc_event_e;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct libvlc_event_t {
    pub _type: c_int,
    pub p_obj: *mut c_void,
    pub u: libvlc_event_t_types::u,
}

pub mod libvlc_event_t_types {
    use super::*;
    use libc::{c_int, c_char, c_float};
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub union u {
        pub media_meta_changed: media_meta_changed,
        pub media_subitem_added: media_subitem_added,
        pub media_duration_changed: media_duration_changed,
        pub media_parsed_changed: media_parsed_changed,
        pub media_freed: media_freed,
        pub media_state_changed: media_state_changed,
        pub media_subitemtree_added: media_subitemtree_added,
        pub media_player_buffering: media_player_buffering,
        pub media_player_position_changed: media_player_position_changed,
        pub media_player_time_changed: media_player_time_changed,
        pub media_player_title_changed: media_player_title_changed,
        pub media_player_seekable_changed: media_player_seekable_changed,
        pub media_player_pausable_changed: media_player_pausable_changed,
        pub media_player_scrambled_changed: media_player_scrambled_changed,
        pub media_player_vout: media_player_vout,
        pub media_list_item_added: media_list_item_added,
        pub media_list_will_add_item: media_list_will_add_item,
        pub media_list_item_deleted: media_list_item_deleted,
        pub media_list_will_delete_item: media_list_will_delete_item,
        pub media_list_player_next_item_set: media_list_player_next_item_set,
        pub media_player_snapshot_taken: media_player_snapshot_taken,
        pub media_player_length_changed: media_player_length_changed,
        pub vlm_media_event: vlm_media_event,
        pub media_player_media_changed: media_player_media_changed,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_meta_changed {
        pub meta_type: libvlc_meta_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_subitem_added {
        pub new_child: *mut libvlc_media_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_duration_changed {
        pub new_duration: i64,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_parsed_changed {
        pub new_status: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_freed {
        pub md: *mut libvlc_media_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_state_changed {
        pub new_state: libvlc_state_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_subitemtree_added {
        pub item: *mut libvlc_media_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_buffering {
        pub new_cache: c_float,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_position_changed {
        pub new_position: c_float,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_time_changed {
        pub new_time: libvlc_time_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_title_changed {
        pub new_titie: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_seekable_changed {
        pub new_seekable: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_pausable_changed {
        pub new_pausable: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_scrambled_changed {
        pub new_scrambled: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_vout {
        pub new_count: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_item_added {
        pub item: *mut libvlc_media_t,
        pub index: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_will_add_item {
        pub item: *mut libvlc_media_t,
        pub index: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_item_deleted {
        pub item: *mut libvlc_media_t,
        pub index: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_will_delete_item {
        pub item: *mut libvlc_media_t,
        pub index: c_int,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_list_player_next_item_set {
        pub item: *mut libvlc_media_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_snapshot_taken {
        pub psz_filename: *mut c_char,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_length_changed {
        pub new_length: libvlc_time_t,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct vlm_media_event {
        pub psz_media_name: *mut c_char,
        pub psz_instance_name: *mut c_char,
    }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct media_player_media_changed {
        pub new_media: *mut libvlc_media_t,
    }
}

// From libvlc_media_list.h

pub enum libvlc_media_list_t {}

extern "C" {
    pub fn libvlc_media_list_new(p_instance: *mut libvlc_instance_t) -> *mut libvlc_media_list_t;
    pub fn libvlc_media_list_release(p_ml: *mut libvlc_media_list_t);
    pub fn libvlc_media_list_retain(p_ml: *mut libvlc_media_list_t);
    pub fn libvlc_media_list_set_media(p_ml: *mut libvlc_media_list_t, p_md: *mut libvlc_media_t);
    pub fn libvlc_media_list_media(p_ml: *mut libvlc_media_list_t) -> *mut libvlc_media_t;
    pub fn libvlc_media_list_add_media(
        p_ml: *mut libvlc_media_list_t, p_md: *mut libvlc_media_t) -> c_int;
    pub fn libvlc_media_list_insert_media(
        p_ml: *mut libvlc_media_list_t, p_md: *mut libvlc_media_t, i_pos: c_int) -> c_int;
    pub fn libvlc_media_list_remove_index(p_ml: *mut libvlc_media_list_t, i_pos: c_int) -> c_int;
    pub fn libvlc_media_list_count(p_ml: *mut libvlc_media_list_t) -> c_int;
    pub fn libvlc_media_list_item_at_index(
        p_ml: *mut libvlc_media_list_t, i_pos: c_int) -> *mut libvlc_media_t;
    pub fn libvlc_media_list_index_of_item(
        p_ml: *mut libvlc_media_list_t, p_md: *mut libvlc_media_t) -> c_int;
    pub fn libvlc_media_list_is_readonly(p_ml: *mut libvlc_media_list_t) -> c_int;
    pub fn libvlc_media_list_lock(p_ml: *mut libvlc_media_list_t);
    pub fn libvlc_media_list_unlock(p_ml: *mut libvlc_media_list_t);
    pub fn libvlc_media_list_event_manager(
        p_ml: *mut libvlc_media_list_t) -> *mut libvlc_event_manager_t;
}

// From libvlc_media_library.h

pub enum libvlc_media_library_t {}

extern "C" {
    pub fn libvlc_media_library_new(p_instance: *mut libvlc_instance_t) -> *mut libvlc_media_library_t;
    pub fn libvlc_media_library_release(p_mlib: *mut libvlc_media_library_t);
    pub fn libvlc_media_library_retain(p_mlib: *mut libvlc_media_library_t);
    pub fn libvlc_media_library_load(p_mlib: *mut libvlc_media_library_t) -> c_int;
    pub fn libvlc_media_library_media_list(
        p_mlib: *mut libvlc_media_library_t) -> *mut libvlc_media_list_t;
}

// From libvlc_media_discoverer.h

pub enum libvlc_media_discoverer_t {}

extern "C" {
    pub fn libvlc_media_discoverer_new_from_name(
        p_inst: *mut libvlc_instance_t, psz_name: *const c_char) -> *mut libvlc_media_discoverer_t;
    pub fn libvlc_media_discoverer_release(p_mdis: *mut libvlc_media_discoverer_t);
    pub fn libvlc_media_discoverer_localized_name(
        p_mdis: *mut libvlc_media_discoverer_t) -> *mut c_char;
    pub fn libvlc_media_discoverer_media_list(
        p_mdis: *mut libvlc_media_discoverer_t) -> *mut libvlc_media_list_t;
    pub fn libvlc_media_discoverer_event_manager(
        p_mdis: *mut libvlc_media_discoverer_t) -> *mut libvlc_event_manager_t;
    pub fn libvlc_media_discoverer_is_running(p_mdis: *mut libvlc_media_discoverer_t) -> c_int;
}

// From libvlc_vlm.h

extern "C" {
    pub fn libvlc_vlm_release(p_instance: *mut libvlc_instance_t);
    pub fn libvlc_vlm_add_broadcast(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, psz_input: *const c_char,
        psz_output: *const c_char, i_options: c_int, ppsz_options: *const *const c_char,
        b_enabled: c_int, b_loop: c_int) -> c_int;
    pub fn libvlc_vlm_add_vod(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, psz_input: *const c_char,
        i_options: c_int, ppsz_options: *const *const c_char, b_enabled: c_int,
        psz_mux: *const c_char) -> c_int;
    pub fn libvlc_vlm_del_media(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char) -> c_int;
    pub fn libvlc_vlm_set_enabled(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, b_enabled: c_int) -> c_int;
    pub fn libvlc_vlm_set_output(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, psz_output: *const c_char) -> c_int;
    pub fn libvlc_vlm_set_input(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, psz_input: *const c_char) -> c_int;
    pub fn libvlc_vlm_add_input(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, psz_input: *const c_char) -> c_int;
    pub fn libvlc_vlm_set_loop(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, b_loop: c_int) -> c_int;
    pub fn libvlc_vlm_set_mux(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, psz_mux: *const c_char) -> c_int;
    pub fn libvlc_vlm_change_media(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, psz_input: *const c_char,
        psz_output: *const c_char, i_options: c_int, ppsz_options: *const *const c_char,
        b_enabled: c_int, b_loop: c_int) -> c_int;
    pub fn libvlc_vlm_play_media(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char) -> c_int;
    pub fn libvlc_vlm_stop_media(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char) -> c_int;
    pub fn libvlc_vlm_pause_media(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char) -> c_int;
    pub fn libvlc_vlm_seek_media(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, f_percentage: c_float) -> c_int;
    pub fn libvlc_vlm_show_media(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char) -> *const c_char;
    pub fn libvlc_vlm_get_media_instance_position(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, i_instance: c_int) -> c_float;
    pub fn libvlc_vlm_get_media_instance_time(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, i_instance: c_int) -> c_int;
    pub fn libvlc_vlm_get_media_instance_length(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, i_instance: c_int) -> c_int;
    pub fn libvlc_vlm_get_media_instance_rate(
        p_instance: *mut libvlc_instance_t, psz_name: *const c_char, i_instance: c_int) -> c_int;
    pub fn libvlc_vlm_get_event_manager(
        p_instance: *mut libvlc_instance_t) -> *mut libvlc_event_manager_t;
}
