use tui::style::{Color, Style};

#[derive(Debug, Clone)]
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
    LightCyan,
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct SettingColors {
    background: Colors,
    header_font: Colors,
    boader: Colors,
    directory: Colors,
    file_item: Colors,
    select: Colors,
}

impl SettingColors {
    fn dark_theme() -> SettingColors {
        SettingColors {
            background: Colors::Black,
            header_font: Colors::Cyan,
            boader: Colors::White,
            directory: Colors::Blue,
            file_item: Colors::Gray,
            select: Colors::LightMagenta,
        }
    }

    fn light_theme() -> SettingColors {
        SettingColors {
            background: Colors::Gray,
            header_font: Colors::Green,
            boader: Colors::DarkGray,
            directory: Colors::Blue,
            file_item: Colors::Black,
            select: Colors::LightRed,
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
        Colors::LightCyan => Color::LightCyan,
        Colors::Rgb(r, g, b) => Color::Rgb(r, g, b),
        _ => Color::Reset,
    };
    Some(c)
}

#[derive(Debug, Clone)]
struct SettingKeybind {
    move_to_next_file_item: Keyboad,
    move_to_prev_file_item: Keyboad,
    move_to_parent_dir: Keyboad,
    move_to_child_dir: Keyboad,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct UserConfig {
    user_colors: SettingColors,
    symbols: SettingSymbols,
    // keybind: UserKeybind,
}

impl UserConfig {
    pub fn deault_dark() -> UserConfig {
        UserConfig {
            user_colors: SettingColors::dark_theme(),
            symbols: SettingSymbols::simple_symbols(),
        }
    }

    pub fn default_light() -> UserConfig {
        UserConfig {
            user_colors: SettingColors::light_theme(),
            symbols: SettingSymbols::example_symbols(),
        }
    }

    pub fn file_style(&self) -> Style {
        let user_color = self.user_colors.file_item.clone();
        let color = color_translator(user_color).unwrap();
        Style::default().fg(color)
    }

    pub fn dir_style(&self) -> Style {
        let user_color = self.user_colors.directory.clone();
        let color = color_translator(user_color).unwrap();
        Style::default().fg(color)
    }

    pub fn select_style(&self) -> Style {
        let user_color = self.user_colors.select.clone();
        let color = color_translator(user_color).unwrap();
        Style::default().fg(color)
    }

    pub fn header_style(&self) -> Style {
        let user_color = self.user_colors.header_font.clone();
        let color = color_translator(user_color).unwrap();
        Style::default().fg(color)
    }

    pub fn boader_style(&self) -> Style {
        let user_color = self.user_colors.boader.clone();
        let color = color_translator(user_color).unwrap();
        Style::default().fg(color)
    }

    pub fn command_style(&self) -> Style {
        let user_color = self.user_colors.boader.clone();
        let color = color_translator(user_color).unwrap();
        Style::default().fg(color)
    }

    pub fn background_style(&self) -> Style {
        let user_color = self.user_colors.background.clone();
        let color = color_translator(user_color).unwrap();
        Style::default().bg(color)
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
