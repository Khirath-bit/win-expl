use nwg::{ListViewExFlags, ListViewStyle, NwgError, ListViewFlags};

use crate::app::BasicApp;

pub fn load(data: &mut BasicApp) -> Result<(), NwgError> {
    nwg::ListView::builder()
        .parent(&data.window)
        .size((100, 540))
        .list_style(ListViewStyle::Detailed)
        .flags(ListViewFlags::NO_HEADER |ListViewFlags::VISIBLE)
        .ex_flags(ListViewExFlags::FULL_ROW_SELECT)
        .position((10, 50))
        .build(&mut data.body.directory_sidebar)?;

    //Add into with path %UserProfile%
    data.body.directory_sidebar.insert_column(nwg::InsertListViewColumn {
        index: Some(0),
        fmt: None,
        width: Some(100),
        text: Some("Name".into()),
    });

    data.body.directory_sidebar.insert_column(nwg::InsertListViewColumn {
        index: Some(1),
        fmt: None,
        width: Some(0),
        text: Some("Path".into()),
    });

    let mut bor_cache = data.cache.settings.borrow_mut();

    if bor_cache.favorite_folders.is_empty() {
        let user_profile_path = std::env::var("UserProfile").unwrap();
        bor_cache.add_favorite_folder("Desktop".into(), user_profile_path.clone() + "\\Desktop");
        bor_cache.add_favorite_folder("Downloads".into(), user_profile_path.clone() + "\\Downloads");
    }

    for (i, folder) in bor_cache.favorite_folders.iter().enumerate() {
        data.body.directory_sidebar.insert_items_row(Some(i as i32), &[folder.name.clone(), folder.path.clone()]);
    }

    nwg::ListView::builder()
        .parent(&data.window)
        .position((120, 50))
        .list_style(ListViewStyle::Detailed)
        .ex_flags(ListViewExFlags::FULL_ROW_SELECT)
        .size((1070, 540))
        .background_color([128, 128, 128])
        .build(&mut data.body.results)?;

    data.body
        .results
        .insert_column(nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(1070 / 2),
            text: Some("Name".into()),
        });

    data.body
        .results
        .insert_column(nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(1070 / 2 / 3),
            text: Some("Date modified".into()),
        });

    data.body
        .results
        .insert_column(nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(1070 / 2 / 3),
            text: Some("Type".into()),
        });

    data.body
        .results
        .insert_column(nwg::InsertListViewColumn {
            index: Some(3),
            fmt: None,
            width: Some(1070 / 2 / 3),
            text: Some("Size".into()),
        });

    data.body
        .results
        .insert_column(nwg::InsertListViewColumn {
            index: Some(4),
            fmt: None,
            width: Some(0),
            text: Some("FULLPATH".into()),
        });

    data.body.results.set_headers_enabled(true);

    nwg::Menu::builder()
        .popup(true)
        .parent(&data.window)
        .build(&mut data.body.item_context_menu)
        .unwrap();

    nwg::MenuItem::builder()
        .parent(&data.body.item_context_menu)
        .text("Copy path")
        .build(&mut data.body.item_context_menu_copy)
        .unwrap();
    nwg::MenuItem::builder()
        .parent(&data.body.item_context_menu)
        .text("Add to favorite")
        .build(&mut data.body.item_context_menu_add_to_fav)
        .unwrap();

    nwg::MenuItem::builder()
        .parent(&data.body.item_context_menu)
        .text("Remove as favorite")
        .build(&mut data.body.item_context_menu_remove_as_fav)
        .unwrap();

    Ok(())
}
