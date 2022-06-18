use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;
use tui::style::{Color, Modifier, Style};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserKeyboad {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    ShiftA,
    ShiftB,
    ShiftC,
    ShiftD,
    ShiftE,
    ShiftF,
    ShiftG,
    ShiftH,
    ShiftI,
    ShiftJ,
    SHiftK,
    ShiftL,
    ShiftM,
    ShiftN,
    ShiftO,
    ShiftP,
    ShiftQ,
    ShiftR,
    ShiftS,
    ShiftT,
    ShiftU,
    ShiftV,
    ShiftW,
    ShiftX,
    ShiftY,
    ShiftZ,
    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlG,
    CtrlH,
    CtrlI,
    CtrlJ,
    CtrlK,
    CtrlL,
    CtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,
    AltA,
    AltB,
    AltC,
    AltD,
    AltE,
    AltF,
    AltG,
    AltH,
    AltI,
    AltJ,
    AltK,
    AltL,
    AltM,
    AltN,
    AltO,
    AltP,
    AltQ,
    AltR,
    AltS,
    AltT,
    AltU,
    AltV,
    AltW,
    AltX,
    AltY,
    AltZ,
    Left,
    Up,
    Down,
    Right,
    Escape,
    Enter,
    Tab,
    Backtab,
    Backspace,
    Shape,
    Period,
    Plus,
    Minus,
    GT,
    LT,
    QuestionMark,
    ExclamationMark,
    Comma,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Home,
    End,
    Space,
    Unknown,
}

// TODO: chang {Key: Command}
// TODO: add function to insert new {key: cmd} easily
pub fn default_vim_movements() -> HashMap<String, String> {
    let mut keybinds: HashMap<String, String> = HashMap::new();
    keybinds.insert("h".to_string(), "move_to_parent_dir".to_string());
    keybinds.insert("j".to_string(), "move_to_next_file_item".to_string());
    keybinds.insert("k".to_string(), "move_to_prev_file_item".to_string());
    keybinds.insert("l".to_string(), "move_to_child_dir".to_string());
    keybinds.insert("Tab".to_string(), "next_dirtab".to_string());
    keybinds.insert("Backtab".to_string(), "prev_dirtab".to_string());
    keybinds.insert("q".to_string(), "quit".to_string());

    keybinds
}

pub fn default_arrow_key() -> HashMap<String, String> {
    let mut keybinds: HashMap<String, String> = HashMap::new();
    keybinds.insert("left".to_string(), "move_to_parent_dir".to_string());
    keybinds.insert("down".to_string(), "move_to_next_file_item".to_string());
    keybinds.insert("up".to_string(), "move_to_prev_file_item".to_string());
    keybinds.insert("right".to_string(), "move_to_child_dir".to_string());
    keybinds.insert("Tab".to_string(), "next_dirtab".to_string());
    keybinds.insert("Backtab".to_string(), "prev_dirtab".to_string());
    keybinds.insert("q".to_string(), "quit".to_string());

    keybinds
}

pub fn default_vim_ctrl_movements() -> HashMap<String, String> {
    let mut keybinds: HashMap<String, String> = HashMap::new();
    keybinds.insert("C-h".to_string(), "move_to_parent_dir".to_string());
    keybinds.insert("C-j".to_string(), "move_to_next_file_item".to_string());
    keybinds.insert("C-k".to_string(), "move_to_prev_file_item".to_string());
    keybinds.insert("C-l".to_string(), "move_to_child_dir".to_string());
    keybinds.insert("Tab".to_string(), "next_dirtab".to_string());
    keybinds.insert("Backtab".to_string(), "prev_dirtab".to_string());
    keybinds.insert("q".to_string(), "quit".to_string());

    keybinds
}

pub fn string_map_to_user_keyboad(
    keybinds: &HashMap<String, String>,
) -> HashMap<UserKeyboad, String> {
    let mut keybind: HashMap<UserKeyboad, String> = HashMap::new();
    for (key, cmd) in keybinds.iter() {
        let user_keyboad = string_to_keyboard(key);
        keybind.insert(user_keyboad, cmd.to_string());
    }
    keybind
}

