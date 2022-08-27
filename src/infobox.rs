use std::io::{stdout, Result, Stdout};

use crossterm::execute;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};

pub struct Infobox<'a> {
    title: &'a str,
    contents: Vec<&'a str>,
}

impl<'a> Infobox<'a> {
    pub fn init() -> Self {
        Self {
            title: "",
            contents: Vec::new(),
        }
    }

    pub fn set_info(mut self, title: &'a str, contents: Vec<&'a str>) -> Self {
        self.title = title;
        self.contents = contents;
        self
    }

    pub fn create_popup(self, popup: Style) {
        const WIDTH: u16 = 30;
        const HEIGHT: u16 = 5;
        let mut terminal = init_area_terminal().expect("Failed to create popup window");
        let r = terminal.size().expect("Failed to get terminal size");
        let area = centered_rect(WIDTH, HEIGHT, r);
        terminal
            .draw(|f| self.render_popup(f, area, popup))
            .expect("Failed to render popup window");
    }

    fn render_popup<B: Backend>(self, f: &mut Frame<B>, area: Rect, popup: Style) {
        f.render_widget(Clear, area);
        let text = self.contents.join("\n");
        let para = Paragraph::new(text)
            .alignment(tui::layout::Alignment::Center)
            .block(
                Block::default()
                    .style(popup)
                    .borders(Borders::ALL)
                    .title(self.title),
            );
        f.render_widget(para, area);
    }
}

fn init_area_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = stdout();
    execute!(stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Length(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Length(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
