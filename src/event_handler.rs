use nwg::EventData;
use std::rc::Rc;
use time::Instant;
use winapi::um::winuser::{self};

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
                    if handle == app.refresh_page_btn {
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
                            let term = SearchEngineParameter::parse_search_term(&txt);
                            if term.is_err() {
                                return;
                            }
                            let res = SearchEngine::search(&term.unwrap(), &pth.unwrap(), 0);
                            if res.is_err() {
                                return; //Search term invalid. TODO: MAYBE inform user, but most likely search term isnt completed yet
                            }
                            app.cache.current_results.replace(res.clone().unwrap());
                            app.body.refresh(res.unwrap());
                            let elapsed = now.elapsed();
                            app.status_bar
                                .search_duration
                                .set_text(&format!("{}ms", elapsed.whole_milliseconds()));
                        }
                    }
                }
                E::OnListViewColumnClick => {
                    if handle == app.body.results {
                        app.body.sort_by_column(
                            &mut app.cache.result_sort_direction.borrow_mut(),
                            evt_data.on_list_view_item_index().1,
                            app.cache.current_results.borrow_mut().to_vec(),
                        );
                    }
                }
                E::OnListViewClick => {
                    if handle == app.body.directory_sidebar {
                        let (row, _col) = evt_data.on_list_view_item_index();
                        if row >= app.body.directory_sidebar.len() {
                            //Clicked on empty field
                            return;
                        }
                        let path = app.body.directory_sidebar.item(row, 1, 260).unwrap().text;
                        app.path_bar.move_into_directory(path);
                        //triggers event
                        app.search_input.set_text("");
                    }
                }
                E::OnListViewDoubleClick => {
                    if handle == app.body.results {
                        let (row, _col) = evt_data.on_list_view_item_index();
                        if row >= app.body.results.len() {
                            //Clicked on empty field
                            return;
                        }
                        let file_type = app.body.results.item(row, 2, 10).unwrap().text;
                        let res = app.body.results.item(row, 4, 260).unwrap();
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
                    if handle == app.body.results {
                        app.body.show_context_menu_results(&evt_data);
                    } else if handle == app.body.directory_sidebar {
                        app.body.show_context_menu_sidebar(&evt_data);
                    }
                }
                E::OnMenuItemSelected => {
                    if handle == app.body.item_context_menu_copy {
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        ctx.set_contents(
                            app.body
                                .results
                                .item(app.body.context_menu_row_index.get(), 4, 260)
                                .unwrap()
                                .text,
                        )
                        .unwrap();
                    } else if handle == app.body.item_context_menu_add_to_fav {
                        let row = app.body.context_menu_row_index.get();
                        let path = app.body.results.item(row, 4, 260).expect("invalid menu row").text;
                        let ind = Some(app.body.directory_sidebar.len() as i32);
                        let name = app.body.results.item(row, 0, 260).expect("invalid menu row").text;
                        nwg::ListView::insert_items_row(&app.body.directory_sidebar, ind, &[name.clone(), path.clone()]);
                        app.cache.settings.borrow_mut().add_favorite_folder(name, path);
                    }
                }
                E::OnMouseWheel => {
                    if handle == app.body.results {
                        app.body.add_page(&app.cache);
                        app.status_bar
                                .result_count
                                .set_text(&format!("{} results", app.body.results.len()));
                    }
                }
                E::OnListViewItemInsert => {
                    if handle == app.body.results {
                        app.status_bar
                                .result_count
                                .set_text(&format!("{} results", app.body.results.len()));
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
