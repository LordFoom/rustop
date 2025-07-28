use anyhow::Result;

use crate::model::ProcessInfo;

///Get processes and monitor them
pub fn get_pids() -> Result<Vec<u32>> {
    let mut pids = Vec::new();
    //get the process ids by parsing the /proc/ directory
    for dir_entry in std::fs::read_dir("/proc")? {
        let entry = dir_entry?;
        if let Some(name) = entry.file_name().to_str() {
            if let Ok(pid) = name.parse::<u32>() {
                pids.push(pid);
            }
        }
    }

    Ok(pids)
}

pub fn get_process_info() -> Result<Vec<ProcessInfo>> {
    let mut process_info_vec = Vec::new();

    for pid in get_pids()? {
        let process_info = parse_process(pid)?;
        process_info_vec.push(process_info);
    }

    Ok(process_info_vec)
}

pub fn parse_process(pid: u32) -> Result<ProcessInfo> {}
