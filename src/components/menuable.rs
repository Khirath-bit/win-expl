use std::rc::Rc;

use nwg::EventData;
use winapi::{shared::windef::POINT, um::winuser::GetCursorPos};

use crate::app::BasicApp;

use super::{search_result_control::SearchResultControl, fav_dir_bar::FavoriteDirSidebar};

pub trait Menuable {
    fn open_menu(&self, evt_data: &EventData);
    fn execute_menu_item_click(&self, item: &nwg::ControlHandle, app: Rc<BasicApp>);
}

impl Menuable for SearchResultControl {
    fn open_menu(&self, evt_data: &nwg::EventData) {
        let (row, _col) = evt_data.on_list_view_item_index();
        if row >= self.list.len() {
            //Clicked on empty field
            return;
        }
        let is_folder = self
            .list
            .item(evt_data.on_list_view_item_index().0, 2, 260)
            .expect("Invalid column clicked")
            .text
            .eq("Directory");
        //ignoring the fact that it might be already added because it doesnt cause any trouble
        self.context_menu_items.add_to_favorites.set_enabled(is_folder);
        let mut cursor_pos: POINT = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut cursor_pos);
        }
        self.context_menu.popup(cursor_pos.x, cursor_pos.y);
        self.context_menu_context_row
            .set(evt_data.on_list_view_item_index().0);
    }

    fn execute_menu_item_click(&self, item: &nwg::ControlHandle, app: Rc<BasicApp>) {
        if item.eq(&self.context_menu_items.copy_path) {
            self.execute_copy_path();
        } else if item.eq(&self.context_menu_items.add_to_favorites) {
            self.execute_add_to_favorites(app.clone());
        } //else it doesnt belong here
    }
}

impl Menuable for FavoriteDirSidebar {
    fn open_menu(&self, evt_data: &nwg::EventData) {
        let (row, _col) = evt_data.on_list_view_item_index();
        if row >= self.list.len() {
            //Clicked on empty field
            return;
        }
        let mut cursor_pos: POINT = POINT { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut cursor_pos);
        }
        self.context_menu.popup(cursor_pos.x, cursor_pos.y);
        self.context_menu_context_row
            .set(evt_data.on_list_view_item_index().0);
    }

    fn execute_menu_item_click(&self, item: &nwg::ControlHandle, _: Rc<BasicApp>) {
        if item.eq(&self.context_menu_items.copy_path) {
            self.execute_copy_path();
        } else if item.eq(&self.context_menu_items.remove) {
            self.execute_remove();
        } //else it doesnt belong here
    }
}
