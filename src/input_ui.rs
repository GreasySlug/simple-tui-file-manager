use std::io;

use crossterm::{event::{read, Event, KeyCode, KeyEvent}, execute};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

pub fn init_input_area_terminal() -> io::Result<DefaultTerminal> {
    let mut stdout = io::stdout();
    execute!(stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = ratatui::Terminal::new(backend)?;
    Ok(terminal)
}

pub fn start_user_input(terminal: &mut DefaultTerminal, line: &mut String) -> io::Result<()> {
    terminal.draw(|f| input_area_ui(f, line))?;
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
        terminal.draw(|f| input_area_ui(f, line))?;
    }
    Ok(())
}

pub fn input_area_ui(f: &mut Frame, line: &str) {
    let input_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Percentage(90)])
        .split(f.area())[0];

    let input_style = Style::default().bg(Color::LightYellow).bg(Color::White);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(input_style);

    let para = Paragraph::new(line).block(block.clone());
    f.render_widget(para, input_area);
}
