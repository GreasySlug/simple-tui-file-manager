use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crossterm::event::{self, Event};
use tui::backend::Backend;
use tui::widgets::ListState;
use tui::Terminal;

use crate::file_item_list::file_item::FileItem;
use crate::file_item_list::Kinds;

use crate::load_config::{
    load_user_config_file, multi_string_map_to_user_keyboad, SettingTheme, UserConfig, UserKeybinds,
};
use crate::path_process::{join_to_crr_dir, make_a_file_item_from_dirpath, pathbuf_to_string_name};
use crate::stacker::StackerVec;
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

    pub fn stacker_mut(&mut self) -> &mut StackerVec {
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
        stateful_dir.file_items()
    }

    pub fn selecting_crr_file_item(&self) -> Option<&FileItem> {
        let stateful_dir = self.selected_statefuldir_ref();
        if !stateful_dir.is_selected() {
            return None;
        }
        if let Some(i) = stateful_dir.state_table().selected() {
            return self.crr_file_items().get(i);
        }

        None
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
            new_stateful_dir.sort_file_items_by_kinds();

            if !new_stateful_dir.is_selected() {
                new_stateful_dir.select_top_file_item();
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
        self.selected_statefuldir_mut().select_next_file_item();
    }

    pub fn move_to_prev_file_item(&mut self) {
        self.selected_statefuldir_mut().select_previous_file_item();
    }

    pub fn shift_to_input_mode(&mut self) {
        if !self.stacker.stacker_is_empty() {
            self.stacker_clear();
        }
        self.mode = Mode::Input;
    }

    pub fn shift_to_normal_mode(&mut self) {
        if !self.stacker.stacker_is_empty() {
            self.stacker_clear();
        }
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
        if let Some(file_item) = self.selecting_crr_file_item() {
            match Kinds::classifiy_kinds(file_item.path(), file_item.meta()) {
                Kinds::Directory(_) => {
                    let dir_name = file_item.name();
                    let new_dir_path = file_item.path().to_path_buf();
                    self.insert_new_statefuldir(new_dir_path);
                    let i = self.tab_index;
                    let name = self.directory_tabs.get_mut(i);
                    *name.unwrap() = dir_name;
                }
                Kinds::File(_) => self.push_command_log("Not directory"),
            }
        }
    }

    pub fn move_to_parent_dir(&mut self) {
        let selected_dir = self.selected_statefuldir_mut();
        let dir_name = selected_dir.dir_name();
        if let Some(parent_path) = self.crr_dir_path().parent() {
            let parent_dir_name = pathbuf_to_string_name(parent_path);
            let parent_path = parent_path.to_owned();
            self.insert_new_statefuldir(parent_path);
            let i = self.tab_index;
            let name = self.directory_tabs.get_mut(i).unwrap();
            *name = parent_dir_name;

            // select the position of crr dir name or select top
            let state_dir = self.selected_statefuldir_mut();
            let dir_pos = state_dir
                .file_items()
                .iter()
                .position(|x| x.name() == dir_name);

            state_dir.select_file_item_by_index(dir_pos);
        }
    }

    fn move_to_top_of_file_item(&mut self) {
        self.selected_statefuldir_mut().select_top_file_item();
    }

    fn move_to_bottom_of_file_item(&mut self) {
        self.selected_statefuldir_mut().select_bottom_file_item();
    }

    fn make_directory(&mut self) {
        let relpath = self.run_user_input().expect("Failed to make teraminal...");
        if relpath.is_empty() {
            return;
        }
        let path = join_to_crr_dir(self, &relpath);

        if path.is_dir() {
            self.push_command_log("The directory already exists");
            return;
        }

        match fs::create_dir_all(&path) {
            Ok(()) => {
                let item = make_a_file_item_from_dirpath(&path);
                self.selected_statefuldir_mut()
                    .push_file_item_and_sort(item);
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
        let mut name = String::with_capacity(MAX_FILE_NAME_SIZE);
        let res = start_user_input(&mut name, self.theme());
        self.be_clear();
        if let Ok(()) = res {
            let name = name.trim().to_owned();
            if name.is_empty() {
                return None;
            }
            return Some(name);
        }
        self.push_command_log("Stopped to input");
        None
    }

    fn stacker_clear(&mut self) {
        self.stacker.clear();
    }

    fn stacker_push_back(&mut self, path: PathBuf) {
        self.stacker.stacker_push(path);
        self.stacker.update_position();
    }

    fn stacker_pop_back(&mut self) -> Option<PathBuf> {
        let item = self.stacker.stacker_pop();
        self.stacker.update_position();
        item
    }

    pub fn stacker_contains(&self, path: &PathBuf) -> bool {
        self.stacker.stacker_contains(path)
    }

    fn stack_crr_file_item(&mut self) {
        if let Some(item) = self.selecting_crr_file_item() {
            let path = item.path().to_path_buf();
            if !self.stacker_contains(&path) {
                self.stacker_push_back(path);
            }
        }
    }

    fn stack_all_file_items(&mut self) {
        let dir = self.selected_statefuldir_mut();
        for item in dir.file_items().to_owned() {
            let path = item.path().to_path_buf();
            if !self.stacker_contains(&path) {
                self.stacker_push_back(path);
            }
        }
    }

    fn stack_dir_recursively(&mut self, path: PathBuf) {
        if !path.exists() {
            self.push_command_log("Doesn't exist");
            return;
        }
        if path.is_file() {
            self.push_command_log("Not Directory");
            return;
        }

        if path.read_dir().is_err() {
            self.push_command_log("Permision denied");
            return;
        }

        for entry in path.read_dir().unwrap().flatten() {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                self.stack_dir_recursively(entry_path.clone())
            }
            if self.stacker_contains(&entry_path) {
                continue;
            }
            self.stacker_push_back(entry_path);
        }
        self.stacker_push_back(path);
    }

    fn stack_crr_dir_recursively(&mut self) {
        if let Some(item) = self.selecting_crr_file_item() {
            let path = item.path().to_path_buf();
            if !self.stacker_contains(&path) {
                self.stack_dir_recursively(path);
            }
        }
    }

    fn pop_file_item(&mut self) {
        if let Some(item) = self.selecting_crr_file_item() {
            let path = item.path().to_path_buf();
            if self.stacker_contains(&path) {
                self.stacker.stacker_delete_with_path(&path);
            }
        }
    }

    fn next_stacker_item(&mut self) {
        self.stacker.next_stacker_item();
    }

    fn previous_stacker_item(&mut self) {
        self.stacker.previous_stacker_item();
    }

    fn remove_file_item_instance(&mut self, path: &Path) {
        let stateful_dir = self.selected_statefuldir_mut();
        stateful_dir.remove_file_item_with_path(path);
    }

    // 完全消去ではなくアプリ内のゴミ箱へ移動させて
    fn delete_file_item_with_path(&mut self, path: &Path) {
        let result = if path.is_dir() {
            fs::remove_dir(path)
        } else {
            fs::remove_file(path)
        };

        match result {
            Ok(_) => {
                self.remove_file_item_instance(path);
            }
            Err(err) => {
                let mss = match err.kind() {
                    io::ErrorKind::NotFound => "Not Found",
                    io::ErrorKind::PermissionDenied => "Permission Denied",
                    // io::ErrorKind::IsADirectory => "It's a Directory",
                    _ => "",
                };
            }
        }
    }

    // 完全消去ではなくアプリ内のゴミ箱へ移動させて
    fn delete_all_in_stacker(&mut self) {
        let stacker = self.stacker.stacker().to_owned();
        for path in stacker.iter() {
            self.delete_file_item_with_path(path);
        }
    }

    ///. ** User Carefully ** ///
    fn delete_directory_including_its_contents(&mut self, path: &Path) {
        match fs::remove_dir_all(path) {
            Ok(_) => {
                self.remove_file_item_instance(path);
            }
            Err(err) => {
                let mss = match err.kind() {
                    io::ErrorKind::NotFound => "Not Found",
                    io::ErrorKind::PermissionDenied => "Permission Denied",
                    // io::ErrorKind::IsADirectory => "It's a Directory",
                    _ => "",
                };
                self.push_command_log(mss);
            }
        }
    }

    /// stacker Copy Mehotds
    fn stacker_copy_file_item_to_crr_dir(&mut self) {
        if self.stacker.stacker_is_empty() {
            return;
        }

        while let Some(from_path) = self.stacker.stack.pop() {
            let name = &pathbuf_to_string_name(&from_path);
            // TODO:ファイル名が移動後のディレクトリ内に存在する場合は上書きをするかどうかを尋ねる
            if self.dir_contain_file_item(name) {
                self.stacker.stack.push(from_path);
                return;
            }
            let dir_path = self.crr_dir_path();
            let to_path = dir_path.join(name);
            // 移動後のディレクトリ名と選択中のファイル名を結合(上書きと同じ)
            // コピーを実行
            let res = fs::copy(&from_path, &to_path);
            match res {
                Ok(_n) => {
                    let item = make_a_file_item_from_dirpath(&to_path);
                    self.selected_statefuldir_mut()
                        .push_file_item_and_sort(item);
                }
                Err(e) => {
                    let mss = match e.kind() {
                        io::ErrorKind::NotFound => "Not Found",
                        io::ErrorKind::PermissionDenied => "Permission Denied",
                        _ => "",
                    };
                    println!("{}", mss);
                    self.stacker.stack.push(from_path);
                }
            }
        }
    }

    fn move_file_item_to_crr_dir(&mut self) {
        // stacker内の選択中のファイルの名前を取得
        if self.stacker.length == 0 {
            return;
        }

        while let Some(from_path) = self.stacker.stack.pop() {
            let name = &pathbuf_to_string_name(&from_path);
            // TODO:ファイル名が移動後のディレクトリ内に存在する場合は上書きをするかどうかを尋ねる
            if self.dir_contain_file_item(name) {
                self.stacker.stack.push(from_path);
                return;
            }
            let dir_path = self.crr_dir_path();
            let to_path = dir_path.join(name);
            // 移動後のディレクトリ名と選択中のファイル名を結合(上書きと同じ)
            // コピーを実行
            let res = fs::copy(&from_path, &to_path);
            match res {
                Ok(_n) => {
                    let item = make_a_file_item_from_dirpath(&to_path);
                    self.selected_statefuldir_mut()
                        .push_file_item_and_sort(item);
                    self.delete_file_item_with_path(&from_path);
                    self.remove_file_item_instance(&from_path);
                }
                Err(e) => {
                    let mss = match e.kind() {
                        io::ErrorKind::NotFound => "Not Found",
                        io::ErrorKind::PermissionDenied => "Permission Denied",
                        _ => "",
                    };
                    println!("Move Command: {}", mss);
                    self.stacker.stacker_push(from_path);
                }
            }
        }
    }
}

// receives input from the user and determines if a command
// it is possible to receive different commands each modes
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut normal = app.normal_user_keybinds();
    let mut input = app.input_user_keybinds();
    let mut stacker = app.stacker_user_keybinds();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        // TODO: Consider a more efficient way to declare the name of each command.
        let res = match app.mode() {
            Mode::Normal => key_matchings(&mut normal),
            Mode::Input => key_matchings(&mut input),
            Mode::Stacker => key_matchings(&mut stacker),
        };

        if let Ok(cmd) = res {
            if cmd == "quit" {
                return Ok(());
            }
            run_commands(&mut app, &cmd);

            if app.be_cleaned {
                terminal.clear()?;
                app.be_cleaned();
            }
        }
    }
}

