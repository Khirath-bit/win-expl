use super::path_bar_control::PathBarControl;

#[derive(Default)]
pub struct HeaderControl {
    pub last_page_btn: nwg::Button,
    pub parent_page_btn: nwg::Button,
    pub refresh_btn: nwg::Button,
    pub search_input: nwg::TextInput,
    pub path_bar: PathBarControl,
    pub copy_path_btn: nwg::Button,
}