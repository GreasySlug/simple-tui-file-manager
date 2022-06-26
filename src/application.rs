use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Debug;
use std::io;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::Terminal;

use crate::file_item_list::file_item::FileItem;
use crate::file_item_list::Kinds;
use crate::input_ui::{init_input_area_terminal, start_user_input};
use crate::load_config::{
    load_user_config_file, multi_string_map_to_user_keyboad, SettingTheme, UserConfig, UserKeyCode,
    UserKeybinds,
};
use crate::path_process::pathbuf_to_string_name;
use crate::state::StatefulDirectory;
use crate::ui::ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Input,
    Stacker,
}

// TODO: Restrictions without reason, so think cost
const DRAIN_SIZE: usize = 50;
const MAX_HIST_SIZE: usize = 500;
const MAX_FILE_NAME_SIZE: usize = 40;

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

    pub fn peeking_selected_statefuldir(&self) -> &StatefulDirectory {
        let selected_tab = self.directory_tabs.get(self.tab_index).unwrap();
        self.dir_map.get(selected_tab).unwrap()
    }

    pub fn crr_dir_path(&self) -> &std::path::Path {
        let selected_stateful_dir = self.peeking_selected_statefuldir();
        selected_stateful_dir.dir_path()
    }

    pub fn crr_file_items(&self) -> &Vec<FileItem> {
        let stateful_dir = self.peeking_selected_statefuldir();
        stateful_dir.file_items_vec()
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

    pub fn shift_to_input_mode(&mut self) {
        self.mode = Mode::Input;
    }

    pub fn shift_to_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn shift_to_stacker_mode(&mut self) {
        self.mode = Mode::Stacker;
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

    pub fn push_command_log(&mut self, command: String) {
        self.limit_command_log();
        self.command_history.push(command);
    }

    pub fn command_history(&self) -> &Vec<String> {
        &self.command_history
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
        state_dir.select_index(dir_pos);
    }

    fn move_to_top_of_file_item(&mut self) {
        self.peek_selected_statefuldir().select_top();
    }

    fn move_to_bottom_of_file_item(&mut self) {
        self.peek_selected_statefuldir().select_bottom();
    }

    fn normal_user_keybinds(&self) -> UserKeybinds {
        let keybind = self.config.normal_keybindings_map();
        let keymap = multi_string_map_to_user_keyboad(&keybind);
        UserKeybinds::new()
            .make_single_keybinds(keymap.clone())
            .make_multiple_keybinds(keymap)
    }

    fn input_user_keybinds(&self) -> UserKeybinds {
        let keybind = self.config.input_keybindings_map();
        let keymap = multi_string_map_to_user_keyboad(&keybind);
        UserKeybinds::new()
            .make_single_keybinds(keymap.clone())
            .make_multiple_keybinds(keymap)
    }

    fn stacker_user_keybinds(&self) -> UserKeybinds {
        let keybind = self.config.stacker_keybindings_map();
        let keymap = multi_string_map_to_user_keyboad(&keybind);
        UserKeybinds::new()
            .make_single_keybinds(keymap.clone())
            .make_multiple_keybinds(keymap)
    }

    fn run_user_input(&mut self) -> Option<String> {
        let mut terminal = init_input_area_terminal().unwrap();
        let mut name = String::with_capacity(MAX_FILE_NAME_SIZE);
        if let Ok(()) = start_user_input(&mut terminal, &mut name) {
            return Some(name);
        }
        None
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut multi_normal = app.normal_user_keybinds();
    let mut multi_input = app.input_user_keybinds();
    let mut multi_stacker = app.stacker_user_keybinds();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // TODO: Consider a more efficient way to declare the name of each command.
        if let Event::Key(key) = event::read()? {
            if app.mode() == &Mode::Normal {
                if let Ok(cmd) = key_matchings(key, &mut multi_normal) {
                    match cmd.as_str() {
                        "move_to_parent_dir" => app.move_to_parent_dir(),
                        "move_to_next_file_item" => app.move_to_next_file_item(),
                        "move_to_prev_file_item" => app.move_to_prev_file_item(),
                        "move_to_child_dir" => app.move_to_child_dir(),
                        "move_to_top_of_file_item" => app.move_to_top_of_file_item(),
                        "move_to_bottom_of_file_item" => app.move_to_bottom_of_file_item(),
                        "next_dirtab" => app.next_dirtab(),
                        "prev_dirtab" => app.prev_dirtab(),
                        "quit" => return Ok(()),
                        "input" => app.shift_to_input_mode(),
                        _ => app.push_command_log(format!("{:?}", key.code)),
                    }
                }
            } else if app.mode() == &Mode::Input {
                if let Ok(cmd) = key_matchings(key, &mut multi_input) {
                    match cmd.as_str() {
                        "next_dirtab" => app.next_dirtab(),
                        "prev_dirtab" => app.prev_dirtab(),
                        "quit" => return Ok(()),
                        "normal" => app.shift_to_normal_mode(),
                        "stacker" => app.shift_to_stacker_mode(),
                        _ => app.push_command_log("No Comands".to_string()),
                    }
                }
            } else if app.mode() == &Mode::Stacker {
                if let Ok(cmd) = key_matchings(key, &mut multi_stacker) {
                    match cmd.as_str() {
                        "next_dirtab" => app.next_dirtab(),
                        "prev_dirtab" => app.prev_dirtab(),
                        "quit" => return Ok(()),
                        "normal" => app.shift_to_normal_mode(),
                        "input" => app.shift_to_input_mode(),
                        _ => app.push_command_log("No Comands".to_string()),
                    }
                }
            }
        }
    }
}

fn key_matchings(first: KeyEvent, keybinds: &mut UserKeybinds) -> io::Result<String> {
    let keybind = keybinds.set_keyevent_first(first);
    if let Some(cmd) = keybind.matching_single_keys() {
        return Ok(cmd);
    }
    if let Some(filtered_keybinds) = keybind.filtering_multi_first_keys() {
        if let Event::Key(second) = event::read()? {
            if let Some(cmd) = keybind
                .set_keyevent_first(second)
                .matching_multi_second_keys()
            {
                return Ok(cmd);
            }
        }
    }

    Ok(String::with_capacity(0))
}
