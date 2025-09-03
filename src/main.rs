use std::{
    io::stdout,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use model::SortBy;
use output::{clear_screen, display_processes_sorted, display_timestamp};
use processes::get_process_info;
use ratatui::{Terminal, prelude::CrosstermBackend};
use tui::run_tui;
use users::UsersCache;

mod app;
mod app_args;
mod model;
mod output;
mod processes;
mod tui;

fn main() -> Result<()> {
    run()
}

fn run() -> Result<()> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let result = run_tui(&mut terminal);
    std::io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode().context("Failed to disable raw mode")?;
    result
}

fn show_processes() -> Result<()> {
    //get all the processes
    let mut user_cache = UsersCache::new();
    let mut refresh_count: u8 = 0;
    //TODO turn this into an enum
    let mut maybe_sort_by = None;
    let mut last_refresh = std::time::Instant::now();
    loop {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') | KeyCode::Char('C') => maybe_sort_by = Some(SortBy::Cpu),
                    KeyCode::Char('m') | KeyCode::Char('M') => maybe_sort_by = Some(SortBy::Memory),
                    KeyCode::Char('p') | KeyCode::Char('P') => maybe_sort_by = Some(SortBy::Pid),
                    KeyCode::Char('n') | KeyCode::Char('N') => maybe_sort_by = Some(SortBy::Name),
                    _ => {}
                }
            }
        };
        if last_refresh.elapsed() >= Duration::from_secs(1) {
            let mut processes = get_process_info(&mut user_cache)?;
            clear_screen();
            display_timestamp();
            display_processes_sorted(&mut processes, &maybe_sort_by)?;
            if refresh_count % 100 == 0 {
                user_cache = UsersCache::new();
                refresh_count = 0;
            } else {
                refresh_count += 1;
            }
            last_refresh = Instant::now();
        }
    }
    Ok(())
}
