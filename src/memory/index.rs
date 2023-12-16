use std::{time::SystemTime, fs, io::BufReader, path::Path, ffi::OsStr};

use serde::{Serialize, Deserialize, de::IntoDeserializer};

use crate::debug;

//TODO
pub struct Index {
    pub root: Option<IndexType>,
    pub modified_date: SystemTime,
}


#[derive(Serialize, Deserialize)]
pub enum IndexType {
    Dir(String, Vec<IndexType>),
    File(String)
}

impl Default for Index {
    fn default() -> Self {
        Self { root: None, modified_date: SystemTime::now() }
    }
}

impl Index {
    //TODO refresh partially by using current index
    pub fn refresh(&mut self, modified_label: &nwg::Label){
        self.root = Some(Self::traverse_and_generate("C:\\", 8));
        let _ = fs::write("index.json", serde_json::to_string_pretty(&self.root).unwrap());
        println!("Done indexing.")
    }

    //Ignores syslink files
    fn traverse_and_generate(root: &str, depth: usize) -> IndexType {
        let mut valid_entries = Vec::new();
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                if entry.file_type().unwrap().is_dir() {
                    if depth > 0 {
                        valid_entries.push(Self::traverse_and_generate(&entry.path().to_string_lossy(), depth-1));
                    }
                } else {
                    valid_entries.push(IndexType::File(entry.file_name().into_string().unwrap()));
                }
            }
        }

        IndexType::Dir(Path::new(root).file_stem().unwrap_or(OsStr::new("C:")).to_string_lossy().to_string(), valid_entries)
    }

    pub fn check_for_updates(&mut self) {
        if let Ok(f) = fs::File::open("index.json") {
            self.root = serde_json::from_reader(BufReader::new(&f)).expect("Invalid index file structure");
            self.modified_date = f.metadata().unwrap().modified().unwrap();
        }
    }
}