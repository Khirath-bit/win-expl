use std::{fs::DirEntry, os::windows::fs::MetadataExt};

use crate::win::file_attributes::FileAttributes;

#[derive(Debug, Clone)]
pub struct SearchEngineParameter {
    pub depth: usize,
    pub term: Option<String>,
    pub extension: Option<String>,
    pub search_readonly_dirs: bool,
    pub search_hidden_dirs: bool,
    pub search_bin_dirs: bool,
    pub search_lib_dirs: bool,
    pub search_tmp_dirs: bool,
    pub search_windows_folder: bool,
}

impl SearchEngineParameter {
    #[allow(clippy::all)]
    pub fn parse_search_term(t: &str) -> Result<SearchEngineParameter, ()> {
        let parts: Vec<String> = t.split('!').map(|s| s.to_string()).collect();

        let file_args: Vec<String> = parts.first().unwrap().split('.').map(|s| s.to_string()).collect();

        let mut params = SearchEngineParameter {
            term: None,
            extension: None,
            depth: 0,
            search_readonly_dirs: false,
            search_hidden_dirs: false,
            search_bin_dirs: false,
            search_lib_dirs: false,
            search_tmp_dirs: false,
            search_windows_folder: false,
        };

        if parts.first().unwrap().contains('.') {
            if file_args.len() > 1 {
                params.term = Some(file_args[0].clone());
                params.extension = Some(file_args[1].clone());
            } else {
                params.extension = Some(file_args[0].clone());
            }
        } else {
            params.term = Some(file_args[0].clone());
        }

        if let Some(ext) = &params.extension {
            if ext.eq(""){
                return Err(()); //No valid search term
            }
        }

        for pa in parts.iter().skip(1) {
            let p = pa.trim();
            if p.starts_with('d') {
                if let Ok(d) = p.replace('d', "").parse::<usize>() {
                    params.depth = d;
                }
            } else if p.eq("r") {
                params.search_readonly_dirs = true;
            } else if p.eq("h") {
                params.search_hidden_dirs = true;
            } else if p.eq("b") {
                params.search_bin_dirs = true;
            } else if p.eq("l") {
                params.search_lib_dirs = true;
            } else if p.eq("t") {
                params.search_tmp_dirs = true;
            } else if p.eq("w") {
                params.search_windows_folder = true;
            }
        }

        Ok(params)
    }

    pub fn dir_can_be_searched(dir: &DirEntry, p: &SearchEngineParameter) -> bool {
        let mut can_be = true;

        let meta_attr = dir.metadata().unwrap().file_attributes();
        let binding = dir.file_name();
        let name = binding.to_str().unwrap();

        if !p.search_readonly_dirs {
            let read_only = meta_attr & FileAttributes::FILE_ATTRIBUTE_READONLY as u32;
            can_be &= read_only == 0;
        }

        if !p.search_hidden_dirs {
            let hidden = meta_attr & FileAttributes::FILE_ATTRIBUTE_HIDDEN as u32;
            can_be &= hidden == 0;
        }

        if !p.search_bin_dirs {
            can_be &= !name.to_lowercase().eq("bin");
        }

        if !p.search_lib_dirs {
            can_be &= !name.to_lowercase().eq("lib");
        }

        if !p.search_tmp_dirs {
            can_be &= !name.to_lowercase().eq("tmp");
        }

        if !p.search_windows_folder {
            can_be &= !name.to_lowercase().eq("windows");
        }

        can_be
    }
}
