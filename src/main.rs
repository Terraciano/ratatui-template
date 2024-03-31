mod app;
mod ui;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};

use crate::ui::*;

use crate::app::*;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode().unwrap();
    let mut stderr = io::stdout();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app)).unwrap();

        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Esc => return Ok(true),
                    KeyCode::Char(value) => App::on_char_press(app, value),
                    _ => {}
                },
            }
        }
    }
}