fn string_to_keyboard(s: &str) -> UserKeyboad {
    match s {
        "a" => UserKeyboad::A,
        "b" => UserKeyboad::B,
        "c" => UserKeyboad::C,
        "d" => UserKeyboad::D,
        "e" => UserKeyboad::E,
        "f" => UserKeyboad::F,
        "g" => UserKeyboad::G,
        "h" => UserKeyboad::H,
        "i" => UserKeyboad::I,
        "j" => UserKeyboad::J,
        "k" => UserKeyboad::K,
        "l" => UserKeyboad::L,
        "m" => UserKeyboad::M,
        "n" => UserKeyboad::N,
        "o" => UserKeyboad::O,
        "p" => UserKeyboad::P,
        "q" => UserKeyboad::Q,
        "r" => UserKeyboad::R,
        "s" => UserKeyboad::S,
        "t" => UserKeyboad::T,
        "u" => UserKeyboad::U,
        "v" => UserKeyboad::V,
        "w" => UserKeyboad::W,
        "x" => UserKeyboad::X,
        "y" => UserKeyboad::Y,
        "z" => UserKeyboad::Z,
        "S-a" => UserKeyboad::ShiftA,
        "S-b" => UserKeyboad::ShiftB,
        "S-c" => UserKeyboad::ShiftC,
        "S-d" => UserKeyboad::ShiftD,
        "S-e" => UserKeyboad::ShiftE,
        "S-f" => UserKeyboad::ShiftF,
        "S-g" => UserKeyboad::ShiftG,
        "S-h" => UserKeyboad::ShiftH,
        "S-i" => UserKeyboad::ShiftI,
        "S-j" => UserKeyboad::ShiftJ,
        "S-k" => UserKeyboad::SHiftK,
        "S-l" => UserKeyboad::ShiftL,
        "S-m" => UserKeyboad::ShiftM,
        "S-n" => UserKeyboad::ShiftN,
        "S-o" => UserKeyboad::ShiftO,
        "S-p" => UserKeyboad::ShiftP,
        "S-q" => UserKeyboad::ShiftQ,
        "S-r" => UserKeyboad::ShiftR,
        "S-s" => UserKeyboad::ShiftS,
        "S-t" => UserKeyboad::ShiftT,
        "S-u" => UserKeyboad::ShiftU,
        "S-v" => UserKeyboad::ShiftV,
        "S-w" => UserKeyboad::ShiftW,
        "S-x" => UserKeyboad::ShiftX,
        "S-y" => UserKeyboad::ShiftY,
        "S-z" => UserKeyboad::ShiftZ,
        "C-a" => UserKeyboad::CtrlA,
        "C-b" => UserKeyboad::CtrlB,
        "C-c" => UserKeyboad::CtrlC,
        "C-d" => UserKeyboad::CtrlD,
        "C-e" => UserKeyboad::CtrlE,
        "C-f" => UserKeyboad::CtrlF,
        "C-g" => UserKeyboad::CtrlG,
        "C-h" => UserKeyboad::CtrlH,
        "C-i" => UserKeyboad::CtrlI,
        "C-j" => UserKeyboad::CtrlJ,
        "C-k" => UserKeyboad::CtrlK,
        "C-l" => UserKeyboad::CtrlL,
        "C-m" => UserKeyboad::CtrlM,
        "C-n" => UserKeyboad::CtrlN,
        "C-o" => UserKeyboad::CtrlO,
        "C-p" => UserKeyboad::CtrlP,
        "C-q" => UserKeyboad::CtrlQ,
        "C-r" => UserKeyboad::CtrlR,
        "C-s" => UserKeyboad::CtrlS,
        "C-t" => UserKeyboad::CtrlT,
        "C-u" => UserKeyboad::CtrlU,
        "C-v" => UserKeyboad::CtrlV,
        "C-w" => UserKeyboad::CtrlW,
        "C-x" => UserKeyboad::CtrlX,
        "C-y" => UserKeyboad::CtrlY,
        "C-z" => UserKeyboad::CtrlZ,
        "A-a" => UserKeyboad::AltA,
        "A-b" => UserKeyboad::AltB,
        "A-c" => UserKeyboad::AltC,
        "A-d" => UserKeyboad::AltD,
        "A-e" => UserKeyboad::AltE,
        "A-f" => UserKeyboad::AltF,
        "A-g" => UserKeyboad::AltG,
        "A-h" => UserKeyboad::AltH,
        "A-i" => UserKeyboad::AltI,
        "A-j" => UserKeyboad::AltJ,
        "A-k" => UserKeyboad::AltK,
        "A-l" => UserKeyboad::AltL,
        "A-m" => UserKeyboad::AltM,
        "A-n" => UserKeyboad::AltN,
        "A-o" => UserKeyboad::AltO,
        "A-p" => UserKeyboad::AltP,
        "A-q" => UserKeyboad::AltQ,
        "A-r" => UserKeyboad::AltR,
        "A-s" => UserKeyboad::AltS,
        "A-t" => UserKeyboad::AltT,
        "A-u" => UserKeyboad::AltU,
        "A-v" => UserKeyboad::AltV,
        "A-w" => UserKeyboad::AltW,
        "A-x" => UserKeyboad::AltX,
        "A-y" => UserKeyboad::AltY,
        "A-z" => UserKeyboad::AltZ,
        "left" => UserKeyboad::Left,
        "up" => UserKeyboad::Up,
        "down" => UserKeyboad::Down,
        "right" => UserKeyboad::Right,
        "escape" => UserKeyboad::Escape,
        "enter" => UserKeyboad::Enter,
        "tab" => UserKeyboad::Tab,
        "Backtab" => UserKeyboad::Backtab,
        "backspace" => UserKeyboad::Backspace,
        "#" => UserKeyboad::Shape,
        "." => UserKeyboad::Period,
        "+" => UserKeyboad::Plus,
        "-" => UserKeyboad::Minus,
        ">" => UserKeyboad::GT,
        "<" => UserKeyboad::LT,
        "?" => UserKeyboad::QuestionMark,
        "!" => UserKeyboad::ExclamationMark,
        "," => UserKeyboad::Comma,
        "f1" => UserKeyboad::F1,
        "f2" => UserKeyboad::F2,
        "f3" => UserKeyboad::F3,
        "f4" => UserKeyboad::F4,
        "f5" => UserKeyboad::F5,
        "f6" => UserKeyboad::F6,
        "f7" => UserKeyboad::F7,
        "f8" => UserKeyboad::F8,
        "f9" => UserKeyboad::F9,
        "f10" => UserKeyboad::F10,
        "f11" => UserKeyboad::F11,
        "f12" => UserKeyboad::F12,
        "home" => UserKeyboad::Home,
        "end" => UserKeyboad::End,
        "space" => UserKeyboad::Space,
        _ => UserKeyboad::Unknown,
    }
}

