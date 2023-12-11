use nwg::NwgError;

use crate::app::BasicApp;
use winapi::shared::minwindef::DWORD;
use winapi::um::{
    wingdi::{CreateSolidBrush, RGB},
    winuser::{self, GCLP_HBRBACKGROUND},
};
use winapi::{shared::winerror::SUCCEEDED, um::dwmapi::DwmSetWindowAttribute};

pub fn load(data: &mut BasicApp) -> Result<(), NwgError> {
    let w = 1200;
    let h = 630;

    let x = (nwg::Monitor::width() - w) / 2;
    let y = (nwg::Monitor::height() - h) / 2;

    nwg::Window::builder()
        .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
        .size((w, h))
        .position((x, y))
        .title("better-explorer")
        .build(&mut data.window)?;

    unsafe {
        let brush = CreateSolidBrush(RGB(50, 50, 50)) as winapi::shared::basetsd::LONG_PTR;
        winuser::SetClassLongPtrA(
            data.window.handle.hwnd().unwrap(),
            GCLP_HBRBACKGROUND,
            brush,
        );
        let result = DwmSetWindowAttribute(
            data.window.handle.hwnd().unwrap(),
            20,
            &1 as *const _ as *const winapi::ctypes::c_void,
            std::mem::size_of_val(&1) as DWORD,
        );

        SUCCEEDED(result);
    }

    nwg::Window::invalidate(&data.window);

    Ok(())
}
