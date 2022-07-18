use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute,
};
use std::io::{self, Stdout};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Frame;
use tui::Terminal;

use super::{
    directory_ui::{directory_ui, matching_directory_ui},
    HEIGHT_OF_UI_ONE_LINE_LENGTH, UI_MIN_PERCENTAGE,
};
use crate::{application::App, load_config::SettingTheme};

#[inline]
pub fn init_input_area_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    execute!(stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

const FILENAME_CAPACITY: usize = 50;
pub fn start_user_input(line: &mut String, theme: &SettingTheme) -> io::Result<()> {
    let mut terminal = init_input_area_terminal().expect("Failed to make input terminal...");
    let input_style = theme.command_style(1).unwrap(); // input color index is 1
    terminal.draw(|f| input_area_ui(f, line, input_style))?;
    while let Event::Key(KeyEvent { code, .. }) = read()? {
        match code {
            KeyCode::Enter => break,
            KeyCode::Char(c) => {
                if line.len() < FILENAME_CAPACITY {
                    line.push(c);
                }
            }
            KeyCode::Backspace => {
                line.pop();
            }
            KeyCode::Esc => {
                line.clear();
                break;
            }
            _ => {}
        }
        terminal.draw(|f| input_area_ui(f, line, input_style))?;
    }

    if is_invalid_file_name(line) {
        line.clear();
    }
    Ok(())
}

// https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.InvalidData
// TODO: more comprehensive
pub fn is_invalid_file_name(line: &str) -> bool {
    const CHARS_CANNOT_BE_USED: [char; 10] = ['\\', '|', '<', '>', '*', ':', '?', '\"', '\'', '/'];
    for c in CHARS_CANNOT_BE_USED.into_iter() {
        if line.find(c).is_none() {
            return false;
        }
    }
    if line.len() == 1 {
        return !(line.contains('/')
            || line.contains('\'')
            || line.contains('\"')
            || line.contains('.'));
    }

    true
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // tabdir
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // input area
            Constraint::Min(UI_MIN_PERCENTAGE),               // directory
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // command area
        ])
        .split(f.size());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(app.theme().command_style(1).unwrap());

    f.render_widget(block, input_area[1]);

    directory_ui(f, app, input_area[2]);
}

pub fn input_area_ui<B: Backend>(f: &mut Frame<B>, line: &str, input_style: Style) {
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // tabdir
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // input area
            Constraint::Min(UI_MIN_PERCENTAGE),               // directory
            Constraint::Length(HEIGHT_OF_UI_ONE_LINE_LENGTH), // command area
        ])
        .split(f.size());

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(input_style);

    f.set_cursor(input_area[1].x + line.len() as u16 + 1, input_area[1].y + 1);

    let para = Paragraph::new(line).block(block);
    f.render_widget(para, input_area[1]);
}

#[cfg(test)]
mod test {
    use crate::ui::input_ui::is_invalid_file_name;

    #[test]
    fn check_invalid_file_name() {
        let char_file_names = ["*", "\\", "|", ".", ":", "<", ">", " ", "/"];
        for name in char_file_names.into_iter() {
            assert!(!is_invalid_file_name(name));
        }

        let file_names = [
            "file*", "*name", "name|", "|name", "name.", ".name", "name:", ":name", "name<",
            "<name", "name>", ">name", "", " name", "name ",
        ];
        for name in file_names.into_iter() {
            assert!(!is_invalid_file_name(name));
        }
    }
}
