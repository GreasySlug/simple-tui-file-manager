use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::style::{Color, Style};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
enum Colors {
    White,
    Black,
    Yellow,
    Blue,
    Green,
    Red,
    Gray,
    DarkGray,
    Cyan,
    Magenta,
    LightBlue,
    LightRed,
    LightGreen,
    LightMagenta,
    LightYellow,
    LightCyan,
    Rgb(u8, u8, u8),
}

pub fn default_vim_movements() -> ModeKeybinds {
    let mut normal: HashMap<String, String> = HashMap::new();
    let iter = [
        ("h", "move_to_parent_dir"),
        ("j", "move_to_next_file_item"),
        ("k", "move_to_prev_file_item"),
        ("l", "move_to_child_dir"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("left", "move_to_parent_dir"),
        ("right", "move_to_child_dir"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("q", "quit"),
        ("i", "input"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        normal.insert(name.to_string(), cmd.to_string());
    }

    let mut input: HashMap<String, String> = HashMap::new();
    let iter = [
        ("C-h", "move_to_parent_dir"),
        ("C-j", "move_to_next_file_item"),
        ("C-k", "move_to_prev_file_item"),
        ("C-l", "move_to_child_dir"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("left", "move_to_parent_dir"),
        ("right", "move_to_child_dir"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("escape", "normal"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        input.insert(name.to_string(), cmd.to_string());
    }

    let mut stacker: HashMap<String, String> = HashMap::new();
    let iter = [
        ("h", "move_to_parent_directory"),
        ("j", "move_to_next_file_item"),
        ("k", "move_to_prev_file_item"),
        ("l", "move_to_child_directory"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("left", "move_to_parent_dir"),
        ("right", "move_to_child_dir"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("q", "quit"),
        ("escape", "normal"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        stacker.insert(name.to_string(), cmd.to_string());
    }

    ModeKeybinds {
        normal,
        input,
        stacker,
    }
}

pub fn default_arrow_key_keybindings() -> ModeKeybinds {
    let mut normal: HashMap<String, String> = HashMap::new();
    let iter = [
        ("Left", "move_to_parent_directory"),
        ("Down", "move_to_next_file_item"),
        ("Up", "move_to_prev_file_item"),
        ("Right", "move_to_child_directory"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("q", "quit"),
        ("i", "input"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        normal.insert(name.to_string(), cmd.to_string());
    }

    let mut input: HashMap<String, String> = HashMap::new();
    let iter = [
        ("Left", "move_to_parent_directory"),
        ("Down", "move_to_next_file_item"),
        ("Up", "move_to_prev_file_item"),
        ("Right", "move_to_child_directory"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("escape", "normal"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        input.insert(name.to_string(), cmd.to_string());
    }

    let mut stacker: HashMap<String, String> = HashMap::new();
    let iter = [
        ("Left", "move_to_parent_directory"),
        ("Down", "move_to_next_file_item"),
        ("Up", "move_to_prev_file_item"),
        ("Right", "move_to_child_directory"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("q", "quit"),
        ("escape", "normal"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        stacker.insert(name.to_string(), cmd.to_string());
    }

    ModeKeybinds {
        normal,
        input,
        stacker,
    }
}

pub fn default_vim_control_key_movements() -> ModeKeybinds {
    let mut normal: HashMap<String, String> = HashMap::new();
    let iter = [
        ("C-h", "move_to_parent_directory"),
        ("C-j", "move_to_next_file_item"),
        ("C-k", "move_to_prev_file_item"),
        ("C-l", "move_to_child_directory"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("left", "move_to_parent_directory"),
        ("right", "move_to_child_directory"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("q", "quit"),
        ("i", "input"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        normal.insert(name.to_string(), cmd.to_string());
    }

    let mut input: HashMap<String, String> = HashMap::new();
    let iter = [
        ("C-h", "move_to_parent_directory"),
        ("C-j", "move_to_next_file_item"),
        ("C-k", "move_to_prev_file_item"),
        ("C-l", "move_to_child_directory"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("left", "move_to_parent_directory"),
        ("right", "move_to_child_directory"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("escape", "normal"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        input.insert(name.to_string(), cmd.to_string());
    }

    let mut stacker: HashMap<String, String> = HashMap::new();
    let iter = [
        ("C-h", "move_to_parent_directory"),
        ("C-j", "move_to_next_file_item"),
        ("C-k", "move_to_prev_file_item"),
        ("C-l", "move_to_child_directory"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("left", "move_to_parent_directory"),
        ("right", "move_to_child_directory"),
        ("Tab", "next_dirtab"),
        ("Backtab", "prev_dirtab"),
        ("q", "quit"),
        ("escape", "normal"),
        ("v", "stacker"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        stacker.insert(name.to_string(), cmd.to_string());
    }

    ModeKeybinds {
        normal,
        input,
        stacker,
    }
}

pub fn string_map_to_user_keybindings(
    keybinds: &HashMap<String, String>,
) -> HashMap<KeyEvent, String> {
    let mut keybindings: HashMap<KeyEvent, String> = HashMap::new();
    for (key, cmd) in keybinds.iter() {
        let user_keyboad = string_to_keyevent(key);
        keybindings.insert(user_keyboad, cmd.to_string());
    }
    keybindings
}

pub fn multi_string_map_to_user_keyboad(
    keybinds: &HashMap<String, String>,
) -> HashMap<Vec<KeyEvent>, String> {
    let mut keybind: HashMap<Vec<KeyEvent>, String> = HashMap::new();
    for (key, cmd) in keybinds.iter() {
        if key.split_whitespace().count() > 1 {
            let keys: Vec<KeyEvent> = key.split_whitespace().map(string_to_keyevent).collect();
            keybind.insert(keys, cmd.to_owned());
        } else {
            let user_keyboad = string_to_keyevent(key);
            keybind.insert(vec![user_keyboad], cmd.to_owned());
        }
    }
    keybind
}

fn string_to_keyevent(s: &str) -> KeyEvent {
    match s {
        // Special keys
        "Tab" => KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
        "Backtab" => KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT),
        "escape" | "Escape" | "Esc" => KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
        "Enter" => KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        "Backspace" => KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
        "Delete" => KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE),
        "Left" => KeyEvent::new(KeyCode::Left, KeyModifiers::NONE),
        "Right" => KeyEvent::new(KeyCode::Right, KeyModifiers::NONE),
        "Up" => KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
        "Down" => KeyEvent::new(KeyCode::Down, KeyModifiers::NONE),
        "Home" => KeyEvent::new(KeyCode::Home, KeyModifiers::NONE),
        "End" => KeyEvent::new(KeyCode::End, KeyModifiers::NONE),
        "PageUp" => KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE),
        "PageDown" => KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE),
        // Lowercase letters
        "a" => KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        "b" => KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE),
        "c" => KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE),
        "d" => KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
        "e" => KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
        "f" => KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE),
        "g" => KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
        "h" => KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
        "i" => KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        "j" => KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
        "k" => KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE),
        "l" => KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
        "m" => KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE),
        "n" => KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE),
        "o" => KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
        "p" => KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE),
        "q" => KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
        "r" => KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE),
        "s" => KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
        "t" => KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        "u" => KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE),
        "v" => KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE),
        "w" => KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE),
        "x" => KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE),
        "y" => KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE),
        "z" => KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE),
        "#" => KeyEvent::new(KeyCode::Char('#'), KeyModifiers::NONE),
        "!" => KeyEvent::new(KeyCode::Char('!'), KeyModifiers::NONE),
        "$" => KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE),
        "%" => KeyEvent::new(KeyCode::Char('%'), KeyModifiers::NONE),
        "&" => KeyEvent::new(KeyCode::Char('&'), KeyModifiers::NONE),
        "'" => KeyEvent::new(KeyCode::Char('\''), KeyModifiers::NONE),
        "(" => KeyEvent::new(KeyCode::Char('('), KeyModifiers::NONE),
        ")" => KeyEvent::new(KeyCode::Char(')'), KeyModifiers::NONE),
        "-" => KeyEvent::new(KeyCode::Char('-'), KeyModifiers::NONE),
        "=" => KeyEvent::new(KeyCode::Char('='), KeyModifiers::NONE),
        "^" => KeyEvent::new(KeyCode::Char('^'), KeyModifiers::NONE),
        "~" => KeyEvent::new(KeyCode::Char('~'), KeyModifiers::NONE),
        "\\" => KeyEvent::new(KeyCode::Char('\\'), KeyModifiers::NONE),
        "|" => KeyEvent::new(KeyCode::Char('|'), KeyModifiers::NONE),
        "@" => KeyEvent::new(KeyCode::Char('@'), KeyModifiers::NONE),
        "[" => KeyEvent::new(KeyCode::Char('['), KeyModifiers::NONE),
        "]" => KeyEvent::new(KeyCode::Char(']'), KeyModifiers::NONE),
        ";" => KeyEvent::new(KeyCode::Char(';'), KeyModifiers::NONE),
        "+" => KeyEvent::new(KeyCode::Char('+'), KeyModifiers::NONE),
        ":" => KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE),
        "*" => KeyEvent::new(KeyCode::Char('*'), KeyModifiers::NONE),
        "?" => KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE),
        "/" => KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE),
        "," => KeyEvent::new(KeyCode::Char(','), KeyModifiers::NONE),
        "." => KeyEvent::new(KeyCode::Char('.'), KeyModifiers::NONE),
        "<" => KeyEvent::new(KeyCode::Char('<'), KeyModifiers::NONE),
        ">" => KeyEvent::new(KeyCode::Char('>'), KeyModifiers::NONE),
        "_" => KeyEvent::new(KeyCode::Char('_'), KeyModifiers::NONE),
        "S-a" => KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT),
        "S-b" => KeyEvent::new(KeyCode::Char('B'), KeyModifiers::SHIFT),
        "S-c" => KeyEvent::new(KeyCode::Char('C'), KeyModifiers::SHIFT),
        "S-d" => KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT),
        "S-e" => KeyEvent::new(KeyCode::Char('E'), KeyModifiers::SHIFT),
        "S-f" => KeyEvent::new(KeyCode::Char('F'), KeyModifiers::SHIFT),
        "S-g" => KeyEvent::new(KeyCode::Char('G'), KeyModifiers::SHIFT),
        "S-h" => KeyEvent::new(KeyCode::Char('H'), KeyModifiers::SHIFT),
        "S-i" => KeyEvent::new(KeyCode::Char('I'), KeyModifiers::SHIFT),
        "S-j" => KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT),
        "S-k" => KeyEvent::new(KeyCode::Char('K'), KeyModifiers::SHIFT),
        "S-l" => KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT),
        "S-m" => KeyEvent::new(KeyCode::Char('M'), KeyModifiers::SHIFT),
        "S-n" => KeyEvent::new(KeyCode::Char('N'), KeyModifiers::SHIFT),
        "S-o" => KeyEvent::new(KeyCode::Char('O'), KeyModifiers::SHIFT),
        "S-p" => KeyEvent::new(KeyCode::Char('P'), KeyModifiers::SHIFT),
        "S-q" => KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::SHIFT),
        "S-r" => KeyEvent::new(KeyCode::Char('R'), KeyModifiers::SHIFT),
        "S-s" => KeyEvent::new(KeyCode::Char('S'), KeyModifiers::SHIFT),
        "S-t" => KeyEvent::new(KeyCode::Char('T'), KeyModifiers::SHIFT),
        "S-u" => KeyEvent::new(KeyCode::Char('U'), KeyModifiers::SHIFT),
        "S-v" => KeyEvent::new(KeyCode::Char('V'), KeyModifiers::SHIFT),
        "S-w" => KeyEvent::new(KeyCode::Char('W'), KeyModifiers::SHIFT),
        "S-x" => KeyEvent::new(KeyCode::Char('X'), KeyModifiers::SHIFT),
        "S-y" => KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::SHIFT),
        "S-z" => KeyEvent::new(KeyCode::Char('Z'), KeyModifiers::SHIFT),
        "C-a" => KeyEvent::new(KeyCode::Char('A'), KeyModifiers::CONTROL),
        "C-b" => KeyEvent::new(KeyCode::Char('B'), KeyModifiers::CONTROL),
        "C-c" => KeyEvent::new(KeyCode::Char('C'), KeyModifiers::CONTROL),
        "C-d" => KeyEvent::new(KeyCode::Char('D'), KeyModifiers::CONTROL),
        "C-e" => KeyEvent::new(KeyCode::Char('E'), KeyModifiers::CONTROL),
        "C-f" => KeyEvent::new(KeyCode::Char('F'), KeyModifiers::CONTROL),
        "C-g" => KeyEvent::new(KeyCode::Char('G'), KeyModifiers::CONTROL),
        "C-h" => KeyEvent::new(KeyCode::Char('H'), KeyModifiers::CONTROL),
        "C-i" => KeyEvent::new(KeyCode::Char('I'), KeyModifiers::CONTROL),
        "C-j" => KeyEvent::new(KeyCode::Char('J'), KeyModifiers::CONTROL),
        "C-k" => KeyEvent::new(KeyCode::Char('K'), KeyModifiers::CONTROL),
        "C-l" => KeyEvent::new(KeyCode::Char('L'), KeyModifiers::CONTROL),
        "C-m" => KeyEvent::new(KeyCode::Char('M'), KeyModifiers::CONTROL),
        "C-n" => KeyEvent::new(KeyCode::Char('N'), KeyModifiers::CONTROL),
        "C-o" => KeyEvent::new(KeyCode::Char('O'), KeyModifiers::CONTROL),
        "C-p" => KeyEvent::new(KeyCode::Char('P'), KeyModifiers::CONTROL),
        "C-q" => KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::CONTROL),
        "C-r" => KeyEvent::new(KeyCode::Char('R'), KeyModifiers::CONTROL),
        "C-s" => KeyEvent::new(KeyCode::Char('S'), KeyModifiers::CONTROL),
        "C-t" => KeyEvent::new(KeyCode::Char('T'), KeyModifiers::CONTROL),
        "C-u" => KeyEvent::new(KeyCode::Char('U'), KeyModifiers::CONTROL),
        "C-v" => KeyEvent::new(KeyCode::Char('V'), KeyModifiers::CONTROL),
        "C-w" => KeyEvent::new(KeyCode::Char('W'), KeyModifiers::CONTROL),
        "C-x" => KeyEvent::new(KeyCode::Char('X'), KeyModifiers::CONTROL),
        "C-y" => KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::CONTROL),
        "C-z" => KeyEvent::new(KeyCode::Char('Z'), KeyModifiers::CONTROL),
        "A-a" => KeyEvent::new(KeyCode::Char('A'), KeyModifiers::ALT),
        "A-b" => KeyEvent::new(KeyCode::Char('B'), KeyModifiers::ALT),
        "A-c" => KeyEvent::new(KeyCode::Char('C'), KeyModifiers::ALT),
        "A-d" => KeyEvent::new(KeyCode::Char('D'), KeyModifiers::ALT),
        "A-e" => KeyEvent::new(KeyCode::Char('E'), KeyModifiers::ALT),
        "A-f" => KeyEvent::new(KeyCode::Char('F'), KeyModifiers::ALT),
        "A-g" => KeyEvent::new(KeyCode::Char('G'), KeyModifiers::ALT),
        "A-h" => KeyEvent::new(KeyCode::Char('H'), KeyModifiers::ALT),
        "A-i" => KeyEvent::new(KeyCode::Char('I'), KeyModifiers::ALT),
        "A-j" => KeyEvent::new(KeyCode::Char('J'), KeyModifiers::ALT),
        "A-k" => KeyEvent::new(KeyCode::Char('K'), KeyModifiers::ALT),
        "A-l" => KeyEvent::new(KeyCode::Char('L'), KeyModifiers::ALT),
        "A-m" => KeyEvent::new(KeyCode::Char('M'), KeyModifiers::ALT),
        "A-n" => KeyEvent::new(KeyCode::Char('N'), KeyModifiers::ALT),
        "A-o" => KeyEvent::new(KeyCode::Char('O'), KeyModifiers::ALT),
        "A-p" => KeyEvent::new(KeyCode::Char('P'), KeyModifiers::ALT),
        "A-q" => KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::ALT),
        "A-r" => KeyEvent::new(KeyCode::Char('R'), KeyModifiers::ALT),
        "A-s" => KeyEvent::new(KeyCode::Char('S'), KeyModifiers::ALT),
        "A-t" => KeyEvent::new(KeyCode::Char('T'), KeyModifiers::ALT),
        "A-u" => KeyEvent::new(KeyCode::Char('U'), KeyModifiers::ALT),
        "A-v" => KeyEvent::new(KeyCode::Char('V'), KeyModifiers::ALT),
        "A-w" => KeyEvent::new(KeyCode::Char('W'), KeyModifiers::ALT),
        "A-x" => KeyEvent::new(KeyCode::Char('X'), KeyModifiers::ALT),
        "A-y" => KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::ALT),
        "A-z" => KeyEvent::new(KeyCode::Char('Z'), KeyModifiers::ALT),
        _ => KeyEvent::new(KeyCode::Null, KeyModifiers::NONE),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UserKeyCode {
    pub first: KeyEvent,
    pub second: Option<KeyEvent>,
    // combo: Vec<KeyEvent>,
    combo: bool,
}

