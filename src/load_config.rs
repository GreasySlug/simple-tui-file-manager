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

#[derive(Debug, Clone, Deserialize)]
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
struct SettingKeybind {
    move_to_next_file_item: Keyboad,
    move_to_prev_file_item: Keyboad,
    move_to_parent_dir: Keyboad,
    move_to_child_dir: Keyboad,
}

impl SettingKeybind {
    fn default_vim_like_movements() -> SettingKeybind {
        SettingKeybind {
            move_to_next_file_item: Keyboad::J,
            move_to_prev_file_item: Keyboad::K,
            move_to_parent_dir: Keyboad::H,
            move_to_child_dir: Keyboad::L,
        }
    }
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
    // keybind: UserKeybind,
}

impl UserConfig {
    pub fn default_dark() -> UserConfig {
        UserConfig {
            background: "dark".to_string(),
            user_colors: SettingColors::dark_theme(),
            symbols: SettingSymbols::simple_symbols(),
        }
    }

    pub fn default_dark_blue() -> UserConfig {
        UserConfig {
            background: "dark".to_string(),
            user_colors: SettingColors::dark_blue_theme(),
            symbols: SettingSymbols::simple_symbols(),
        }
    }

    pub fn default_light() -> UserConfig {
        UserConfig {
            background: "light".to_string(),
            user_colors: SettingColors::light_theme(),
            symbols: SettingSymbols::example_symbols(),
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
    use super::load_user_config_file;

    #[test]
    fn can_parse_ron_file() {}
}
