use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame,
};

use crate::{file_item_list::Kinds, load_config::UserConfig, StatefulDirectory};

pub fn ui<B: Backend>(
    f: &mut Frame<B>,
    dir: &mut StatefulDirectory,
    tabs: Vec<String>,
    index: usize,
    command_hist: Vec<String>,
) {
    let theme = UserConfig::default_light();
    let file_style = theme.file_style();
    let dir_style = theme.dir_style();
    let selecting_style = theme.select_style();
    let header_style = theme.header_style();
    let background_style = theme.background_style();
    let tab_style = theme.boader_style();
    let dir_block_style = theme.boader_style();
    let tab_highlight_style = theme
        .select_style()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);

    let file_symbol = theme.file_symbol();
    let dir_symbol = theme.dir_symbol();
    let select_symbol = theme.select_symbol();

    let current_dir_path = dir.crr_dir_name();
    let header_titles = ["", "", "name", "permission", "size", "date"]
        .iter()
        .map(|h| Cell::from(*h).style(header_style));

    let header_constraints = [
        Constraint::Length(1),  //  blank space
        Constraint::Length(2),  // file item's icon
        Constraint::Length(20), // file name
        Constraint::Length(10), // permission
        Constraint::Length(10), // size
        Constraint::Length(10), // date
    ];

    let main_windows_constrains = [
        Constraint::Length(3), // tab
        Constraint::Min(0),    // directory
        Constraint::Length(3), // command
    ]
    .as_ref();
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(main_windows_constrains)
        .split(size);

    let background_window = Block::default().style(background_style);
    f.render_widget(background_window, size);

    let tab_titles = tabs
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![Span::raw(first), Span::raw(rest)])
        })
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(index)
        .style(tab_style)
        .highlight_style(tab_highlight_style);

    f.render_widget(tabs, chunks[0]);

    // TODO: Display and hide the header and each element with bool
    let header_cells = Row::new(header_titles).style(header_style).bottom_margin(1);

    let file_item_iter = dir.file_items_vec();
    let file_items_list = file_item_iter.iter().map(|file_item| {
        let name = file_item.name();
        let perm = if file_item.get_permission() {
            format!("{:>4}", "r")
        } else {
            format!("{:>4}", "rx")
        };
        let size = file_item.get_file_item_size();
        let date = file_item.get_created_date_and_time();

        let lines = if file_item.kinds() == Kinds::Directory(true)
            || file_item.kinds() == Kinds::Directory(false)
        {
            vec![
                Span::raw(" "),
                Span::styled(dir_symbol, dir_style),
                Span::styled(name, dir_style),
                Span::raw(perm),
                Span::raw(size),
                Span::raw(date),
            ]
        } else {
            vec![
                Span::raw(" "),
                Span::styled(file_symbol, file_style),
                Span::styled(name, file_style),
                Span::raw(perm),
                Span::raw(size),
                Span::raw(date),
            ]
        };
        Row::new(lines)
    });

    let items = Table::new(file_items_list)
        .header(header_cells)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(dir_block_style)
                .title(current_dir_path),
        )
        .highlight_style(selecting_style)
        .highlight_symbol(select_symbol)
        .widths(&header_constraints);

    f.render_stateful_widget(items, chunks[1], &mut dir.state_table());

    command_display_ui(f, command_hist, chunks[2]);
}

fn command_display_ui<B: Backend>(f: &mut Frame<B>, command_hist: Vec<String>, rect: Rect) {
    let theme = UserConfig::default_light();
    let block_style = theme.command_style();
    let sample_style = tui::style::Style::default().fg(tui::style::Color::Black);
    let block = Block::default()
        .style(block_style)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);
    if command_hist.is_empty() {
        f.render_widget(block, rect);
        return;
    }
    let i = command_hist.len() - 1;
    let last_command: Option<&String> = command_hist.get(i);
    if let Some(cmd) = last_command {
        let para = Paragraph::new(cmd.clone()).block(block);
        f.render_widget(para, rect);
    }
}
