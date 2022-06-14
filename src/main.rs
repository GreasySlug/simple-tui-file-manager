use application::{run_app, App};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use path_process::{current_dir_path, get_home_directory_path, pathbuf_to_string_name};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

mod application;
mod file_item_list;
mod input_ui;
mod keymapping;
mod load_config;
mod path_process;
mod state;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let crr_dir_path = current_dir_path();
    let dir_name = pathbuf_to_string_name(&crr_dir_path);
    let mut app = App::new();
    app.insert_new_statefuldir(crr_dir_path);
    app.push_new_dirname_to_dirtab(dir_name);

    let home_dir_path = get_home_directory_path();
    if let Some(path) = home_dir_path {
        let dir_name = pathbuf_to_string_name(&path);
        app.insert_new_statefuldir(path);
        app.push_new_dirname_to_dirtab(dir_name);
    }

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