fn key_matchings(keybinds: &mut UserKeybinds) -> io::Result<String> {
    if let Event::Key(key) = event::read()? {
        keybinds.set_keyevent(key);
        // matching a key bindings without combo
        if let Some(cmd) = keybinds.matching_single_keys() {
            return Ok(cmd);
        }

        keybinds.filtering_multi_first_keys();
        if keybinds.has_keycomb() {
            if let Event::Key(second) = event::read()? {
                keybinds.set_keyevent(second);
                if let Some(cmd) = keybinds.matching_multi_second_keys() {
                    return Ok(cmd);
                }
            }
        }
    }

    Ok(String::with_capacity(0))
}

fn run_commands(app: &mut App, cmd: &str) {
    match cmd {
        // mode
        "normal" => app.shift_to_normal_mode(),
        "input" => app.shift_to_input_mode(),
        "stacker" => app.shift_to_stacker_mode(),

        // comman commands
        "move_to_parent_dir" => app.move_to_parent_dir(),
        "move_to_next_file_item" => app.move_to_next_file_item(),
        "move_to_prev_file_item" => app.move_to_prev_file_item(),
        "move_to_child_dir" => app.move_to_child_dir(),
        "move_to_top_of_file_item" => app.move_to_top_of_file_item(),
        "move_to_bottom_of_file_item" => app.move_to_bottom_of_file_item(),
        "next_dirtab" => app.next_dirtab(),
        "prev_dirtab" => app.prev_dirtab(),

        // normal commands
        // "add_directory_to_dirtab" => app.add_dir_to_dirtab(),
        // "display_or_hide_hidden_file" => app.display_or_hide_hedden_file(),
        // "use_editor" => app.edit_crr_file_item(),

        // input commands
        "make_directory" => app.make_directory(),
        "make_file" => app.make_file(),
        "edit" => app.user_edit_file_item(),
        // "rename_file_item" => app.rename_file_name(),
        // "search_file_items" => app.search_file_items(),
        // "search_file_items_by_using_re" => app.search_file_items_by_using_re()

        // stacker commands
        "stacker_select" => app.stack_crr_file_item(),
        "stacker_unselect" => app.pop_file_item(),
        "stacker_select_all_recursively" => app.stack_crr_dir_recursively(),
        "stacker_select_all" => app.stack_all_file_items(),
        "stacker_next_file_item" => app.next_stacker_item(),
        "stacker_prev_file_item" => app.previous_stacker_item(),
        "stacker_paste" => app.stacker_copy_file_item_to_crr_dir(),
        "stacker_move" => app.move_file_item_to_crr_dir(),
        _ => {}
    }
    app.push_command_log(cmd);
}
