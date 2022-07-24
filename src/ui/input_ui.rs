use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;

use super::{HEIGHT_OF_UI_ONE_LINE_LENGTH, UI_MIN_PERCENTAGE};
use crate::{application::App, load_config::SettingTheme};

pub fn input_area_ui<B: Backend>(
    f: &mut Frame<B>,
    line: &str,
    input_style: Style,
    index: u16,
    name: &str,
) {
    const OFFSET: u16 = 1;
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // input area
            Constraint::Min(UI_MIN_PERCENTAGE),               // directory
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // command area
        ])
        .split(f.size());

    let block = Block::default()
        .borders(Borders::ALL)
        .style(input_style)
        .title(name)
        .border_style(input_style);

    f.set_cursor(input_area[0].x + index + OFFSET, input_area[0].y + OFFSET);

    let para = Paragraph::new(line).block(block);
    f.render_widget(para, input_area[0]);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, themes: &SettingTheme) {
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // input
            Constraint::Min(UI_MIN_PERCENTAGE),               // directory
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // command
        ])
        .split(f.size());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(themes.input_command_style());

    f.render_widget(block, input_area[0]);
}
