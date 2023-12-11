use nwg::{ButtonFlags, NwgError, TextBoxFlags};
use winapi::um::winuser::{self};

use crate::{app::BasicApp, resource_manager::ResourceType};

pub fn load(data: &mut BasicApp) -> Result<(), NwgError> {
    nwg::Button::builder()
        .size((30, 30))
        .position((10, 10))
        .parent(&data.window)
        .flags(ButtonFlags::ICON | ButtonFlags::VISIBLE)
        .bitmap(Some(
            &data.resource_manager.get_bitmap(ResourceType::ArrowLeft)?,
        ))
        .build(&mut data.last_page_btn)?;

    unsafe {
        winuser::EnableWindow(data.last_page_btn.handle.hwnd().unwrap(), 0);
    }

    nwg::Button::builder()
        .size((30, 30))
        .position((80, 10))
        .parent(&data.window)
        .flags(ButtonFlags::ICON | ButtonFlags::VISIBLE)
        .bitmap(Some(
            &data.resource_manager.get_bitmap(ResourceType::Refresh)?,
        ))
        .build(&mut data.refresh_page_btn)?;

    nwg::TextInput::builder()
        .position((900, 10))
        .size((290, 30))
        .parent(&data.window)
        .placeholder_text(Some("Search..."))
        .build(&mut data.search_input)?;

    nwg::TextBox::builder()
        .position((120, 10))
        .size((670, 30))
        .text("C:")
        .flags(TextBoxFlags::VISIBLE)
        .parent(&data.window)
        .build(&mut data.path_bar.view)?;

    nwg::Button::builder()
        .size((30, 30))
        .position((795, 10))
        .parent(&data.window)
        .flags(ButtonFlags::ICON | ButtonFlags::VISIBLE)
        .bitmap(Some(&data.resource_manager.get_bitmap(ResourceType::Copy)?))
        .build(&mut data.copy_path_btn)?;

    Ok(())
}
