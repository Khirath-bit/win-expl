use nwg::{ListViewExFlags, ListViewFlags, ListViewStyle, NwgError, ButtonFlags, TextBoxFlags, LabelFlags};
use winapi::um::winuser::{self, SS_RIGHT};

use crate::{app::BasicApp, resource_manager::ResourceType};

use super::{fav_dir_bar::FavoriteDirSidebar, search_result_control::SearchResultControl, header_control::HeaderControl, path_bar_control::PathBarControl, status_bar_control::StatusBarControl};

pub trait Control {
    fn load_components(app: &mut BasicApp) -> Result<(), NwgError>;
}

impl Control for StatusBarControl {
    fn load_components(app: &mut BasicApp) -> Result<(), NwgError> {
        nwg::Label::builder()
            .parent(&app.window)
            .background_color(Some([50, 50, 50]))
            .flags(LabelFlags::ELIPSIS | LabelFlags::VISIBLE)
            .position((10, 600))
            .size((100, 20))
            .build(&mut app.status_bar.result_count)?;

        unsafe {
            nwg::Label::builder()
                .parent(&app.window)
                .background_color(Some([50, 50, 50]))
                .flags(
                    LabelFlags::ELIPSIS
                        | LabelFlags::VISIBLE
                        | LabelFlags::from_bits_unchecked(SS_RIGHT),
                )
                .text("200ms")
                .position((120, 600))
                .size((100, 20))
                .build(&mut app.status_bar.search_duration)?;
        }

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
            .background_color([128, 128, 128])
            .build(&mut app.search_results.list)?;

        app.search_results.list.insert_column(nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(1070 / 2),
            text: Some("Name".into()),
        });

        app.search_results.list.insert_column(nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(1070 / 2 / 3),
            text: Some("Date modified".into()),
        });

        app.search_results.list.insert_column(nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(1070 / 2 / 3),
            text: Some("Type".into()),
        });

        app.search_results.list.insert_column(nwg::InsertListViewColumn {
            index: Some(3),
            fmt: None,
            width: Some(1070 / 2 / 3),
            text: Some("Size".into()),
        });

        app.search_results.list.insert_column(nwg::InsertListViewColumn {
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
            .text("Add to favorite")
            .build(&mut app.search_results.context_menu_items.add_to_favorites)
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
