use std::cell::Cell;

use crate::search_engine::SearchEngineResult;
use time::{format_description, OffsetDateTime};

#[derive(Default)]
pub struct ResultList {
    pub view: nwg::ListView,
    pub item_context_menu: nwg::Menu,
    pub item_context_menu_copy: nwg::MenuItem,
    pub context_menu_row_index: Cell<usize>,
}

#[derive(Default, Clone, Debug)]
pub enum SortDirection {
    Asc,
    Desc,
    #[default]
    None,
}

impl ResultList {
    //TODO: sorts and REFRESHES the column, maybe optimize later
    pub fn sort_by_column(
        &self,
        sort_dir: &mut (usize, SortDirection),
        col_index: usize,
        mut results: Vec<SearchEngineResult>,
    ) {
        if sort_dir.0 != col_index && col_index != 4 {
            //Not supported by type
            sort_dir.0 = col_index;
            sort_dir.1 = SortDirection::Asc;
        }

        match col_index {
            1 => match sort_dir.1 {
                SortDirection::Desc => results.sort_by(|a, b| b.modified.cmp(&a.modified)),
                _ => results.sort_by(|a, b| a.modified.cmp(&b.modified)),
            },
            3 => match sort_dir.1 {
                SortDirection::Desc => results.sort_by(|a, b| b.size.cmp(&a.size)),
                _ => results.sort_by(|a, b| a.size.cmp(&b.size)),
            },
            0 => match sort_dir.1 {
                SortDirection::Desc => {
                    results.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()))
                }
                _ => results.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
            },
            _ => return, //Sorting by type is not supported
        }

        match sort_dir.1 {
            SortDirection::Desc => sort_dir.1 = SortDirection::Asc,
            _ => sort_dir.1 = SortDirection::Desc,
        }

        //TODO: OPTIMIZE to no cloning
        self.refresh(results.clone());
    }

    pub fn refresh(&self, results: Vec<SearchEngineResult>) {
        self.view.clear();
        for (ind, f) in results.iter().enumerate() {
            let chrono_time: OffsetDateTime = f.modified.into();

            let time = chrono_time
                .format(&format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap())
                .unwrap()
                .to_string();

            let file_type_str = match f.file_type {
                t if t.is_dir() => "Directory",
                t if t.is_file() => "File",
                t if t.is_symlink() => "Symlink",
                _ => "Unknown",
            };

            let size = match f.file_type {
                t if t.is_dir() => "".into(),
                t if t.is_file() => format!("{} KiB", std::cmp::max(f.size / 1000, 1)),
                t if t.is_symlink() => format!("{} KiB", std::cmp::max(f.size / 1000, 1)),
                _ => "".into(),
            };
            nwg::ListView::insert_items_row(
                &self.view,
                Some(ind.try_into().unwrap()),
                &[
                    f.name.clone(),
                    time,
                    file_type_str.into(),
                    size,
                    f.full_path.clone(),
                ],
            );
        }
    }
}
