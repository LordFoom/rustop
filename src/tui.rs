use std::time::Duration;

use crossterm::event::{self, Event};
use ratatui::{Frame, Terminal, prelude::Backend};

use crate::app::App;
use anyhow::Result;

pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if app.should_quit {
            break;
        }
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key.code);
            }
        }
        app.update_processes()?;
    }
    Ok(())
}

pub fn ui(f: &mut Frame, app: &mut App) {}
