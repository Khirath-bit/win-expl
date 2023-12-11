use nwg::{LabelFlags, NwgError};
use winapi::um::winuser::SS_RIGHT;

use crate::app::BasicApp;

#[derive(Default)]
pub struct ExplStatusBar {
    pub result_count: nwg::Label,
    pub search_duration: nwg::Label,
}

impl ExplStatusBar {
    pub fn load(data: &mut BasicApp) -> Result<(), NwgError> {
        nwg::Label::builder()
            .parent(&data.window)
            .background_color(Some([50, 50, 50]))
            .flags(LabelFlags::ELIPSIS | LabelFlags::VISIBLE)
            .position((10, 600))
            .size((100, 20))
            .build(&mut data.status_bar.result_count)?;

        unsafe {
            nwg::Label::builder()
                .parent(&data.window)
                .background_color(Some([50, 50, 50]))
                .flags(
                    LabelFlags::ELIPSIS
                        | LabelFlags::VISIBLE
                        | LabelFlags::from_bits_unchecked(SS_RIGHT),
                )
                .text("200ms")
                .position((1090, 600))
                .size((100, 20))
                .build(&mut data.status_bar.search_duration)?;
        }

        Ok(())
    }
}
