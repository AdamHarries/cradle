//! Retrieve metadata for a particular track, expressed as a file path
//!
//! This is deliberately abstracted to allow for (future) different implementation of metadata retrieval. At present, we already want to be able to choose between using gstreamer or using symphonia (at compile time), and there may be more platform-specific options in the future. 
//! We use a combination of trait-based dispatch along with #[cfg(...)] gating particular implementations to select the one that we want. Users can then just call `get_provider`, which will deliver a working provider depending on the compile time features/options.

use std::os::linux::fs::MetadataExt;

use crate::track::Track;

use super::track::TrackPath;
use anyhow::Result;

#[derive(Debug)]
pub struct Metadata {
    title: Result<String>,
    artist: Result<String>,
}

pub trait MetaDataProvider {
    fn get(self, path: &TrackPath) -> Result<Metadata>;
}

#[cfg(not(feature = "symphonia_metadata"))]
pub struct GStreamerMetadataProvider {}

#[cfg(not(feature = "symphonia_metadata"))]
impl MetaDataProvider for GStreamerMetadataProvider {
    fn get(self, path: &TrackPath) -> Result<Metadata> {
        todo!();
    }
}

#[cfg(not(feature = "symphonia_metadata"))]
pub fn get_provider() -> impl MetaDataProvider {
    GStreamerMetadataProvider {}
}

#[cfg(feature = "symphonia_metadata")]
pub struct SymphoniaMetadataProvider {}

#[cfg(feature = "symphonia_metadata")]
impl MetaDataProvider for SymphoniaMetadataProvider {
    fn get(self, path: &TrackPath) -> Result<Metadata> {
        todo!();
    }
}

#[cfg(feature = "symphonia_metadata")]
pub fn get_provider() -> impl MetaDataProvider {
    SymphoniaMetadataProvider {}
}
