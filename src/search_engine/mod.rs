use std::{fs::{self, DirEntry, FileType}, time::SystemTime, os::windows::fs::MetadataExt};

use crate::{debug, search_engine::parameter_parser::SearchEngineParameter};

mod parameter_parser;

#[derive(Default, Clone)]
pub struct SearchEngine {}

#[derive(Clone, Debug)]
pub struct SearchEngineResult {
    pub name: String,
    pub modified: SystemTime,
    pub file_type: FileType,
    pub size: u64,
    pub full_path: String
}

impl From<DirEntry> for SearchEngineResult {
    fn from(value: DirEntry) -> Self {
        let metadata = value.metadata().unwrap();

        SearchEngineResult { 
            name: value.file_name().to_str().unwrap().into(), 
            modified: metadata.modified().unwrap(), 
            file_type: metadata.file_type(), 
            size: metadata.file_size(),
            full_path: value.path().to_str().unwrap().into() }
    }
}


impl SearchEngine {
    pub fn search<'a>(t: &'a str, current_directory: &'a str, curr_depth: usize) -> impl Iterator<Item = SearchEngineResult> {
        let params = SearchEngineParameter::parse_search_term(t);

        let results = fs::read_dir(current_directory);
        #[allow(unused_assignments)]
        let mut s_e_results : Vec<SearchEngineResult> = Vec::new();

        match results {
            Ok(r) => {
                s_e_results = r.filter_map(|entry| {
                    let result = SearchEngineResult::from(entry.unwrap());
                    if result.name.contains(&params.term) {
                        Some(result)
                    } else {
                        None
                    }
                })
                .collect();
            },
            Err(_) => return vec![].into_iter(),
        }

        if curr_depth == params.depth {
            return s_e_results.into_iter();
        }

        let mut additional_results : Vec<SearchEngineResult> = Vec::new();

        let res_iter = fs::read_dir(current_directory)
        .unwrap().filter_map(|f| {
            let r = f.unwrap();
            match r.file_type().unwrap().is_dir() && SearchEngineParameter::dir_can_be_searched(&r, &params) {
                true => Some(r),
                false => None,
            }
        });

        for f in res_iter {
            debug!(&f.path());
            let mut test : Vec<SearchEngineResult> = SearchEngine::search(t, f.path().to_str().unwrap(), curr_depth+1).collect();
            additional_results.append(&mut test);
        }

        s_e_results.append(&mut additional_results);
    
        s_e_results.into_iter()
    }
}