mod app;
pub mod components;
mod event_handler;
mod memory;
mod resource_manager;
pub mod search_engine;
mod win;
mod settings;

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