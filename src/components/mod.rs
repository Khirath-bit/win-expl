use nwg::NwgError;

use crate::app::BasicApp;

mod window;
mod header;
mod body;
pub mod list;
pub mod path_bar;

pub fn load_components(data: &mut BasicApp) -> Result<(), NwgError>{
    window::load(data)?;
    header::load(data)?;
    body::load(data)
}