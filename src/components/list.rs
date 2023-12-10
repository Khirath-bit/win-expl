use time::{OffsetDateTime, format_description};
use crate::search_engine::SearchEngineResult;

#[derive(Default)]
pub struct ResultList {
    pub view: nwg::ListView,
    pub item_context_menu: nwg::Menu,
    pub item_context_menu_copy: nwg::MenuItem 
}

impl ResultList {
    //TODO: sorts and REFRESHES the column, maybe optimize later
    pub fn sort_by_column(&self, col_index: i32, mut results: Vec<SearchEngineResult>) {
        match col_index {
            1 => {
                results.sort_by(|a,b| a.modified.cmp(&b.modified));
            }
            3 => {
                results.sort_by(|a,b| a.size.cmp(&b.size));
            }
            0 => {
                results.sort_by(|a,b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }
            _ => return //Sorting by type is not supported
        }
        //TODO: OPTIMIZE to no cloning
        self.refresh(results.clone().into_iter());
    }

    pub fn refresh(&self, results: impl Iterator<Item = SearchEngineResult>) {
        self.view.clear();
        for (ind, f) in results.enumerate() {
            let chrono_time: OffsetDateTime = f.modified.into();

            let time = chrono_time.format(&format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap()).unwrap().to_string();
            
            let file_type_str = match f.file_type {
                t if t.is_dir() => "Directory",
                t if t.is_file() => "File",
                t if t.is_symlink() => "Symlink",
                _ => "Unknown",
            };
            
            let size = match f.file_type {
                t if t.is_dir() => "".into(),
                t if t.is_file() => format!("{} KiB", std::cmp::max(f.size/1000, 1)),
                t if t.is_symlink() => format!("{} KiB", std::cmp::max(f.size/1000, 1)),
                _ => "".into(),
            };

            nwg::ListView::insert_items_row(&self.view, Some(ind.try_into().unwrap()), &[f.name, time, file_type_str.into(), size, f.full_path]);
        } 

    }

}