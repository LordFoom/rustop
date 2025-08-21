use std::{thread, time::Duration};

use anyhow::Result;
use crossterm::event::{self, Event};
use output::{clear_screen, display_processes, display_processes_sorted, display_timestamp};
use processes::get_process_info;
use ratatui::crossterm::event::KeyCode;
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
    let mut sort_by = "cpu";
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('c') | KeyCode::Char('C') => sort_by = "cpu",
                    KeyCode::Char('m') | KeyCode::Char('M') => sort_by = "mem",
                    KeyCode::Char('p') | KeyCode::Char('P') => sort_by = "pid",
                    KeyCode::Char('n') | KeyCode::Char('N') => sort_by = "name",
                    _ => {}
                }
            }
        };
        let processes = get_process_info(&mut user_cache)?;
        clear_screen();
        display_timestamp();
        display_processes_sorted(&mut processes, sort_by)?;
        if refresh_count % 100 == 0 {
            user_cache = UsersCache::new();
            refresh_count = 0;
        } else {
            refresh_count += 1;
        }
        thread::sleep(Duration::from_secs(2));
    }
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
