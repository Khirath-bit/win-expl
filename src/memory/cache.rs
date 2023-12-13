use std::cell::RefCell;

use crate::{components::search_result_control::SortDirection, search_engine::SearchEngineResult, settings::Settings};

#[derive(Default)]
pub struct Cache {
    pub current_results: RefCell<Vec<SearchEngineResult>>,
    pub result_sort_direction: RefCell<(usize, SortDirection)>,
    pub settings: RefCell<Settings>
}
