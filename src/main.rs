use std::{error::Error, io};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use tui_textarea::{Input, Key, TextArea};

mod models;
use models::{
    app::{App, Filter, InputMode},
    config::AppConfig,
};
use models::{
    app::{InputArea, PageLayout},
    formatter::Formatter,
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    let conf = AppConfig::load_config();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let formatter_config = bat::config::Config {
        colored_output: true,
        theme: conf.colors.bat_color_sheme.clone(),
        ..Default::default()
    };
    let formatter_assets = bat::assets::HighlightingAssets::from_binary();
    let formatter = Formatter::new(&formatter_config, &formatter_assets);
    let text_areas = [
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
        TextArea::default(),
    ];

    let app = App::new();
    let res = run_app(&mut terminal, app, text_areas, conf, formatter);

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
    mut text_areas: [TextArea; 4],
    conf: AppConfig,
    formatter: Formatter,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app, &mut text_areas, &conf, &formatter))?;
        match app.input_mode {
            models::app::InputMode::Normal => match crossterm::event::read()?.into() {
                Input {
                    key: Key::Char('q'),
                    ..
                } => return Ok(()),
                Input {
                    key: Key::Char('h'),
                    ..
                } => app.layout = PageLayout::Horizontal,
                Input {
                    key: Key::Char('v'),
                    ..
                } => app.layout = PageLayout::Vertical,
                Input {
                    key: Key::Char('n'),
                    ..
                } => app.input_mode = InputMode::Editing,
                Input {
                    key: Key::Char('s'),
                    ..
                } => {
                    app.input_mode = InputMode::SubTask;
                    app.input_area = InputArea::Task;
                }
                Input {
                    key: Key::Char('c'),
                    ..
                } => app.change_status(),
                Input {
                    key: Key::Char('a'),
                    ..
                } => {
                    app.input_mode = InputMode::Comment;
                    app.input_area = InputArea::Comment;
                }
                Input {
                    key: Key::Char('A'),
                    shift: true,
                    ..
                } => app.delete_comment(),
                Input {
                    key: Key::Char('D'),
                    ..
                } => app.delete(),
                Input {
                    key: Key::Char('E'),
                    shift: true,
                    ..
                } => match app.edit_last_comment() {
                    Some(i) => {
                        text_areas[3] = TextArea::from(i.lines());
                        app.input_mode = InputMode::CommentEdit;
                        app.input_area = InputArea::Comment;
                    }
                    None => (),
                },
                Input {
                    key: Key::Char('e'),
                    ..
                } => match app.edit() {
                    Some(i) => {
                        if i.3 {
                            app.input_mode = InputMode::SubTaskModify;
                            app.input_area = InputArea::Task;
                        } else {
                            app.input_mode = InputMode::Modify;
                        }
                        text_areas[0] = TextArea::from(i.0.lines());
                        text_areas[1] = TextArea::from(i.1.lines());
                        text_areas[2] = TextArea::from(i.2.lines());
                    }
                    None => (),
                },
                Input {
                    key: Key::Down | Key::Char('j'),
                    ..
                } => app.next(),
                Input {
                    key: Key::Up | Key::Char('k'),
                    ..
                } => app.previous(),
                Input {
                    key: Key::Char('?'),
                    ..
                } => {
                    app.input_mode = InputMode::Help;
                }
                Input {
                    key: Key::Char(','),
                    ..
                } => app.scroll_down(),
                Input {
                    key: Key::Char('.'),
                    ..
                } => app.scroll_up(),
                Input {
                    key: Key::Char('f'),
                    ..
                } => app.input_mode = InputMode::FilterMode,
                _ => {}
            },
            models::app::InputMode::FilterMode => match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    app.input_mode = InputMode::Normal;
                }
                Input {
                    key: Key::Char('n'),
                    ..
                } => {
                    app.filter_items(Filter::New);
                }
                Input {
                    key: Key::Char('h'),
                    ..
                } => {
                    app.filter_items(Filter::Hold);
                }
                Input {
                    key: Key::Char('i'),
                    ..
                } => {
                    app.filter_items(Filter::InProgress);
                }
                Input {
                    key: Key::Char('d'),
                    ..
                } => {
                    app.filter_items(Filter::Done);
                }
                Input {
                    key: Key::Char('o'),
                    ..
                } => {
                    app.filter_items(Filter::NotDone);
                }
                Input {
                    key: Key::Char('a'),
                    ..
                } => {
                    app.filter_items(Filter::All);
                }
                _ => {}
            },
            _ => match crossterm::event::read()?.into() {
                Input { key: Key::Esc, .. } => {
                    text_areas[0] = TextArea::default();
                    text_areas[1] = TextArea::default();
                    text_areas[2] = TextArea::default();
                    text_areas[3] = TextArea::default();
                    app.input_mode = InputMode::Normal;
                    app.input_area = InputArea::Topic;
                }
                Input {
                    key: Key::Char('s'),
                    ctrl: true,
                    ..
                } => {
                    app.create(
                        &text_areas[0],
                        &text_areas[1],
                        &text_areas[2],
                        &text_areas[3],
                    );
                    text_areas[0] = TextArea::default();
                    text_areas[1] = TextArea::default();
                    text_areas[2] = TextArea::default();
                    text_areas[3] = TextArea::default();
                }
                Input { key: Key::Tab, .. } => {
                    app.change_input_area();
                }
                input => {
                    text_areas[app.input_area.index()].input(input);
                }
            },
        }
    }
}
