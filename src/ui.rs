pub mod command_ui;
pub mod directory_ui;
pub mod input_ui;
pub mod stacker_ui;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Modifier;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};
use tui::Frame;

use crate::application::{App, Mode};

use self::command_ui::command_ui;
use self::directory_ui::directory_ui;
use self::stacker_ui::stacker_ui;

const HEIGHT_OF_UI_ONE_LINE_LENGTH: u16 = 3;
const UI_MIN_PERCENTAGE: u16 = 0;
const HEIGHT_OF_UI_INFO_LENGTH: u16 = 10;
const HEIGHT_OF_UI_FILE_LENGTH: u16 = 10;
const HEIGHT_OF_UI_ICON_LENGTH: u16 = 2;
const HEIGHT_OF_UI_MARGIN_LENGTH: u16 = 1;

const HEADER_TITLES: [&str; 7] = ["", "", "name", "extension", "permission", "size", "date"];

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let background_style = app.theme().background_style();
    let tab_highlight_style = app.theme().select_style().add_modifier(Modifier::BOLD);

    // possible to toggle tab and command window
    let main_windows_constrains = [
        Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // tab
        Constraint::Min(UI_MIN_PERCENTAGE),               // directory
        Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // command
    ]
    .as_ref();
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(main_windows_constrains)
        .split(size);

    let (index, file_items) = {
        let state = app.selected_statefuldir_ref();
        (
            state.state_table().selected().unwrap_or(0),
            state.file_items().len(),
        )
    };

    let rate = index as f32 / file_items as f32;
    command_ui(
        f,
        app.command_history(),
        chunks[2],
        app.theme().command_styles(),
        app.mode(),
        rate,
    );

    // let index = app.tab_index();
    let tabs = app.dirtab();
    let mode = app.mode();

    let background_window = Block::default().style(background_style);
    f.render_widget(background_window, size);

    let tab_titles: Vec<Spans> = tabs
        .iter()
        .map(|t| Spans::from(vec![Span::raw(t)]))
        .collect();
    match mode {
        Mode::Normal => {
            let tabs = Tabs::new(tab_titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tab_index())
                .style(app.theme().command_style(0).unwrap())
                .highlight_style(tab_highlight_style);

            f.render_widget(tabs, chunks[0]);
            directory_ui(f, app, chunks[1]);
        }
        Mode::Input => {
            let tabs = Tabs::new(tab_titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tab_index())
                .style(app.theme().command_style(1).unwrap())
                .highlight_style(tab_highlight_style);

            f.render_widget(tabs, chunks[0]);
            input_ui::ui(f, app);
        }
        Mode::Stacker => {
            let tabs = Tabs::new(tab_titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tab_index())
                .style(app.theme().command_style(2).unwrap())
                .highlight_style(tab_highlight_style);

            f.render_widget(tabs, chunks[0]);
            stacker_ui(f, app, chunks[1]);
        }
    }
}
