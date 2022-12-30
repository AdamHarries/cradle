//! Tools describing and manipulating audio files
//! 
//! There are multiple different possible ways for describing a track. We could talk about an on-disk path, we could describe the track title (or other metadata) etc.


use std::path::PathBuf;
use anyhow::{Result, ensure};
use thiserror::Error;
use tempfile::tempfile;
use super::metadata::*;

#[derive(Error, Debug)]
pub enum TrackPathError {
    #[error("Track path does not exist")]
    NotFound, 
    #[error("Track path is not a file")]
    IsNotFile,
    #[error("Could not compute canonical file path")]
    BadCanon(std::io::Error),

}

/// A trackpath is a reference to a track that (at the time of validation) is known to exist, and is known to be a file that is accessible. Once we can construct a trackpath, we can then use it to construct an even more trustworthy Track, which ensures that we control the file, and can read metadata etc.
#[derive(Debug)]
pub struct TrackPath { 
    path: PathBuf,
}

impl TrackPath {
    /// Try to construct a TrackPath from a path buffer.
    fn new(path: PathBuf) -> Result<TrackPath> { 
        ensure!(path.exists(), TrackPathError::NotFound);
        ensure!(path.is_file(), TrackPathError::IsNotFile);
        match path.canonicalize() {
            Ok(canonpath) => Ok(TrackPath {path: path}), 
            Err(e) => Err(TrackPathError::BadCanon(e).into()),
        }
    }

    /// Get the underlying path stored by the TrackPath
    fn get_path(self) -> PathBuf {
        self.path
    }
}

/// A track stores a fleshed-out reference to a specific track. It includes metadata read from the track, as well as the path etc.
#[derive(Debug)]
pub struct Track { 
    path: TrackPath, 
    metadata: Result<Metadata>,

}

impl Track { 
    fn new(path: TrackPath) -> Result<Track> {
        let mdp = get_provider();
        let metadata = mdp.get(&path);
        Ok(Track {
            path: path, 
            metadata: metadata,
        })
    }
}