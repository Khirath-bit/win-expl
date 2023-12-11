use nwg::NwgError;

use crate::app::BasicApp;

use self::status_bar::ExplStatusBar;

mod body;
mod header;
pub mod list;
pub mod path_bar;
pub mod status_bar;
mod window;

pub fn load_components(data: &mut BasicApp) -> Result<(), NwgError> {
    window::load(data)?;
    header::load(data)?;
    body::load(data)?;
    ExplStatusBar::load(data)
}
