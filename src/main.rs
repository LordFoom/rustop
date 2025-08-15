use anyhow::Result;
use output::display_processes;
use processes::get_process_info;
use users::UsersCache;

mod app_args;
mod model;
mod output;
mod processes;

fn main() -> Result<()> {
    //get all the processes
    let mut user_cache = UsersCache::new();
    let processes = get_process_info(&mut user_cache)?;
    display_processes(processes);
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
