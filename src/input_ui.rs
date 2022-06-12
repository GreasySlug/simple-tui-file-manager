use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn input_ui<B: Backend>(f: &mut Frame<B>, rect: Rect) -> io::Result<String> {
    let mut line = String::with_capacity(40);
    let input_style = Style::default().bg(Color::LightYellow).bg(Color::Black);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(input_style);
    f.render_widget(block.clone(), rect);
    while let Event::Key(KeyEvent { code, .. }) = read()? {
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
        let para = Paragraph::new(line.clone()).block(block.clone());
        f.render_widget(para, rect)
    }
    Ok(line)
}
