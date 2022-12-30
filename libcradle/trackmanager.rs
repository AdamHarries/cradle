//! Track queueing and querying between a front-end interface and back-end audio provider.
//!
//! A TrackManager is the "middle-end" object that facilitates (indirect) communication between a front-end interface (such as a server, or direct CI interface) and a back-end audio service. This is needed as both the back-end and front-end need to be able to asynchronously add or remove data from a shared pool in response to user queries, or audio changes (such as finishing a track). This is where the trackManager comes in. It provides an interface to front-end code which supports adding tracks, querying tracks, and controlling playback, as well as an interface to back-end code which supports querying tracks (when playing a "playlist") and communicating audio state (such as progress) to front-end code asynchronously.
use std::collections::VecDeque;

use crate::track::{Track, TrackPath};
use std::path::PathBuf;
use anyhow::Result;

/// A PlaybackState describes whether or not music is currently playing, stopped, or paused during playback.
#[derive(Debug)]
pub enum PlaybackState {
    /// Music is stopped, no track is loaded
    Stopped,
    /// Music is playing, a track is loaded
    Playing(Track),
    /// Music is stopped, but a track is loaded
    Paused(Track),
}

pub struct TrackManager {
    /// The current state of playback, along with the (possibly) currently playing track.
    pub playbackstate: PlaybackState,
    /// Upcoming tracks
    pub queue: VecDeque<Track>, 
    /// Previous played tracks
    pub played: Vec<Track>,
}

impl TrackManager { 
    /// Create a new trackManager from scratch
    pub fn new() -> TrackManager {
        TrackManager { 
            playbackstate: PlaybackState::Stopped, 
            queue: VecDeque::new(),
            played: Vec::new(),
        }
    }

    /// Enqueue a track. It is the front-end's responsibility to ensure that it is a known/correct track file, that is readable, with metadata etc
    /// Enqueueing when the playback is stopped (with an empty playlist) will change the state to "paused", with the enqueued track as the paused track. 
    pub fn enqueue(mut self, track: Track) -> () {
        // Use a match to check the value of playbackstate. This way we don't need to implement PartialEq etc for the various structs that are a part of PlaybackState.
        match self.playbackstate {
            PlaybackState::Stopped =>  {
                if self.queue.len() == 0 {
                    self.playbackstate = PlaybackState::Paused(track);
                }else{
                    self.queue.push_back(track);
                }
            }, 
            _ => {
                self.queue.push_back(track);
            }
        }
    }

    fn start_playback(mut self) -> () {

    }
}
