use nwg::{
    ButtonFlags, CheckBoxFlags, LabelFlags, ListViewExFlags, ListViewFlags, ListViewStyle,
    NwgError, TextBoxFlags,
};
use time::{OffsetDateTime, format_description};
use winapi::um::winuser::{self, BS_AUTOCHECKBOX, SS_RIGHT};

use crate::{app::BasicApp, resource_manager::ResourceType};

use super::{
    fav_dir_bar::FavoriteDirSidebar, header_control::HeaderControl,
    path_bar_control::PathBarControl, search_result_control::SearchResultControl,
    status_bar_control::StatusBarControl,
};

pub trait Control {
    fn load_components(app: &mut BasicApp) -> Result<(), NwgError>;
}

impl Control for StatusBarControl {
    fn load_components(app: &mut BasicApp) -> Result<(), NwgError> {
        nwg::Label::builder()
            .parent(&app.window)
            .background_color(Some([0x32, 0x32, 0x32]))
            .flags(LabelFlags::ELIPSIS | LabelFlags::VISIBLE)
            .position((10, 600))
            .size((100, 20))
            .build(&mut app.status_bar.result_count)?;

        unsafe {
            nwg::Label::builder()
                .parent(&app.window)
                .background_color(Some([0x32, 0x32, 0x32]))
                .flags(
                    LabelFlags::ELIPSIS
                        | LabelFlags::VISIBLE
                        | LabelFlags::from_bits_unchecked(SS_RIGHT),
                )
                .text("200ms")
                .position((120, 600))
                .size((100, 20))
                .build(&mut app.status_bar.search_duration)?;

            nwg::Label::builder()
                .parent(&app.window)
                .background_color(Some([0x32, 0x32, 0x32]))
                .flags(
                    LabelFlags::ELIPSIS
                        | LabelFlags::VISIBLE
                        | LabelFlags::from_bits_unchecked(SS_RIGHT),
                )
                .position((800, 600))
                .size((200, 20))
                .build(&mut app.status_bar.index_date)?;
        }

        nwg::Button::builder()
            .size((30, 30))
            .position((1010, 595))
            .parent(&app.window)
            .flags(ButtonFlags::ICON | ButtonFlags::VISIBLE)
            .bitmap(Some(
                &app.resource_manager.get_bitmap(ResourceType::Refresh)?,
            ))
            .build(&mut app.status_bar.index_refresh)?;

        unsafe {
            nwg::CheckBox::builder()
                .size((10, 20))
                .position((1050, 600))
                .parent(&app.window)
                .text("")
                .background_color(Some([0x32, 0x32, 0x32]))
                .flags(CheckBoxFlags::VISIBLE | CheckBoxFlags::from_bits_unchecked(BS_AUTOCHECKBOX))
                .focus(true)
                .build(&mut app.status_bar.index_usage)?;
        }

        let index = app.cache.index.get_mut();
        index.check_for_updates();
        let chrono_time: OffsetDateTime = index.modified_date.into();

        let time = chrono_time
            .format(&format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap())
            .unwrap()
            .to_string();

        app.status_bar.index_date.set_text(&time);

        Ok(())
    }
}

impl Control for PathBarControl {
    fn load_components(app: &mut BasicApp) -> Result<(), NwgError> {
        nwg::TextBox::builder()
            .position((120, 10))
            .size((670, 30))
            .text("C:")
            .flags(TextBoxFlags::VISIBLE)
            .parent(&app.window)
            .build(&mut app.header.path_bar.view)?;

        Ok(())
    }
}

impl Control for HeaderControl {
    fn load_components(app: &mut BasicApp) -> Result<(), NwgError> {
        nwg::Button::builder()
            .size((30, 30))
            .position((10, 10))
            .parent(&app.window)
            .flags(ButtonFlags::ICON | ButtonFlags::VISIBLE)
            .bitmap(Some(
                &app.resource_manager.get_bitmap(ResourceType::ArrowLeft)?,
            ))
            .build(&mut app.header.last_page_btn)?;

        nwg::Button::builder()
            .size((30, 30))
            .position((47, 10))
            .parent(&app.window)
            .flags(ButtonFlags::ICON | ButtonFlags::VISIBLE)
            .bitmap(Some(
                &app.resource_manager.get_bitmap(ResourceType::ArrowUp)?,
            ))
            .build(&mut app.header.parent_page_btn)?;
        unsafe {
            winuser::EnableWindow(app.header.last_page_btn.handle.hwnd().unwrap(), 0);
        }

        nwg::Button::builder()
            .size((30, 30))
            .position((80, 10))
            .parent(&app.window)
            .flags(ButtonFlags::ICON | ButtonFlags::VISIBLE)
            .bitmap(Some(
                &app.resource_manager.get_bitmap(ResourceType::Refresh)?,
            ))
            .build(&mut app.header.refresh_btn)?;

        nwg::TextInput::builder()
            .position((900, 10))
            .size((290, 30))
            .parent(&app.window)
            .placeholder_text(Some("Search..."))
            .build(&mut app.header.search_input)?;

        nwg::Button::builder()
            .size((30, 30))
            .position((795, 10))
            .parent(&app.window)
            .flags(ButtonFlags::ICON | ButtonFlags::VISIBLE)
            .bitmap(Some(&app.resource_manager.get_bitmap(ResourceType::Copy)?))
            .build(&mut app.header.copy_path_btn)?;

        //Call here because its part of the header
        PathBarControl::load_components(app)?;

        Ok(())
    }
}

