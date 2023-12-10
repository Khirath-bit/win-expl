use nwg::NwgError;

use crate::app::BasicApp;
use winapi::um::{winuser::{self, GCLP_HBRBACKGROUND}, wingdi::{CreateSolidBrush, RGB}};

pub fn load(data: &mut BasicApp) -> Result<(), NwgError> {
    let w = 1200;
    let h = 600;

    let x = (nwg::Monitor::width()-w)/2;
    let y = (nwg::Monitor::height()-h)/2;

    nwg::Window::builder()
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((w, h))
            .position((x, y))
            .title("better-explorer")
            .build(&mut data.window)?;

    unsafe {
        let brush = CreateSolidBrush(RGB(50, 50, 50)) as winapi::shared::basetsd::LONG_PTR;
        winuser::SetClassLongPtrA(data.window.handle.hwnd().unwrap(), GCLP_HBRBACKGROUND, brush);
    }
    nwg::Window::invalidate(&data.window);

    Ok(())
}