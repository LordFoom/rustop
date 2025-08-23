use std::{thread, time::Duration};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use model::SortBy;
use output::{clear_screen, display_processes, display_processes_sorted, display_timestamp};
use processes::get_process_info;
use users::UsersCache;

mod app_args;
mod model;
mod output;
mod processes;

fn main() -> Result<()> {
    //get all the processes
    let mut user_cache = UsersCache::new();
    let mut refresh_count: u8 = 0;
    //TODO turn this into an enum
    let mut maybe_sort_by = None;
    loop {
        if event::poll(Duration::from_millis(100))? {
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
        thread::sleep(Duration::from_secs(2));
    }
    Ok(())
}

mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    pub fn test_display_processes() -> Result<()> {
        let mut user_cache = UsersCache::new();
        let processes = get_process_info(&mut user_cache)?;
        display_processes(processes)?;
        Ok(())
    }
}
