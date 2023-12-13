use crate::components::fav_dir_bar::FavoriteDirSidebar;
use crate::components::header_control::HeaderControl;
use crate::components::load_components;
use crate::components::search_result_control::SearchResultControl;
use crate::components::status_bar_control::StatusBarControl;
use crate::event_handler;
use crate::memory::cache::Cache;
use crate::resource_manager::Resources;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct BasicApp {
    pub window: nwg::Window,
    pub body_layout: nwg::GridLayout,
    pub file_dialog: nwg::FileDialog,
    pub header: HeaderControl,
    pub search_results: SearchResultControl,
    pub fav_dir_bar: FavoriteDirSidebar,
    pub resource_manager: Resources,
    pub status_bar: StatusBarControl,
    pub cache: Cache,
}

impl BasicApp {
    pub fn display_error(&self, e: String) {
        nwg::modal_error_message(&self.window, "Error occurred", &e);
    }
}

pub struct BasicAppUi {
    pub inner: Rc<BasicApp>,
    pub default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl nwg::NativeUi<BasicAppUi> for BasicApp {
    fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
        load_components(&mut data)?;

        let mut ui = BasicAppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        event_handler::handle_events(&mut ui);

        ui.inner.header.search_input.set_text("");

        Ok(ui)
    }
}

impl Drop for BasicAppUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}