pub fn mappings_crossterm_keyevent_to_userkeyboad(key: &KeyEvent) -> UserKeyboad {
    match (key.code, key.modifiers) {
        (KeyCode::Char(c), KeyModifiers::CONTROL) => match c {
            'a' => UserKeyboad::CtrlA,
            'b' => UserKeyboad::CtrlB,
            'c' => UserKeyboad::CtrlC,
            'd' => UserKeyboad::CtrlD,
            'e' => UserKeyboad::CtrlE,
            'f' => UserKeyboad::CtrlF,
            'g' => UserKeyboad::CtrlG,
            'h' => UserKeyboad::CtrlH,
            'i' => UserKeyboad::CtrlI,
            'j' => UserKeyboad::CtrlJ,
            'k' => UserKeyboad::CtrlK,
            'l' => UserKeyboad::CtrlL,
            'm' => UserKeyboad::CtrlM,
            'n' => UserKeyboad::CtrlN,
            'o' => UserKeyboad::CtrlO,
            'p' => UserKeyboad::CtrlP,
            'q' => UserKeyboad::CtrlQ,
            'r' => UserKeyboad::CtrlR,
            's' => UserKeyboad::CtrlS,
            't' => UserKeyboad::CtrlT,
            'u' => UserKeyboad::CtrlU,
            'v' => UserKeyboad::CtrlV,
            'w' => UserKeyboad::CtrlW,
            'x' => UserKeyboad::CtrlX,
            'y' => UserKeyboad::CtrlY,
            'z' => UserKeyboad::CtrlZ,
            _ => UserKeyboad::Unknown,
        },
        (KeyCode::Char(c), KeyModifiers::SHIFT) => match c {
            'A' => UserKeyboad::ShiftA,
            'B' => UserKeyboad::ShiftB,
            'C' => UserKeyboad::ShiftC,
            'D' => UserKeyboad::ShiftD,
            'E' => UserKeyboad::ShiftE,
            'F' => UserKeyboad::ShiftF,
            'G' => UserKeyboad::ShiftG,
            'H' => UserKeyboad::ShiftH,
            'I' => UserKeyboad::ShiftI,
            'J' => UserKeyboad::ShiftJ,
            'K' => UserKeyboad::SHiftK,
            'L' => UserKeyboad::ShiftL,
            'M' => UserKeyboad::ShiftM,
            'N' => UserKeyboad::ShiftN,
            'O' => UserKeyboad::ShiftO,
            'P' => UserKeyboad::ShiftP,
            'Q' => UserKeyboad::ShiftQ,
            'R' => UserKeyboad::ShiftR,
            'S' => UserKeyboad::ShiftS,
            'T' => UserKeyboad::ShiftT,
            'U' => UserKeyboad::ShiftU,
            'V' => UserKeyboad::ShiftV,
            'W' => UserKeyboad::ShiftW,
            'X' => UserKeyboad::ShiftX,
            'Y' => UserKeyboad::ShiftY,
            'Z' => UserKeyboad::ShiftZ,
            _ => UserKeyboad::Unknown,
        },
        (KeyCode::Char(c), KeyModifiers::ALT) => match c {
            'a' => UserKeyboad::AltA,
            'b' => UserKeyboad::AltB,
            'c' => UserKeyboad::AltC,
            'd' => UserKeyboad::AltD,
            'e' => UserKeyboad::AltE,
            'f' => UserKeyboad::AltF,
            'g' => UserKeyboad::AltG,
            'h' => UserKeyboad::AltH,
            'i' => UserKeyboad::AltI,
            'j' => UserKeyboad::AltJ,
            'k' => UserKeyboad::AltK,
            'l' => UserKeyboad::AltL,
            'm' => UserKeyboad::AltM,
            'n' => UserKeyboad::AltN,
            'o' => UserKeyboad::AltO,
            'p' => UserKeyboad::AltP,
            'q' => UserKeyboad::AltQ,
            'r' => UserKeyboad::AltR,
            's' => UserKeyboad::AltS,
            't' => UserKeyboad::AltT,
            'u' => UserKeyboad::AltU,
            'v' => UserKeyboad::AltV,
            'w' => UserKeyboad::AltW,
            'x' => UserKeyboad::AltX,
            'y' => UserKeyboad::AltY,
            'z' => UserKeyboad::AltZ,
            _ => UserKeyboad::Unknown,
        },
        (KeyCode::Char(c), _) => match c {
            'a' => UserKeyboad::A,
            'b' => UserKeyboad::B,
            'c' => UserKeyboad::C,
            'd' => UserKeyboad::D,
            'e' => UserKeyboad::E,
            'f' => UserKeyboad::F,
            'g' => UserKeyboad::G,
            'h' => UserKeyboad::H,
            'i' => UserKeyboad::I,
            'j' => UserKeyboad::J,
            'k' => UserKeyboad::K,
            'l' => UserKeyboad::L,
            'm' => UserKeyboad::M,
            'n' => UserKeyboad::N,
            'o' => UserKeyboad::O,
            'p' => UserKeyboad::P,
            'q' => UserKeyboad::Q,
            'r' => UserKeyboad::R,
            's' => UserKeyboad::S,
            't' => UserKeyboad::T,
            'u' => UserKeyboad::U,
            'v' => UserKeyboad::V,
            'w' => UserKeyboad::W,
            'x' => UserKeyboad::X,
            'y' => UserKeyboad::Y,
            'z' => UserKeyboad::Z,
            '#' => UserKeyboad::Shape,
            '.' => UserKeyboad::Period,
            '+' => UserKeyboad::Plus,
            '-' => UserKeyboad::Minus,
            '>' => UserKeyboad::GT,
            '<' => UserKeyboad::LT,
            '?' => UserKeyboad::QuestionMark,
            '!' => UserKeyboad::ExclamationMark,
            ',' => UserKeyboad::Comma,
            ' ' => UserKeyboad::Space,
            _ => UserKeyboad::Unknown,
        },
        (KeyCode::F(n), _) => match n {
            1 => UserKeyboad::F1,
            2 => UserKeyboad::F2,
            3 => UserKeyboad::F3,
            4 => UserKeyboad::F4,
            5 => UserKeyboad::F5,
            6 => UserKeyboad::F6,
            7 => UserKeyboad::F7,
            8 => UserKeyboad::F8,
            9 => UserKeyboad::F9,
            10 => UserKeyboad::F10,
            11 => UserKeyboad::F11,
            12 => UserKeyboad::F12,
            _ => UserKeyboad::Unknown,
        },
        (KeyCode::BackTab, _) => UserKeyboad::Backtab,
        (KeyCode::Esc, _) => UserKeyboad::Escape,
        (KeyCode::Left, _) => UserKeyboad::Left,
        (KeyCode::Up, _) => UserKeyboad::Up,
        (KeyCode::Down, _) => UserKeyboad::Down,
        (KeyCode::Right, _) => UserKeyboad::Right,
        (KeyCode::Home, _) => UserKeyboad::Home,
        (KeyCode::End, _) => UserKeyboad::End,
        (KeyCode::Tab, _) => UserKeyboad::Tab,
        (_, _) => todo!(),
    }
}

