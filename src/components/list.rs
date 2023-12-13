use std::cell::Cell;

use crate::{memory::cache::Cache, search_engine::SearchEngineResult};
use nwg::EventData;
use time::{format_description, OffsetDateTime};
use winapi::{
    shared::windef::POINT,
    um::winuser::{GetCursorPos, GetScrollPos, GetScrollRange, SB_VERT},
};

#[derive(Default)]
pub struct BodyControls {
    pub results: nwg::ListView,
    pub item_context_menu: nwg::Menu,
    pub item_context_menu_copy: nwg::MenuItem,
    pub item_context_menu_add_to_fav: nwg::MenuItem,
    pub item_context_menu_remove_as_fav: nwg::MenuItem,
    pub directory_sidebar: nwg::ListView,
    pub context_menu_row_index: Cell<usize>,
    context_menu_target: Cell<ContextMenuTarget>,
}

#[derive(Default)]
enum ContextMenuTarget {
    #[default]
    None,
    Sidebar,
    ResultsList,
}

struct ListItemInsert {
    ind: Option<i32>,
    items: Vec<String>,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
    #[default]
    None,
}

impl BodyControls {
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
        self.results.clear();
        //TODO: magic number
        let items = Self::prep_data(results, 0, 50);
        for item in items {
            nwg::ListView::insert_items_row(&self.results, item.ind, item.items.as_slice());
        }
    }

    pub fn add_page(&self, cache: &Cache) {
        let hnd = self.results.handle.hwnd().unwrap();
        let mut max_pos = 0;
        let mut min_pos = 0;
        unsafe {
            GetScrollRange(hnd, SB_VERT as i32, &mut min_pos, &mut max_pos);
        }
        let vertical_scroll_pos = unsafe { GetScrollPos(hnd, SB_VERT as i32) };
        if (vertical_scroll_pos - max_pos).abs() != 25 {
            //25 strangely appeared to be the offset to the bottom, maybe because of the style?
            return;
        }
        let curr_res = cache.current_results.borrow_mut();
        let len = self.results.len();
        if len == curr_res.len() {
            return;
        }

        //TODO: remove/work around clone!
        let e = curr_res.clone();
        let prep = Self::prep_data(e, len, 50);

        for (ind, res) in prep.iter().enumerate() {
            nwg::ListView::insert_items_row(
                &self.results,
                Some((ind + len - 1) as i32),
                res.items.as_slice(),
            );
        }
    }

    pub fn show_context_menu_results(&self, evt_data: &EventData) {
        if !Self::valid_context_menu_click(&self.results, evt_data) {
            return;
        }
        self.context_menu_target.set(ContextMenuTarget::ResultsList);
        let is_folder = self
            .results
            .item(evt_data.on_list_view_item_index().0, 2, 260)
            .expect("Invalid column clicked")
            .text
            .eq("Directory");
        self.item_context_menu_remove_as_fav.set_enabled(false);
        //ignoring the fact that it might be already added because it doesnt cause any trouble
        self.item_context_menu_add_to_fav.set_enabled(is_folder);
        self.show_context_menu(evt_data);
    }

    pub fn show_context_menu_sidebar(&self, evt_data: &EventData) {
        if !Self::valid_context_menu_click(&self.directory_sidebar, evt_data) {
            return;
        }
        self.context_menu_target.set(ContextMenuTarget::Sidebar);
        self.item_context_menu_add_to_fav.set_enabled(false);
        self.item_context_menu_remove_as_fav.set_enabled(true);
        self.show_context_menu(evt_data);
    }

    fn valid_context_menu_click(list_view: &nwg::ListView, evt_data: &EventData) -> bool {
        let (row, _col) = evt_data.on_list_view_item_index();
        if row >= list_view.len() {
            //Clicked on empty field
            return false;
        }

        true
    }

    fn show_context_menu(&self, evt_data: &EventData) {
        let mut cursor_pos: POINT = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut cursor_pos);
        }
        self.item_context_menu.popup(cursor_pos.x, cursor_pos.y);
        self.context_menu_row_index
            .set(evt_data.on_list_view_item_index().0);
    }

    fn prep_data(
        results: Vec<SearchEngineResult>,
        skip: usize,
        take: usize,
    ) -> Vec<ListItemInsert> {
        let mut items: Vec<ListItemInsert> = Vec::new();
        for (ind, f) in results.iter().skip(skip).take(take).enumerate() {
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

            items.push(ListItemInsert {
                ind: Some(ind.try_into().unwrap()),
                items: vec![
                    f.name.clone(),
                    time,
                    file_type_str.into(),
                    size,
                    f.full_path.clone(),
                ],
            });
        }

        items
    }
}
