use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use file_item_list::{directory_item::Directory, file_item::FileItem};
use path_process::{get_current_dir_path, make_dirpath_info_files_vec, pathbuf_to_string_name};
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    io,
    path::PathBuf,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Tabs},
    Frame, Terminal,
};

mod file_item_list;
mod path_process;

#[derive(Debug, Clone)]
struct StatefulDirectory {
    directory: Directory,
    file_items: Vec<FileItem>,
    state: ListState,
}

impl StatefulDirectory {
    fn new(dir_path: PathBuf) -> StatefulDirectory {
        StatefulDirectory {
            directory: Directory::new(dir_path),
            file_items: make_dirpath_info_files_vec(dir_path.clone()),
            state: ListState::default(),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.file_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.file_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

#[derive(Debug)]
struct App {
    directory_tabs: Vec<String>,
    tab_index: usize,
    item_map: HashMap<String, StatefulDirectory>,
}

impl App {
    pub fn new() -> Self {
        App {
            directory_tabs: Vec::new(),
            tab_index: 0,
            item_map: HashMap::new(),
        }
    }

    pub fn insert_new_item(&mut self, dir_path: PathBuf) {
        let dir_name = pathbuf_to_string_name(&dir_path);
        let new_dir = StatefulDirectory::new(dir_path);
        if let Entry::Vacant(item) = self.item_map.entry(dir_name) {
            item.insert(new_dir);
            self.push_new_dir_name(dir_name);
        }
    }

    pub fn push_new_dir_name(&mut self, dir_name: String) {
        if !self.directory_tabs.contains(&dir_name) {
            self.directory_tabs.push(dir_name)
        }
    }

    pub fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.directory_tabs.len();
    }

    pub fn prev_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.directory_tabs.len() - 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let crr_dir_path = get_current_dir_path();
    let mut app = App::new();
    app.insert_new_item(crr_dir_path);
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let background_block =
        Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(background_block, size);

    let titles = app
        .directory_tabs
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tab_index)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(tabs, chunks[0]);

    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .map(|i| {
            let mut lines = vec![Spans::from(i.0)];
            for _ in 0..i.1 {
                lines.push(Spans::from(Span::styled(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
            }
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    match app.tab_index {
        0 => f.render_stateful_widget(items, chunks[0], &mut app.items.state),
        2 => {
            let inner = Block::default().title("Innter02").borders(Borders::ALL);
            f.render_widget(inner, chunks[1]);
        }
        _ => unreachable!(),
    }
}
