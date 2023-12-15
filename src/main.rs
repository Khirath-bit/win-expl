use app::BasicApp;
use nwg::NativeUi;

mod app;
mod components;
mod event_handler;
mod memory;
mod resource_manager;
pub mod search_engine;
mod win;
mod settings;
pub mod ring_buffer;

pub struct Test {}

#[macro_export]
macro_rules! debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                dbg!($($e),+)
            }
        }
    };
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
