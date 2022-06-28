use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Debug;
use std::io;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyEvent};
use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Terminal;

use crate::file_item_list::file_item::FileItem;
use crate::file_item_list::Kinds;

use crate::load_config::{
    load_user_config_file, multi_string_map_to_user_keyboad, SettingTheme, UserConfig, UserKeybinds,
};
use crate::path_process::{join_to_crr_dir, pathbuf_to_string_name};
use crate::state::StatefulDirectory;
use crate::ui::input_ui::start_user_input;
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

pub struct StackerVec {
    state: ListState,
    stack: Vec<PathBuf>,
    length: usize,
}

impl StackerVec {
    fn new() -> StackerVec {
        StackerVec {
            state: ListState::default(),
            stack: Vec::new(),
            length: 0,
        }
    }

    pub fn stacker(&self) -> &Vec<PathBuf> {
        &self.stack
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn next_stacker_item(&mut self) {
        if self.length < 1 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.stack.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_stacker_item(&mut self) {
        if self.length < 1 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.stack.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_top(&mut self) {
        if self.length < 1 {
            return;
        }
        self.state.select(Some(0));
    }
}

// TODO: Do I have to load  use config in this struct?
pub struct App {
    directory_tabs: Vec<String>,
    tab_index: usize,
    dir_map: HashMap<String, StatefulDirectory>,
    command_history: Vec<String>,
    stacker: StackerVec,
    mode: Mode,
    config: UserConfig,
    be_cleaned: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            directory_tabs: Vec::new(),
            tab_index: 0,
            dir_map: HashMap::new(),
            command_history: Vec::new(),
            stacker: StackerVec::new(),
            mode: Mode::Normal,
            config: load_user_config_file(),
            be_cleaned: false,
        }
    }

    fn be_clear(&mut self) {
        self.be_cleaned = true;
    }

    fn be_cleaned(&mut self) {
        self.be_cleaned = false;
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn stacker_items(&mut self) -> &mut StackerVec {
        &mut self.stacker
    }

    // The current directory should be selected, so that tab and hashmap must existe.
    pub fn selected_statefuldir_mut(&mut self) -> &mut StatefulDirectory {
        let selected_tab = self.directory_tabs.get(self.tab_index).unwrap();
        self.dir_map.get_mut(selected_tab).unwrap()
    }

    pub fn selected_statefuldir_ref(&self) -> &StatefulDirectory {
        let selected_tab = self.directory_tabs.get(self.tab_index).unwrap();
        self.dir_map.get(selected_tab).unwrap()
    }

    pub fn crr_dir_path(&self) -> &std::path::Path {
        let selected_stateful_dir = self.selected_statefuldir_ref();
        selected_stateful_dir.dir_path()
    }

    pub fn crr_file_items(&self) -> &Vec<FileItem> {
        let stateful_dir = self.selected_statefuldir_ref();
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
        self.selected_statefuldir_mut().select_next();
    }

    pub fn move_to_prev_file_item(&mut self) {
        self.selected_statefuldir_mut().select_previous();
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

    pub fn push_command_log(&mut self, command: &str) {
        self.limit_command_log();
        self.command_history.push(command.to_string());
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
        let select_dir = self.selected_statefuldir_mut();
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
        let selected_dir = self.selected_statefuldir_mut();
        let dir_name = selected_dir.crr_dir_name();
        let parent_path = selected_dir.crr_dir_parent_path().clone();
        let parent_dir_name = pathbuf_to_string_name(&parent_path);
        self.insert_new_statefuldir(parent_path);
        let i = self.tab_index;
        let name = self.directory_tabs.get_mut(i).unwrap();
        *name = parent_dir_name;

        // select the position of crr dir name or select top
        let dir_pos = self
            .selected_statefuldir_mut()
            .file_items_vec()
            .iter()
            // .inspect(|x| println!("{:?}", x.name() == dir_name))
            .position(|x| x.name() == dir_name);

        let state_dir = self.selected_statefuldir_mut();
        state_dir.select_index(dir_pos);
    }

    fn move_to_top_of_file_item(&mut self) {
        self.selected_statefuldir_mut().select_top();
    }

    fn move_to_bottom_of_file_item(&mut self) {
        self.selected_statefuldir_mut().select_bottom();
    }

    fn make_directory(&mut self) {
        let relpath = self.run_user_input().expect("Failed to make teraminal...");
        // TODO: replpathが空やディレクトリ名に使えない文字がある時には早期リターン
        let path = join_to_crr_dir(self, &relpath);

        if path.is_dir() {
            self.push_command_log("The directory already exists");
            return;
        }

        let res = std::fs::create_dir_all(&path);
        match res {
            Ok(()) => {
                let index = self.selected_statefuldir_ref().state_table().selected();

                // TODO: newで作り直すのはコストが高い時がある(個数が多い時Homeなど)
                //  → 追加したディレクトリだけFileItem化してディレクトリに追加する
                let mut updated = StatefulDirectory::new(self.crr_dir_path().to_path_buf());
                updated.sort_by_kinds();
                updated.select_index(index);
                *self.selected_statefuldir_mut() = updated;
            }
            Err(error) => {
                let message = match error.kind() {
                    io::ErrorKind::PermissionDenied => "Permission denied",
                    io::ErrorKind::NotFound => "Not Found",
                    _ => unreachable!(),
                };
                self.push_command_log(message);
            }
        }
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
        // let mut terminal = init_input_area_terminal().unwrap();
        let mut name = String::with_capacity(MAX_FILE_NAME_SIZE);
        if let Ok(()) = start_user_input(&mut name, self.theme()) {
            self.be_clear();
            if name.is_empty() {
                return None;
            }
            return Some(name);
        }
        self.be_clear();
        self.push_command_log("Stopped to input");
        None
    }

    fn stacker_push_back(&mut self, path: PathBuf) {
        self.stacker.stack.push(path);
    }

    fn stacker_pop_back(&mut self) {
        self.stacker.stack.pop();
    }

    pub fn stacker_contains(&self, path: &PathBuf) -> bool {
        self.stacker.stack.contains(path)
    }

    fn stacker_remove_match_to_path(&mut self, path: &PathBuf) {
        if let Some(i) = self.stacker.stack.iter().position(|p| p == path) {
            self.stacker.stack.remove(i);
        }
    }

    fn select_crr_file_item(&mut self) {
        let dir = self.selected_statefuldir_ref();
        let dir_state = dir.state_table();
        if let Some(i) = dir_state.selected() {
            if let Some(item) = dir.file_items_vec().get(i) {
                let path = item.path().to_path_buf();
                self.stacker_push_back(path);
            }
        }
    }

    fn unselect_crr_file_item(&mut self) {
        let dir = self.selected_statefuldir_ref();
        let dir_state = dir.state_table();
        if let Some(i) = dir_state.selected() {
            if let Some(item) = dir.file_items_vec().get(i) {
                let path = item.path().to_path_buf();
                if self.stacker_contains(&path) {
                    self.stacker_remove_match_to_path(&path);
                }
            }
        }
    }

    fn select_all_file_items(&mut self) {
        let dir = self.selected_statefuldir_mut();
        for item in dir.file_items_vec().to_owned() {
            let path = item.path().to_path_buf();
            self.stacker_push_back(path);
        }
    }

    fn next_stacker_item(&mut self) {
        self.stacker.next_stacker_item();
    }

    fn previous_stacker_item(&mut self) {
        self.stacker.previous_stacker_item();
    }
}

// ユーザーからの入力を受け取りコマンドかどうかを判断
// モードごとに異なるコマンドを受け付けることが可能
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
                    if cmd == "quit" {
                        return Ok(());
                    }
                    run_commands(&mut app, &cmd, &key);
                }
            } else if app.mode() == &Mode::Input {
                if let Ok(cmd) = key_matchings(key, &mut multi_input) {
                    if cmd == "quit" {
                        return Ok(());
                    }
                    run_commands(&mut app, &cmd, &key);
                }
            } else if app.mode() == &Mode::Stacker {
                if let Ok(cmd) = key_matchings(key, &mut multi_stacker) {
                    if cmd == "quit" {
                        return Ok(());
                    }
                    run_commands(&mut app, &cmd, &key);
                }
            }
        }
        if app.be_cleaned {
            terminal.clear()?;
            app.be_cleaned();
        }
    }
}

