mod command_ui;
mod directory_ui;
pub mod input_ui;
mod searcher_ui;
mod stacker_ui;

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Modifier;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};
use tui::Frame;

use crate::application::{App, Mode};
use crate::load_config::SettingTheme;

use self::command_ui::command_ui;
use self::directory_ui::directory_ui;
use self::stacker_ui::stacker_ui;

const HEIGHT_OF_UI_ONE_LINE_LENGTH: u16 = 3;
const UI_MIN_PERCENTAGE: u16 = 0;

const INFO_LENGTH: u16 = 10;
const FILE_LENGTH: u16 = 40;
const ICON_LENGTH: u16 = 4;
const MARGIN_LENGTH: u16 = 2;
const PERMISION_LENGTH: u16 = 4;
const NEW_HEADER_TITLES: [&str; 6] = ["perm", "size", "date", "", "", "name"];

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, themes: &SettingTheme) {
    let background_style = themes.background_style();
    let tab_highlight_style = themes.select_style().add_modifier(Modifier::BOLD);

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

    // let index = app.tab_index();
    let tabs = app.dirtab();
    let mode = app.mode();

    let background_window = Block::default().style(background_style);
    f.render_widget(background_window, size);

    let tab_titles: Vec<Spans> = tabs
        .iter()
        .map(|t| Spans::from(vec![Span::raw(t)]))
        .collect();

    // Put above command_ui inside
    match mode {
        Mode::Normal => {
            let tabs = Tabs::new(tab_titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tab_index())
                .style(themes.normal_command_style())
                .highlight_style(tab_highlight_style);

            f.render_widget(tabs, chunks[0]);
            directory_ui(f, app, chunks[1], themes);
        }
        Mode::Input => {
            input_ui::ui(f, app, themes);
            directory_ui(f, app, chunks[1], themes);
        }
        Mode::Stacker => {
            let tabs = Tabs::new(tab_titles)
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .select(app.tab_index())
                .style(themes.stacker_command_style())
                .highlight_style(tab_highlight_style);

            f.render_widget(tabs, chunks[0]);
            stacker_ui(f, app, chunks[1], themes);
        }
        Mode::Searcher => {
            searcher_ui::ui(f, app, chunks[1], themes);
        }
    }

    let (index, file_items) = {
        let state = app.selecting_statefuldir_ref();
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
        themes.command_styles(),
        app.mode(),
        rate,
    );
}
