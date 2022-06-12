use crossterm::event::{KeyCode, KeyEvent};

use crate::application::{App, Mode};

pub fn input_keybindings(code: KeyCode, mut line: String) -> String {
    match code {
        KeyCode::Enter => return line.to_owned(),
        KeyCode::Char(c) => line.push(c),
        KeyCode::Backspace => {
            line.pop();
        }
        KeyCode::Esc => {
            line.clear();
            return line;
        }
        _ => {}
    }
    line
}

pub fn main_keybindings(key: KeyEvent, mode: Mode, app: &mut App) -> bool {
    match (mode, key.code) {
        (Mode::Normal, KeyCode::Char('q')) => return true,
        (Mode::Normal, KeyCode::Char('j') | KeyCode::Down) => {
            let selected_dir = app.peek_selected_statefuldir();
            selected_dir.select_next();
        }
        (Mode::Normal, KeyCode::Char('k') | KeyCode::Up) => {
            let selected_dir = app.peek_selected_statefuldir();
            selected_dir.select_previous();
        }
        (Mode::Normal, KeyCode::Char('h') | KeyCode::Left) => app.move_to_parent_dir(),
        (Mode::Normal, KeyCode::Char('l') | KeyCode::Right) => app.move_to_child_dir(),
        (Mode::Normal, KeyCode::Tab) => app.next_dirtab(),
        (Mode::Normal, KeyCode::BackTab) => app.prev_dirtab(),
        (_, _) => {}
    }

    false
}
