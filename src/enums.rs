// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use vlc_sys as sys;

macro_rules! define_enum {
    ($enum_name:ident, $original_type:ident; $($value:ident = $c_value:ident,)*) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        #[repr(C)]
        pub enum $enum_name {
            $(
                $value = sys::$c_value as isize,
            )*
        }

        impl From<sys::$original_type> for $enum_name {
            fn from(a: sys::$original_type) -> Self {
                match a {
                    $(
                        sys::$c_value => Self::$value,
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}

define_enum!(
    LogLevel, libvlc_log_level;
    Debug = libvlc_log_level_LIBVLC_DEBUG,
    Dotice = libvlc_log_level_LIBVLC_NOTICE,
    Warning = libvlc_log_level_LIBVLC_WARNING,
    Error = libvlc_log_level_LIBVLC_ERROR,
);

define_enum!(
    Meta, libvlc_meta_t;
    Title = libvlc_meta_t_libvlc_meta_Title,
    Artist = libvlc_meta_t_libvlc_meta_Artist,
    Genre = libvlc_meta_t_libvlc_meta_Genre,
    Copyright = libvlc_meta_t_libvlc_meta_Copyright,
    Album = libvlc_meta_t_libvlc_meta_Album,
    TrackNumber = libvlc_meta_t_libvlc_meta_TrackNumber,
    Description = libvlc_meta_t_libvlc_meta_Description,
    Rating = libvlc_meta_t_libvlc_meta_Rating,
    Date = libvlc_meta_t_libvlc_meta_Date,
    Setting = libvlc_meta_t_libvlc_meta_Setting,
    URL = libvlc_meta_t_libvlc_meta_URL,
    Language = libvlc_meta_t_libvlc_meta_Language,
    NowPlaying = libvlc_meta_t_libvlc_meta_NowPlaying,
    Publisher = libvlc_meta_t_libvlc_meta_Publisher,
    EncodedBy = libvlc_meta_t_libvlc_meta_EncodedBy,
    ArtworkURL = libvlc_meta_t_libvlc_meta_ArtworkURL,
    TrackID = libvlc_meta_t_libvlc_meta_TrackID,
    TrackTotal = libvlc_meta_t_libvlc_meta_TrackTotal,
    Director = libvlc_meta_t_libvlc_meta_Director,
    Season = libvlc_meta_t_libvlc_meta_Season,
    Episode = libvlc_meta_t_libvlc_meta_Episode,
    ShowName = libvlc_meta_t_libvlc_meta_ShowName,
    Actors = libvlc_meta_t_libvlc_meta_Actors,
);

define_enum!(
    State, libvlc_state_t;
    NothingSpecial = libvlc_state_t_libvlc_NothingSpecial,
    Opening = libvlc_state_t_libvlc_Opening,
    Buffering = libvlc_state_t_libvlc_Buffering,
    Playing = libvlc_state_t_libvlc_Playing,
    Paused = libvlc_state_t_libvlc_Paused,
    Stopped = libvlc_state_t_libvlc_Stopped,
    Ended = libvlc_state_t_libvlc_Ended,
    Error = libvlc_state_t_libvlc_Error,
);

define_enum!(
    TrackType, libvlc_track_type_t;
    Unknown = libvlc_track_type_t_libvlc_track_unknown,
    Audio = libvlc_track_type_t_libvlc_track_audio,
    Video = libvlc_track_type_t_libvlc_track_video,
    Text = libvlc_track_type_t_libvlc_track_text,
);

define_enum!(
    Position, libvlc_position_t;
    Disable = libvlc_position_t_libvlc_position_disable,
    Center = libvlc_position_t_libvlc_position_center,
    Left = libvlc_position_t_libvlc_position_left,
    Right = libvlc_position_t_libvlc_position_right,
    Top = libvlc_position_t_libvlc_position_top,
    TopLeft = libvlc_position_t_libvlc_position_top_left,
    TopRight = libvlc_position_t_libvlc_position_top_right,
    Bottom = libvlc_position_t_libvlc_position_bottom,
    BottomLeft = libvlc_position_t_libvlc_position_bottom_left,
    BottomRight = libvlc_position_t_libvlc_position_bottom_right,
);

define_enum!(
    VideoAdjustOption, libvlc_video_adjust_option_t;
    Enable = libvlc_video_adjust_option_t_libvlc_adjust_Enable,
    Contrast = libvlc_video_adjust_option_t_libvlc_adjust_Contrast,
    Brightness = libvlc_video_adjust_option_t_libvlc_adjust_Brightness,
    Hue = libvlc_video_adjust_option_t_libvlc_adjust_Hue,
    Saturation = libvlc_video_adjust_option_t_libvlc_adjust_Saturation,
    Gamma = libvlc_video_adjust_option_t_libvlc_adjust_Gamma,
);

// libvlc 3.0
// define_enum!(
//     ParseFlag, libvlc_media_parse_flag_t;
//     DoInteract = libvlc_media_parse_flag_t_libvlc_media_do_interact,
//     FetchLocal = libvlc_media_parse_flag_t_libvlc_media_fetch_local,
//     FetchNetwork = libvlc_media_parse_flag_t_libvlc_media_fetch_network,
//     ParseLocal = libvlc_media_parse_flag_t_libvlc_media_parse_local,
//     ParseNetwork = libvlc_media_parse_flag_t_libvlc_media_parse_network,
// );

define_enum!(
    EventType, libvlc_event_e;
    MediaMetaChanged = libvlc_event_e_libvlc_MediaMetaChanged,
    MediaSubItemAdded = libvlc_event_e_libvlc_MediaSubItemAdded,
    MediaDurationChanged = libvlc_event_e_libvlc_MediaDurationChanged,
    MediaParsedChanged = libvlc_event_e_libvlc_MediaParsedChanged,
    MediaFreed = libvlc_event_e_libvlc_MediaFreed,
    MediaStateChanged = libvlc_event_e_libvlc_MediaStateChanged,
    MediaSubItemTreeAdded = libvlc_event_e_libvlc_MediaSubItemTreeAdded,
    MediaPlayerMediaChanged = libvlc_event_e_libvlc_MediaPlayerMediaChanged,
    MediaPlayerNothingSpecial = libvlc_event_e_libvlc_MediaPlayerNothingSpecial,
    MediaPlayerOpening = libvlc_event_e_libvlc_MediaPlayerOpening,
    MediaPlayerBuffering = libvlc_event_e_libvlc_MediaPlayerBuffering,
    MediaPlayerPlaying = libvlc_event_e_libvlc_MediaPlayerPlaying,
    MediaPlayerPaused = libvlc_event_e_libvlc_MediaPlayerPaused,
    MediaPlayerStopped = libvlc_event_e_libvlc_MediaPlayerStopped,
    MediaPlayerForward = libvlc_event_e_libvlc_MediaPlayerForward,
    MediaPlayerBackward = libvlc_event_e_libvlc_MediaPlayerBackward,
    MediaPlayerEndReached = libvlc_event_e_libvlc_MediaPlayerEndReached,
    MediaPlayerEncounteredError = libvlc_event_e_libvlc_MediaPlayerEncounteredError,
    MediaPlayerTimeChanged = libvlc_event_e_libvlc_MediaPlayerTimeChanged,
    MediaPlayerPositionChanged = libvlc_event_e_libvlc_MediaPlayerPositionChanged,
    MediaPlayerSeekableChanged = libvlc_event_e_libvlc_MediaPlayerSeekableChanged,
    MediaPlayerPausableChanged = libvlc_event_e_libvlc_MediaPlayerPausableChanged,
    MediaPlayerTitleChanged = libvlc_event_e_libvlc_MediaPlayerTitleChanged,
    MediaPlayerSnapshotTaken = libvlc_event_e_libvlc_MediaPlayerSnapshotTaken,
    MediaPlayerLengthChanged = libvlc_event_e_libvlc_MediaPlayerLengthChanged,
    MediaPlayerVout = libvlc_event_e_libvlc_MediaPlayerVout,
    MediaPlayerScrambledChanged = libvlc_event_e_libvlc_MediaPlayerScrambledChanged,
    MediaListItemAdded = libvlc_event_e_libvlc_MediaListItemAdded,
    MediaListWillAddItem = libvlc_event_e_libvlc_MediaListWillAddItem,
    MediaListItemDeleted = libvlc_event_e_libvlc_MediaListItemDeleted,
    MediaListWillDeleteItem = libvlc_event_e_libvlc_MediaListWillDeleteItem,
    MediaListViewItemAdded = libvlc_event_e_libvlc_MediaListViewItemAdded,
    MediaListViewWillAddItem = libvlc_event_e_libvlc_MediaListViewWillAddItem,
    MediaListViewItemDeleted = libvlc_event_e_libvlc_MediaListViewItemDeleted,
    MediaListViewWillDeleteItem = libvlc_event_e_libvlc_MediaListViewWillDeleteItem,
    MediaListPlayerPlayed = libvlc_event_e_libvlc_MediaListPlayerPlayed,
    MediaListPlayerNextItemSet = libvlc_event_e_libvlc_MediaListPlayerNextItemSet,
    MediaListPlayerStopped = libvlc_event_e_libvlc_MediaListPlayerStopped,
    MediaDiscovererStarted = libvlc_event_e_libvlc_MediaDiscovererStarted,
    MediaDiscovererEnded = libvlc_event_e_libvlc_MediaDiscovererEnded,
    VlmMediaAdded = libvlc_event_e_libvlc_VlmMediaAdded,
    VlmMediaRemoved = libvlc_event_e_libvlc_VlmMediaRemoved,
    VlmMediaChanged = libvlc_event_e_libvlc_VlmMediaChanged,
    VlmMediaInstanceStarted = libvlc_event_e_libvlc_VlmMediaInstanceStarted,
    VlmMediaInstanceStopped = libvlc_event_e_libvlc_VlmMediaInstanceStopped,
    VlmMediaInstanceStatusInit = libvlc_event_e_libvlc_VlmMediaInstanceStatusInit,
    VlmMediaInstanceStatusOpening = libvlc_event_e_libvlc_VlmMediaInstanceStatusOpening,
    VlmMediaInstanceStatusPlaying = libvlc_event_e_libvlc_VlmMediaInstanceStatusPlaying,
    VlmMediaInstanceStatusPause = libvlc_event_e_libvlc_VlmMediaInstanceStatusPause,
    VlmMediaInstanceStatusEnd = libvlc_event_e_libvlc_VlmMediaInstanceStatusEnd,
    VlmMediaInstanceStatusError = libvlc_event_e_libvlc_VlmMediaInstanceStatusError,
);
