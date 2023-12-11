use std::cell::RefCell;

use crate::{components::list::SortDirection, search_engine::SearchEngineResult};

#[derive(Default)]
pub struct Cache {
    pub current_results: RefCell<Vec<SearchEngineResult>>,
    pub result_sort_direction: RefCell<(usize, SortDirection)>,
}
