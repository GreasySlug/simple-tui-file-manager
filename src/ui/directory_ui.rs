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
    path_process::pathbuf_to_string_name,
};

pub fn directory_ui<B: Backend>(f: &mut Frame<B>, app: &mut App, directory_window: Rect) {
    // TODO: Display and hide the header and each element with bool
    let header_style = app.theme().header_style();
    let header_titles = ["", "", "name", "extension", "permission", "size", "date"]
        .iter()
        .map(|h| Cell::from(*h).style(header_style));

    let header_constraints = [
        Constraint::Length(1),      //  margin
        Constraint::Length(2),      // file item's icon
        Constraint::Percentage(30), // file name
        Constraint::Length(8),      // file extension
        Constraint::Length(10),     // permission
        Constraint::Length(10),     // size
        Constraint::Length(10),     // date
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

    let dir = app.peek_selected_statefuldir();
    f.render_stateful_widget(items, directory_window[0], &mut dir.state_table());
}
