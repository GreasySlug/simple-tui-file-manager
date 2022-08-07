use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Debug;
use std::io::Result as ioResult;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use regex::Regex;
use tui::backend::Backend;
use tui::widgets::TableState;
use tui::Terminal;

use crate::file_item_list::file_item::FileItem;
use crate::file_item_list::Kinds;

use crate::input::{init_input_area_terminal, start_user_input};
use crate::load_config::{
    load_user_config_file, multi_string_map_to_user_keyboad, SettingTheme, UserConfig, UserKeybinds,
};

use crate::path_process::{
    get_user_profile_path, join_to_crr_dir, make_a_file_item_from_dirpath, pathbuf_to_string_name,
    user_commands,
};

use crate::searcher::Searcher;
use crate::stacker::StackerVec;
use crate::state::StatefulDirectory;
use crate::ui::input_ui::input_area_ui;
use crate::ui::ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Input,
    Stacker,
    Searcher,
}

// TODO: Restrictions without reason, so think cost
const DRAIN_SIZE: usize = 50;
const MAX_HIST_SIZE: usize = 1000;
const MAX_FILE_NAME_SIZE: usize = 40;

// TODO: Do I have to load  use config in this struct?
pub struct App {
    directory_tabs: Vec<(PathBuf, String)>,
    tab_index: usize,
    dir_map: HashMap<PathBuf, StatefulDirectory>,
    command_history: Vec<String>,
    stacker: StackerVec,
    mode: Mode,
    config: UserConfig,
    be_cleaned: bool,
    editor: String,
    show_hidden_files: bool,
    searcher: Searcher,
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
            editor: String::new(),
            show_hidden_files: false,
            // traial
            searcher: Searcher::new(),
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

    pub fn config(&self) -> &UserConfig {
        &self.config
    }

    pub fn show_hidden_files(&self) -> bool {
        self.show_hidden_files
    }

