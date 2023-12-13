#[cfg(test)]
mod tests {
    use std::{fs::{self}, io::Error, path::Path};
    use win_expl::search_engine::{SearchEngine, parameter_parser::SearchEngineParameter};

    fn init_folder_structure() -> Result<(), Error> {
        if Path::new("tests/search_engine_tests_tmp").exists() {
            return Ok(());
        }
        fs::create_dir("tests/search_engine_tests_tmp")?;
        fs::create_dir("tests/search_engine_tests_tmp\\Windows")?;
        fs::create_dir("tests/search_engine_tests_tmp\\bin")?;
        fs::create_dir("tests/search_engine_tests_tmp\\tmp")?;
        fs::create_dir("tests/search_engine_tests_tmp\\lib")?;
        fs::create_dir("tests/search_engine_tests_tmp\\Test")?;
        fs::File::create("tests/search_engine_tests_tmp\\Windows\\tmp-win.txt")?;
        fs::File::create("tests/search_engine_tests_tmp\\bin\\tmp-bin.txt")?;
        fs::File::create("tests/search_engine_tests_tmp\\tmp\\tmp-tmptxt")?;
        fs::File::create("tests/search_engine_tests_tmp\\lib\\tmp-lib.txt")?;
        fs::File::create("tests/search_engine_tests_tmp\\Test\\tmp-test.txt")?;
    
        Ok(())
    }

    #[test]
    fn test_folder_ignored(){
        init_folder_structure().unwrap();
        let root = "tests/search_engine_tests_tmp";
        let t = ".txt!d1";
        let p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(!p.search_windows_folder);
        assert!(!p.search_hidden_dirs);
        assert!(!p.search_bin_dirs);
        assert!(!p.search_lib_dirs);
        assert!(!p.search_readonly_dirs);
        assert!(!p.search_tmp_dirs);
        let result = SearchEngine::search(&p, root, 0).expect("Failed to search");
        assert!(result.iter().all(|r| !(r.file_type.is_dir() 
        && (r.name.eq("tmp-win") || r.name.starts_with("tmp-bin") || r.name.eq("tmp-tmp") || r.name.eq("tmp-lib")))), "Blacklisted folder was falsely returned!");
        assert!(result.len() == 1);
        assert!(result.first().unwrap().name.starts_with("tmp-test"));
    }

    #[test]
    fn test_only_windows_folder_ignored(){
        init_folder_structure().unwrap();
        let root = "tests/search_engine_tests_tmp";
        let t = ".txt!d1!r!h!b!l!t";
        let p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(!p.search_windows_folder);
        let result = SearchEngine::search(&p, root, 0).expect("Failed to search");
        assert!(result.iter().all(|r| !(r.file_type.is_dir() 
        && r.name.eq("tmp-win"))), "Blacklisted folder was falsely returned!");
        assert!(result.len() == 4);
    }

    #[test]
    fn test_only_lib_folder_ignored(){
        init_folder_structure().unwrap();
        let root = "tests/search_engine_tests_tmp";
        let t = ".txt!d1!r!h!b!w!t";
        let p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(!p.search_lib_dirs);
        let result = SearchEngine::search(&p, root, 0).expect("Failed to search");
        assert!(result.iter().all(|r| !(r.file_type.is_dir() 
        && r.name.eq("tmp-lib"))), "Blacklisted folder was falsely returned!");
        assert!(result.len() == 4);
    }

    #[test]
    fn test_only_bin_folder_ignored(){
        init_folder_structure().unwrap();
        let root = "tests/search_engine_tests_tmp";
        let t = ".txt!d1!r!h!l!w!t";
        let p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(!p.search_bin_dirs);
        let result = SearchEngine::search(&p, root, 0).expect("Failed to search");
        assert!(result.iter().all(|r| !(r.file_type.is_dir() 
        && r.name.eq("tmp-bin"))), "Blacklisted folder was falsely returned!");
        assert!(result.len() == 4);
    }

    #[test]
    fn test_only_tmp_folder_ignored(){
        init_folder_structure().unwrap();
        let root = "tests/search_engine_tests_tmp";
        let t = ".txt!d1!r!h!l!w!b";
        let p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(!p.search_tmp_dirs);
        let result = SearchEngine::search(&p, root, 0).expect("Failed to search");
        assert!(result.iter().all(|r| !(r.file_type.is_dir() 
        && r.name.eq("tmp-tmp"))), "Blacklisted folder was falsely returned!");
        assert!(result.len() == 4);
    }
    
    #[test]
    fn test_depth_correctly_used(){
        init_folder_structure().unwrap();
        let root = "tests/search_engine_tests_tmp";
        let mut t = ".txt!d0";
        let mut p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(p.depth == 0);
    
        t = ".txt";
        p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(p.depth == 0);
    
        t = ".txt!d=5";
        p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(p.depth == 0);
    
        let result = SearchEngine::search(&p, root, 0).expect("Failed to search");
        assert!(result.is_empty());
    
        t = ".txt!d1";
        p = SearchEngineParameter::parse_search_term(t).expect("Failed to parse paramter");
        assert!(p.depth == 1);
    
        let result = SearchEngine::search(&p, root, 0).expect("Failed to search");
        assert!(result.len() == 1);
    
        let result = SearchEngine::search(&p, root, 1).expect("Failed to search");
        assert!(result.is_empty());
    }
    
}
