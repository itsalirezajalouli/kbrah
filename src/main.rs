mod app;
mod ui;

use std::{error::Error, io, time};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run the loop
    let mut app = App::new();
    app.add_map();
    let res = run_app(&mut terminal, &mut app);

    // since app has changed the state of the user’s terminal, we need to undo
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            println!("Exit!");
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app <B: Backend> (terminal: & mut Terminal<B>,
                         app: &mut App) -> io::Result<bool> {
    let mut current_text: Vec<char> = app.current_text.chars().rev().collect();
    loop {
        // take a frame (f) and pass to ui function to draw
        terminal.draw(|f| ui(f, app))?; // immutable borrow
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('i') => {
                        app.start_time = Some(time::Instant::now());
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },

                CurrentScreen::Stats => match key.code {
                    KeyCode::Char('i') => {
                        app.start_time = Some(time::Instant::now());
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Esc | KeyCode::Enter => {
                        app.reset();
                    }

                    _ => {}
                },

                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y')  => { return Ok(true) },
                    KeyCode::Char('n')  => { 
                        app.reset();
                        app.current_screen = CurrentScreen::Main; }
                    _ => {}
                }
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing = Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value=> {
                                        app.save_key_value();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.pop();
                                    }
                                    CurrentlyEditing::Value=> {
                                        app.value_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.reset();
                            current_text = app.current_text.chars().rev().collect();
                        }
                        KeyCode::Tab => {app.toggle_editing()}
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        current_text = handle_insert(current_text,
                                        app, value);
                                        app.update_wpm();
                                    }
                                    CurrentlyEditing::Value => {
                                        // app.value_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn handle_insert(mut current_text: Vec<char>, app: &mut App, value: char) -> Vec<char> {
    if value == current_text[current_text.len() - 1] {
        if app.key_input.len() > 0 {
            let ind = app.key_input.len() + 1;
            if ind != app.right_nums.len() {
                app.lefts[ind - 1] = ' ';
                app.rights[ind - 1] = ' ';
                app.rights[ind] = app.right_nums[ind];
                app.lefts[ind] = app.left_nums[ind];
            } else { app.go_stats();
                return app.current_text.clone().chars().collect()};
        }
        app.wrong = false;
        app.cursor = app.current_text
            .as_bytes()[1] as char;
        let char = current_text.pop();
        app.key_input.push(char.unwrap());
        current_text.reverse();
        let new_str = current_text.clone().into_iter()
            .map(|i| i.to_string())
            .collect::<String>();
        app.edit_text(&new_str);
        current_text.reverse();
    } else {
        if !app.wrong {
            app.wrong = true;
            app.mistakes += 1;
            app.update_accuracy();
        }
    }
    return current_text
}
