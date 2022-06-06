use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use file_item_list::{directory_item::Directory, file_item::FileItem, Kinds};
use path_process::{
    get_current_dir_path, get_home_directory_path, make_info_files_from_dirpath,
    pathbuf_to_string_name,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    io,
    path::PathBuf,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Tabs},
    Frame, Terminal,
};

mod file_item_list;
mod path_process;

#[derive(Debug, Clone)]
struct StatefulDirectory {
    directory: Directory,
    file_items: Vec<FileItem>,
    length: usize,
    state: TableState,
}

impl StatefulDirectory {
    fn new(dir_path: PathBuf) -> StatefulDirectory {
        let file_item = make_info_files_from_dirpath(&dir_path);
        StatefulDirectory {
            directory: Directory::new(dir_path),
            length: file_item.len(),
            file_items: file_item,
            state: TableState::default(),
        }
    }

    fn select_top(&mut self) {
        if self.length < 1 {
            return;
        }
        self.state.select(Some(0));
    }

    fn select_next(&mut self) {
        if self.length < 1 {
            return;
        }
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

    fn select_previous(&mut self) {
        if self.length < 1 {
            return;
        }
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

    fn is_selected(&self) -> bool {
        self.state.selected().is_some()
    }

    fn sort_by_kinds(&mut self) {
        self.file_items
            .sort_by(|a, b| b.kinds().partial_cmp(&a.kinds()).unwrap());
    }
}

#[derive(Debug)]
struct App {
    directory_tabs: Vec<String>,
    tab_index: usize,
    dir_map: HashMap<String, StatefulDirectory>,
}

impl App {
    pub fn new() -> Self {
        App {
            directory_tabs: Vec::new(),
            tab_index: 0,
            dir_map: HashMap::new(),
        }
    }

    // The current directory should be selected, so that tab and hashmap must be existed.
    pub fn get_selected_statefuldir(&mut self) -> &mut StatefulDirectory {
        let selected_tab = self.directory_tabs.get(self.tab_index).unwrap();
        self.dir_map.get_mut(selected_tab).unwrap()
    }

    pub fn get_tab_index(&self) -> usize {
        self.tab_index
    }

    pub fn get_list_of_dir_tabs(&self) -> Vec<String> {
        self.directory_tabs.clone()
    }

    pub fn insert_new_statefuldir(&mut self, dir_path: PathBuf) {
        let dir_name = pathbuf_to_string_name(&dir_path);
        if let Entry::Vacant(item) = self.dir_map.entry(dir_name.clone()) {
            let mut new_dir = StatefulDirectory::new(dir_path);
            new_dir.sort_by_kinds();
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

    let crr_dir_path = get_current_dir_path();
    let mut app = App::new();
    app.insert_new_statefuldir(crr_dir_path);

    let home_dir_path = get_home_directory_path();
    if let Some(path) = home_dir_path {
        app.insert_new_statefuldir(path);
    }

    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    terminal.clear()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        let index = app.get_tab_index();
        let tabs = app.get_list_of_dir_tabs();
        let selected_dir = app.get_selected_statefuldir();
        terminal.draw(|f| ui(f, selected_dir, tabs, index))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') | KeyCode::Down => selected_dir.select_next(),
                KeyCode::Char('k') | KeyCode::Up => selected_dir.select_previous(),
                KeyCode::Tab => app.next_tab(),
                KeyCode::BackTab => app.prev_tab(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, dir: &mut StatefulDirectory, tabs: Vec<String>, index: usize) {
    // TODO: use config file
    let ayu_white = Color::Rgb(250, 250, 250);
    let ayu_yellow = Color::Rgb(231, 197, 71);
    let ayu_cyan = Color::Rgb(54, 163, 217);
    let ayu_perple = Color::Rgb(163, 122, 204);
    let ayu_red = Color::Rgb(255, 51, 51);
    let ayu_orange = Color::Rgb(255, 106, 0);
    let ayu_gray = Color::Rgb(217, 216, 216);
    let ayu_darkgray = Color::Rgb(92, 103, 115);
    let dir_symbol = "▸";
    let file_symbol = " ";
    let selecting_symbol = ">>";
    let file_style = Style::default().fg(ayu_darkgray);
    let dir_style = Style::default().fg(ayu_cyan);
    let selecting_style = Style::default().fg(ayu_yellow);
    let header_style = Style::default().fg(ayu_perple).add_modifier(Modifier::BOLD);

    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let background_style = Block::default().style(Style::default().bg(ayu_white).fg(Color::Black));
    f.render_widget(background_style, size);

    let tab_titles = tabs
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(index)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(tabs, chunks[0]);

    // TODO: Display and hide the header and each element with bool
    let header = ["", "name", "permission", "size", "date"]
        .iter()
        .map(|h| Cell::from(*h).style(header_style));
    let header = Row::new(header)
        .style(file_style)
        .height(1)
        .bottom_margin(1);

    let file_items_list = dir.file_items.iter().map(|file_item| {
        let name = file_item.name();
        let perm = if file_item.get_permission() {
            "r"
        } else {
            "wr"
        };
        let size = file_item.get_file_item_size();
        let date = file_item.get_created_date_and_time();
        let lines = if file_item.kinds() == Kinds::Directory {
            vec![
                Span::raw(dir_symbol),
                Span::styled(name, dir_style),
                Span::raw(perm),
                Span::raw(size),
                Span::raw(date),
            ]
        } else {
            vec![
                Span::raw(file_symbol),
                Span::styled(name, file_style),
                Span::raw(perm),
                Span::raw(size),
                Span::raw(date),
            ]
        };
        Row::new(lines)
    });

    let current_dir_path = dir.directory.get_path().to_str().unwrap().to_string();
    let items = Table::new(file_items_list)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(current_dir_path),
        )
        .highlight_style(
            Style::default()
                .patch(selecting_style)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(selecting_symbol)
        .widths(&[
            Constraint::Length(5),
            Constraint::Percentage(30),
            Constraint::Length(5),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ]);

    f.render_stateful_widget(items, chunks[1], &mut dir.state);
}
