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

#[derive(Debug)]
pub enum SessionChange {
    Added {
        session_id: String
    },
    Removed {
        session_id: String
    }
}