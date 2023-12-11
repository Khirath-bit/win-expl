use nwg::EventData;
use std::rc::Rc;
use time::Instant;
use winapi::{
    shared::windef::POINT,
    um::winuser::{self, GetCursorPos},
};

use crate::{
    app::{BasicApp, BasicAppUi},
    search_engine::SearchEngine,
    win::key_codes::VirtualKeyCode,
};
use clipboard::{ClipboardContext, ClipboardProvider};

pub fn handle_events(ui: &mut BasicAppUi) {
    use nwg::Event as E;
    let evt_ui = Rc::downgrade(&ui.inner);
    let handle_events = move |evt, evt_data: EventData, handle| {
        if let Some(app) = evt_ui.upgrade() {
            match evt {
                E::OnButtonClick => {
                    if handle == app.hello_button {
                        BasicApp::say_hello(&app);
                    } else if handle == app.refresh_page_btn {
                        app.search_input.set_text("");
                        //TODO: error handling
                        //not required because setting the text triggers an event app.result_list.refresh(SearchEngine::default().search("", &app.path_bar.get_path().unwrap()))
                    } else if handle == app.last_page_btn {
                        //Only triggers when enabled
                        app.path_bar.move_one_up();
                        //triggers event
                        app.search_input.set_text("");
                    } else if handle == app.copy_path_btn {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(app.path_bar.get_path().unwrap()).unwrap();
                    }
                }
                E::OnWindowClose => {
                    if handle == app.window {
                        nwg::stop_thread_dispatch();
                    }
                }
                E::OnTextInput => {
                    if handle == app.search_input {
                        let txt = app.search_input.text();
                        let pth = app.path_bar.get_path();
                        if let Err(e) = pth {
                            app.display_error(e);
                        } else {
                            let now = Instant::now();
                            let res = SearchEngine::search(&txt, &pth.unwrap(), 0);
                            app.cache.current_results.replace(res.clone());
                            app.result_list.refresh(res);
                            let elapsed = now.elapsed();
                            app.status_bar
                                .result_count
                                .set_text(&format!("{} results", app.result_list.view.len()));
                            app.status_bar
                                .search_duration
                                .set_text(&format!("{}ms", elapsed.whole_milliseconds()));
                        }
                    }
                }
                E::OnListViewColumnClick => {
                    if handle == app.result_list.view {
                        app.result_list.sort_by_column(
                            &mut app.cache.result_sort_direction.borrow_mut(),
                            evt_data.on_list_view_item_index().1,
                            app.cache.current_results.borrow_mut().to_vec(),
                        );
                    }
                }
                E::OnListViewDoubleClick => {
                    if handle == app.result_list.view {
                        let (row, _col) = evt_data.on_list_view_item_index();
                        let file_type = app.result_list.view.item(row, 2, 10).unwrap().text;
                        let res = app.result_list.view.item(row, 4, 260).unwrap();
                        if !file_type.eq("Directory") {
                            let _ = open::that(&res.text);
                            return;
                        }

                        let path = res.text;

                        app.path_bar.move_into_directory(path);
                        //triggers event
                        app.search_input.set_text("");
                    }
                }
                E::OnKeyPress => {
                    //VKRETURN = Enter https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
                    if handle == app.path_bar.view
                        && evt_data.on_key() == VirtualKeyCode::VK_RETURN as u32
                    {
                        //triggers event
                        app.search_input.set_text("");
                    }
                }
                E::OnListViewRightClick => {
                    if handle == app.result_list.view {
                        let mut cursor_pos: POINT = POINT { x: 0, y: 0 };
                        unsafe {
                            GetCursorPos(&mut cursor_pos);
                        }
                        app.result_list
                            .item_context_menu
                            .popup(cursor_pos.x, cursor_pos.y);
                        app.result_list
                            .context_menu_row_index
                            .set(evt_data.on_list_view_item_index().0)
                    }
                }
                E::OnMenuItemSelected => {
                    if handle == app.result_list.item_context_menu_copy {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(
                            app.result_list
                                .view
                                .item(app.result_list.context_menu_row_index.get(), 4, 260)
                                .unwrap()
                                .text,
                        )
                        .unwrap();
                    }
                }
                _ => {}
            }

            //TODO optimize to only update if an event was called that can change the depth. This way it fires a million times
            if app.path_bar.depth() > 0 {
                unsafe {
                    winuser::EnableWindow(app.last_page_btn.handle.hwnd().unwrap(), 1);
                }
            } else {
                unsafe {
                    winuser::EnableWindow(app.last_page_btn.handle.hwnd().unwrap(), 0);
                }
            }
        }
    };

    *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
        &ui.inner.window.handle,
        handle_events,
    ));
}
