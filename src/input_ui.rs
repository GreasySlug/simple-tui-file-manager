use std::io::{self, Stdout};

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent},
    execute,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};

use crate::load_config::SettingTheme;

pub fn init_input_area_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    execute!(stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn start_user_input<B: Backend>(
    terminal: &mut Terminal<B>,
    line: &mut String,
    theme: &SettingTheme,
) -> io::Result<()> {
    let input_style = theme.command_style()[1];
    terminal.draw(|f| input_area_ui(f, line, input_style))?;
    while let Event::Key(KeyEvent { code, .. }) = read().expect("Failed to get user input") {
        match code {
            KeyCode::Enter => break,
            KeyCode::Char(c) => line.push(c),
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
    Ok(())
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
