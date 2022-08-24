use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::application::Mode;

const INDICATOR_BLOCKS: [&str; 7] = [" ", "▁", "▂", "▃", "▄", "▅", "▆"];
pub fn command_ui<B: Backend>(
    f: &mut Frame<B>,
    cmd_hist: &[String],
    cmd_window: Rect,
    cmd_styles: [Style; 4],
    cmd_mode: &Mode,
    rate: f32,
) {
    let cmd_style = match cmd_mode {
        Mode::Normal => cmd_styles[0],
        Mode::Input => cmd_styles[1],
        Mode::Stacker => cmd_styles[2],
        Mode::Searcher => cmd_styles[3],
    };
    let cmd_background = Block::default()
        .style(cmd_style)
        .borders(Borders::ALL)
        .border_type(BorderType::Double);
    f.render_widget(cmd_background, cmd_window);

    let block = Block::default();

    let cmd_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(20),
            Constraint::Percentage(60),
            Constraint::Min(20),
        ])
        .margin(1)
        .split(cmd_window);

    let index = rate * INDICATOR_BLOCKS.len() as f32 - 1.0;
    let index = if index.lt(&0.0) { 0 } else { index as usize };
    let uni_block = INDICATOR_BLOCKS[index];
    let right_block = format!("{}% : {}", (rate * 100_f32) as usize, uni_block);
    let uni_block = Paragraph::new(right_block)
        .block(block.clone())
        .alignment(Alignment::Right);
    f.render_widget(uni_block, cmd_layout[2]);

    if let Some(cmd) = cmd_hist.last() {
        let para = Paragraph::new(cmd.clone())
            .block(block.clone())
            .style(cmd_style);
        f.render_widget(para, cmd_layout[1]);
    } else {
        f.render_widget(Block::default().style(cmd_style), cmd_layout[1]);
    }

    let mode_str = match cmd_mode {
        Mode::Normal => "Normal",
        Mode::Input => "Input",
        Mode::Stacker => "Stacker",
        Mode::Searcher => "searcher",
    };

    let mode_str = Paragraph::new(mode_str)
        .block(block)
        .alignment(Alignment::Left);
    f.render_widget(mode_str, cmd_layout[0]);
}
