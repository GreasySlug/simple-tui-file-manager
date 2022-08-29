use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, path::PathBuf, u8};
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
    HexCode(String),
}

pub fn default_vim_movements() -> ModeKeybindings {
    let mut normal: HashMap<String, String> = HashMap::new();
    let iter = [
        ("h", "move_to_parent_dir"),
        ("j", "move_to_next_file_item"),
        ("k", "move_to_prev_file_item"),
        ("l", "move_to_child_dir"),
        ("left", "move_to_parent_dir"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("right", "move_to_child_dir"),
        ("tab", "next_dirtab"),
        ("S-tab", "prev_dirtab"),
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
        (": q", "quit"),
        ("esc", "normal"),
        ("S-v", "stacker"),
        ("h", "move_to_parent_dir"),
        ("j", "move_to_next_file_item"),
        ("k", "move_to_prev_file_item"),
        ("l", "move_to_child_dir"),
        ("left", "move_to_parent_dir"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("right", "move_to_child_dir"),
        ("tab", "next_dirtab"),
        ("S-tab", "prev_dirtab"),
        ("m", "make_directory"),
        ("i", "make_file"),
        ("enter", "edit"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        input.insert(name.to_string(), cmd.to_string());
    }

    let mut stacker: HashMap<String, String> = HashMap::new();
    let iter = [
        (": q", "quit"),
        ("S-i", "input"),
        ("esc", "normal"),
        ("h", "move_to_parent_dir"),
        ("j", "move_to_next_file_item"),
        ("k", "move_to_prev_file_item"),
        ("l", "move_to_child_dir"),
        ("left", "move_to_parent_dir"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("right", "move_to_child_dir"),
        ("tab", "next_dirtab"),
        ("S-tab", "prev_dirtab"),
        ("C-n", "stacker_next_file_item"),
        ("C-p", "stacker_prev_file_item"),
        ("s", "stacker_toggle_select"),
        ("u", "stacker_pop"),
        ("C-s", "stacker_select_all_recursively"),
        ("C-a", "stacker_select_all"),
        ("p", "stacker_paste"),
        ("S-p", "stacker_stacking_paste"),
        ("m", "stacker_move"),
        ("d", "stacker_delete"),
        ("S-d", "stacker_delete_all"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        stacker.insert(name.to_string(), cmd.to_string());
    }

    let mut searcher: HashMap<String, String> = HashMap::new();
    let iter = [
        ("esc", "normal"),
        ("left", "move_to_parent_dir"),
        ("down", "move_to_next_file_item"),
        ("up", "move_to_prev_file_item"),
        ("right", "move_to_child_dir"),
        ("s", "stacker_toggle_select"),
        ("u", "stacker_pop"),
        ("C-a", "stacker_select_all"),
    ]
    .into_iter();
    for (name, cmd) in iter {
        searcher.insert(name.to_string(), cmd.to_string());
    }

    ModeKeybindings {
        normal,
        input,
        stacker,
        searcher,
    }
}

pub fn multi_string_map_to_user_keyboard(
    keybindings: &HashMap<String, String>,
) -> HashMap<Vec<KeyEvent>, String> {
    let mut keybind: HashMap<Vec<KeyEvent>, String> = HashMap::new();
    for (key, cmd) in keybindings.iter() {
        if key.split_whitespace().count() > 1 {
            let keys: Vec<KeyEvent> = key.split_whitespace().map(string_to_keyevent).collect();
            keybind.insert(keys, cmd.to_owned());
        } else {
            let user_keyboard = string_to_keyevent(key);
            keybind.insert(vec![user_keyboard], cmd.to_owned());
        }
    }
    keybind
}

fn keyevent_classify_modifier(m: char) -> KeyModifiers {
    match m {
        'S' => KeyModifiers::SHIFT,
        'C' => KeyModifiers::CONTROL,
        'A' => KeyModifiers::ALT,
        _ => KeyModifiers::NONE,
    }
}

fn keyevent_classify_code(is_f: bool, s: &str) -> KeyCode {
    if is_f {
        let (_f, s) = s.split_at(1);
        if let Ok(n) = s.parse::<u8>() {
            return KeyCode::F(n);
        }
    }

    match s {
        "esc" => KeyCode::Esc,
        "enter" | "return" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "right" => KeyCode::Right,
        "left" => KeyCode::Left,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "space" => KeyCode::Char(' '),
        _ => {
            let c: Vec<char> = s.chars().collect(); // 'a', 'b'など
            if c.len() > 1 {
                panic!("Not implement yet: {:?}", c);
            } else {
                KeyCode::Char(c[0])
            }
        }
    }
}

fn keyboard_classify(s: Vec<&str>) -> KeyEvent {
    if s.is_empty() {
        return KeyEvent {
            code: KeyCode::Null,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
    }
    if s.len() == 1 {
        let is_f = s[0].contains('f');
        let code = keyevent_classify_code(is_f, s[0]);
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    } else {
        let mut keyevents: Vec<KeyModifiers> = Vec::new();
        let mut code: KeyCode = KeyCode::Null;
        for tkn in s.into_iter() {
            if tkn.contains('A') || tkn.contains('C') || tkn.contains('S') {
                let c = tkn.chars().collect::<Vec<char>>()[0];
                keyevents.push(keyevent_classify_modifier(c));
            } else {
                let is_f = tkn.contains('f');
                code = keyevent_classify_code(is_f, tkn);
            }
        }
        match (keyevents.get(0), keyevents.get(1), keyevents.get(2)) {
            (Some(&modi00), Some(&modi01), Some(&modi02)) => KeyEvent {
                code,
                modifiers: modi00 | modi01 | modi02,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            (Some(&modi00), Some(&modi01), None) => KeyEvent {
                code,
                modifiers: modi00 | modi01,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            (Some(&modi00), None, None) => KeyEvent {
                code,
                modifiers: modi00,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            _ => KeyEvent {
                code: KeyCode::Null,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
        }
    }
}

fn string_to_keyevent(s: &str) -> KeyEvent {
    let s: Vec<&str> = s.split('-').collect();
    keyboard_classify(s)
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
pub struct UserKeybindings {
    single: Keybind,
    multi: Keybind,
    filtered_multi: Option<Keybind>,
    key: KeyEvent,
}

impl UserKeybindings {
    pub fn new() -> Self {
        Self {
            single: HashMap::new(),
            multi: HashMap::new(),
            filtered_multi: None,
            key: KeyEvent {
                code: KeyCode::Null,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
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
        let filtered_keybindings: HashMap<UserKeyCode, String> = self
            .multi
            .iter()
            .filter(|(keycode, _cmd)| keycode.first == self.key && keycode.second.is_some())
            .map(|(key, cmd)| (key.to_owned(), cmd.to_owned()))
            .collect();
        if filtered_keybindings.is_empty() {
            return;
        }

        self.filtered_multi = Some(filtered_keybindings);
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

    pub fn make_single_keybindings(
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

    pub fn make_multiple_keybindings(
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
    boarder: Colors,
    directory: Colors,
    file_item: Colors,
    select: Colors,
    header: Colors,
    warning_background: Colors,
    warning_foreground: Colors,
    error_background: Colors,
    error_foreground: Colors,
    // (bg, fg): normal, input, stacker
    command: Vec<(Colors, Colors)>,
}

impl SettingTheme {
    fn dark_theme() -> SettingTheme {
        SettingTheme {
            background: Colors::Black,
            header: Colors::Cyan,
            boarder: Colors::White,
            directory: Colors::Blue,
            file_item: Colors::Gray,
            select: Colors::LightMagenta,
            warning_background: Colors::Rgb(238, 163, 23),
            warning_foreground: Colors::White,
            error_background: Colors::Red,
            error_foreground: Colors::White,
            command: vec![
                (Colors::Black, Colors::Cyan),  // normal
                (Colors::White, Colors::Blue),  // input
                (Colors::Magenta, Colors::Red), // select
                (Colors::White, Colors::Red),   // search
            ],
        }
    }

    fn _light_theme() -> SettingTheme {
        SettingTheme {
            background: Colors::White,
            header: Colors::Green,
            boarder: Colors::Black,
            directory: Colors::Blue,
            file_item: Colors::Black,
            select: Colors::LightRed,
            warning_background: Colors::Rgb(238, 163, 23),
            warning_foreground: Colors::White,
            error_background: Colors::Red,
            error_foreground: Colors::White,
            command: vec![
                (Colors::Blue, Colors::White),    // normal
                (Colors::Green, Colors::White),   // input
                (Colors::Magenta, Colors::White), // select
                (Colors::Green, Colors::White),   // search
            ],
        }
    }

    fn dark_blue_theme() -> SettingTheme {
        SettingTheme {
            background: Colors::Rgb(39, 67, 100),
            header: Colors::Green,
            boarder: Colors::Rgb(97, 169, 252),
            directory: Colors::Blue,
            file_item: Colors::Gray,
            select: Colors::Green,
            warning_background: Colors::Rgb(238, 163, 23),
            warning_foreground: Colors::White,
            error_background: Colors::Red,
            error_foreground: Colors::White,
            command: vec![
                (Colors::Blue, Colors::Black),
                (Colors::Green, Colors::Blue),
                (Colors::Magenta, Colors::Blue),
                (Colors::Green, Colors::Blue),
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

    pub fn boarder_style(&self) -> Style {
        let user_color = &self.boarder;
        style_formatter(user_color, true, false)
    }

    pub fn command_styles(&self) -> [Style; 4] {
        let user_color = &self.command;
        let mut styles: [Style; 4] = [Style::default(); 4];
        for (i, (bg, fg)) in user_color.iter().enumerate() {
            let fg = style_formatter(fg, true, false);
            let bg = style_formatter(bg, false, true);
            styles[i] = bg.patch(fg);
        }
        styles
    }

    pub fn normal_command_style(&self) -> Style {
        if let Some((bg, fg)) = self.command.get(0) {
            let bg_style = style_formatter(bg, false, true);
            let fg_style = style_formatter(fg, true, false);
            fg_style.patch(bg_style)
        } else {
            Style::default().fg(Color::White).bg(Color::Black)
        }
    }

    pub fn input_command_style(&self) -> Style {
        if let Some((bg, fg)) = self.command.get(1) {
            let bg_style = style_formatter(bg, false, true);
            let fg_style = style_formatter(fg, true, false);
            fg_style.patch(bg_style)
        } else {
            Style::default().fg(Color::White).bg(Color::Black)
        }
    }

    pub fn stacker_command_style(&self) -> Style {
        if let Some((bg, fg)) = self.command.get(2) {
            let bg_style = style_formatter(bg, false, true);
            let fg_style = style_formatter(fg, true, false);
            fg_style.patch(bg_style)
        } else {
            Style::default().fg(Color::White).bg(Color::Black)
        }
    }

    pub fn searcher_command_style(&self) -> Style {
        if let Some((bg, fg)) = self.command.get(3) {
            let bg_style = style_formatter(bg, false, true);
            let fg_style = style_formatter(fg, true, false);
            fg_style.patch(bg_style)
        } else {
            Style::default().fg(Color::White).bg(Color::Black)
        }
    }

    pub fn background_style(&self) -> Style {
        let user_color = &self.background;
        style_formatter(user_color, false, true)
    }

    pub fn _error_style(&self) -> Style {
        let user_color = &self.error_background;
        let bg = style_formatter(user_color, false, true);
        let user_color = &self.error_foreground;
        let fg = style_formatter(user_color, true, false);
        bg.patch(fg)
    }

    pub fn warning_style(&self) -> Style {
        let user_color = &self.warning_background;
        let bg = style_formatter(user_color, false, true);
        let user_color = &self.warning_foreground;
        let fg = style_formatter(user_color, true, false);
        bg.patch(fg)
    }
}

fn hex_to_colorcode(code: &str) -> (u8, u8, u8) {
    let code = if code.starts_with('#') {
        let (_shape, code) = code.split_at(1);
        code
    } else {
        code
    };
    let str_to_16 = |x| -> u8 { u8::from_str_radix(x, 16).unwrap_or_default() };
    let (r, gb) = code.split_at(2);
    let (g, b) = gb.split_at(2);
    let mut rgb = [0; 3];
    for (i, j) in [r, g, b].iter().enumerate() {
        let splitted_str = j.split_at(1);
        let x: u16 = (str_to_16(splitted_str.0) + 1) as u16;
        let y: u16 = (str_to_16(splitted_str.1) + 1) as u16;
        rgb[i] = (x * y - 1) as u8;
    }
    (rgb[0], rgb[1], rgb[2])
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
        Colors::HexCode(code) => {
            let (r, g, b) = hex_to_colorcode(code);
            Color::Rgb(r, g, b)
        }
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
pub struct ModeKeybindings {
    pub normal: HashMap<String, String>,
    pub input: HashMap<String, String>,
    pub stacker: HashMap<String, String>,
    pub searcher: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    editor: String,
    show_hidden_files: bool,
}

impl Settings {
    fn default_vim() -> Self {
        Self {
            editor: "vim".to_string(),
            show_hidden_files: true,
        }
    }
}

#[cfg(not(target_os = "windows"))]
impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: "vi".to_string(),
            show_hidden_files: false,
        }
    }
}

#[cfg(target_os = "windows")]
impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: "notepad.exe".to_string(),
            show_hidden_files: false,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserConfig {
    theme: SettingTheme,
    user_settings: Settings,
    symbols: HashMap<FileItems, String>,
    user_keybindings: ModeKeybindings,
    additional_directories: Vec<String>,
}

impl UserConfig {
    pub fn default_dark() -> UserConfig {
        UserConfig {
            theme: SettingTheme::dark_theme(),
            symbols: simple_symbols(),
            user_settings: Settings::default(),
            user_keybindings: default_vim_movements(),
            additional_directories: vec![],
        }
    }

    pub fn default_dark_blue() -> UserConfig {
        UserConfig {
            theme: SettingTheme::dark_blue_theme(),
            symbols: example_symbols(),
            user_settings: Settings::default_vim(),
            user_keybindings: default_vim_movements(),
            additional_directories: vec![],
        }
    }

    pub fn _default_light() -> UserConfig {
        UserConfig {
            theme: SettingTheme::_light_theme(),
            symbols: example_symbols(),
            user_settings: Settings::default_vim(),
            user_keybindings: default_vim_movements(),
            additional_directories: vec![],
        }
    }
    pub fn symbols(&self) -> &HashMap<FileItems, String> {
        &self.symbols
    }

    pub fn theme(&self) -> &SettingTheme {
        &self.theme
    }

    pub fn normal_keybindings_map(&self) -> HashMap<String, String> {
        self.user_keybindings.normal.clone()
    }

    pub fn input_keybindings_map(&self) -> HashMap<String, String> {
        self.user_keybindings.input.clone()
    }

    pub fn stacker_keybindings_map(&self) -> HashMap<String, String> {
        self.user_keybindings.stacker.clone()
    }

    pub fn searcher_keybindings_map(&self) -> HashMap<String, String> {
        self.user_keybindings.searcher.clone()
    }

    pub fn additional_directory(&self) -> Vec<String> {
        self.additional_directories.clone()
    }

    pub fn user_editor(&self) -> String {
        self.user_settings.editor.clone()
    }

    pub fn show_hidden_files(&self) -> bool {
        self.user_settings.show_hidden_files
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
    let dir_path = std::env::var("APPDATA");
    #[cfg(not(target_os = "windows"))]
    let dir_path = { std::env::var("config") };

    if let Ok(env_path) = dir_path {
        let mut path = PathBuf::from(env_path);
        path.push("simple_file_manager");
        path.push("config.ron");
        match File::open(path) {
            Ok(f) => {
                let config: Result<UserConfig, ron::de::SpannedError> = ron::de::from_reader(f);
                if let Ok(config) = config {
                    config
                } else {
                    UserConfig::default_dark_blue()
                }
            }
            // TODO: logging this e
            Err(_e) => UserConfig::default_dark(),
        }
    } else {
        panic!("Failed to get env value");
    }
}

#[cfg(test)]
mod test {

    use std::path::PathBuf;

    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use ron::de;

    use crate::load_config::UserConfig;

    use super::{hex_to_colorcode, string_to_keyevent};

    #[test]
    fn can_read_ron_file() {
        let path = "config.ron";
        let f = std::fs::File::open(path);
        assert!(f.is_ok());
        let config: Result<UserConfig, de::SpannedError> = ron::de::from_reader(f.unwrap());
        match &config {
            Ok(_) => {}
            Err(e) => println!("{e:#?}"),
        }
        assert!(config.is_ok());
        let config = config.unwrap();
        let keybindings = config.stacker_keybindings_map();
        println!("{:#?}", keybindings);
    }

    #[test]
    fn can_parse_user_single_keybind() {
        let keybindings = ["a", "S-a", "A-a", "C-a", "C-S-A-a"];
        let keyevents = [
            KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
        ];

        for (binds, keyevent) in keybindings.into_iter().zip(&keyevents) {
            let key = string_to_keyevent(binds);
            assert_eq!(&key, keyevent);
        }
    }

    #[test]
    fn check_config_file_in_env_dir() {
        let env_name = std::env::var("APPDATA").unwrap();
        let mut path = PathBuf::from(env_name);
        path.push("simple_file_manager");
        path.push("config.ron");
        println!("{:#?}", path);
        assert!(path.exists());
    }

    #[test]
    fn check_hashcode_to_rgb() {
        let color_code = "#ffffff";
        let rgb = (255, 255, 255);
        assert_eq!(hex_to_colorcode(color_code), rgb);

        let color_code = "#000000";
        let rgb = (0, 0, 0);
        assert_eq!(hex_to_colorcode(color_code), rgb);
    }
}
