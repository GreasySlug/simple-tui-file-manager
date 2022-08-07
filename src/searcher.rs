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
        self.set_regex();
    }

    pub fn remove_char(&mut self) {
        if self.name().is_empty() {
            self.clear_regex();
            return;
        }
        self.name.remove(self.index);
        self.set_regex();
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

    pub fn set_regex(&mut self) {
        if self.name().is_empty() {
            self.re = None;
            return;
        }
        self.re = Regex::new(self.name()).ok();
    }

    pub fn clear_regex(&mut self) {
        self.re = None;
    }

    pub fn push_searched_item(&mut self, path: PathBuf) {
        self.searched_items.push(path);
    }

    pub fn remove_file_path(&mut self) -> Option<PathBuf> {
        if let Some(i) = self.state.selected() {
            let path = self.searched_items.remove(i);
            self.re = Regex::new(self.name()).ok();
            Some(path)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.searched_items.is_empty()
    }

    pub fn filter_push(&mut self, item: FileItem) {
        if self.re.is_none() {
            return;
        }
        let re = self.re.as_ref().unwrap();
        if item.find(re).is_some() {
            self.push_searched_item(item.path().to_owned());
        }
    }

    pub fn file_items_ref(&self) -> &Vec<PathBuf> {
        &self.searched_items
    }
}

#[cfg(test)]
mod test {
    use super::Searcher;

    #[test]
    fn init_regex_test() {
        let mut searcher = Searcher::new();
        searcher.insert_char('s');
        searcher.add_index();
        searcher.set_regex();
        println!("{:?}", searcher.re);
        println!("{:?}", searcher.index());
        assert!(searcher.get_regex().is_some());

        searcher.insert_char('s');
        searcher.add_index();
        searcher.set_regex();
        println!("{:?}", searcher.re);
        println!("{:?}", searcher.index());
        assert!(searcher.get_regex().is_some());
    }
}
