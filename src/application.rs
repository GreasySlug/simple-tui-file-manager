use std::collections::{hash_map::Entry, HashMap};
use std::io;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::Terminal;

use crate::file_item_list::Kinds;
use crate::path_process::pathbuf_to_string_name;
use crate::state::StatefulDirectory;
use crate::ui::ui;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Input,
    Stacker,
}

#[derive(Debug)]
pub struct App {
    directory_tabs: Vec<String>,
    tab_index: usize,
    dir_map: HashMap<String, StatefulDirectory>,
    command_history: Vec<String>,
    mode: Mode,
}

impl App {
    pub fn new() -> Self {
        App {
            directory_tabs: Vec::new(),
            tab_index: 0,
            dir_map: HashMap::new(),
            command_history: Vec::new(),
            mode: Mode::Normal,
        }
    }

    // The current directory should be selected, so that tab and hashmap must existe.
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
        if let Entry::Vacant(item) = self.dir_map.entry(dir_name) {
            let mut new_stateful_dir = StatefulDirectory::new(dir_path);

            new_stateful_dir.sort_by_kinds();

            if !new_stateful_dir.is_selected() {
                new_stateful_dir.select_top();
            }
            item.insert(new_stateful_dir);
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

    pub fn push_command_log(&mut self, command: &KeyCode) {
        let cmm = format!("{:?}", command);
        self.command_history.push(cmm)
    }

    pub fn _pop_command_log(&mut self) -> Option<String> {
        self.command_history.pop()
    }

    pub fn command_history(&self) -> Vec<String> {
        self.command_history.clone()
    }

    pub fn move_to_child_dir(&mut self) {
        let select_dir = self.peek_selected_statefuldir();
        if let Some(file_item) = select_dir.selecting_file_item() {
            match Kinds::classifiy_kinds(file_item.path(), file_item.meta()) {
                Kinds::Directory(_) => {
                    let dir_name = pathbuf_to_string_name(file_item.path());
                    let new_dir_path = file_item.path().to_path_buf();
                    self.insert_new_statefuldir(new_dir_path);
                    let i = self.tab_index;
                    let name = self.directory_tabs.get_mut(i);
                    *name.unwrap() = dir_name;
                }
                Kinds::File(_) => {}
            }
        }
    }

    pub fn move_to_parent_dir(&mut self) {
        let selected_dir = self.peek_selected_statefuldir();
        let parent_path = selected_dir.crr_dir_parent_path().clone();
        let parent_dir_name = pathbuf_to_string_name(&parent_path);
        self.insert_new_statefuldir(parent_path);
        let i = self.tab_index;
        let name = self.directory_tabs.get_mut(i).unwrap();
        *name = parent_dir_name;
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        let mode = app.mode;
        let index = app.get_dirtab_index();
        let tabs = app.get_list_of_dirtab();
        let commands_history = app.command_history();
        let selected_dir = app.peek_selected_statefuldir();
        terminal.draw(|f| ui(f, selected_dir, tabs, index, commands_history, mode))?;
        if let Event::Key(key) = event::read()? {
            match (mode, key.code) {
                (Mode::Normal, KeyCode::Char('q')) => return Ok(()),
                (Mode::Normal, KeyCode::Char('j') | KeyCode::Down) => selected_dir.select_next(),
                (Mode::Normal, KeyCode::Char('k') | KeyCode::Up) => selected_dir.select_previous(),
                (Mode::Normal, KeyCode::Char('h') | KeyCode::Left) => app.move_to_parent_dir(),
                (Mode::Normal, KeyCode::Char('l') | KeyCode::Right) => app.move_to_child_dir(),
                (Mode::Normal, KeyCode::Tab) => app.next_dirtab(),
                (Mode::Normal, KeyCode::BackTab) => app.prev_dirtab(),
                (_, _) => {}
            }
            app.push_command_log(&key.code);
        }
    }
}
