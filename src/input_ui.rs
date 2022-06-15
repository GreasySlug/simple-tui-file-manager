use std::io;

use crossterm::event::{read, Event, KeyEvent};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::keymapping::input_keybindings;

pub fn input_ui<B: Backend>(f: &mut Frame<B>, rect: Rect) -> io::Result<String> {
    let mut line = String::with_capacity(40);
    let input_style = Style::default().bg(Color::LightYellow).bg(Color::Black);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(input_style);

    f.render_widget(block.clone(), rect);
    while let Event::Key(KeyEvent { code, .. }) = read()? {
        line = input_keybindings(code, line);
        let para = Paragraph::new(line.clone()).block(block.clone());
        f.render_widget(para, rect)
    }
    Ok(line)
}
