use anyhow::Result;
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
    Ok(())
}
