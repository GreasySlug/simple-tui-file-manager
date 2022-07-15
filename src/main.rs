use application::{run_app, App};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use path_process::{pathbuf_to_string_name, working_dir_path};
use std::env;
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

mod application;
mod file_item_list;
mod load_config;
mod path_process;
mod stacker;
mod state;
mod ui;

const HELP_SENTENCE: &str = "#
A Simple TUI File Manager => s(t)fm

`sfm -V` | `sfm --version` => Display version
`sfm -H` | `sfm --help` => Display help

## Default Operation

### Common Operation

Normal | Input | Stacker
h, j, k, l, Left, Up, Right, Down => cursor move
tab, Shift+tab => move tab
";

fn main() -> Result<(), Box<dyn Error>> {
    // 引数を受け取る
    let args: Vec<String> = env::args().collect();
    // sfm -V, sfm -H
    let arg = &args.get(2);
    match arg {
        Some(arg) => match arg.as_str() {
            "-V" | "--version" => {
                let version: &str = env!("CARGO_PKG_VERSION");
                println!("{version}");
                return Ok(());
            }
            "-H" | "--help" => {
                println!("{HELP_SENTENCE}");
                return Ok(());
            }
            _ => {
                println!("Invalid args");
                return Ok(());
            }
        },
        None => {}
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // current working directory
    let crr_dir_path = working_dir_path();
    let dir_name = pathbuf_to_string_name(&crr_dir_path);
    let mut app = App::new();
    app.insert_new_statefuldir(crr_dir_path);
    app.push_new_dirname_to_dirtab(dir_name);

    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    terminal.clear()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