fn key_matchings(key: KeyEvent, keybinds: &mut UserKeybinds) -> io::Result<String> {
    keybinds.set_keyevent(key);
    // コンボがないキーでのマッチング
    if let Some(cmd) = keybinds.matching_single_keys() {
        return Ok(cmd);
    }

    // keyコンボがあるキーのみのフィルタターを作成
    keybinds.filtering_multi_first_keys();
    // コンボがあるキーでのマッチング
    if keybinds.has_keycomb() {
        if let Event::Key(second) = event::read()? {
            keybinds.set_keyevent(second);
            if let Some(cmd) = keybinds.matching_multi_second_keys() {
                return Ok(cmd);
            }
        }
    }

    Ok(String::with_capacity(0))
}

fn run_commands(app: &mut App, cmd: &str, key: &KeyEvent) {
    match cmd {
        // comman commands
        "move_to_parent_dir" => app.move_to_parent_dir(),
        "move_to_next_file_item" => app.move_to_next_file_item(),
        "move_to_prev_file_item" => app.move_to_prev_file_item(),
        "move_to_child_dir" => app.move_to_child_dir(),
        "move_to_top_of_file_item" => app.move_to_top_of_file_item(),
        "move_to_bottom_of_file_item" => app.move_to_bottom_of_file_item(),
        "next_dirtab" => app.next_dirtab(),
        "prev_dirtab" => app.prev_dirtab(),
        "normal" => app.shift_to_normal_mode(),
        "input" => app.shift_to_input_mode(),
        "stacker" => app.shift_to_stacker_mode(),
        // normal commands

        // input commands
        "make_directory" => app.make_directory(),

        // stacker commands
        "select_current_file_item" => app.select_crr_file_item(),
        "unselect_current_file_item" => app.unselect_crr_file_item(),
        "select_all_file_items" => app.select_all_file_items(),
        "next_stacker_file_item" => app.next_stacker_item(),
        "prev_stacker_file_item" => app.previous_stacker_item(),
        "stacker_pop_back" => app.stacker_pop_back(),
        _ => app.push_command_log(&format!("{:?} {:?}", key.code, key.modifiers)),
    }
}
