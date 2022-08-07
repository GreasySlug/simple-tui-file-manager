use tui::{
    backend::Backend,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Clear, Paragraph},
    Frame,
};

pub struct Infobox {
    title: String,
    contents: Vec<String>,
    popup_toggle: bool,
}

impl Infobox {
    pub fn init() -> Self {
        Self {
            title: String::new(),
            contents: Vec::new(),
            popup_toggle: false,
        }
    }

    pub fn set_info(mut self, title: String, contents: Vec<String>) -> Self {
        self.title = title;
        self.contents = contents;
        self
    }

    pub fn turned_on(mut self) -> Self {
        self.popup_toggle = true;
        self
    }

    pub fn turned_off(mut self) -> Self {
        self.popup_toggle = false;
        self
    }

    pub fn render_popup<B: Backend>(self, f: &mut Frame<B>, area: Rect, style: Style) -> Self {
        f.render_widget(Clear, area);
        let text = self.contents.join("\n");
        let para = Paragraph::new(text).block(
            Block::default()
                .border_type(BorderType::Rounded)
                .border_style(style)
                .title(&*self.title),
        );
        f.render_widget(para, area);
        self
    }
}
