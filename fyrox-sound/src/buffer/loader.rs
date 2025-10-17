//! Sound buffer loader.

use crate::buffer::{DataSource, SoundBuffer};
use fyrox_core::{uuid::Uuid, TypeUuidProvider};
use fyrox_resource::{
    loader::{BoxedLoaderFuture, LoaderPayload, ResourceLoader},
    state::LoadError,
};
use std::{path::PathBuf, sync::Arc};

pub use fyrox_resource::io::{FsResourceIo, ResourceIo};

/// Defines sound buffer resource import options.
#[derive(Clone, Default, Debug)]
pub struct SoundBufferImportOptions {
    /// Whether the buffer is streaming or not.
    pub stream: bool,
}

/// Default implementation for sound buffer loading.
pub struct SoundBufferLoader {
    /// Default import options for sound buffer resources.
    pub default_import_options: SoundBufferImportOptions,
}

impl ResourceLoader for SoundBufferLoader {
    fn extensions(&self) -> &[&str] {
        &["wav", "ogg"]
    }

    fn data_type_uuid(&self) -> Uuid {
        SoundBuffer::type_uuid()
    }

    fn load(&self, path: PathBuf, io: Arc<dyn ResourceIo>) -> BoxedLoaderFuture {
        Box::pin(async move {
            let io = io.as_ref();

            let source = DataSource::from_file(&path, io)
                .await
                .map_err(LoadError::new)?;

            let result = SoundBuffer::raw_generic(source);
            match result {
                Ok(buffer) => Ok(LoaderPayload::new(buffer)),
                Err(_) => Err(LoadError::new("Invalid data source.")),
            }
        })
    }
}
