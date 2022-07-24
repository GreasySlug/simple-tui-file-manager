use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute,
};
use std::io::{stdout, Result, Stdout};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::{load_config::SettingTheme, ui::input_ui::input_area_ui};

#[inline]
pub fn init_input_area_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = stdout();
    execute!(stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

const FILE_LENGTH: usize = 40;
pub fn start_user_input(line: &mut String, theme: &SettingTheme, name: &str) -> Result<()> {
    let mut terminal = init_input_area_terminal().expect("Failed to make input terminal...");
    let input_style = theme.input_command_style(); // input color index is 1
    let mut index = 0;
    terminal.draw(|f| input_area_ui(f, line, input_style, index, name))?;
    while let Event::Key(KeyEvent { code, .. }) = read()? {
        match code {
            KeyCode::Enter => break,
            KeyCode::Char(c) => {
                if line.len() < FILE_LENGTH {
                    line.insert(index as usize, c);
                    index += 1;
                }
            }
            KeyCode::Backspace => {
                if index > 0 {
                    index -= 1;
                    line.remove(index as usize);
                }
            }
            KeyCode::Left => {
                if index > 0 {
                    index -= 1;
                }
            }
            KeyCode::Right => {
                if index < line.len() as u16 {
                    index += 1;
                }
            }
            KeyCode::Esc => {
                line.clear();
                break;
            }
            _ => {}
        }
        terminal.draw(|f| input_area_ui(f, line, input_style, index, name))?;
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

#[cfg(test)]
mod test {
    use crate::input::is_invalid_file_name;

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