impl UserKeyCode {
    pub fn single_new(first: KeyEvent) -> Self {
        Self {
            first,
            second: None,
            // coc![],
            combo: false,
        }
    }

    pub fn multi_new(first: KeyEvent, second: KeyEvent) -> Self {
        Self {
            first,
            second: Some(second),
            // combo,
            combo: true,
        }
    }
}

type Keybind = HashMap<UserKeyCode, String>;
#[derive(Debug, PartialEq, Eq)]
pub struct UserKeybinds {
    single: Keybind,
    multi: Keybind,
    filtered_multi: Option<Keybind>,
    key: KeyEvent,
}

impl UserKeybinds {
    pub fn new() -> Self {
        Self {
            single: HashMap::new(),
            multi: HashMap::new(),
            filtered_multi: None,
            key: KeyEvent::new(KeyCode::Null, KeyModifiers::NONE),
        }
    }

    pub fn set_keyevent(&mut self, key: KeyEvent) {
        self.key = key;
    }

    pub fn matching_single_keys(&self) -> Option<String> {
        let mut filtered = self
            .single
            .iter()
            .filter(|(keycode, _cmd)| keycode.first == self.key && keycode.second.is_none());
        if filtered.size_hint().1.unwrap() == 0 {
            None
        } else if let Some((_, cmd)) = filtered.next() {
            Some(cmd.to_owned())
        } else {
            None
        }
    }

