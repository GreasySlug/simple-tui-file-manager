use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute,
};
use std::io::{self, Stdout};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Style;
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;
use tui::Terminal;

use crate::load_config::SettingTheme;

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
    while let Event::Key(KeyEvent { code, .. }) = read().expect("Failed to get user input") {
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

    if is_valid_file_name(line) {
        line.clear();
    }
    Ok(())
}

// https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.InvalidData
// TODO: more comprehensive
const CHARS_CANNOT_BE_USED: [char; 10] = ['\\', '|', '<', '>', '*', ':', '?', '\"', '\'', '/'];
fn is_valid_file_name(line: &str) -> bool {
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

pub fn input_area_ui<B: Backend>(f: &mut Frame<B>, line: &str, input_style: Style) {
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(90)])
        .split(f.size())[0];

    f.render_widget(Clear, input_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(input_style);

    let para = Paragraph::new(line).block(block);
    f.render_widget(para, input_area);
}

#[cfg(test)]
mod test {
    use crate::ui::input_ui::is_valid_file_name;

    #[test]
    fn check_invalid_file_name() {
        let char_file_names = ["*", "\\", "|", ".", ":", "<", ">", " ", "/"];
        for name in char_file_names.into_iter() {
            assert!(!is_valid_file_name(name));
        }

        let file_names = [
            "file*", "*name", "name|", "|name", "name.", ".name", "name:", ":name", "name<",
            "<name", "name>", ">name", "", " name", "name ",
        ];
        for name in file_names.into_iter() {
            assert!(!is_valid_file_name(name));
        }
    }
}
