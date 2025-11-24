use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame,
};

use crate::{
    application::{App, Mode},
    file_item_list::Kinds,
    load_config::FileItems,
    path_process::pathbuf_to_string_name,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let file_style = app.theme().file_style();
    let dir_style = app.theme().dir_style();
    let selecting_style = app.theme().select_style().add_modifier(Modifier::BOLD);
    let header_style = app.theme().header_style();
    let background_style = app.theme().background_style();
    let tab_style = app.theme().border_style();
    let dir_block_style = app.theme().border_style();
    let tab_highlight_style = app.theme().select_style().add_modifier(Modifier::BOLD);

    // possible to toggle tab and command window
    let main_windows_constrains = [
        Constraint::Length(3), // tab
        Constraint::Min(0),    // directory
        Constraint::Length(3), // command
    ]
    .as_ref();
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(main_windows_constrains)
        .split(size);

    let (index, file_items) = {
        let state = app.peek_selected_statefuldir();
        (
            state.state_table().selected().unwrap_or(0),
            state.file_items_vec().len(),
        )
    };

    let rate = index as f32 / file_items as f32;
    command_display_ui(
        f,
        app.command_history(),
        chunks[2],
        app.theme().command_style(),
        app.mode(),
        rate,
    );

    // let index = app.tab_index();
    let tabs = app.dirtab();
    let mode = app.mode();

    let header_titles = ["", "", "name", "permission", "size", "date"]
        .iter()
        .map(|h| Cell::from(*h).style(header_style));

    let header_constraints = [
        Constraint::Length(1),  //  margin
        Constraint::Length(2),  // file item's icon
        Constraint::Length(20), // file name
        Constraint::Length(10), // permission
        Constraint::Length(10), // size
        Constraint::Length(10), // date
    ];

    let background_window = Block::default().style(background_style);
    f.render_widget(background_window, size);

    match mode {
        Mode::Normal => {
            let tab_titles: Vec<Line> = tabs
                .iter()
                .map(|t| Line::from(vec![Span::raw(t)]))
                .collect();

            let tabs = Tabs::new(tab_titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tab_index())
                .style(tab_style)
                .highlight_style(tab_highlight_style);

            f.render_widget(tabs, chunks[0]);
        }
        Mode::Input => {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(background_style);
            f.render_widget(block, chunks[0]);
        }
        Mode::Stacker => todo!(),
    }
    // TODO: Display and hide the header and each element with bool
    let header_cells = Row::new(header_titles).style(header_style).bottom_margin(1);

    let file_symbol = app.symbols(&FileItems::File);
    let dir_symbol = app.symbols(&FileItems::Directory);
    let select_symbol = app.symbols(&FileItems::Select);

    let current_dir_path = pathbuf_to_string_name(app.crr_dir_path());
    let file_item_iter = app.crr_file_items();

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
                Span::styled(&dir_symbol, dir_style),
                Span::styled(name, dir_style),
                Span::raw(perm),
                Span::raw(size),
                Span::raw(date),
            ]
        } else {
            vec![
                Span::raw(" "),
                Span::styled(&file_symbol, file_style),
                Span::styled(name, file_style),
                Span::raw(perm),
                Span::raw(size),
                Span::raw(date),
            ]
        };
        Row::new(lines)
    });

    let directory_window = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(chunks[1]);

    let items = Table::new(file_items_list, header_constraints)
        .header(header_cells)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(dir_block_style)
                .title(current_dir_path),
        )
        .row_highlight_style(selecting_style)
        .highlight_symbol(select_symbol.as_str());

    let dir = app.peek_selected_statefuldir();
    f.render_stateful_widget(items, directory_window[0], &mut dir.state_table());
}

const BLOCK_ELEMENTS: [&str; 7] = [" ", "▁", "▂", "▃", "▄", "▅", "▆"];
fn command_display_ui(
    f: &mut Frame,
    cmd_hist: &[String],
    cmd_window: Rect,
    cmd_styles: [Style; 3],
    cmd_mode: &Mode,
    rate: f32,
) {
    let cmd_style = match cmd_mode {
        Mode::Normal => cmd_styles[0],
        Mode::Input => cmd_styles[1],
        Mode::Stacker => cmd_styles[2],
    };
    let cmd_background = Block::default()
        .style(cmd_style)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);
    f.render_widget(cmd_background, cmd_window);

    let block = Block::default();

    let cmd_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(8),
            Constraint::Percentage(60),
            Constraint::Min(20),
        ])
        .margin(1)
        .split(cmd_window);

    let uni_block = BLOCK_ELEMENTS[(rate * BLOCK_ELEMENTS.len() as f32) as usize];
    let right_block = format!("{}% : {}", (rate * 100_f32) as usize, uni_block);
    let uni_block = Paragraph::new(right_block)
        .block(block.clone())
        .alignment(Alignment::Right);
    f.render_widget(uni_block, cmd_layout[2]);

    if let Some(cmd) = cmd_hist.last() {
        let para = Paragraph::new(cmd.clone())
            .block(block.clone())
            .style(cmd_style);
        f.render_widget(para, cmd_layout[1]);
    } else {
        f.render_widget(Block::default().style(cmd_style), cmd_layout[1]);
    }

    let mode_str = match cmd_mode {
        Mode::Normal => "Normal",
        Mode::Input => "Input",
        Mode::Stacker => "Stacker",
    };

    let mode_str = Paragraph::new(mode_str)
        .block(block)
        .alignment(Alignment::Left);
    f.render_widget(mode_str, cmd_layout[0]);
}
