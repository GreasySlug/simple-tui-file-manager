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
    file_item_list::{file_item, Kinds},
    load_config::FileItems,
};

use super::{
    HEADER_TITLES, HEIGHT_OF_UI_FILE_LENGTH, HEIGHT_OF_UI_ICON_LENGTH, HEIGHT_OF_UI_INFO_LENGTH,
    HEIGHT_OF_UI_MARGIN_LENGTH, UI_MIN_PERCENTAGE,
};

pub fn directory_ui<B: Backend>(f: &mut Frame<B>, app: &App, directory_window: Rect) {
    // TODO: Display and hide the header and each element with bool
    let header_style = app.theme().header_style();
    let header_titles = HEADER_TITLES
        .iter()
        .map(|h| Cell::from(*h).style(header_style));

    let header_constraints = [
        Constraint::Length(HEIGHT_OF_UI_MARGIN_LENGTH), //  margin
        Constraint::Length(HEIGHT_OF_UI_ICON_LENGTH),   // file item's icon
        Constraint::Length(HEIGHT_OF_UI_FILE_LENGTH),   // file name
        Constraint::Min(UI_MIN_PERCENTAGE),             // file extension
        Constraint::Min(UI_MIN_PERCENTAGE),             // permission
        Constraint::Length(HEIGHT_OF_UI_INFO_LENGTH),   // size
        Constraint::Length(HEIGHT_OF_UI_INFO_LENGTH),   // date
    ];
    let header_cells = Row::new(header_titles).style(header_style).bottom_margin(1);

    let file_symbol = app.symbols(&FileItems::File);
    let dir_symbol = app.symbols(&FileItems::Directory);
    let select_symbol = app.symbols(&FileItems::Select);

    let current_dir_path = pathbuf_to_string_name(app.crr_dir_path());
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
            Span::raw(" "),
            Span::raw(name),
            Span::raw(perm),
            Span::raw(size),
            Span::raw(date),
        ];

        if file_item.kinds() == Kinds::Directory(true)
            || file_item.kinds() == Kinds::Directory(false)
        {
            lines.insert(1, Span::styled(&dir_symbol, dir_style));
        } else {
            lines.insert(1, Span::styled(&file_symbol, file_style));
        };

        match file_item.extension() {
            Some(ex) => match ex {
                file_item::Extension::C => lines.insert(3, Span::raw("C")),
                file_item::Extension::CPlusPlus => lines.insert(3, Span::raw("C++")),
                file_item::Extension::CSharp => lines.insert(3, Span::raw("C#")),
                file_item::Extension::Go => lines.insert(3, Span::raw("Go")),
                file_item::Extension::Java => lines.insert(3, Span::raw("Java")),
                file_item::Extension::JavaScript => lines.insert(3, Span::raw("JS")),
                file_item::Extension::Markdown => lines.insert(3, Span::raw("MD")),
                file_item::Extension::Rust => lines.insert(3, Span::raw("Rust")),
                file_item::Extension::Ruby => lines.insert(3, Span::raw("Ruby")),
                file_item::Extension::Python => lines.insert(3, Span::raw("Py")),
                file_item::Extension::Perl => lines.insert(3, Span::raw("Perl")),
                file_item::Extension::Toml => lines.insert(3, Span::raw("Toml")),
                file_item::Extension::Unknwon => lines.insert(3, Span::raw("n/a")),
            },
            None => lines.insert(2, Span::raw("N/A")),
        };
        Row::new(lines)
    });

    let directory_window = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(directory_window);

    let dir_block_style = app.theme().boader_style();
    let selecting_style = app.theme().select_style().add_modifier(Modifier::BOLD);
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
    f.render_stateful_widget(items, directory_window[0], &mut dir.state_table());
}
