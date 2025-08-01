use anyhow::Result;
use anyhow::anyhow;

use crate::model::ProcessInfo;
use crate::model::ProcessState;

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

pub fn parse_process(pid: u32) -> Result<ProcessInfo> {
    let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid))?;

    let stat_parts: Vec<&str> = stat.split_whitespace().collect();
    if stat_parts.len() < 3 {
        return Err(anyhow!("Stat format is incorrect"));
    }

    let pid = stat_parts[0];
    let name = stat_parts[1].trim_matches(['(', ')']);
    let state_char = stat_parts[2].chars().next().unwrap_or_else(|| 'q');
    let state = ProcessState::from(state_char);
    let ppid = stat_parts[3];

    let process_info = ProcessInfo{
        pid: pid.parse::<u64>()?,
        ppid: ppid.parse::<u64>()?,
        name: todo!(),
        command: todo!(),
        cpu_percent: todo!(),
        memory_kb: todo!(),
        start_time: todo!(),
        state,
        user: todo!(),
        priority: todo!(),
        nice: todo!(),
        num_threads: todo!(),
        virtual_memory_kb: todo!(),
        cpu_time_total: todo!(),
        session_id: todo!(),
        terminal: todo!(),
    }
}
