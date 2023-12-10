use std::{fs::DirEntry, os::windows::fs::MetadataExt};

pub struct SearchEngineParameter {
    pub depth: usize,
    pub term: String,
    pub search_readonly_dirs: bool,
    pub search_hidden_dirs: bool,
    pub search_bin_dirs: bool,
    pub search_lib_dirs: bool,
    pub search_tmp_dirs: bool
}
#[allow(non_camel_case_types)]
enum FileAttributes {
    FILE_ATTRIBUTE_READONLY = 1,
    FILE_ATTRIBUTE_HIDDEN = 2,
}

impl SearchEngineParameter {
    pub fn parse_search_term(t: &str) -> SearchEngineParameter {
        let parts: Vec<String> = t.split('!').map(|s| s.to_string()).collect();

        let mut params = SearchEngineParameter {
            term: parts.first().unwrap().into(),
            depth: 0,
            search_readonly_dirs: false,
            search_hidden_dirs: false,
            search_bin_dirs: false,
            search_lib_dirs: false,
            search_tmp_dirs: false,
        };

        for pa in parts.iter().skip(1) {
            let p = pa.trim();
            if p.starts_with("d=") {
                if let Ok(d) = p.replace("d=", "").parse::<usize>() {
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
            }
        }

        params
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
            can_be &= !name.eq("bin");
        }

        if !p.search_lib_dirs {
            can_be &= !name.eq("lib");
        }

        if !p.search_tmp_dirs {
            can_be &= !name.eq("tmp");
        }

        can_be
    }
}