    #[cfg(target_os = "windows")]
    fn user_settings(&mut self) {
        self.editor = self.config().user_editor();
        self.show_hidden_files = self.config().show_hidden_files();
        let config = self.config();
        let additional_directories = config.additional_directory();
        for dir in additional_directories.into_iter() {
            if let Some(path) = get_user_profile_path(&dir) {
                self.push_new_dirname_to_dirtab(path.clone());
                self.insert_new_statefuldir(path);
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn user_settings(&mut self) {
        self.editor = self.config().user_editor();
        self.show_hidden_files = self.config().show_hidden_files();
        let config = self.config();
        let additional_directories = config.additional_directory();
        for dir in additional_directories.into_iter() {
            if let Some(path) = get_user_profile_path(&dir) {
                self.push_new_dirname_to_dirtab(pathbuf_to_string_name(&path));
                self.insert_new_statefuldir(path);
            }
        }
    }

    // The current directory should be selected, so that tab and hashmap must existe.
    pub fn selecting_statefuldir_mut(&mut self) -> &mut StatefulDirectory {
        let selected_tab = self.directory_tabs.get(self.tab_index).unwrap();
        self.dir_map.get_mut(&selected_tab.0).unwrap()
    }

    pub fn selecting_statefuldir_ref(&self) -> &StatefulDirectory {
        let selected_tab = self.directory_tabs.get(self.tab_index).unwrap();
        self.dir_map.get(&selected_tab.0).unwrap()
    }

    pub fn selecting_dir_path(&self) -> &std::path::Path {
        let selected_stateful_dir = self.selecting_statefuldir_ref();
        selected_stateful_dir.dir_path()
    }

    pub fn selecting_dir_file_items(&self) -> &Vec<FileItem> {
        let stateful_dir = self.selecting_statefuldir_ref();
        stateful_dir.file_items()
    }

    fn selecting_dir_contain_name(&self, name: &str) -> bool {
        self.selecting_statefuldir_ref().contain_name(name)
    }

    fn dirtab_contains_dirpath(&self, path: &PathBuf) -> bool {
        self.directory_tabs.iter().any(|(p, _name)| p == path)
    }

    pub fn selecting_crr_file_item(&self) -> Option<&FileItem> {
        let stateful_dir = self.selecting_statefuldir_ref();
        if !stateful_dir.is_selected() {
            return None;
        }
        if let Some(i) = stateful_dir.state_table().selected() {
            return self.selecting_dir_file_items().get(i);
        }

        None
    }

    pub fn tab_index(&self) -> usize {
        self.tab_index
    }

    pub fn dirtab(&self) -> &Vec<(PathBuf, String)> {
        &self.directory_tabs
    }

    pub fn insert_new_statefuldir(&mut self, dir_path: PathBuf) {
        let is_show = self.show_hidden_files();
        if let Entry::Vacant(item) = self.dir_map.entry(dir_path.clone()) {
            let mut new_stateful_dir = StatefulDirectory::new(dir_path, is_show);

            // Sorted by name in each of the files and directories
            new_stateful_dir.sort_file_items_by_kinds();

            if !new_stateful_dir.is_selected() {
                new_stateful_dir.select_top_file_item();
            }
            item.insert(new_stateful_dir);
        }
    }

    pub fn push_new_dirname_to_dirtab(&mut self, path: PathBuf) {
        let name = pathbuf_to_string_name(&path);
        let new_tab = (path, name);
        if !self.directory_tabs.contains(&new_tab) {
            self.directory_tabs.push(new_tab)
        }
    }

    pub fn next_dirtab(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.directory_tabs.len();
    }

    pub fn move_to_next_file_item(&mut self) {
        self.selecting_statefuldir_mut().select_next_file_item();
    }

    pub fn move_to_prev_file_item(&mut self) {
        self.selecting_statefuldir_mut().select_previous_file_item();
    }

    pub fn shift_to_input_mode(&mut self) {
        if !self.stacker.stacker_is_empty() {
            self.stacker_clear(); // TODO: switch?
        }
        self.mode = Mode::Input;
    }

    pub fn shift_to_normal_mode(&mut self) {
        if !self.stacker.stacker_is_empty() {
            self.stacker_clear(); // TODO: switch?
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
                    let new_dir_path = file_item.path().to_path_buf();
                    let new_dir_name = pathbuf_to_string_name(&new_dir_path);
                    self.insert_new_statefuldir(new_dir_path.clone());
                    let i = self.tab_index;
                    if let Some(dirtab) = self.directory_tabs.get_mut(i) {
                        *dirtab = (new_dir_path, new_dir_name);
                    }
                }
                Kinds::File(_) => self.push_command_log("Not directory"),
            }
        }
    }

    pub fn move_to_parent_dir(&mut self) {
        let selected_dir = self.selecting_statefuldir_mut();
        let dir_name = selected_dir.dir_name();
        if let Some(parent_path) = self.selecting_dir_path().parent() {
            let parent_path = parent_path.to_owned();
            let parent_name = pathbuf_to_string_name(&parent_path);
            self.insert_new_statefuldir(parent_path.clone());
            let i = self.tab_index;
            let dirtab = self.directory_tabs.get_mut(i).unwrap();
            *dirtab = (parent_path, parent_name);

            // select the position of crr dir name or select top
            let state_dir = self.selecting_statefuldir_mut();
            let dir_pos = state_dir
                .file_items()
                .iter()
                .position(|x| x.name() == dir_name);

            state_dir.select_file_item_by_index(dir_pos);
        }
    }

    fn move_to_top_of_file_item(&mut self) {
        self.selecting_statefuldir_mut().select_top_file_item();
    }

    fn move_to_bottom_of_file_item(&mut self) {
        self.selecting_statefuldir_mut().select_bottom_file_item();
    }

    fn inseart_new_file_item_instance(&mut self, mut path: PathBuf) {
        loop {
            let original_path = path.clone();
            if !path.parent().is_none() {
                return;
            }
            // if let Some(dirstate) = self.dir_map.get_mut(&path) {
            //     let item = make_a_file_item_from_dirpath(&original_path);
            //     dirstate.push_file_item_and_sort(item);
            // }
        }
    }

    ///
    /// In input mode, make directories.
    ///  ex1) dirname
    ///  ex2) dirname01/dirname02
    /// None: File cannot be created.
    ///  ex) sample.txt means a "sample.txt" directory not a file
    ///
    fn make_directory(&mut self) {
        let relpath = self.run_user_input("Input directory name");
        if relpath.is_none() {
            self.push_command_log("Failed to make directory");
            return;
        }
        let path = join_to_crr_dir(self, &relpath.unwrap());

        if path.is_dir() {
            self.push_command_log("Duplicate name");
            return;
        }

        match fs::create_dir_all(&path) {
            Ok(()) => {
                self.inseart_new_file_item_instance(path);
            }
            Err(error) => {
                let message = match error.kind() {
                    io::ErrorKind::PermissionDenied => "Permission denied",
                    io::ErrorKind::NotFound => "Not Found",
                    _ => "Duplicate name",
                };
                self.push_command_log(message);
            }
        }
    }

    fn make_file(&mut self) {
        let relpath = self.run_user_input("Input file name");

        if relpath.is_none() {
            self.push_command_log("Failed to make file");
            return;
        }

        let file_name = relpath.unwrap();
        if file_name.contains(|c| c == '\\') {
            let path = join_to_crr_dir(self, &file_name);
            self.make_file_with_dir(&path);
        }

        let path = join_to_crr_dir(self, &file_name);

        if path.is_dir() {
            self.push_command_log("Duplicate name");
            return;
        }

        match fs::File::create(&path) {
            Ok(_) => {
                self.inseart_new_file_item_instance(path);
            }
            Err(error) => {
                let message = match error.kind() {
                    io::ErrorKind::PermissionDenied => "Permission denied",
                    io::ErrorKind::NotFound => "Not Found",
                    _ => "Diplicate name",
                };
                self.push_command_log(message);
            }
        }
    }

    fn make_file_with_dir(&mut self, path: &Path) {
        if let Some(parent_path) = path.parent() {
            self.make_dir_with_path(parent_path);
            match fs::File::create(path) {
                Ok(_) => {}
                Err(error) => {
                    let message = match error.kind() {
                        io::ErrorKind::PermissionDenied => "Permission denied",
                        io::ErrorKind::NotFound => "Not Found",
                        _ => "Diplicate name",
                    };
                    self.push_command_log(message);
                }
            }
        }
    }

    fn make_dir_with_path(&mut self, path: &Path) {
        match fs::create_dir_all(path) {
            Ok(_) => self.inseart_new_file_item_instance(path.to_owned()),
            Err(error) => {
                let message = match error.kind() {
                    io::ErrorKind::PermissionDenied => "Permission denied",
                    io::ErrorKind::NotFound => "Not Found",
                    _ => "Duplicate name",
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

    fn searcher_user_keybinds(&self) -> UserKeybinds {
        let keybind = self.config.searcher_keybindings_map();
        let keymap = multi_string_map_to_user_keyboad(&keybind);
        UserKeybinds::new()
            .make_single_keybinds(keymap.clone())
            .make_multiple_keybinds(keymap)
    }

    ///
    /// one line user input
    ///
    fn run_user_input(&mut self, cmd: &str) -> Option<String> {
        let mut name = String::with_capacity(MAX_FILE_NAME_SIZE);
        let res = start_user_input(&mut name, self.theme(), cmd);
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
    }

    ///
    /// If the item is not selected, select it
    /// if the item is selected, select it
    ///
    fn stacker_handle_selecter(&mut self) {
        let item = self.selecting_statefuldir_ref().get_selected_file_item();
        if let Some(item) = item {
            if self.stacker_contains(item.path()) {
                self.stacker.remove_selecting_item();
            } else {
                let path = item.path();
                if !self.stacker_contains(path) {
                    self.stacker_push_back(path.to_path_buf());
                }
            }
        }
    }

    ///
    /// unselect latest
    ///
    fn stacker_deselected(&mut self) {
        self.stacker.stacker_pop();
    }

    pub fn stacker_contains(&self, path: &Path) -> bool {
        self.stacker.stacker_contains(path)
    }

    ///
    /// select all in current directory
    ///
    fn stacker_all_file_items(&mut self) {
        let items = self.selecting_dir_file_items().clone();
        for item in items {
            let path = item.path().to_path_buf();
            if !self.stacker_contains(&path) {
                self.stacker_push_back(path);
            }
        }
    }

    ///
    /// directories are flatted and the structure breaks.
    ///
    fn stacker_push_dir_recursively(&mut self, path: PathBuf) {
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
                self.stacker_push_dir_recursively(entry_path.clone())
            }
            if self.stacker_contains(&entry_path) {
                continue;
            }
            self.stacker_push_back(entry_path);
        }
        self.stacker_push_back(path);
    }

    fn stacker_crr_dir_recursively(&mut self) {
        if let Some(item) = self.selecting_crr_file_item() {
            let path = item.path().to_path_buf();
            if !self.stacker_contains(&path) {
                self.stacker_push_dir_recursively(path);
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

    fn stacker_next_item(&mut self) {
        self.stacker.next_stacker_item();
    }

    fn stacker_previous_item(&mut self) {
        self.stacker.previous_stacker_item();
    }

    /// func to delete remaining instance after moveing or deleting etc...
    fn remove_file_item_instance(&mut self, path: &Path) {
        let stateful_dir = self.selecting_statefuldir_mut();
        stateful_dir.remove_file_item_with_path(path);
    }

    ///
    /// Function to remove a path from a vec of Stacker
    /// If no selection is made, do nothing with the return value,
    /// and use it to delete an instance of FileItem vec by a path if it is used to move, delete, etc.
    ///
    fn remove_path_in_stacker(&mut self, path: &Path) -> Option<PathBuf> {
        self.stacker.stacker_take_by_path(path)
    }

    ///
    /// The item in the stacker, at the position where the cursor is in the selection, is deleted.
    /// complete delete, cannot be undone
    ///
    fn stacker_delete_selecting_item(&mut self) {
        let stacker_stete = self.stacker.state_mut();
        if let Some(i) = stacker_stete.selected() {
            let path = self.stacker.stacker_remove(i);
            if path.is_dir() {
                self.delete_directory_including_its_contents(&path);
            } else {
                self.stacker_delete_selecting_item();
            }
            self.remove_path_in_stacker(&path);
        }
    }

    /// TODO: Instead of deleting it completely, it is better to move it to the Recycle Bin in the application so that it can be undone, etc.
    fn stacker_delete_all(&mut self) {
        let stacker = self.stacker.stack_ref().to_owned();
        for path in stacker.iter() {
            self.delete_file_item_with_path(path);
        }
    }

    ///
    ///. ** User Carefully ** ///
    /// I think it would be better to separate the function between full deletion and move to trashbox.
    ///
    fn delete_file_item_with_path(&mut self, path: &Path) {
        let result = if path.is_dir() {
            fs::remove_dir(path)
        } else {
            fs::remove_file(path)
        };

        match result {
            Ok(_) => {
                self.remove_file_item_instance(path);
                let stateful_dir = self.selecting_statefuldir_mut();
                stateful_dir.remove_file_item_with_path(path);
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

    ///
    ///. ** Use Carefully ** ///
    /// delete directory and its contents
    ///
    fn delete_directory_including_its_contents(&mut self, path: &Path) {
        match fs::remove_dir_all(path) {
            Ok(_) => {}
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

        while let Some(from_path) = self.stacker.stacker_pop() {
            let name = &pathbuf_to_string_name(&from_path);
            if from_path.exists() {
                return;
            }

            // TODO:if name is duplecated, y/n is displayed  y is pass,and other charactors are return. w
            if self.selecting_dir_contain_name(name) {
                self.stacker.stacker_push(from_path);
                continue;
            }

            let dir_path = self.selecting_dir_path();
            let to_path = dir_path.join(name);
            let res = fs::copy(&from_path, &to_path);
            match res {
                Ok(_n) => {
                    let item = make_a_file_item_from_dirpath(&to_path);
                    if self.selecting_dir_contain_name(item.name_ref()) {
                        break;
                    }
                    self.selecting_statefuldir_mut()
                        .push_file_item_and_sort(item);
                }
                Err(e) => {
                    let mss = match e.kind() {
                        io::ErrorKind::NotFound => "Not Found",
                        io::ErrorKind::PermissionDenied => "Permission Denied",
                        _ => "Duplecate name",
                    };
                    println!("{}", mss);
                    self.stacker.stacker_push(from_path);
                }
            }
        }
    }

    // stacker move methods
    fn stcker_move_file_item_to_crr_dir(&mut self) {
        if self.stacker.stacker_is_empty() {
            return;
        }

        while let Some(from_path) = self.stacker.remove_selecting_item() {
            let name = &pathbuf_to_string_name(&from_path);
            // TODO:if name is duplecated, y/n is displayed  y is pass,and other charactors are return. w
            if self.selecting_dir_contain_name(name) {
                self.stacker_push_back(from_path);
                self.push_command_log("Duplicate name");
                return;
            }

            let dir_path = self.selecting_dir_path();
            let to_path = dir_path.join(name);
            let res = fs::copy(&from_path, &to_path);
            match res {
                Ok(_n) => {
                    let item = make_a_file_item_from_dirpath(&to_path);
                    self.selecting_statefuldir_mut()
                        .push_file_item_and_sort(item);
                    self.delete_file_item_with_path(&from_path);
                    self.remove_file_item_instance(&from_path);
                }
                Err(e) => {
                    let mss = match e.kind() {
                        io::ErrorKind::NotFound => "Not Found",
                        io::ErrorKind::PermissionDenied => "Permission Denied",
                        _ => "Duplecate name",
                    };
                    self.push_command_log(mss);
                    self.stacker.stacker_push(from_path);
                }
            }
        }
    }

    ///
    /// open file with user editor, like vim, emacs
    ///
    fn user_edit_file_item(&mut self) {
        if self.selecting_crr_file_item().is_none() {
            return;
        }
        self.shift_to_normal_mode();
        if let Some(item) = self.selecting_crr_file_item() {
            if user_commands(&self.editor, vec![item.path()]).is_err() {
                self.push_command_log("Failed to edit");
            }
        }
        self.be_clear();
        self.shift_to_input_mode();
    }

    /// *** Seacher Mode *** ///

    fn shift_to_searcher_mode(&mut self) {
        self.mode = Mode::Searcher;
    }

    fn shift_to_normal_mode_from_searcher(&mut self) {
        self.searcher_init();
        self.mode = Mode::Normal;
    }

    pub fn regex_ref(&self) -> Option<&Regex> {
        self.searcher.get_regex()
    }

    fn searhing_name(&self) -> &str {
        self.searcher.name()
    }

    fn searcher_next(&mut self) {
        self.searcher.next_stacker_item();
    }
    fn searcher_prev(&mut self) {
        self.searcher.previous_stacker_item();
    }

    pub fn searcher_init(&mut self) {
        self.searcher.clear_regex();
        self.searcher.init_name();
        self.searcher.init_index();
    }

    pub fn searcher_state(&mut self) -> &mut TableState {
        self.searcher.state()
    }

    fn remove_file_path_searcher(&mut self) -> Option<PathBuf> {
        self.searcher.remove_file_path()
    }

    fn searcher_move_to_child_dir(&mut self) {
        if let Some(path) = self.remove_file_path_searcher() {
            if let Ok(meta) = path.metadata() {
                match Kinds::classifiy_kinds(&path, &meta) {
                    Kinds::Directory(_) => {
                        let name = pathbuf_to_string_name(&path);
                        self.insert_new_statefuldir(path.clone());
                        let i = self.tab_index;
                        if let Some(tab) = self.directory_tabs.get_mut(i) {
                            *tab = (path, name);
                        }
                    }
                    Kinds::File(_) => self.push_command_log("Not directory"),
                }
            }
            self.searcher_init();
            self.mode = Mode::Normal;
        }
    }

    fn searcher_stack_all_items(&mut self) {
        // TODO: I don't want to use clone()
        let item_paths = self.searcher.file_items_ref().clone();
        for path in item_paths {
            self.stacker_push_back(path);
        }
        self.mode = Mode::Stacker;
    }

    fn searcher_make_found_items(&mut self) {
        let items = self.selecting_dir_file_items().clone();
        for item in items {
            self.searcher.filter_push(item);
        }
    }

    fn searcher_is_empty(&self) -> bool {
        self.searcher.is_empty()
    }
}

///
/// &str(s) are used for state transitions
/// this situation is not good, I think.
/// so I want to use enum to make state
/// transitions with fixed values
///
const SEARCHING: &str = "searching";
const SEARCHING_FIXED: &str = "fix";
const SEARCHING_STOP: &str = "stop";
const QUIT: &str = "quit";
// receives input from the user and determines if a command
// it is possible to receive different commands each modes
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> ioResult<()> {
    // TODO: only directory settings exit, but default settings be going to be added.
    app.user_settings();
    let mut normal = app.normal_user_keybinds();
    let mut input = app.input_user_keybinds();
    let mut stacker = app.stacker_user_keybinds();
    let mut searcher = app.searcher_user_keybinds();
    let themes = app.theme().clone();
    let mut res = SEARCHING.to_string();
    loop {
        terminal.draw(|f| ui(f, &mut app, &themes))?;
        // TODO: Consider a more efficient way to declare the name of each command.
        let res = match app.mode() {
            Mode::Normal => key_matchings(&mut normal),
            Mode::Input => key_matchings(&mut input),
            Mode::Stacker => key_matchings(&mut stacker),
            Mode::Searcher => searching_handling(&mut app, &mut res, &mut searcher, &themes),
        };
        if let Ok(cmd) = res {
            let cmd = cmd.as_str();
            if cmd == QUIT {
                return Ok(());
            }
            if cmd == SEARCHING_FIXED {
                app.searcher_make_found_items();
                if app.searcher_is_empty() {
                    handle_modal_seacher(&mut app);
                }
            }
            if cmd == SEARCHING {
                continue;
            }
            run_commands(&mut app, cmd);

            if app.be_cleaned {
                terminal.clear()?;
                app.be_cleaned();
            }
        } else {
            // TODO: logging io message
        }
    }
}

fn handle_modal_seacher(app: &mut App) {
    if app.mode() == &Mode::Searcher {
        app.shift_to_normal_mode_from_searcher();
    } else {
        app.shift_to_normal_mode();
    }
}

fn searching_handling(
    app: &mut App,
    res: &mut String,
    search: &mut UserKeybinds,
    themes: &SettingTheme,
) -> ioResult<String> {
    if res == SEARCHING_FIXED {
        key_matchings(search)
    } else {
        *res = searching_files_by_name(app, themes);
        match res.as_str() {
            "stop" => Ok(String::new()),
            "continue" => Ok(SEARCHING.to_string()),
            "fix" => Ok(SEARCHING_FIXED.to_string()),
            "searching" => Ok(SEARCHING.to_string()),
            _ => {
                panic!("Failed to searching...");
            }
        }
    }
}

fn searching_files_by_name(app: &mut App, themes: &SettingTheme) -> String {
    let style = themes.searcher_command_style();
    let mut terminal =
        init_input_area_terminal().expect("Failed to make searching input area terminal...");

    terminal
        .draw(|f| {
            input_area_ui(
                f,
                app.searhing_name(),
                style,
                app.searcher.index() as u16,
                "Input Searching file name",
            )
        })
        .expect("failed to make input area window");

    if let Event::Key(KeyEvent { code, .. }) = event::read().expect("Failed to input") {
        match code {
            KeyCode::Enter => {
                return SEARCHING_FIXED.to_string();
            }
            KeyCode::Esc => {
                app.searcher_init();
                app.mode = Mode::Normal;
                return SEARCHING_STOP.to_string();
            }
            KeyCode::Char(c) => {
                app.searcher.insert_char(c);
                app.searcher.add_index();
            }
            KeyCode::Backspace => {
                app.searcher.sub_index();
                app.searcher.remove_char();
            }
            KeyCode::Left => {
                app.searcher.sub_index();
            }
            KeyCode::Right => {
                app.searcher.add_index();
            }
            _ => {}
        }
        terminal
            .draw(|f| {
                input_area_ui(
                    f,
                    app.searhing_name(),
                    style,
                    app.searcher.index() as u16,
                    "Input searching file name",
                )
            })
            .expect("failed to make input area window");
    }
    SEARCHING.to_string()
}

fn key_matchings(keybinds: &mut UserKeybinds) -> ioResult<String> {
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
        // "normal" => app.shift_to_normal_mode(),
        "normal" => handle_modal_seacher(app),
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
        "search_file_items" => app.shift_to_searcher_mode(),

        // input commands
        "make_directory" => app.make_directory(),
        "make_file" => app.make_file(),
        "edit" => app.user_edit_file_item(),
        // "rename_file_item" => app.rename_file_name(),
        // "search_file_items_by_using_re" => app.search_file_items_by_using_re()

        // stacker commands
        "stacker_toggle_select" => app.stacker_handle_selecter(),
        "stacker_pop" => app.stacker_deselected(),
        "stacker_select_all_recursively" => app.stacker_crr_dir_recursively(),
        "stacker_select_all" => app.stacker_all_file_items(),
        "stacker_next_file_item" => app.stacker_next_item(),
        "stacker_prev_file_item" => app.stacker_previous_item(),
        "stacker_paste" => app.stacker_copy_file_item_to_crr_dir(),
        "stacker_move" => app.stcker_move_file_item_to_crr_dir(),

        // searcher commands
        "searcher_next_file_item" => app.searcher_next(),
        "searcher_prev_file_item" => app.searcher_prev(),
        "searcher_move_to_child_dir" => app.searcher_move_to_child_dir(),
        "searcher_select_all" => app.searcher_stack_all_items(),
        _ => {}
    }
}
