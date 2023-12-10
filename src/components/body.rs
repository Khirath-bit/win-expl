use nwg::{NwgError, ListViewStyle, ListViewExFlags};

use crate::app::BasicApp;

pub fn load(data: &mut BasicApp) -> Result<(), NwgError>{
    nwg::Button::builder()
    .text("Say my name")
    .parent(&data.window)
    .size((100, 540))
    .position((10, 50))
    .build(&mut data.hello_button)?;

    nwg::ListView::builder()
    .parent(&data.window)
    .position((120, 50))
    .list_style(ListViewStyle::Detailed)
    .ex_flags(ListViewExFlags::FULL_ROW_SELECT)
    .size((1070, 540))
    .background_color([128, 128, 128])
    .build(&mut data.result_list.view)?;

    data.result_list.view.insert_column(nwg::InsertListViewColumn{
        index: Some(0),
        fmt: None,
        width: Some(1070/2),
        text: Some("Name".into()),
    });

    data.result_list.view.insert_column(nwg::InsertListViewColumn{
        index: Some(1),
        fmt: None,
        width: Some(1070/2/3),
        text: Some("Date modified".into()),
    });

    data.result_list.view.insert_column(nwg::InsertListViewColumn{
        index: Some(2),
        fmt: None,
        width: Some(1070/2/3),
        text: Some("Type".into()),
    });

    data.result_list.view.insert_column(nwg::InsertListViewColumn{
        index: Some(3),
        fmt: None,
        width: Some(1070/2/3),
        text: Some("Size".into()),
    });

    data.result_list.view.insert_column(nwg::InsertListViewColumn {
        index: Some(4),
        fmt: None,
        width: Some(0),
        text: Some("FULLPATH".into()),
    });

    data.result_list.view.set_headers_enabled(true);

    Ok(())
}