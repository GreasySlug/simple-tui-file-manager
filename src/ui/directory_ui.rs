use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::{
    application::App,
    file_item_list::Kinds,
    load_config::{FileItems, SettingTheme},
};

use super::{
    FILE_LENGTH, HEADER_TITLES, ICON_LENGTH, INFO_LENGTH, MARGIN_LENGTH, PERMISION_LENGTH,
};

pub fn directory_ui<B: Backend>(
    f: &mut Frame<B>,
    app: &App,
    directory_window: Rect,
    themes: &SettingTheme,
) {
    // TODO: Display and hide the header and each element with bool
    let header_style = themes.header_style();
    let header_titles = HEADER_TITLES
        .iter()
        .map(|h| Cell::from(*h).style(header_style));

    let header_constraints = [
        Constraint::Length(PERMISION_LENGTH), // permission
        Constraint::Length(INFO_LENGTH),      // size
        Constraint::Length(INFO_LENGTH),      // date
        Constraint::Length(MARGIN_LENGTH),    // margin
        Constraint::Length(ICON_LENGTH),      // file item's icon
        Constraint::Length(FILE_LENGTH),      // file name
    ];
    let header_cells = Row::new(header_titles).style(header_style).bottom_margin(1);

    let file_symbol = app.symbols(&FileItems::File);
    let dir_symbol = app.symbols(&FileItems::Directory);

    let file_items = app.selecting_dir_file_items();

    let file_style = themes.file_style();
    let dir_style = themes.dir_style();
    let file_items_list = file_items.iter().map(|file_item| {
        let name = file_item.name();
        let perm = if file_item.get_permission() {
            format!("{:>4}", "r")
        } else {
            format!("{:>4}", "rx")
        };
        let size = file_item.get_file_item_size();
        let date = file_item.get_created_date_and_time();
        let mut lines = vec![
            Span::raw(perm),
            Span::raw(size),
            Span::raw(date),
            Span::raw(" "),
        ];

        if file_item.kinds() == Kinds::Directory(true)
            || file_item.kinds() == Kinds::Directory(false)
        {
            lines.push(Span::styled(&dir_symbol, dir_style));
            lines.push(Span::styled(name, dir_style));
        } else {
            lines.push(Span::styled(&file_symbol, file_style));
            lines.push(Span::styled(name, file_style));
        };

        Row::new(lines)
    });

    let directory_window = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(directory_window);

    let dir_block_style = themes.boader_style();
    let selecting_style = themes.select_style().add_modifier(Modifier::BOLD);
    let select_symbol = app.symbols(&FileItems::Select);
    let dir_path = app.selecting_dirtab().1.as_str();
    let items = Table::new(file_items_list)
        .header(header_cells)
        .widths(&header_constraints)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(dir_path)
                .style(dir_block_style),
        )
        .highlight_style(selecting_style)
        .highlight_symbol(&select_symbol);

    let dir = app.selecting_statefuldir_ref();
    f.render_stateful_widget(items, directory_window[0], &mut dir.state_table());
}
