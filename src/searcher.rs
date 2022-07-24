use regex::Regex;
use std::path::PathBuf;
use tui::widgets::TableState;

use crate::file_item_list::file_item::FileItem;

pub struct Searcher {
    // traial
    state: TableState,
    re: Option<Regex>,
    name: String,
    index: usize,
    searched_items: Vec<PathBuf>,
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            re: None,
            name: String::new(),
            index: 0,
            searched_items: Vec::new(),
        }
    }

    pub fn state(&mut self) -> &mut TableState {
        &mut self.state
    }

    pub fn next_stacker_item(&mut self) {
        if self.searched_items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.searched_items.len() - 1 {
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
        if self.searched_items.is_empty() {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.searched_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn init_name(&mut self) {
        self.name = String::new();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn insert_char(&mut self, c: char) {
        self.name.insert(self.index, c);
    }

    pub fn remove_char(&mut self) {
        if self.name().is_empty() {
            return;
        }
        self.name.remove(self.index);
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn init_index(&mut self) {
        self.index = 0;
    }

    pub fn add_index(&mut self) {
        if self.index < self.name().len() {
            self.index += 1;
        }
    }

    pub fn sub_index(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn get_regex(&self) -> Option<&Regex> {
        self.re.as_ref()
    }

    pub fn set_regex(&mut self, re: Regex) {
        self.re = Some(re);
    }

    pub fn clear_regex(&mut self) {
        self.re = None;
    }

    pub fn push_searched_item(&mut self, path: PathBuf) {
        self.searched_items.push(path);
    }

    pub fn pop_searched_item(&mut self) -> Option<PathBuf> {
        self.searched_items.pop()
    }

    pub fn remove_file_path(&mut self) -> Option<PathBuf> {
        if let Some(i) = self.state.selected() {
            Some(self.searched_items.remove(i))
        } else {
            None
        }
    }
    ///
    /// so much expensive
    /// TODO: change fuzzy muccher
    ///
    pub fn new_regex(&mut self) {
        let lien = self.name();
        if lien.is_empty() {
            self.clear_regex();
        } else {
            let ptn: String = self.name().chars().map(|c| format!("({}.*?)", c)).collect();
            let re = Regex::new(&ptn);
            if let Ok(re) = re {
                self.set_regex(re);
            }
        }
    }

    pub fn make_filter_vec(&mut self, items: Vec<FileItem>) -> Vec<FileItem> {
        if let Some(re) = self.get_regex() {
            items
                .into_iter()
                .filter(|item| item.find(re).is_some())
                .collect::<Vec<FileItem>>()
        } else {
            Vec::new()
        }
    }
}
