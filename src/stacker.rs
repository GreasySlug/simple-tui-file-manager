use std::path::Path;
use std::path::PathBuf;
use tui::widgets::ListState;

pub struct StackerVec {
    state: ListState,
    stack: Vec<PathBuf>,
}

impl StackerVec {
    pub fn new() -> StackerVec {
        StackerVec {
            state: ListState::default(),
            stack: Vec::new(),
        }
    }

    pub fn next_stacker_item(&mut self) {
        if self.stacker_is_empty() {
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
        if self.stacker_is_empty() {
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

    fn selecting_item_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn update_position(&mut self) {
        let i = self.stack.len().checked_sub(1);
        self.state.select(i)
    }

    pub fn stack_ref(&self) -> &Vec<PathBuf> {
        &self.stack
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn clear(&mut self) {
        self.stack.clear();
        self.state.select(None);
    }

    pub fn stacker_pop(&mut self) -> Option<PathBuf> {
        let path = self.stack.pop();
        self.update_position();
        path
    }

    pub fn stacker_push(&mut self, path: PathBuf) {
        self.stack.push(path);
        self.update_position();
    }

    pub fn stacker_remove(&mut self, i: usize) -> PathBuf {
        let path = self.stack.remove(i);
        self.update_position();
        path
    }

    pub fn remove_selecting_item(&mut self) -> Option<PathBuf> {
        if let Some(i) = self.selecting_item_index() {
            let path = self.stack.remove(i);
            self.update_position();
            return Some(path);
        }
        None
    }

    pub fn stacker_take_by_path(&mut self, path: &Path) -> Option<PathBuf> {
        if let Some(i) = self.stack.iter().position(|x| x.as_path() == path) {
            return Some(self.stacker_remove(i));
        }
        None
    }

    pub fn stacker_delete_with_path(&mut self, path: &PathBuf) {
        if let Some(i) = self.stack.iter().position(|p| p == path) {
            self.stacker_remove(i);
        }
    }

    pub fn stacker_contains(&self, path: &Path) -> bool {
        self.stack.contains(&path.to_owned())
    }

    pub fn stacker_is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
