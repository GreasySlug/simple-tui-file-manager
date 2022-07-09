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

    pub fn update_position(&mut self) {
        let i = self.stack.len().checked_sub(1);
        self.state.select(i)
    }

    pub fn stacker_ref(&self) -> &Vec<PathBuf> {
        &self.stack
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn stacker_pop(&mut self) -> Option<PathBuf> {
        self.stack.pop()
    }

    pub fn stacker_push(&mut self, path: PathBuf) {
        self.stack.push(path);
    }

    pub fn stacker_remove(&mut self, i: usize) -> PathBuf {
        let path = self.stack.remove(i);
        self.update_position();
        path
    }

    pub fn stacker_delete_with_path(&mut self, path: &PathBuf) {
        if let Some(i) = self.stack.iter().position(|p| p == path) {
            self.stacker_remove(i);
        }
    }

    pub fn stacker_contains(&self, path: &PathBuf) -> bool {
        self.stack.contains(path)
    }

    pub fn stacker_is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
