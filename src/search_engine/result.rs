use std::{time::SystemTime, fs::{FileType, DirEntry}, os::windows::fs::MetadataExt};

#[derive(Clone, Debug)]
pub struct SearchEngineResult {
    pub name: String,
    pub modified: SystemTime,
    pub file_type: FileType,
    pub size: u64,
    pub full_path: String,
}

impl From<&DirEntry> for SearchEngineResult {
    fn from(value: &DirEntry) -> Self {
        let metadata = value.metadata().unwrap();

        SearchEngineResult {
            name: value.file_name().to_str().unwrap().into(),
            modified: metadata.modified().unwrap(),
            file_type: metadata.file_type(),
            size: metadata.file_size(),
            full_path: value.path().to_str().unwrap().into(),
        }
    }
}