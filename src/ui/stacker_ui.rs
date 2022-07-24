use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, List, ListItem, Row, Table},
    Frame,
};

use crate::{
    application::App,
    file_item_list::Kinds,
    load_config::{FileItems, SettingTheme},
    path_process::pathbuf_to_string_name,
};

use super::{
    FILE_LENGTH, ICON_LENGTH, INFO_LENGTH, MARGIN_LENGTH, NEW_HEADER_TITLES, PERMISION_LENGTH,
};

pub fn stacker_ui<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    directory_window: Rect,
    themes: &SettingTheme,
) {
    let header_style = themes.header_style();
    let header_titles = NEW_HEADER_TITLES
        .iter()
        .map(|h| Cell::from(*h).style(header_style));

    let header_constraints = [
        Constraint::Length(PERMISION_LENGTH), // permission
        Constraint::Length(INFO_LENGTH),      // size
        Constraint::Length(INFO_LENGTH),      // date
        Constraint::Length(MARGIN_LENGTH),    //  margin
        Constraint::Length(ICON_LENGTH),      // file item's icon
        Constraint::Length(FILE_LENGTH),      // file name
    ];
    let header_cells = Row::new(header_titles).style(header_style).bottom_margin(1);

    let file_symbol = app.symbols(&FileItems::File);
    let dir_symbol = app.symbols(&FileItems::Directory);
    let select_symbol = app.symbols(&FileItems::Select);

    let current_dir_path = pathbuf_to_string_name(app.crr_dir_path());
    let file_item_iter = app.crr_file_items();

    let file_style = themes.file_style();
    let dir_style = themes.dir_style();
    let file_items_list = file_item_iter.iter().map(|file_item| {
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
            Span::raw(name),
        ];

        if file_item.kinds() == Kinds::Directory(true)
            || file_item.kinds() == Kinds::Directory(false)
        {
            lines.insert(4, Span::styled(&dir_symbol, dir_style));
        } else {
            lines.insert(4, Span::styled(&file_symbol, file_style));
        };

        if app.stacker_contains(&file_item.path().to_path_buf()) {
            Row::new(lines).style(
                themes.select_style().patch(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC),
                ),
            )
        } else {
            Row::new(lines)
        }
    });

    let directory_window = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(directory_window);

    let dir_block_style = themes.boader_style();
    let selecting_style = themes.select_style().add_modifier(Modifier::BOLD);
    let items = Table::new(file_items_list)
        .header(header_cells)
        .widths(&header_constraints)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(dir_block_style)
                .title(current_dir_path),
        )
        .highlight_style(selecting_style)
        .highlight_symbol(&select_symbol);

    let dir = app.selected_statefuldir_ref();

    let (term_col, _) = crossterm::terminal::size().expect("failed to get terminal size...");
    // Determine just the right size
    let layout = if term_col < 100 {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(directory_window[0])
    } else {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(directory_window[0])
    };
    f.render_stateful_widget(items, layout[0], &mut dir.state_table());

    stacking_item_ui(f, app, layout[1], themes);
}

pub fn stacking_item_ui<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    stack_window: Rect,
    themes: &SettingTheme,
) {
    let select_symbol = app.symbols(&FileItems::Select);
    let select_file_style = themes.file_style();
    let select_style = themes.select_style();
    let dir_block_style = themes.boader_style();
    let items = app.stacker_mut();
    let state_items: Vec<ListItem> = items
        .stack_ref()
        .iter()
        .map(|path| {
            let path_name = pathbuf_to_string_name(path);
            let span = Span::styled(path_name, select_file_style);
            ListItem::new(span)
        })
        .collect();

    let list_item = List::new(state_items)
        .highlight_style(select_style)
        .highlight_symbol(&select_symbol)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(dir_block_style),
        );

    f.render_stateful_widget(list_item, stack_window, items.state_mut())
}
