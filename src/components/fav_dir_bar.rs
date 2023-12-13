use std::cell::Cell;
use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Default)]
pub struct FavoriteDirSidebar {
    pub list: nwg::ListView,
    pub context_menu: nwg::Menu,
    pub context_menu_items: FavoriteDirSidebarMenuItems,
    pub (super) context_menu_context_row: Cell<usize>,
}
#[derive(Default)]
pub struct FavoriteDirSidebarMenuItems {
    pub remove: nwg::MenuItem,
    pub copy_path: nwg::MenuItem,
}

impl FavoriteDirSidebar {
    pub (super) fn execute_remove(&self) {}

    pub (super) fn execute_copy_path(&self) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(
            self.list
                .item(self.context_menu_context_row.get(), 1, 260)
                .unwrap()
                .text,
        )
        .unwrap();
    }
}