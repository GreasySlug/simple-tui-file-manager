use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserKeyboad {
    H,
    J,
    K,
    L,
    Q,
    ShiftH,
    ShiftJ,
    SHiftK,
    ShiftL,
    ShiftQ,
    CtrlH,
    CtrlJ,
    CtrlK,
    CtrlL,
    CtrLQ,
    Left,
    Up,
    Down,
    Right,
    Escape,
    Enter,
    Tab,
    Tabspace,
    Backspace,
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
    keybinds.insert("Tabspace".to_string(), "prev_dirtab".to_string());
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
    keybinds.insert("Tabspace".to_string(), "prev_dirtab".to_string());
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
    keybinds.insert("Tabspace".to_string(), "prev_dirtab".to_string());
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
        "h" => UserKeyboad::H,
        "j" => UserKeyboad::J,
        "k" => UserKeyboad::K,
        "l" => UserKeyboad::L,
        "q" => UserKeyboad::Q,
        "S-h" => UserKeyboad::ShiftH,
        "S-j" => UserKeyboad::ShiftJ,
        "S-k" => UserKeyboad::SHiftK,
        "S-l" => UserKeyboad::ShiftL,
        "C-h" => UserKeyboad::CtrlH,
        "C-j" => UserKeyboad::CtrlJ,
        "C-k" => UserKeyboad::CtrlK,
        "C-l" => UserKeyboad::CtrlL,
        "left" => UserKeyboad::Left,
        "up" => UserKeyboad::Up,
        "down" => UserKeyboad::Down,
        "right" => UserKeyboad::Right,
        "tab" => UserKeyboad::Tab,
        "tabspace" => UserKeyboad::Tabspace,
        _ => UserKeyboad::Unknown,
    }
}

pub fn crossterm_keycode_to_commands(key: &KeyEvent) -> UserKeyboad {
    match key.code {
        KeyCode::BackTab => UserKeyboad::Tabspace,
        KeyCode::Esc => UserKeyboad::Escape,
        KeyCode::Left => UserKeyboad::Left,
        KeyCode::Up => UserKeyboad::Up,
        KeyCode::Down => UserKeyboad::Down,
        KeyCode::Right => UserKeyboad::Right,
        KeyCode::Tab => UserKeyboad::Tab,
        KeyCode::Char(c) => match c {
            'q' => UserKeyboad::Q,
            'l' => UserKeyboad::L,
            'j' => UserKeyboad::J,
            'k' => UserKeyboad::K,
            'h' => UserKeyboad::H,
            _ => UserKeyboad::Unknown,
        },
        _ => UserKeyboad::Unknown,
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
    let c = match color {
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
    };
    c
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
    use ron::de;

    use crate::load_config::UserConfig;

    use super::{string_to_keyboard, UserKeyboad};

    #[test]
    fn can_parse_ron_file() {
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
            "l", "h", "j", "k", "C-h", "C-j", "C-k", "C-l", "S-h", "S-j", "S-k", "S-l", "q",
        ];

        let enum_keys = [
            UserKeyboad::L,
            UserKeyboad::H,
            UserKeyboad::J,
            UserKeyboad::K,
            UserKeyboad::CtrlH,
            UserKeyboad::CtrlJ,
            UserKeyboad::CtrlK,
            UserKeyboad::CtrlL,
            UserKeyboad::ShiftH,
            UserKeyboad::ShiftJ,
            UserKeyboad::SHiftK,
            UserKeyboad::ShiftL,
            UserKeyboad::Q,
        ];

        for (s, k) in strings.iter().zip(enum_keys.iter()) {
            let s = s.to_string();
            assert_eq!(string_to_keyboard(&s), *k);
        }
    }
}
