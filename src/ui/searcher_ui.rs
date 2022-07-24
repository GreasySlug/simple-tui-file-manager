use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::Frame;

use crate::load_config::SettingTheme;
use crate::{
    application::App, file_item_list::Kinds, load_config::FileItems,
    path_process::pathbuf_to_string_name,
};

use super::{
    directory_ui, FILE_LENGTH, ICON_LENGTH, INFO_LENGTH, MARGIN_LENGTH, NEW_HEADER_TITLES,
    PERMISION_LENGTH,
};

pub fn ui<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    directory_window: Rect,
    themes: &SettingTheme,
) {
    let header_style = app.theme().header_style();
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

    let current_dir_path = pathbuf_to_string_name(app.selecting_dir_path());

    let file_style = app.theme().file_style();
    let dir_style = app.theme().dir_style();

    let directory_window = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(directory_window);

    let dir = app.selecting_dir_file_items().clone();
    let file_item_list = app.make_seacher_vector(dir);
    app.new_regex();
    if let Some(re) = app.regex_ref() {
        let dir_block_style = themes.boader_style();
        let selecting_style = themes
            .select_style()
            .patch(Style::default().add_modifier(Modifier::BOLD));
        let file_item_list = file_item_list.iter().flat_map(|file_item| {
            if let Some((s, r)) = file_item.find(re) {
                let perm = if file_item.get_permission() {
                    format!("{:>4}", "r")
                } else {
                    format!("{:>4}", "rx")
                };
                let size = file_item.get_file_item_size();
                let date = file_item.get_created_date_and_time();
                let mut lines = vec![
                    Cell::from(perm),
                    Cell::from(size),
                    Cell::from(date),
                    Cell::from(" "),
                ];

                if file_item.kinds() == Kinds::Directory(true)
                    || file_item.kinds() == Kinds::Directory(false)
                {
                    lines.push(Cell::from(Span::styled(&dir_symbol, dir_style)));
                } else {
                    lines.push(Cell::from(Span::styled(&file_symbol, file_style)));
                };

                let name = {
                    let m = Span::styled(s, selecting_style);
                    let name = file_item.name();
                    let s = Span::raw(name[..r.start].to_owned());
                    let e = Span::raw(name[r.end..].to_owned());
                    Spans::from(vec![s, m, e])
                };

                lines.push(Cell::from(name));
                Some(Row::new(lines))
            } else {
                None
            }
        });
        let items = Table::new(file_item_list)
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

        f.render_stateful_widget(items, directory_window[0], app.searcher_state());
    } else {
        directory_ui(f, app, directory_window[0], themes);
    }
}
