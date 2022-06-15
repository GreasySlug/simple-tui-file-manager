use std::collections::HashMap;

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
enum Keyboad {
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

#[derive(Debug, Clone, Deserialize)]
struct UserKeybinds {
    keybinds: HashMap<String, String>,
}

impl UserKeybinds {
    fn default_vim_movements() -> UserKeybinds {
        let mut keybinds: HashMap<String, String> = HashMap::new();
        keybinds.insert("move_to_parent_dir".to_string(), "h".to_string());
        keybinds.insert("move_to_next_file_item".to_string(), "j".to_string());
        keybinds.insert("move_to_prev_file_item".to_string(), "k".to_string());
        keybinds.insert("move_to_child_dir".to_string(), "l".to_string());
        keybinds.insert("quit".to_string(), "q".to_string());

        UserKeybinds { keybinds }
    }

    fn default_arrow_key() -> UserKeybinds {
        let mut keybinds: HashMap<String, String> = HashMap::new();
        keybinds.insert("move_to_parent_dir".to_string(), "left".to_string());
        keybinds.insert("move_to_next_file_item".to_string(), "down".to_string());
        keybinds.insert("move_to_prev_file_item".to_string(), "up".to_string());
        keybinds.insert("move_to_child_dir".to_string(), "right".to_string());
        keybinds.insert("quit".to_string(), "q".to_string());

        UserKeybinds { keybinds }
    }

    fn default_vim_ctrl_movements() -> UserKeybinds {
        let mut keybinds: HashMap<String, String> = HashMap::new();
        keybinds.insert("move_to_parent_dir".to_string(), "C-h".to_string());
        keybinds.insert("move_to_next_file_item".to_string(), "C-j".to_string());
        keybinds.insert("move_to_prev_file_item".to_string(), "C-k".to_string());
        keybinds.insert("move_to_child_dir".to_string(), "C-l".to_string());
        keybinds.insert("quit".to_string(), "q".to_string());

        UserKeybinds { keybinds }
    }

    fn string_to_keyboard(self) -> HashMap<String, Keyboad> {
        let mut keybind: HashMap<String, Keyboad> = HashMap::new();
        for (cmd, key) in self.keybinds.into_iter() {
            let key = string_to_keyboard(&key);
            keybind.insert(cmd, key);
        }
        keybind
    }
}

fn string_to_keyboard(s: &str) -> Keyboad {
    match s {
        "h" => Keyboad::H,
        "j" => Keyboad::J,
        "k" => Keyboad::K,
        "l" => Keyboad::K,
        "S-h" => Keyboad::ShiftH,
        "S-j" => Keyboad::ShiftJ,
        "C-k" => Keyboad::CtrlK,
        "S-l" => Keyboad::ShiftL,
        "left" => Keyboad::Left,
        "up" => Keyboad::Up,
        "down" => Keyboad::Down,
        "right" => Keyboad::Right,
        _ => Keyboad::Unknown,
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SettingColors {
    background: Colors,
    boader: Colors,
    directory: Colors,
    file_item: Colors,
    select: Colors,
    header: Colors,
    command: Colors,
}

impl SettingColors {
    fn dark_theme() -> SettingColors {
        SettingColors {
            background: Colors::Black,
            header: Colors::Cyan,
            boader: Colors::White,
            directory: Colors::Blue,
            file_item: Colors::Gray,
            select: Colors::LightMagenta,
            command: Colors::White,
        }
    }

    fn light_theme() -> SettingColors {
        SettingColors {
            background: Colors::Gray,
            header: Colors::Green,
            boader: Colors::Gray,
            directory: Colors::Blue,
            file_item: Colors::Black,
            select: Colors::LightRed,
            command: Colors::Gray,
        }
    }

    fn dark_blue_theme() -> SettingColors {
        SettingColors {
            background: Colors::Rgb(39, 67, 100),
            header: Colors::Green,
            boader: Colors::Rgb(97, 169, 252),
            directory: Colors::Blue,
            file_item: Colors::Gray,
            select: Colors::Green,
            command: Colors::Rgb(97, 169, 252),
        }
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

#[derive(Debug, Clone, Deserialize)]
struct SettingSymbols {
    file: String,
    directory: String,
    select: String,
}

impl SettingSymbols {
    fn example_symbols() -> SettingSymbols {
        SettingSymbols {
            file: "📜".to_string(),
            directory: "📁".to_string(),
            select: "⋙".to_string(),
        }
    }

    fn simple_symbols() -> SettingSymbols {
        SettingSymbols {
            file: " ".to_string(),
            directory: "▸".to_string(),
            select: ">>".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserConfig {
    background: String,
    user_colors: SettingColors,
    symbols: SettingSymbols,
    user_keybinds: UserKeybinds,
}

impl UserConfig {
    pub fn default_dark() -> UserConfig {
        UserConfig {
            background: "dark".to_string(),
            user_colors: SettingColors::dark_theme(),
            symbols: SettingSymbols::simple_symbols(),
            user_keybinds: UserKeybinds::default_vim_movements(),
        }
    }

    pub fn default_dark_blue() -> UserConfig {
        UserConfig {
            background: "dark".to_string(),
            user_colors: SettingColors::dark_blue_theme(),
            symbols: SettingSymbols::simple_symbols(),
            user_keybinds: UserKeybinds::default_vim_ctrl_movements(),
        }
    }

    pub fn default_light() -> UserConfig {
        UserConfig {
            background: "light".to_string(),
            user_colors: SettingColors::light_theme(),
            symbols: SettingSymbols::example_symbols(),
            user_keybinds: UserKeybinds::default_arrow_key(),
        }
    }

    pub fn file_style(&self) -> Style {
        let user_color = self.user_colors.file_item.clone();
        style_formatter(user_color, true, false)
    }

    pub fn dir_style(&self) -> Style {
        let user_color = self.user_colors.directory.clone();
        style_formatter(user_color, true, false)
    }

    pub fn select_style(&self) -> Style {
        let user_color = self.user_colors.select.clone();
        style_formatter(user_color, true, false)
    }

    pub fn header_style(&self) -> Style {
        let user_color = self.user_colors.header.clone();
        style_formatter(user_color, true, false)
    }

    pub fn boader_style(&self) -> Style {
        let user_color = self.user_colors.boader.clone();
        style_formatter(user_color, true, false)
    }

    pub fn command_style(&self) -> Style {
        let user_color = self.user_colors.command.clone();
        style_formatter(user_color, true, false)
    }

    pub fn background_style(&self) -> Style {
        let user_color = self.user_colors.background.clone();
        style_formatter(user_color, false, true)
    }

    pub fn file_symbol(&self) -> &str {
        &self.symbols.file
    }

    pub fn dir_symbol(&self) -> &str {
        &self.symbols.directory
    }

    pub fn select_symbol(&self) -> &str {
        &self.symbols.select
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
    let f = std::fs::File::open(path);
    if let Ok(f) = f {
        let config: UserConfig = match ron::de::from_reader(f) {
            Ok(x) => x,
            Err(_) => UserConfig::default_dark_blue(),
        };
        config
    } else {
        UserConfig::default_dark_blue()
    }
}

#[cfg(test)]
mod test {
    use ron::de;

    use crate::load_config::UserConfig;

    #[test]
    fn can_parse_ron_file() {
        let path = "config.ron";
        let f = std::fs::File::open(path);
        assert!(f.is_ok());
        let config: Result<UserConfig, de::Error> = ron::de::from_reader(f.unwrap());
        assert!(config.is_ok());
    }
}
