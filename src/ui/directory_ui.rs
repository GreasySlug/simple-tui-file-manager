use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::{application::App, file_item_list::Kinds, load_config::FileItems};

use super::{
    FILE_LENGTH, ICON_LENGTH, INFO_LENGTH, MARGIN_LENGTH, NEW_HEADER_TITLES, PERMISION_LENGTH,
};

pub fn directory_ui<B: Backend>(f: &mut Frame<B>, app: &App, directory_window: Rect) {
    // TODO: Display and hide the header and each element with bool
    let header_style = app.theme().header_style();
    let header_titles = NEW_HEADER_TITLES
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

    let file_item_iter = app.crr_file_items();

    let file_style = app.theme().file_style();
    let dir_style = app.theme().dir_style();
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

        Row::new(lines)
    });

    let directory_window = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(directory_window);

    let dir_block_style = app.theme().boader_style();
    let selecting_style = app.theme().select_style().add_modifier(Modifier::BOLD);
    let select_symbol = app.symbols(&FileItems::Select);
    let items = Table::new(file_items_list)
        .header(header_cells)
        .widths(&header_constraints)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(dir_block_style),
        )
        .highlight_style(selecting_style)
        .highlight_symbol(&select_symbol);

    let dir = app.selected_statefuldir_ref();
    f.render_stateful_widget(items, directory_window[0], &mut dir.state_table());
}

pub fn matching_directory_ui<B: Backend>(f: &mut Frame<B>, app: &mut App, directory_window: Rect) {
    // TODO: Display and hide the header and each element with bool
    let header_style = app.theme().header_style();
    let header_titles = NEW_HEADER_TITLES
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

    app.new_regex();
    if let Some(rgx) = app.matching_regex() {
        let file_symbol = app.symbols(&FileItems::File);
        let dir_symbol = app.symbols(&FileItems::Directory);
        let file_items = app.crr_file_items();

        let file_style = app.theme().file_style();
        let dir_style = app.theme().dir_style();
        let match_color = app.theme().select_style();
        let file_items_list = file_items.iter().flat_map(|file_item| {
            let res = file_item.find(rgx);
            if let Some((s, r)) = res {
                let margin = Span::raw(" ");
                let symbol = if file_item.kinds() == Kinds::Directory(true)
                    || file_item.kinds() == Kinds::Directory(false)
                {
                    Span::styled(dir_symbol.to_owned(), dir_style)
                } else {
                    Span::styled(file_symbol.to_owned(), file_style)
                };
                let sym = Cell::from(Spans(vec![margin, symbol]));
                let name = file_item.name();
                let lines = {
                    let mtch_str = Span::styled(s, match_color);
                    let (start, _) = name.split_at(r.start);
                    let margin = Span::raw(" ");
                    let (_, end) = name.split_at(r.end);
                    vec![
                        margin,
                        Span::raw(start.to_owned()),
                        mtch_str,
                        Span::raw(end.to_owned()),
                    ]
                };

                let perm = if file_item.get_permission() {
                    format!("{:>4}", "r")
                } else {
                    format!("{:>4}", "rx")
                };
                let perm = Cell::from(Span::raw(perm));
                let size = file_item.get_file_item_size();
                let size = Cell::from(Span::raw(size));
                let date = file_item.get_created_date_and_time();
                let date = Cell::from(Span::raw(date));
                let margin = Cell::default();
                let lines = Cell::from(Spans::from(lines));
                Some(Row::new(vec![perm, size, date, margin, sym, lines]))
            } else {
                None
            }
        });

        let directory_window = Layout::default()
            .constraints([Constraint::Percentage(100)])
            .split(directory_window);

        let dir_block_style = app.theme().boader_style();
        let selecting_style = app.theme().select_style().add_modifier(Modifier::BOLD);
        let select_symbol = app.symbols(&FileItems::Select);
        let items = Table::new(file_items_list)
            .header(header_cells)
            .widths(&header_constraints)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(dir_block_style),
            )
            .highlight_style(selecting_style)
            .highlight_symbol(&select_symbol);

        let dir = app.selected_statefuldir_ref();
        f.render_stateful_widget(items, directory_window[0], &mut dir.state_table());
    } else {
        directory_ui(f, app, directory_window);
    }
}
