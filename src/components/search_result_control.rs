use crate::{app::BasicApp, memory::cache::Cache, search_engine::result::SearchEngineResult};
use clipboard::{ClipboardContext, ClipboardProvider};
use lhash::{Md5, Sha1, Sha256, Sha512};
use std::{cell::Cell, fs, rc::Rc};
use time::{format_description, OffsetDateTime};
use winapi::um::winuser::{GetScrollPos, GetScrollRange, SB_VERT};
use std::fmt::Write;

#[derive(Default)]
pub struct SearchResultControl {
    pub list: nwg::ListView,
    pub context_menu: nwg::Menu,
    pub context_menu_items: SearchResultControlMenuItems,
    pub(super) context_menu_context_row: Cell<usize>,
}
#[derive(Default)]
pub struct SearchResultControlMenuItems {
    pub add_to_favorites: nwg::MenuItem,
    pub copy_path: nwg::MenuItem,
    pub copy_name: nwg::MenuItem,
    pub md5_hash: nwg::MenuItem,
    pub sha1_hash: nwg::MenuItem,
    pub sha256_hash: nwg::MenuItem,
    pub sha512_hash: nwg::MenuItem,
    pub seperator: Vec<nwg::MenuSeparator>
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

pub enum HashFunction {
    MD5,
    SHA1,
    SHA256,
    SHA512,
}

impl SearchResultControl {
    pub(super) fn execute_add_to_favorites(&self, app: Rc<BasicApp>) {
        let row = self.context_menu_context_row.get();
        let path = self.list.item(row, 4, 260).expect("invalid menu row").text;
        let ind = Some(app.fav_dir_bar.list.len() as i32);
        let name = self.list.item(row, 0, 260).expect("invalid menu row").text;
        nwg::ListView::insert_items_row(&app.fav_dir_bar.list, ind, &[name.clone(), path.clone()]);
        app.cache
            .settings
            .borrow_mut()
            .add_favorite_folder(name, path);
    }

    pub(super) fn execute_copy_path(&self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(
            self.list
                .item(self.context_menu_context_row.get(), 1, 260)
                .unwrap()
                .text,
        )
        .unwrap();
    }

    pub(super) fn execute_copy_name(&self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(
            self.list
                .item(self.context_menu_context_row.get(), 0, 260)
                .unwrap()
                .text,
        )
        .unwrap();
    }

    pub(super) fn copy_file_hash(&self, hf: HashFunction) {
        let path = self
            .list
            .item(self.context_menu_context_row.get(), 4, 260)
            .unwrap()
            .text;
        let file = fs::read(path).unwrap();

        let hash: Vec<u8> = match hf {
            HashFunction::MD5 => {
                let mut alg = Md5::new();
                alg.update(&file);
                alg.result().to_vec()
            }
            HashFunction::SHA1 => {
                let mut alg = Sha1::new();
                alg.update(&file);
                alg.result().to_vec()
            }
            HashFunction::SHA256 => {
                let mut alg = Sha256::new();
                alg.update(&file);
                alg.result().to_vec()
            }
            HashFunction::SHA512 => {
                let mut alg = Sha512::new();
                alg.update(&file);
                alg.result().to_vec()
            }
        };

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(hash.iter().fold(String::new(), |mut acc, &byte| {
            write!(acc, "{:02X}", byte).expect("Failed to write to String");
            acc
        }))
        .unwrap();
    }

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
        self.list.clear();
        //TODO: magic number
        let items = Self::prep_data(results, 0, 50);
        for item in items {
            nwg::ListView::insert_items_row(&self.list, item.ind, item.items.as_slice());
        }
    }

    pub fn add_page(&self, cache: &Cache) {
        let hnd = self.list.handle.hwnd().unwrap();
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
        let len = self.list.len();
        if len == curr_res.len() {
            return;
        }

        //TODO: remove/work around clone!
        let e = curr_res.clone();
        let prep = Self::prep_data(e, len, 50);

        for (ind, res) in prep.iter().enumerate() {
            nwg::ListView::insert_items_row(
                &self.list,
                Some((ind + len - 1) as i32),
                res.items.as_slice(),
            );
        }
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
