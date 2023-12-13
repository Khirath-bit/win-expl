use std::{
    fs::{self, DirEntry, FileType},
    os::windows::fs::MetadataExt,
    time::SystemTime,
};

use crate::search_engine::parameter_parser::SearchEngineParameter;

pub mod parameter_parser;

#[derive(Default, Clone)]
pub struct SearchEngine {}

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
#[allow(clippy::all)]
impl SearchEngine {
    pub fn search<'a>(
        p: &'a SearchEngineParameter,
        current_directory: &'a str,
        curr_depth: usize,
    ) -> Result<Vec<SearchEngineResult>, ()> {
        let results = fs::read_dir(current_directory);
        #[allow(unused_assignments)]
        let mut s_e_results: Vec<SearchEngineResult> = Vec::new();
        match results {
            Ok(r) => {
                s_e_results = r
                    .filter_map(|entry| {
                        let e = entry.unwrap();
                        let mut is_ok = match &p.term {
                            Some(t) => e.file_name().to_string_lossy().contains(t),
                            None => true,
                        };

                        is_ok &= match &p.extension {
                            Some(t) => e.path().to_string_lossy().split('.').last().unwrap().contains(t),
                            None => true,
                        };

                        if is_ok {
                            Some(SearchEngineResult::from(&e))
                        } else {
                            None
                        }
                    })
                    .collect();
            }
            Err(_) => return Err(()),
        }

        if curr_depth == p.depth {
            return Ok(s_e_results);
        }
        
        let mut additional_results: Vec<SearchEngineResult> = Vec::new();

        let res_iter = fs::read_dir(current_directory).unwrap().filter_map(|f| {
            let r = f.unwrap();
            match r.file_type().unwrap().is_dir()
                && SearchEngineParameter::dir_can_be_searched(&r, p)
            {
                true => Some(r),
                false => None,
            }
        });

        for f in res_iter {
            if let Ok(mut test) = SearchEngine::search(p, f.path().to_str().unwrap(), curr_depth + 1) {
                additional_results.append(&mut test);
            }
        }

        s_e_results.append(&mut additional_results);

        Ok(s_e_results)
    }
}
