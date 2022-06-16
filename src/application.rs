use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Debug;
use std::io;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::Terminal;

use crate::file_item_list::Kinds;
use crate::load_config::{
    load_user_config_file, simgle_mapping_crossterm_keycode_to_commands,
    string_map_to_user_keyboad, SettingTheme, UserConfig, UserKeyboad,
};
use crate::path_process::pathbuf_to_string_name;
use crate::state::StatefulDirectory;
use crate::ui::ui;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Input,
    Stacker,
}

// TODO: Restrictions without reason, so think cost
const DRAIN_SIZE: usize = 50;
const MAX_HIST_SIZE: usize = 500;

// TODO: Do I have to load  use config in this struct?
#[derive(Debug)]
pub struct App {
    directory_tabs: Vec<String>,
    tab_index: usize,
    dir_map: HashMap<String, StatefulDirectory>,
    command_history: Vec<String>,
    mode: Mode,
    config: UserConfig,
}

impl App {
    pub fn new() -> Self {
        App {
            directory_tabs: Vec::new(),
            tab_index: 0,
            dir_map: HashMap::new(),
            command_history: Vec::new(),
            mode: Mode::Normal,
            config: load_user_config_file(),
        }
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    // The current directory should be selected, so that tab and hashmap must existe.
    pub fn peek_selected_statefuldir(&mut self) -> &mut StatefulDirectory {
        let selected_tab = self.directory_tabs.get(self.tab_index).unwrap();
        self.dir_map.get_mut(selected_tab).unwrap()
    }

    pub fn tab_index(&self) -> usize {
        self.tab_index
    }

    pub fn dirtab(&self) -> &Vec<String> {
        &self.directory_tabs
    }

    pub fn insert_new_statefuldir(&mut self, dir_path: PathBuf) {
        let dir_name = pathbuf_to_string_name(&dir_path);
        if let Entry::Vacant(item) = self.dir_map.entry(dir_name) {
            let mut new_stateful_dir = StatefulDirectory::new(dir_path);

            // Sorted by name in each of the files and directories
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

    pub fn move_to_next_file_item(&mut self) {
        self.peek_selected_statefuldir().select_next();
    }

    pub fn move_to_prev_file_item(&mut self) {
        self.peek_selected_statefuldir().select_previous();
    }

    pub fn prev_dirtab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.directory_tabs.len() - 1;
        }
    }

    pub fn limit_command_log(&mut self) {
        // TODO: Think about Max limit
        if self.command_history.len() > MAX_HIST_SIZE {
            self.command_history.drain(0..DRAIN_SIZE);
        }
    }

    pub fn push_command_keycode_log(&mut self, command: &KeyCode) {
        self.limit_command_log();
        let cmm = format!("{:?}", command);
        self.command_history.push(cmm);
    }

    pub fn push_command_error_log(&mut self, command: String) {
        self.limit_command_log();
        self.command_history.push(command);
    }

    pub fn _pop_command_log(&mut self) -> Option<String> {
        self.command_history.pop()
    }

    pub fn command_history(&self) -> Vec<String> {
        self.command_history.clone()
    }

    pub fn theme(&self) -> &SettingTheme {
        self.config.theme()
    }

    pub fn symbols(&self, item: &crate::load_config::FileItems) -> String {
        let config = &self.config;
        config.symbols().get(item).unwrap().to_owned()
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

    // If it's the first time you go to the parent directory, selects the top one.
    // The second and subsequent times, select the directory you were in before the move.
    // I want to select the directory before moving from the beginning.
    pub fn move_to_parent_dir(&mut self) {
        let selected_dir = self.peek_selected_statefuldir();
        let dir_name = selected_dir.crr_dir_name();
        let parent_path = selected_dir.crr_dir_parent_path().clone();
        let parent_dir_name = pathbuf_to_string_name(&parent_path);
        self.insert_new_statefuldir(parent_path);
        let i = self.tab_index;
        let name = self.directory_tabs.get_mut(i).unwrap();
        *name = parent_dir_name;

        // select the position of crr dir name or select top
        let dir_pos = self
            .peek_selected_statefuldir()
            .file_items_vec()
            .iter()
            // .inspect(|x| println!("{:?}", x.name() == dir_name))
            .position(|x| x.name() == dir_name);

        let state_dir = self.peek_selected_statefuldir();
        state_dir.select_index(dir_pos)
    }

    fn user_keymapings(&self) -> HashMap<UserKeyboad, String> {
        let keymap = self.config.keybindings_map();
        string_map_to_user_keyboad(keymap)
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let keymap = app.user_keymapings();
    loop {
        // let selected_dir = app.peek_selected_statefuldir();
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            let key_code = simgle_mapping_crossterm_keycode_to_commands(&key);
            match keymap.get(&key_code) {
                Some(cmd) => {
                    match cmd.as_str() {
                        "move_to_parent_dir" => app.move_to_parent_dir(),
                        "move_to_next_file_item" => app.move_to_next_file_item(),
                        "move_to_prev_file_item" => app.move_to_prev_file_item(),
                        "move_to_child_dir" => app.move_to_child_dir(),
                        "next_dirtab" => app.next_dirtab(),
                        "prev_dirtab" => app.prev_dirtab(),
                        "quit" => return Ok(()),
                        _ => app.push_command_error_log("No Commands".to_string()),
                    }
                    app.push_command_keycode_log(&key.code);
                }
                None => app.push_command_error_log("None".to_string()),
            }
        }
    }
}
