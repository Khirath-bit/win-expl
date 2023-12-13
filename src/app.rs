use crate::components::list::BodyControls;
use crate::components::load_components;
use crate::components::path_bar::PathBar;
use crate::components::status_bar::ExplStatusBar;
use crate::event_handler;
use crate::memory::cache::Cache;
use crate::resource_manager::Resources;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct BasicApp {
    pub window: nwg::Window,
    pub name_edit: nwg::TextInput,
    pub last_page_btn: nwg::Button,
    pub next_page_btn: nwg::Button,
    pub refresh_page_btn: nwg::Button,
    pub copy_path_btn: nwg::Button,
    pub search_input: nwg::TextInput,
    pub body_layout: nwg::GridLayout,
    pub file_dialog: nwg::FileDialog,
    pub body: BodyControls,
    pub path_bar: PathBar,
    pub resource_manager: Resources,
    pub status_bar: ExplStatusBar,
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

        ui.inner.search_input.set_text("");

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

// impl Deref for BasicAppUi {
//     type Target = BasicApp;

//     fn deref(&self) -> &BasicApp {
//         &self.inner.into()
//     }
// }
