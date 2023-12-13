use nwg::EventData;
use std::rc::Rc;
use time::Instant;
use winapi::um::winuser::{self};
use crate::components::menuable::Menuable;

use crate::{
    app::BasicAppUi,
    search_engine::{SearchEngine, parameter_parser::SearchEngineParameter},
    win::key_codes::VirtualKeyCode
};
use clipboard::{ClipboardContext, ClipboardProvider};

pub fn handle_events(ui: &mut BasicAppUi) {
    use nwg::Event as E;
    let evt_ui = Rc::downgrade(&ui.inner);
    let handle_events = move |evt, evt_data: EventData, handle| {
        if let Some(app) = evt_ui.upgrade() {
            match evt {
                E::OnButtonClick => {
                    if handle == app.header.refresh_btn {
                        app.header.refresh_btn.set_text("");
                        //TODO: error handling
                        //not required because setting the text triggers an event app.result_list.refresh(SearchEngine::default().search("", &app.path_bar.get_path().unwrap()))
                    } else if handle == app.header.last_page_btn {
                        //Only triggers when enabled
                        app.header.path_bar.move_one_up();
                        //triggers event
                        app.header.search_input.set_text("");
                    } else if handle == app.header.copy_path_btn {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(app.header.path_bar.get_path().unwrap()).unwrap();
                    }
                }
                E::OnWindowClose => {
                    if handle == app.window {
                        nwg::stop_thread_dispatch();
                    }
                }
                E::OnTextInput => {
                    if handle == app.header.search_input {
                        let txt = app.header.search_input.text();
                        let pth = app.header.path_bar.get_path();
                        if let Err(e) = pth {
                            app.display_error(e);
                        } else {
                            let now = Instant::now();
                            let term = SearchEngineParameter::parse_search_term(&txt);
                            if term.is_err() {
                                return;
                            }
                            let res = SearchEngine::search(&term.unwrap(), &pth.unwrap(), 0);
                            if res.is_err() {
                                return; //Search term invalid. TODO: MAYBE inform user, but most likely search term isnt completed yet
                            }
                            app.cache.current_results.replace(res.clone().unwrap());
                            app.search_results.refresh(res.unwrap());
                            let elapsed = now.elapsed();
                            app.status_bar
                                .search_duration
                                .set_text(&format!("{}ms", elapsed.whole_milliseconds()));
                        }
                    }
                }
                E::OnListViewColumnClick => {
                    if handle == app.search_results.list{
                        app.search_results.sort_by_column(
                            &mut app.cache.result_sort_direction.borrow_mut(),
                            evt_data.on_list_view_item_index().1,
                            app.cache.current_results.borrow_mut().to_vec(),
                        );
                    }
                }
                E::OnListViewClick => {
                    if handle == app.fav_dir_bar.list {
                        let (row, _col) = evt_data.on_list_view_item_index();
                        if row >= app.fav_dir_bar.list.len() {
                            //Clicked on empty field
                            return;
                        }
                        let path = app.fav_dir_bar.list.item(row, 1, 260).unwrap().text;
                        app.header.path_bar.move_into_directory(path);
                        //triggers event
                        app.header.search_input.set_text("");
                    }
                }
                E::OnListViewDoubleClick => {
                    if handle == app.search_results.list {
                        let (row, _col) = evt_data.on_list_view_item_index();
                        if row >= app.search_results.list.len() {
                            //Clicked on empty field
                            return;
                        }
                        let file_type = app.search_results.list.item(row, 2, 10).unwrap().text;
                        let res = app.search_results.list.item(row, 4, 260).unwrap();
                        if !file_type.eq("Directory") {
                            let _ = open::that(&res.text);
                            return;
                        }

                        let path = res.text;

                        app.header.path_bar.move_into_directory(path);
                        //triggers event
                        app.header.search_input.set_text("");
                    }
                }
                E::OnKeyPress => {
                    //VKRETURN = Enter https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
                    if handle == app.header.path_bar.view
                        && evt_data.on_key() == VirtualKeyCode::VK_RETURN as u32
                    {
                        //triggers event
                        app.header.search_input.set_text("");
                    }
                }
                E::OnListViewRightClick => {
                    if handle == app.search_results.list {
                        app.search_results.open_menu(&evt_data);
                    } else if handle == app.fav_dir_bar.list {
                        app.fav_dir_bar.open_menu(&evt_data);
                    }                    
                }
                E::OnMenuItemSelected => {
                    app.search_results.execute_menu_item_click(&handle, Rc::clone(&app));
                    app.fav_dir_bar.execute_menu_item_click(&handle, Rc::clone(&app));
                }
                E::OnMouseWheel => {
                    if handle == app.search_results.list {
                        app.search_results.add_page(&app.cache);
                        app.status_bar
                                .result_count
                                .set_text(&format!("{} results", app.search_results.list.len()));
                    }
                }
                E::OnListViewItemInsert => {
                    if handle == app.search_results.list {
                        app.status_bar
                                .result_count
                                .set_text(&format!("{} results", app.search_results.list.len()));
                    }
                }
                _ => {}
            }

            //TODO optimize to only update if an event was called that can change the depth. This way it fires a million times
            if app.header.path_bar.depth() > 0 {
                unsafe {
                    winuser::EnableWindow(app.header.last_page_btn.handle.hwnd().unwrap(), 1);
                }
            } else {
                unsafe {
                    winuser::EnableWindow(app.header.last_page_btn.handle.hwnd().unwrap(), 0);
                }
            }
        }
    };

    *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
        &ui.inner.window.handle,
        handle_events,
    ));
}