    pub fn filtering_multi_first_keys(&mut self) {
        let filtered_keybinds: HashMap<UserKeyCode, String> = self
            .multi
            .iter()
            .filter(|(keycode, _cmd)| keycode.first == self.key && keycode.second.is_some())
            .map(|(key, cmd)| (key.to_owned(), cmd.to_owned()))
            .collect();
        if filtered_keybinds.is_empty() {
            return;
        }

        self.filtered_multi = Some(filtered_keybinds);
    }

    pub fn matching_multi_second_keys(&self) -> Option<String> {
        self.filtered_multi.as_ref()?;

        if let Some((_, cmb_cmd)) = self
            .filtered_multi
            .as_ref()
            .unwrap()
            .iter()
            .find(|(keycode, _)| keycode.second.is_some() && keycode.second.unwrap() == self.key)
        {
            return Some(cmb_cmd.to_owned());
        }

        None
    }

    pub fn has_keycomb(&self) -> bool {
        self.filtered_multi.is_some()
    }

    pub fn make_single_keybinds(
        mut self,
        config_user_keybind: HashMap<Vec<KeyEvent>, String>,
    ) -> Self {
        let single: Keybind = config_user_keybind
            .into_iter()
            .filter(|(key, _cmd)| key.len() == 1)
            .map(|(x, c)| (UserKeyCode::single_new(x[0]), c))
            .collect();
        self.single = single;

        self
    }

