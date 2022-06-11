use application::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use path_process::{get_current_dir_path, get_home_directory_path, pathbuf_to_string_name};
use state::StatefulDirectory;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use ui::ui;

mod application;
mod file_item_list;
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

    let crr_dir_path = get_current_dir_path();
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        let index = app.get_dirtab_index();
        let tabs = app.get_list_of_dirtab();
        let selected_dir = app.peek_selected_statefuldir();
        terminal.draw(|f| ui(f, selected_dir, tabs, index))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') | KeyCode::Down => selected_dir.select_next(),
                KeyCode::Char('k') | KeyCode::Up => selected_dir.select_previous(),
                KeyCode::Char('h') | KeyCode::Left => app.move_to_parent_dir(),
                KeyCode::Char('l') | KeyCode::Right => app.move_to_child_dir(),
                KeyCode::Tab => app.next_dirtab(),
                KeyCode::BackTab => app.prev_dirtab(),
                _ => {}
            }
        }
    }
}
