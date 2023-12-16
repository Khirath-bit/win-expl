#[derive(Default)]
pub struct StatusBarControl {
    pub result_count: nwg::Label,
    pub search_duration: nwg::Label,
    pub index_date: nwg::Label,
    pub index_usage: nwg::CheckBox,
    pub index_refresh: nwg::Button
}