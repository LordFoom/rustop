use std::{thread, time::Duration};

use anyhow::Result;
use output::{clear_screen, display_processes, display_timestamp};
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
    loop {
        let processes = get_process_info(&mut user_cache)?;
        clear_screen();
        display_timestamp();
        display_processes(processes)?;
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
