use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;
use tui::style::{Color, Style};

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
        ("h", "move_to_parent_dir"),
        ("j", "move_to_next_file_item"),
        ("k", "move_to_prev_file_item"),
        ("l", "move_to_child_dir"),
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

pub fn default_arrow_key() -> ModeKeybinds {
    let mut normal: HashMap<String, String> = HashMap::new();
    let iter = [
        ("Left", "move_to_parent_dir"),
        ("Down", "move_to_next_file_item"),
        ("Up", "move_to_prev_file_item"),
        ("Right", "move_to_child_dir"),
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
        ("Left", "move_to_parent_dir"),
        ("Down", "move_to_next_file_item"),
        ("Up", "move_to_prev_file_item"),
        ("Right", "move_to_child_dir"),
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
        ("Left", "move_to_parent_dir"),
        ("Down", "move_to_next_file_item"),
        ("Up", "move_to_prev_file_item"),
        ("Right", "move_to_child_dir"),
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

pub fn default_vim_ctrl_movements() -> ModeKeybinds {
    let mut normal: HashMap<String, String> = HashMap::new();
    let iter = [
        ("C-h", "move_to_parent_dir"),
        ("C-j", "move_to_next_file_item"),
        ("C-k", "move_to_prev_file_item"),
        ("C-l", "move_to_child_dir"),
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
        ("C-h", "move_to_parent_dir"),
        ("C-j", "move_to_next_file_item"),
        ("C-k", "move_to_prev_file_item"),
        ("C-l", "move_to_child_dir"),
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

pub fn string_map_to_user_keyboad(keybinds: &HashMap<String, String>) -> HashMap<KeyEvent, String> {
    let mut keybind: HashMap<KeyEvent, String> = HashMap::new();
    for (key, cmd) in keybinds.iter() {
        let user_keyboad = string_to_keyevent(key);
        keybind.insert(user_keyboad, cmd.to_string());
    }
    keybind
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
        "a" => KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
        },
        "b" => KeyEvent {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::NONE,
        },
        "c" => KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::NONE,
        },
        "d" => KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::NONE,
        },
        "e" => KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::NONE,
        },
        "f" => KeyEvent {
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::NONE,
        },
        "g" => KeyEvent {
            code: KeyCode::Char('g'),
            modifiers: KeyModifiers::NONE,
        },
        "h" => KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::NONE,
        },
        "i" => KeyEvent {
            code: KeyCode::Char('i'),
            modifiers: KeyModifiers::NONE,
        },
        "j" => KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::NONE,
        },
        "k" => KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::NONE,
        },
        "l" => KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::NONE,
        },
        "m" => KeyEvent {
            code: KeyCode::Char('m'),
            modifiers: KeyModifiers::NONE,
        },
        "n" => KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::NONE,
        },
        "o" => KeyEvent {
            code: KeyCode::Char('o'),
            modifiers: KeyModifiers::NONE,
        },
        "p" => KeyEvent {
            code: KeyCode::Char('p'),
            modifiers: KeyModifiers::NONE,
        },
        "q" => KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
        },
        "r" => KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::NONE,
        },
        "s" => KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::NONE,
        },
        "t" => KeyEvent {
            code: KeyCode::Char('t'),
            modifiers: KeyModifiers::NONE,
        },
        "u" => KeyEvent {
            code: KeyCode::Char('u'),
            modifiers: KeyModifiers::NONE,
        },
        "v" => KeyEvent {
            code: KeyCode::Char('v'),
            modifiers: KeyModifiers::NONE,
        },
        "w" => KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::NONE,
        },
        "x" => KeyEvent {
            code: KeyCode::Char('x'),
            modifiers: KeyModifiers::NONE,
        },
        "y" => KeyEvent {
            code: KeyCode::Char('y'),
            modifiers: KeyModifiers::NONE,
        },
        "z" => KeyEvent {
            code: KeyCode::Char('z'),
            modifiers: KeyModifiers::NONE,
        },
        "#" => KeyEvent {
            code: KeyCode::Char('#'),
            modifiers: KeyModifiers::NONE,
        },
        "!" => KeyEvent {
            code: KeyCode::Char('!'),
            modifiers: KeyModifiers::NONE,
        },
        "$" => KeyEvent {
            code: KeyCode::Char('$'),
            modifiers: KeyModifiers::NONE,
        },
        "%" => KeyEvent {
            code: KeyCode::Char('%'),
            modifiers: KeyModifiers::NONE,
        },
        "&" => KeyEvent {
            code: KeyCode::Char('&'),
            modifiers: KeyModifiers::NONE,
        },
        "'" => KeyEvent {
            code: KeyCode::Char('\''),
            modifiers: KeyModifiers::NONE,
        },
        "(" => KeyEvent {
            code: KeyCode::Char('('),
            modifiers: KeyModifiers::NONE,
        },
        ")" => KeyEvent {
            code: KeyCode::Char(')'),
            modifiers: KeyModifiers::NONE,
        },
        "-" => KeyEvent {
            code: KeyCode::Char('-'),
            modifiers: KeyModifiers::NONE,
        },
        "=" => KeyEvent {
            code: KeyCode::Char('='),
            modifiers: KeyModifiers::NONE,
        },
        "^" => KeyEvent {
            code: KeyCode::Char('^'),
            modifiers: KeyModifiers::NONE,
        },
        "~" => KeyEvent {
            code: KeyCode::Char('~'),
            modifiers: KeyModifiers::NONE,
        },
        "\\" => KeyEvent {
            code: KeyCode::Char('\\'),
            modifiers: KeyModifiers::NONE,
        },
        "|" => KeyEvent {
            code: KeyCode::Char('|'),
            modifiers: KeyModifiers::NONE,
        },
        "@" => KeyEvent {
            code: KeyCode::Char('@'),
            modifiers: KeyModifiers::NONE,
        },
        "[" => KeyEvent {
            code: KeyCode::Char('['),
            modifiers: KeyModifiers::NONE,
        },
        "]" => KeyEvent {
            code: KeyCode::Char(']'),
            modifiers: KeyModifiers::NONE,
        },
        ";" => KeyEvent {
            code: KeyCode::Char(';'),
            modifiers: KeyModifiers::NONE,
        },
        "+" => KeyEvent {
            code: KeyCode::Char('+'),
            modifiers: KeyModifiers::NONE,
        },
        ":" => KeyEvent {
            code: KeyCode::Char(':'),
            modifiers: KeyModifiers::NONE,
        },
        "*" => KeyEvent {
            code: KeyCode::Char('*'),
            modifiers: KeyModifiers::NONE,
        },
        "?" => KeyEvent {
            code: KeyCode::Char('?'),
            modifiers: KeyModifiers::NONE,
        },
        "/" => KeyEvent {
            code: KeyCode::Char('/'),
            modifiers: KeyModifiers::NONE,
        },
        "," => KeyEvent {
            code: KeyCode::Char(','),
            modifiers: KeyModifiers::NONE,
        },
        "." => KeyEvent {
            code: KeyCode::Char('.'),
            modifiers: KeyModifiers::NONE,
        },
        "<" => KeyEvent {
            code: KeyCode::Char('<'),
            modifiers: KeyModifiers::NONE,
        },
        ">" => KeyEvent {
            code: KeyCode::Char('>'),
            modifiers: KeyModifiers::NONE,
        },
        "_" => KeyEvent {
            code: KeyCode::Char('_'),
            modifiers: KeyModifiers::NONE,
        },
        "escape" => KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
        },
        "tab" => KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
        },
        "S-tab" => KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
        },
        "f1" => KeyEvent {
            code: KeyCode::F(1),
            modifiers: KeyModifiers::NONE,
        },
        "f2" => KeyEvent {
            code: KeyCode::F(2),
            modifiers: KeyModifiers::NONE,
        },
        "f3" => KeyEvent {
            code: KeyCode::F(3),
            modifiers: KeyModifiers::NONE,
        },
        "f4" => KeyEvent {
            code: KeyCode::F(4),
            modifiers: KeyModifiers::NONE,
        },
        "f5" => KeyEvent {
            code: KeyCode::F(5),
            modifiers: KeyModifiers::NONE,
        },
        "f6" => KeyEvent {
            code: KeyCode::F(6),
            modifiers: KeyModifiers::NONE,
        },
        "f7" => KeyEvent {
            code: KeyCode::F(7),
            modifiers: KeyModifiers::NONE,
        },
        "f8" => KeyEvent {
            code: KeyCode::F(8),
            modifiers: KeyModifiers::NONE,
        },
        "f9" => KeyEvent {
            code: KeyCode::F(9),
            modifiers: KeyModifiers::NONE,
        },
        "f10" => KeyEvent {
            code: KeyCode::F(10),
            modifiers: KeyModifiers::NONE,
        },
        "f11" => KeyEvent {
            code: KeyCode::F(11),
            modifiers: KeyModifiers::NONE,
        },
        "f12" => KeyEvent {
            code: KeyCode::F(12),
            modifiers: KeyModifiers::NONE,
        },
        "S-a" => KeyEvent {
            code: KeyCode::Char('A'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-b" => KeyEvent {
            code: KeyCode::Char('B'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-c" => KeyEvent {
            code: KeyCode::Char('C'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-d" => KeyEvent {
            code: KeyCode::Char('D'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-e" => KeyEvent {
            code: KeyCode::Char('E'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-f" => KeyEvent {
            code: KeyCode::Char('F'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-g" => KeyEvent {
            code: KeyCode::Char('G'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-h" => KeyEvent {
            code: KeyCode::Char('H'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-i" => KeyEvent {
            code: KeyCode::Char('I'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-j" => KeyEvent {
            code: KeyCode::Char('J'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-k" => KeyEvent {
            code: KeyCode::Char('K'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-l" => KeyEvent {
            code: KeyCode::Char('L'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-m" => KeyEvent {
            code: KeyCode::Char('M'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-n" => KeyEvent {
            code: KeyCode::Char('N'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-o" => KeyEvent {
            code: KeyCode::Char('O'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-p" => KeyEvent {
            code: KeyCode::Char('P'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-q" => KeyEvent {
            code: KeyCode::Char('Q'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-r" => KeyEvent {
            code: KeyCode::Char('R'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-s" => KeyEvent {
            code: KeyCode::Char('S'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-t" => KeyEvent {
            code: KeyCode::Char('T'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-u" => KeyEvent {
            code: KeyCode::Char('U'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-v" => KeyEvent {
            code: KeyCode::Char('V'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-w" => KeyEvent {
            code: KeyCode::Char('W'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-x" => KeyEvent {
            code: KeyCode::Char('X'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-y" => KeyEvent {
            code: KeyCode::Char('Y'),
            modifiers: KeyModifiers::SHIFT,
        },
        "S-z" => KeyEvent {
            code: KeyCode::Char('Z'),
            modifiers: KeyModifiers::SHIFT,
        },
        "C-a" => KeyEvent {
            code: KeyCode::Char('A'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-b" => KeyEvent {
            code: KeyCode::Char('B'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-c" => KeyEvent {
            code: KeyCode::Char('C'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-d" => KeyEvent {
            code: KeyCode::Char('D'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-e" => KeyEvent {
            code: KeyCode::Char('E'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-f" => KeyEvent {
            code: KeyCode::Char('F'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-g" => KeyEvent {
            code: KeyCode::Char('G'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-h" => KeyEvent {
            code: KeyCode::Char('H'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-i" => KeyEvent {
            code: KeyCode::Char('I'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-j" => KeyEvent {
            code: KeyCode::Char('J'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-k" => KeyEvent {
            code: KeyCode::Char('K'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-l" => KeyEvent {
            code: KeyCode::Char('L'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-m" => KeyEvent {
            code: KeyCode::Char('M'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-n" => KeyEvent {
            code: KeyCode::Char('N'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-o" => KeyEvent {
            code: KeyCode::Char('O'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-p" => KeyEvent {
            code: KeyCode::Char('P'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-q" => KeyEvent {
            code: KeyCode::Char('Q'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-r" => KeyEvent {
            code: KeyCode::Char('R'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-s" => KeyEvent {
            code: KeyCode::Char('S'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-t" => KeyEvent {
            code: KeyCode::Char('T'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-u" => KeyEvent {
            code: KeyCode::Char('U'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-v" => KeyEvent {
            code: KeyCode::Char('V'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-w" => KeyEvent {
            code: KeyCode::Char('W'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-x" => KeyEvent {
            code: KeyCode::Char('X'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-y" => KeyEvent {
            code: KeyCode::Char('Y'),
            modifiers: KeyModifiers::CONTROL,
        },
        "C-z" => KeyEvent {
            code: KeyCode::Char('Z'),
            modifiers: KeyModifiers::CONTROL,
        },
        "A-a" => KeyEvent {
            code: KeyCode::Char('A'),
            modifiers: KeyModifiers::ALT,
        },
        "A-b" => KeyEvent {
            code: KeyCode::Char('B'),
            modifiers: KeyModifiers::ALT,
        },
        "A-c" => KeyEvent {
            code: KeyCode::Char('C'),
            modifiers: KeyModifiers::ALT,
        },
        "A-d" => KeyEvent {
            code: KeyCode::Char('D'),
            modifiers: KeyModifiers::ALT,
        },
        "A-e" => KeyEvent {
            code: KeyCode::Char('E'),
            modifiers: KeyModifiers::ALT,
        },
        "A-f" => KeyEvent {
            code: KeyCode::Char('F'),
            modifiers: KeyModifiers::ALT,
        },
        "A-g" => KeyEvent {
            code: KeyCode::Char('G'),
            modifiers: KeyModifiers::ALT,
        },
        "A-h" => KeyEvent {
            code: KeyCode::Char('H'),
            modifiers: KeyModifiers::ALT,
        },
        "A-i" => KeyEvent {
            code: KeyCode::Char('I'),
            modifiers: KeyModifiers::ALT,
        },
        "A-j" => KeyEvent {
            code: KeyCode::Char('J'),
            modifiers: KeyModifiers::ALT,
        },
        "A-k" => KeyEvent {
            code: KeyCode::Char('K'),
            modifiers: KeyModifiers::ALT,
        },
        "A-l" => KeyEvent {
            code: KeyCode::Char('L'),
            modifiers: KeyModifiers::ALT,
        },
        "A-m" => KeyEvent {
            code: KeyCode::Char('M'),
            modifiers: KeyModifiers::ALT,
        },
        "A-n" => KeyEvent {
            code: KeyCode::Char('N'),
            modifiers: KeyModifiers::ALT,
        },
        "A-o" => KeyEvent {
            code: KeyCode::Char('O'),
            modifiers: KeyModifiers::ALT,
        },
        "A-p" => KeyEvent {
            code: KeyCode::Char('P'),
            modifiers: KeyModifiers::ALT,
        },
        "A-q" => KeyEvent {
            code: KeyCode::Char('Q'),
            modifiers: KeyModifiers::ALT,
        },
        "A-r" => KeyEvent {
            code: KeyCode::Char('R'),
            modifiers: KeyModifiers::ALT,
        },
        "A-s" => KeyEvent {
            code: KeyCode::Char('S'),
            modifiers: KeyModifiers::ALT,
        },
        "A-t" => KeyEvent {
            code: KeyCode::Char('T'),
            modifiers: KeyModifiers::ALT,
        },
        "A-u" => KeyEvent {
            code: KeyCode::Char('U'),
            modifiers: KeyModifiers::ALT,
        },
        "A-v" => KeyEvent {
            code: KeyCode::Char('V'),
            modifiers: KeyModifiers::ALT,
        },
        "A-w" => KeyEvent {
            code: KeyCode::Char('W'),
            modifiers: KeyModifiers::ALT,
        },
        "A-x" => KeyEvent {
            code: KeyCode::Char('X'),
            modifiers: KeyModifiers::ALT,
        },
        "A-y" => KeyEvent {
            code: KeyCode::Char('Y'),
            modifiers: KeyModifiers::ALT,
        },
        "A-z" => KeyEvent {
            code: KeyCode::Char('Z'),
            modifiers: KeyModifiers::ALT,
        },
        _ => KeyEvent {
            code: KeyCode::Null,
            modifiers: KeyModifiers::NONE,
        },
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
            key: KeyEvent {
                code: KeyCode::Null,
                modifiers: KeyModifiers::NONE,
            },
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
    boader: Colors,
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
            boader: Colors::White,
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
            boader: Colors::Black,
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
            boader: Colors::Rgb(97, 169, 252),
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
        let user_color = &self.file_item;
        style_formatter(user_color, true, false)
    }

    pub fn dir_style(&self) -> Style {
        let user_color = &self.directory;
        style_formatter(user_color, true, false)
    }

    pub fn select_style(&self) -> Style {
        let user_color = &self.select;
        style_formatter(user_color, true, false)
    }

    pub fn header_style(&self) -> Style {
        let user_color = &self.header;
        style_formatter(user_color, true, false)
    }

    pub fn boader_style(&self) -> Style {
        let user_color = &self.boader;
        style_formatter(user_color, true, false)
    }

    pub fn command_styles(&self) -> [Style; 3] {
        let user_color = &self.command;
        let mut styles: [Style; 3] = [Style::default(); 3];
        for (i, (bg, fg)) in user_color.iter().enumerate() {
            let fg = style_formatter(fg, true, false);
            let bg = style_formatter(bg, false, true);
            styles[i] = bg.patch(fg);
        }
        styles
    }

    pub fn command_style(&self, i: usize) -> Option<Style> {
        if let Some(command_color) = &self.command.get(i) {
            let fg = style_formatter(&command_color.0, true, false);
            let bg = style_formatter(&command_color.1, false, true);
            return Some(fg.patch(bg));
        }
        None
    }

    pub fn background_style(&self) -> Style {
        let user_color = &self.background;
        style_formatter(user_color, false, true)
    }
}

fn color_translator(color: &Colors) -> Option<Color> {
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
        Colors::Rgb(r, g, b) => Color::Rgb(*r, *g, *b),
        _ => Color::Reset,
    };
    Some(c)
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
            user_keybinds: default_vim_ctrl_movements(),
        }
    }

    pub fn default_light() -> UserConfig {
        UserConfig {
            theme: SettingTheme::light_theme(),
            symbols: example_symbols(),
            user_keybinds: default_arrow_key(),
        }
    }
    pub fn symbols(&self) -> &HashMap<FileItems, String> {
        &self.symbols
    }

    pub fn theme(&self) -> &SettingTheme {
        &self.theme
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

fn style_formatter(color: &Colors, is_fg: bool, is_bg: bool) -> Style {
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
            let config: Result<UserConfig, ron::de::Error> = ron::de::from_reader(f);
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
    use ron::de;

    use crate::load_config::UserConfig;

    #[test]
    fn can_read_ron_file() {
        let path = "config.ron";
        let f = std::fs::File::open(path);
        assert!(f.is_ok());
        let config: Result<UserConfig, de::Error> = ron::de::from_reader(f.unwrap());
        match &config {
            Ok(_) => {}
            Err(e) => println!("{e:#?}"),
        }
        assert!(config.is_ok());
        let config = config.unwrap();
        let keybinds = config.input_keybindings_map();
        println!("{:#?}", keybinds);
    }
}
