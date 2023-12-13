use nwg::NwgError;

use crate::app::BasicApp;

use self::{
    control::Control, fav_dir_bar::FavoriteDirSidebar,
    search_result_control::SearchResultControl, status_bar_control::StatusBarControl, header_control::HeaderControl,
};

pub mod control;
pub mod fav_dir_bar;
pub mod header_control;
pub mod menuable;
pub mod path_bar_control;
pub mod search_result_control;
pub mod status_bar_control;
mod window;

pub fn load_components(data: &mut BasicApp) -> Result<(), NwgError> {
    window::load(data)?;
    HeaderControl::load_components(data)?;
    FavoriteDirSidebar::load_components(data)?;
    SearchResultControl::load_components(data)?;
    StatusBarControl::load_components(data)?;

    Ok(())
}