impl Control for SearchResultControl {
    fn load_components(app: &mut BasicApp) -> Result<(), NwgError> {
        nwg::ListView::builder()
            .parent(&app.window)
            .position((120, 50))
            .list_style(ListViewStyle::Detailed)
            .ex_flags(ListViewExFlags::FULL_ROW_SELECT)
            .size((1070, 540))
            .background_color([0x32, 0x32, 0x32])
            .build(&mut app.search_results.list)?;

        app.search_results
            .list
            .insert_column(nwg::InsertListViewColumn {
                index: Some(0),
                fmt: None,
                width: Some(1070 / 2),
                text: Some("Name".into()),
            });

        app.search_results
            .list
            .insert_column(nwg::InsertListViewColumn {
                index: Some(1),
                fmt: None,
                width: Some(1070 / 2 / 3),
                text: Some("Date modified".into()),
            });

        app.search_results
            .list
            .insert_column(nwg::InsertListViewColumn {
                index: Some(2),
                fmt: None,
                width: Some(1070 / 2 / 3),
                text: Some("Type".into()),
            });

        app.search_results
            .list
            .insert_column(nwg::InsertListViewColumn {
                index: Some(3),
                fmt: None,
                width: Some(1070 / 2 / 3),
                text: Some("Size".into()),
            });

        app.search_results
            .list
            .insert_column(nwg::InsertListViewColumn {
                index: Some(4),
                fmt: None,
                width: Some(0),
                text: Some("FULLPATH".into()),
            });

        app.search_results.list.set_headers_enabled(true);

        nwg::Menu::builder()
            .popup(true)
            .parent(&app.window)
            .build(&mut app.search_results.context_menu)
            .unwrap();

        nwg::MenuItem::builder()
            .parent(&app.search_results.context_menu)
            .text("Copy path")
            .build(&mut app.search_results.context_menu_items.copy_path)
            .unwrap();
        nwg::MenuItem::builder()
            .parent(&app.search_results.context_menu)
            .text("Copy name")
            .build(&mut app.search_results.context_menu_items.copy_name)
            .unwrap();

        let mut seperator: nwg::MenuSeparator = Default::default();
        nwg::MenuSeparator::builder()
            .parent(&app.search_results.context_menu)
            .build(&mut seperator)
            .unwrap();
        app.search_results
            .context_menu_items
            .seperator
            .push(seperator);

        nwg::MenuItem::builder()
            .parent(&app.search_results.context_menu)
            .text("Add to favorite")
            .build(&mut app.search_results.context_menu_items.add_to_favorites)
            .unwrap();

        let mut seperator2: nwg::MenuSeparator = Default::default();
        nwg::MenuSeparator::builder()
            .parent(&app.search_results.context_menu)
            .build(&mut seperator2)
            .unwrap();
        app.search_results
            .context_menu_items
            .seperator
            .push(seperator2);

        nwg::MenuItem::builder()
            .parent(&app.search_results.context_menu)
            .text("MD5 hash")
            .build(&mut app.search_results.context_menu_items.md5_hash)
            .unwrap();

        nwg::MenuItem::builder()
            .parent(&app.search_results.context_menu)
            .text("SHA1 hash")
            .build(&mut app.search_results.context_menu_items.sha1_hash)
            .unwrap();
        nwg::MenuItem::builder()
            .parent(&app.search_results.context_menu)
            .text("SHA256 hash")
            .build(&mut app.search_results.context_menu_items.sha256_hash)
            .unwrap();
        nwg::MenuItem::builder()
            .parent(&app.search_results.context_menu)
            .text("SHA512 hash")
            .build(&mut app.search_results.context_menu_items.sha512_hash)
            .unwrap();

        Ok(())
    }
}

impl Control for FavoriteDirSidebar {
    fn load_components(app: &mut BasicApp) -> Result<(), NwgError> {
        nwg::ListView::builder()
            .parent(&app.window)
            .size((100, 540))
            .list_style(ListViewStyle::Detailed)
            .flags(ListViewFlags::NO_HEADER | ListViewFlags::VISIBLE)
            .ex_flags(ListViewExFlags::FULL_ROW_SELECT)
            .position((10, 50))
            .build(&mut app.fav_dir_bar.list)?;

        //Add into with path %UserProfile%
        app.fav_dir_bar
            .list
            .insert_column(nwg::InsertListViewColumn {
                index: Some(0),
                fmt: None,
                width: Some(100),
                text: Some("Name".into()),
            });

        app.fav_dir_bar
            .list
            .insert_column(nwg::InsertListViewColumn {
                index: Some(1),
                fmt: None,
                width: Some(0),
                text: Some("Path".into()),
            });

        let mut bor_cache = app.cache.settings.borrow_mut();

        if bor_cache.favorite_folders.is_empty() {
            let user_profile_path = std::env::var("UserProfile").unwrap();
            bor_cache
                .add_favorite_folder("Desktop".into(), user_profile_path.clone() + "\\Desktop");
            bor_cache.add_favorite_folder(
                "Downloads".into(),
                user_profile_path.clone() + "\\Downloads",
            );
        }

        for (i, folder) in bor_cache.favorite_folders.iter().enumerate() {
            app.fav_dir_bar
                .list
                .insert_items_row(Some(i as i32), &[folder.name.clone(), folder.path.clone()]);
        }

        nwg::Menu::builder()
            .popup(true)
            .parent(&app.window)
            .build(&mut app.fav_dir_bar.context_menu)
            .unwrap();

        nwg::MenuItem::builder()
            .parent(&app.fav_dir_bar.context_menu)
            .text("Remove as favorite")
            .build(&mut app.fav_dir_bar.context_menu_items.remove)
            .unwrap();

        nwg::MenuItem::builder()
            .parent(&app.fav_dir_bar.context_menu)
            .text("Copy path")
            .build(&mut app.fav_dir_bar.context_menu_items.copy_path)
            .unwrap();

        Ok(())
    }
}
