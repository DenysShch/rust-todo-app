mod models;

use models::{
    app::{App, InputMode},
    config::AppConfig,
    ui::ui,
};

use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let conf = AppConfig::load_config();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app, conf);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    conf: AppConfig,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app, &conf))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.insert_app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Esc => app.info_popup = false,
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Char('D') => app.delete(),
                        KeyCode::Char('s') => app.change_status(),
                        KeyCode::Char('n') => {
                            app.info_popup = false;
                            app.insert_app.clear_popup();
                            if app.insert_popup {
                                app.insert_app.input_mode = InputMode::Normal;
                            } else {
                                app.insert_app.input_mode = InputMode::Editing;
                            }
                            app.insert_popup = !app.insert_popup
                        }
                        KeyCode::Char('e') => {
                            app.info_popup = false;
                            app.insert_app.edit = true;
                            app.edit();
                            if app.insert_popup {
                                app.insert_app.input_mode = InputMode::Normal;
                            } else {
                                app.insert_app.input_mode = InputMode::Editing;
                            }
                            app.insert_popup = !app.insert_popup;
                        }
                        KeyCode::Char('1') => {
                            app.sort_by_topic();
                        }
                        KeyCode::Char('2') => {
                            app.sort_by_status();
                        }
                        KeyCode::Char('3') => {
                            app.sort_by_start();
                        }
                        KeyCode::Char('4') => {
                            app.sort_by_end();
                        }
                        KeyCode::Char('?') => {
                            app.insert_popup = false;
                            app.info_popup = !app.info_popup
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Char('|') => {
                            app.insert_app.add_new_line();
                        }
                        KeyCode::Tab => {
                            app.insert_app.switch_enter_mode();
                        }
                        KeyCode::Enter => {
                            if app.insert_app.edit {
                                app.modify();
                                app.insert_app.edit = false
                            } else {
                                app.create();
                            }
                            app.insert_app.input_mode = InputMode::Normal;
                            app.insert_popup = !app.insert_popup
                        }
                        KeyCode::Char(to_insert) => {
                            app.insert_app.enter_char(to_insert);
                        }
                        KeyCode::Backspace => {
                            app.insert_app.delete_char();
                        }
                        KeyCode::Left => {
                            app.insert_app.move_cursor_left();
                        }
                        KeyCode::Right => {
                            app.insert_app.move_cursor_right();
                        }
                        KeyCode::Esc => {
                            app.insert_app.input_mode = InputMode::Normal;
                            app.insert_popup = false;
                            app.info_popup = false;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
