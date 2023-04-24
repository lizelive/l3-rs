use futures::stream::StreamExt;
use object_store::{local::LocalFileSystem, path::Path, ObjectStore};
use std::sync::Arc;

use thiserror::Error;

use bytes::Bytes;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Error listing files")]
    List,

    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Store Error: {0}")]
    Store(#[from] object_store::Error),
}

#[tokio::test]
async fn test() -> Result<(), StoreError> {
    use StoreError::*;
    // create an ObjectStore
    let object_store = get_object_store()?;

    // Recursively list all files below the 'data' path.
    // 1. On AWS S3 this would be the 'data/' prefix
    // 2. On a local filesystem, this would be the 'data' directory
    let prefix = Path::from("data");

    // Get an `async` stream of Metadata objects:
    let list_stream = object_store.list(Some(&prefix)).await;
    let list_stream = list_stream.or(Err(List))?;

    let hello_world = crate::time::now();

    let hello_world_bytes = Bytes::from(hello_world);

    object_store
        .put(&Path::from(format!("{}/heartbeat", prefix)), hello_world_bytes)
        .await?;

    // Print a line about each object based on its metadata
    // using for_each from `StreamExt` trait.

    list_stream
        .for_each(move |meta| async {
            if let Ok(meta) = meta {
                println!("object: {:#?}", meta);
            }
        })
        .await;
    Ok(())
}

fn get_object_store() -> Result<impl ObjectStore, StoreError> {
    let prefix = "./ice";

    // create directory if it doesn't exist
    std::fs::create_dir_all(prefix)?;

    let object_store = LocalFileSystem::new_with_prefix(prefix)?;
    Ok(object_store)
}