    pub fn make_multiple_keybinds(
        mut self,
        config_user_keybind: HashMap<Vec<KeyEvent>, String>,
    ) -> Self {
        let multi: Keybind = config_user_keybind
            .into_iter()
            .filter(|(key, _cmd)| key.len() > 1)
            .map(|(x, c)| (UserKeyCode::multi_new(x[0], x[1]), c))
            .collect();
        self.multi = multi;

        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SettingTheme {
    background: Colors,
    border: Colors,
    directory: Colors,
    file_item: Colors,
    select: Colors,
    header: Colors,
    // (bg, fg): normal, input, stacker
    command: Vec<(Colors, Colors)>,
}

impl SettingTheme {
    fn dark_theme() -> SettingTheme {
        SettingTheme {
            background: Colors::Black,
            header: Colors::Cyan,
            border: Colors::White,
            directory: Colors::Blue,
            file_item: Colors::Gray,
            select: Colors::LightMagenta,
            command: vec![
                (Colors::Green, Colors::Cyan),
                (Colors::White, Colors::Blue),
                (Colors::Magenta, Colors::Red),
            ],
        }
    }

    fn light_theme() -> SettingTheme {
        SettingTheme {
            background: Colors::White,
            header: Colors::Green,
            border: Colors::Black,
            directory: Colors::Blue,
            file_item: Colors::Black,
            select: Colors::LightRed,
            command: vec![
                (Colors::Blue, Colors::White),
                (Colors::Green, Colors::White),
                (Colors::Magenta, Colors::White),
            ],
        }
    }

    fn dark_blue_theme() -> SettingTheme {
        SettingTheme {
            background: Colors::Rgb(39, 67, 100),
            header: Colors::Green,
            border: Colors::Rgb(97, 169, 252),
            directory: Colors::Blue,
            file_item: Colors::Gray,
            select: Colors::Green,
            command: vec![
                (Colors::Blue, Colors::Black),
                (Colors::Green, Colors::Blue),
                (Colors::Magenta, Colors::Blue),
            ],
        }
    }

    pub fn file_style(&self) -> Style {
        let user_color = self.file_item.clone();
        style_formatter(user_color, true, false)
    }

    pub fn dir_style(&self) -> Style {
        let user_color = self.directory.clone();
        style_formatter(user_color, true, false)
    }

    pub fn select_style(&self) -> Style {
        let user_color = self.select.clone();
        style_formatter(user_color, true, false)
    }

    pub fn header_style(&self) -> Style {
        let user_color = self.header.clone();
        style_formatter(user_color, true, false)
    }

    pub fn border_style(&self) -> Style {
        let user_color = self.border.clone();
        style_formatter(user_color, true, false)
    }

    pub fn command_style(&self) -> [Style; 3] {
        let user_color = self.command.clone();
        let mut styles: [Style; 3] = [Style::default(); 3];
        for (i, (bg, fg)) in user_color.into_iter().enumerate() {
            let bg = style_formatter(bg, true, false);
            let fg = style_formatter(fg, false, true);
            styles[i] = bg.patch(fg);
        }
        styles
    }

    pub fn background_style(&self) -> Style {
        let user_color = self.background.clone();
        style_formatter(user_color, false, true)
    }
}

fn color_translator(color: Colors) -> Option<Color> {
    let c = match color {
        Colors::White => Color::White,
        Colors::Black => Color::Black,
        Colors::Yellow => Color::Yellow,
        Colors::Blue => Color::Blue,
        Colors::Green => Color::Green,
        Colors::Red => Color::Red,
        Colors::Gray => Color::Gray,
        Colors::DarkGray => Color::DarkGray,
        Colors::Cyan => Color::Cyan,
        Colors::Magenta => Color::Magenta,
        Colors::LightBlue => Color::LightBlue,
        Colors::LightRed => Color::LightRed,
        Colors::LightGreen => Color::LightGreen,
        Colors::LightMagenta => Color::LightMagenta,
        Colors::LightYellow => Color::LightYellow,
        Colors::LightCyan => Color::LightCyan,
        Colors::Rgb(r, g, b) => Color::Rgb(r, g, b),
    };
    Some(c)
}

fn tui_color_transformer(color: Color) -> Colors {
    match color {
        Color::Reset => Colors::Rgb(0, 0, 0),
        Color::Black => Colors::Black,
        Color::Red => Colors::Red,
        Color::Green => Colors::Green,
        Color::Yellow => Colors::Yellow,
        Color::Blue => Colors::Blue,
        Color::Magenta => Colors::Magenta,
        Color::Cyan => Colors::Cyan,
        Color::Gray => Colors::Gray,
        Color::DarkGray => Colors::DarkGray,
        Color::LightRed => Colors::LightRed,
        Color::LightGreen => Colors::LightGreen,
        Color::LightYellow => Colors::LightYellow,
        Color::LightBlue => Colors::LightBlue,
        Color::LightMagenta => Colors::LightMagenta,
        Color::LightCyan => Colors::LightCyan,
        Color::White => Colors::White,
        Color::Rgb(r, g, b) => Colors::Rgb(r, g, b),
        Color::Indexed(_) => Colors::Rgb(255, 255, 255),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Hash)]
pub enum FileItems {
    File,
    Directory,
    Select,
}

fn example_symbols() -> HashMap<FileItems, String> {
    let map: HashMap<FileItems, String> = HashMap::from([
        (FileItems::File, "📜".to_string()),
        (FileItems::Directory, "📁".to_string()),
        (FileItems::Select, "⋙".to_string()),
    ]);
    map
}

fn simple_symbols() -> HashMap<FileItems, String> {
    let map: HashMap<FileItems, String> = HashMap::from([
        (FileItems::File, " ".to_string()),
        (FileItems::Directory, "▸".to_string()),
        (FileItems::Select, ">>".to_string()),
    ]);
    map
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModeKeybinds {
    pub normal: HashMap<String, String>,
    pub input: HashMap<String, String>,
    pub stacker: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserConfig {
    theme: SettingTheme,
    symbols: HashMap<FileItems, String>,
    user_keybinds: ModeKeybinds,
}

impl UserConfig {
    pub fn default_dark() -> UserConfig {
        UserConfig {
            theme: SettingTheme::dark_theme(),
            symbols: simple_symbols(),
            user_keybinds: default_vim_movements(),
        }
    }

    pub fn default_dark_blue() -> UserConfig {
        UserConfig {
            theme: SettingTheme::dark_blue_theme(),
            symbols: example_symbols(),
            user_keybinds: default_vim_control_key_movements(),
        }
    }

    pub fn default_light() -> UserConfig {
        UserConfig {
            theme: SettingTheme::light_theme(),
            symbols: example_symbols(),
            user_keybinds: default_arrow_key_keybindings(),
        }
    }
    pub fn symbols(&self) -> &HashMap<FileItems, String> {
        &self.symbols
    }

    pub fn theme(&self) -> &SettingTheme {
        &self.theme
    }

    fn keybindings_map(&self) -> ModeKeybinds {
        self.user_keybinds.clone()
    }

    pub fn normal_keybindings_map(&self) -> HashMap<String, String> {
        self.user_keybinds.normal.clone()
    }

    pub fn input_keybindings_map(&self) -> HashMap<String, String> {
        self.user_keybinds.input.clone()
    }

    pub fn stacker_keybindings_map(&self) -> HashMap<String, String> {
        self.user_keybinds.stacker.clone()
    }
}

fn style_formatter(color: Colors, is_fg: bool, is_bg: bool) -> Style {
    let color = color_translator(color).unwrap();
    match (is_fg, is_bg) {
        (true, true) => panic!("is_fg and is_bg is not always true"), // review required
        (true, false) => Style::default().fg(color),
        (false, true) => Style::default().bg(color),
        (false, false) => panic!("is_fg or is_bg is always true"),
    }
}

pub fn load_user_config_file() -> UserConfig {
    // Each Windows, Mac(Linux)
    // Consider specifying PATH in each OS
    let path = "config.ron";
    match std::fs::File::open(path) {
        Ok(f) => {
            let config: Result<UserConfig, ron::error::SpannedError> = ron::de::from_reader(f);
            if let Ok(config) = config {
                config
            } else {
                UserConfig::default_dark_blue()
            }
        }
        // TODO: logging this e
        Err(e) => UserConfig::default_dark(),
    }
}

#[cfg(test)]
mod test {
    use ron::error::SpannedError;

    use crate::load_config::UserConfig;

    #[test]
    fn can_read_ron_file() {
        let path = "config.ron";
        let f = std::fs::File::open(path);
        assert!(f.is_ok());
        let config: Result<UserConfig, SpannedError> = ron::de::from_reader(f.unwrap());
        match &config {
            Ok(_) => {}
            Err(e) => println!("{e:#?}"),
        }
        assert!(config.is_ok());
        let config = config.unwrap();
        let keybinds = config.keybindings_map();
        println!("{:#?}", keybinds);
    }
}