// pub fn simgle_mapping_crossterm_keycode_to_commands(key: &KeyEvent) -> UserKeyboad {
//     match key.code {
//         KeyCode::BackTab => UserKeyboad::Backtab,
//         KeyCode::Esc => UserKeyboad::Escape,
//         KeyCode::Left => UserKeyboad::Left,
//         KeyCode::Up => UserKeyboad::Up,
//         KeyCode::Down => UserKeyboad::Down,
//         KeyCode::Right => UserKeyboad::Right,
//         KeyCode::Home => UserKeyboad::Home,
//         KeyCode::End => UserKeyboad::End,
//         KeyCode::Tab => UserKeyboad::Tab,
//         KeyCode::F(n) => match n {
//             1 => UserKeyboad::F1,
//             2 => UserKeyboad::F2,
//             3 => UserKeyboad::F3,
//             4 => UserKeyboad::F4,
//             5 => UserKeyboad::F5,
//             6 => UserKeyboad::F6,
//             7 => UserKeyboad::F7,
//             8 => UserKeyboad::F8,
//             9 => UserKeyboad::F9,
//             10 => UserKeyboad::F10,
//             11 => UserKeyboad::F11,
//             12 => UserKeyboad::F12,
//             _ => UserKeyboad::Unknown,
//         },
//         KeyCode::Char(c) => match c {
//             'A' => UserKeyboad::ShiftA,
//             'B' => UserKeyboad::ShiftB,
//             'C' => UserKeyboad::ShiftC,
//             'D' => UserKeyboad::ShiftD,
//             'E' => UserKeyboad::ShiftE,
//             'F' => UserKeyboad::ShiftF,
//             'G' => UserKeyboad::ShiftG,
//             'H' => UserKeyboad::ShiftH,
//             'I' => UserKeyboad::ShiftI,
//             'J' => UserKeyboad::ShiftJ,
//             'K' => UserKeyboad::SHiftK,
//             'L' => UserKeyboad::ShiftL,
//             'M' => UserKeyboad::ShiftM,
//             'N' => UserKeyboad::ShiftN,
//             'O' => UserKeyboad::ShiftO,
//             'P' => UserKeyboad::ShiftP,
//             'Q' => UserKeyboad::ShiftQ,
//             'R' => UserKeyboad::ShiftR,
//             'S' => UserKeyboad::ShiftS,
//             'T' => UserKeyboad::ShiftT,
//             'U' => UserKeyboad::ShiftU,
//             'V' => UserKeyboad::ShiftV,
//             'W' => UserKeyboad::ShiftW,
//             'X' => UserKeyboad::ShiftX,
//             'Y' => UserKeyboad::ShiftY,
//             'Z' => UserKeyboad::ShiftZ,
//             _ => UserKeyboad::Unknown,
//         },
//         KeyCode::Char(c) if key.modifiers == KeyModifiers::CONTROL => match c {
//             'a' => UserKeyboad::CtrlA,
//             'b' => UserKeyboad::CtrlB,
//             'c' => UserKeyboad::CtrlC,
//             'd' => UserKeyboad::CtrlD,
//             'e' => UserKeyboad::CtrlE,
//             'f' => UserKeyboad::CtrlF,
//             'g' => UserKeyboad::CtrlG,
//             'h' => UserKeyboad::CtrlH,
//             'i' => UserKeyboad::CtrlI,
//             'j' => UserKeyboad::CtrlJ,
//             'k' => UserKeyboad::CtrlK,
//             'l' => UserKeyboad::CtrlL,
//             'm' => UserKeyboad::CtrlM,
//             'n' => UserKeyboad::CtrlN,
//             'o' => UserKeyboad::CtrlO,
//             'p' => UserKeyboad::CtrlP,
//             'q' => UserKeyboad::CtrlQ,
//             'r' => UserKeyboad::CtrlR,
//             's' => UserKeyboad::CtrlS,
//             't' => UserKeyboad::CtrlT,
//             'u' => UserKeyboad::CtrlU,
//             'v' => UserKeyboad::CtrlV,
//             'w' => UserKeyboad::CtrlW,
//             'x' => UserKeyboad::CtrlX,
//             'y' => UserKeyboad::CtrlY,
//             'z' => UserKeyboad::CtrlZ,
//             '#' => UserKeyboad::Space,
//             _ => UserKeyboad::Unknown,
//         },

