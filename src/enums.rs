// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LogLevel {
    Debug = 0,
    Notice = 2,
    Warning = 3,
    Error = 4,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Meta {
    Title,
    Artist,
    Genre,
    Copyright,
    Album,
    TrackNumber,
    Description,
    Rating,
    Date,
    Setting,
    URL,
    Language,
    NowPlaying,
    Publisher,
    EncodedBy,
    ArtworkURL,
    TrackID,
    TrackTotal,
    Director,
    Season,
    Episode,
    ShowName,
    Actors
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum State {
    NothingSpecial = 0,
    Opening,
    Buffering,
    Playing,
    Paused,
    Stopped,
    Ended,
    Error
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TrackType {
    Unknown = -1,
    Audio   = 0,
    Video   = 1,
    Text    = 2
}

// #[repr(C)]
// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// pub enum ParseFlag {
//     ParseLocal,
//     ParseNetwork,
//     FetchLocal,
//     FetchNetwork,
// }

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EventType {
    MediaMetaChanged = 0,
    MediaSubItemAdded,
    MediaDurationChanged,
    MediaParsedChanged,
    MediaFreed,
    MediaStateChanged,
    MediaSubItemTreeAdded,

    MediaPlayerMediaChanged = 0x100,
    MediaPlayerNothingSpecial,
    MediaPlayerOpening,
    MediaPlayerBuffering,
    MediaPlayerPlaying,
    MediaPlayerPaused,
    MediaPlayerStopped,
    MediaPlayerForward,
    MediaPlayerBackward,
    MediaPlayerEndReached,
    MediaPlayerEncounteredError,
    MediaPlayerTimeChanged,
    MediaPlayerPositionChanged,
    MediaPlayerSeekableChanged,
    MediaPlayerPausableChanged,
    MediaPlayerTitleChanged,
    MediaPlayerSnapshotTaken,
    MediaPlayerLengthChanged,
    MediaPlayerVout,
    MediaPlayerScrambledChanged,

    MediaListItemAdded = 0x200,
    MediaListWillAddItem,
    MediaListItemDeleted,
    MediaListWillDeleteItem,

    MediaListViewItemAdded = 0x300,
    MediaListViewWillAddItem,
    MediaListViewItemDeleted,
    MediaListViewWillDeleteItem,

    MediaListPlayerPlayed = 0x400,
    MediaListPlayerNextItemSet,
    MediaListPlayerStopped,

    MediaDiscovererStarted = 0x500,
    MediaDiscovererEnded,

    VlmMediaAdded = 0x600,
    VlmMediaRemoved,
    VlmMediaChanged,
    VlmMediaInstanceStarted,
    VlmMediaInstanceStopped,
    VlmMediaInstanceStatusInit,
    VlmMediaInstanceStatusOpening,
    VlmMediaInstanceStatusPlaying,
    VlmMediaInstanceStatusPause,
    VlmMediaInstanceStatusEnd,
    VlmMediaInstanceStatusError
}
