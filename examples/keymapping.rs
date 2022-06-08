use std::{io::stdout, time::Duration};

use crossterm::{
    event::{
        poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

fn second_match_event() -> Result<()> {
    if poll(Duration::from_millis(1000))? {
        let read_event = read()?;
        match read_event {
            Event::Key(KeyEvent {
                modifiers: KeyModifiers::CONTROL,
                code,
            }) => {
                println!(" Second Cotrl + {:?}", code);
            }
            Event::Key(KeyEvent {
                code,
                modifiers: KeyModifiers::SHIFT,
            }) => {
                println!(" Second Shift + {:?}", code);
            }

            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _,
            }) => {
                return Ok(());
            }
            Event::Key(KeyEvent { code, modifiers: _ }) => {
                println!(" Pressed: {:?}", code);
            }
            _ => {}
        }
    } else {
        println!("\nSecond key was not pressed in time...");
    }
    Ok(())
}

fn first_match_event(read_event: Event) -> Result<String> {
    match read_event {
        Event::Key(KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code,
        }) => {
            print!("Control + {:?}", code);
            second_match_event()?;
        }
        Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::SHIFT,
        }) => {
            print!("Shift + {:?}", code);
            second_match_event()?;
        }
        Event::Key(KeyEvent {
            code: KeyCode::Esc,
            modifiers: _,
        }) => {
            return Ok("Escape".to_string());
        }
        Event::Key(KeyEvent { code, modifiers: _ }) => {
            println!("Pressed: {:?}", code);
        }
        _ => {}
    }
    Ok("Hi".to_string())
}

fn print_events() -> Result<()> {
    loop {
        let init_event = read()?;

        let result = first_match_event(init_event)?;
        if result == *"Escape" {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}
