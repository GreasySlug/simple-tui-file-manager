use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use path_process::{
    get_current_dir_path, get_home_directory_path, make_info_files_from_dirpath,
    pathbuf_to_string_name,
};
use state::StatefulDirectory;
use std::{
    collections::{hash_map::Entry, HashMap},
    error::Error,
    io,
    path::PathBuf,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

mod file_item_list;
mod path_process;
mod state;
mod ui;

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
    pub fn peek_selected_statefuldir(&mut self) -> &mut StatefulDirectory {
        let selected_tab = self.directory_tabs.get(self.tab_index).unwrap();
        self.dir_map.get_mut(selected_tab).unwrap()
    }

    pub fn get_dirtab_index(&self) -> usize {
        self.tab_index
    }

    pub fn get_list_of_dirtab(&self) -> Vec<String> {
        self.directory_tabs.clone()
    }

    pub fn insert_new_statefuldir(&mut self, dir_path: PathBuf) {
        let dir_name = pathbuf_to_string_name(&dir_path);
        if let Entry::Vacant(item) = self.dir_map.entry(dir_name.clone()) {
            let mut new_stateful_dir = StatefulDirectory::new(dir_path);
            new_stateful_dir.sort_by_kinds();
            if new_stateful_dir.is_selected() {
                new_stateful_dir.select_top();
            }
            item.insert(new_stateful_dir);
            self.push_new_dirname_to_dirtab(dir_name);
        }
    }

    pub fn push_new_dirname_to_dirtab(&mut self, dir_name: String) {
        if !self.directory_tabs.contains(&dir_name) {
            self.directory_tabs.push(dir_name)
        }
    }

    pub fn next_dirtab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.directory_tabs.len();
    }

    pub fn prev_dirtab(&mut self) {
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
        let index = app.get_dirtab_index();
        let tabs = app.get_list_of_dirtab();
        let selected_dir = app.peek_selected_statefuldir();
        terminal.draw(|f| ui(f, selected_dir, tabs, index))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') | KeyCode::Down => selected_dir.select_next(),
                KeyCode::Char('k') | KeyCode::Up => selected_dir.select_previous(),
                KeyCode::Tab => app.next_dirtab(),
                KeyCode::BackTab => app.prev_dirtab(),
                _ => {}
            }
        }
    }
}