//         KeyCode::Char(c) if key.modifiers == KeyModifiers::ALT => match c {
//             'a' => UserKeyboad::AltA,
//             'b' => UserKeyboad::AltB,
//             'c' => UserKeyboad::AltC,
//             'd' => UserKeyboad::AltD,
//             'e' => UserKeyboad::AltE,
//             'f' => UserKeyboad::AltF,
//             'g' => UserKeyboad::AltG,
//             'h' => UserKeyboad::AltH,
//             'i' => UserKeyboad::AltI,
//             'j' => UserKeyboad::AltJ,
//             'k' => UserKeyboad::AltK,
//             'l' => UserKeyboad::AltL,
//             'm' => UserKeyboad::AltM,
//             'n' => UserKeyboad::AltN,
//             'o' => UserKeyboad::AltO,
//             'p' => UserKeyboad::AltP,
//             'q' => UserKeyboad::AltQ,
//             'r' => UserKeyboad::AltR,
//             's' => UserKeyboad::AltS,
//             't' => UserKeyboad::AltT,
//             'u' => UserKeyboad::AltU,
//             'v' => UserKeyboad::AltV,
//             'w' => UserKeyboad::AltW,
//             'x' => UserKeyboad::AltX,
//             'y' => UserKeyboad::AltY,
//             'z' => UserKeyboad::AltZ,
//             '#' => UserKeyboad::Space,
//             _ => UserKeyboad::Unknown,
//         },
//         KeyCode::Char(c) => match c {
//             'a' => UserKeyboad::A,
//             'b' => UserKeyboad::B,
//             'c' => UserKeyboad::C,
//             'd' => UserKeyboad::D,
//             'e' => UserKeyboad::E,
//             'f' => UserKeyboad::F,
//             'g' => UserKeyboad::G,
//             'h' => UserKeyboad::H,
//             'i' => UserKeyboad::I,
//             'j' => UserKeyboad::J,
//             'k' => UserKeyboad::K,
//             'l' => UserKeyboad::L,
//             'm' => UserKeyboad::M,
//             'n' => UserKeyboad::N,
//             'o' => UserKeyboad::O,
//             'p' => UserKeyboad::P,
//             'q' => UserKeyboad::Q,
//             'r' => UserKeyboad::R,
//             's' => UserKeyboad::S,
//             't' => UserKeyboad::T,
//             'u' => UserKeyboad::U,
//             'v' => UserKeyboad::V,
//             'w' => UserKeyboad::W,
//             'x' => UserKeyboad::X,
//             'y' => UserKeyboad::Y,
//             'z' => UserKeyboad::Z,
//             '#' => UserKeyboad::Shape,
//             '.' => UserKeyboad::Period,
//             '+' => UserKeyboad::Plus,
//             '-' => UserKeyboad::Minus,
//             '>' => UserKeyboad::GT,
//             '<' => UserKeyboad::LT,
//             '?' => UserKeyboad::QuestionMark,
//             '!' => UserKeyboad::ExclamationMark,
//             ',' => UserKeyboad::Comma,
//             ' ' => UserKeyboad::Space,
//             _ => UserKeyboad::Unknown,
//         },
//         _ => UserKeyboad::Unknown,
//     }
// }

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

    pub fn boader_style(&self) -> Style {
        let user_color = self.boader.clone();
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
        _ => Color::Reset,
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
pub struct UserConfig {
    theme: SettingTheme,
    symbols: HashMap<FileItems, String>,
    user_keybinds: HashMap<String, String>,
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
            symbols: simple_symbols(),
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

    pub fn keybindings_map(&self) -> &HashMap<String, String> {
        &self.user_keybinds
    }
}

fn style_formatter(color: Colors, is_fg: bool, is_bg: bool) -> Style {
    let color = color_translator(color).unwrap();
    match (is_fg, is_bg) {
        (true, true) => panic!("is_fg and is_bg is not always true"),
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
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ron::de;

    use crate::load_config::{mappings_crossterm_keyevent_to_userkeyboad, UserConfig};

    use super::{string_to_keyboard, UserKeyboad};

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
        let keybinds = config.keybindings_map();
        println!("{:#?}", keybinds);
    }

    #[test]
    fn can_repace_string_to_enum() {
        let sl = "l".to_string();
        let el = UserKeyboad::L;
        assert_eq!(string_to_keyboard(&sl), el);
        let strings = [
            "l", "h", "j", "k", "q", "C-h", "C-j", "C-k", "C-l", "S-h", "S-j", "S-k", "S-l", "A-h",
            "A-j", "A-k", "A-l", "S-n", "S-i",
        ];

        let enum_keys = [
            UserKeyboad::L,
            UserKeyboad::H,
            UserKeyboad::J,
            UserKeyboad::K,
            UserKeyboad::Q,
            UserKeyboad::CtrlH,
            UserKeyboad::CtrlJ,
            UserKeyboad::CtrlK,
            UserKeyboad::CtrlL,
            UserKeyboad::ShiftH,
            UserKeyboad::ShiftJ,
            UserKeyboad::SHiftK,
            UserKeyboad::ShiftL,
            UserKeyboad::AltH,
            UserKeyboad::AltJ,
            UserKeyboad::AltK,
            UserKeyboad::AltL,
            UserKeyboad::ShiftN,
            UserKeyboad::ShiftI,
        ];

        for (s, k) in strings.iter().zip(enum_keys.iter()) {
            let s = s.to_string();
            assert_eq!(string_to_keyboard(&s), *k);
        }
    }

    #[test]
    fn can_crosster_keyevent_to_userkyboad() {
        let key_events = [
            (
                KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL),
                UserKeyboad::CtrlH,
            ),
            (
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL),
                UserKeyboad::CtrlJ,
            ),
            (
                KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL),
                UserKeyboad::CtrlK,
            ),
            (
                KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL),
                UserKeyboad::CtrlL,
            ),
            (
                KeyEvent::new(KeyCode::Char('H'), KeyModifiers::SHIFT),
                UserKeyboad::ShiftH,
            ),
            (
                KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT),
                UserKeyboad::ShiftJ,
            ),
            (
                KeyEvent::new(KeyCode::Char('K'), KeyModifiers::SHIFT),
                UserKeyboad::SHiftK,
            ),
            (
                KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT),
                UserKeyboad::ShiftL,
            ),
        ];

        for (key_event, user_key) in key_events.into_iter() {
            let ans = mappings_crossterm_keyevent_to_userkeyboad(&key_event);
            assert_eq!(ans, user_key);
        }
    }
}
