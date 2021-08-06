use std::path::PathBuf;

#[derive(Debug)]
pub enum DeviceChange {
    Added {
        syspath: PathBuf
    },
    Removed {
        syspath: PathBuf
    }
}