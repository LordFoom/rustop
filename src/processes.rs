use crate::model::ProcessInfo;
use crate::model::ProcessState;
use anyhow::Result;
use anyhow::anyhow;
use users::{Users, UsersCache};

// static USER_CACHE: LazyLock<UsersCache> = LazyLock::new(|| UsersCache::new());

///Get processes and monitor them
pub fn get_pids() -> Result<Vec<u64>> {
    let mut pids = Vec::new();
    //get the process ids by parsing the /proc/ directory
    for dir_entry in std::fs::read_dir("/proc")? {
        let entry = dir_entry?;
        if let Some(name) = entry.file_name().to_str() {
            if let Ok(pid) = name.parse::<u64>() {
                pids.push(pid);
            }
        }
    }

    Ok(pids)
}

pub fn get_process_info(user_cache: &mut UsersCache) -> Result<Vec<ProcessInfo>> {
    let mut process_info_vec = Vec::new();

    for pid in get_pids()? {
        let process_info = parse_process(pid, user_cache)?;
        process_info_vec.push(process_info);
    }

    Ok(process_info_vec)
}

pub fn parse_process(pid: u64, user_cache: &mut UsersCache) -> Result<ProcessInfo> {
    let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid))?;
    // println!("Raw stat line: {stat}");

    // Parse the stat file correctly - handle command name in parentheses
    let start_paren = stat
        .find('(')
        .ok_or_else(|| anyhow!("No opening parenthesis in stat"))?;
    let end_paren = stat
        .rfind(')')
        .ok_or_else(|| anyhow!("No closing parenthesis in stat"))?;

    // Extract parts
    let pid_str = stat[..start_paren].trim();
    let name = &stat[start_paren + 1..end_paren];
    let rest = &stat[end_paren + 2..]; // Skip ") "

    // Split the remaining fields
    let stat_parts: Vec<&str> = rest.split_whitespace().collect();

    if stat_parts.len() < 23 {
        return Err(anyhow!(
            "Insufficient fields in stat file: got {}, need at least 23",
            stat_parts.len()
        ));
    }

    let file_pid = pid_str.parse::<u64>()?;
    let state_char = stat_parts[0].chars().next().unwrap_or('?');
    let state = ProcessState::from(state_char);
    let ppid = stat_parts[1].parse::<u64>()?;
    let session_id = stat_parts[3].parse::<u64>().unwrap_or(0);
    let tty_nr = stat_parts[4].parse::<u64>().unwrap_or(0);
    let utime = stat_parts[11].parse::<u64>().unwrap_or(0);
    let stime = stat_parts[12].parse::<u64>().unwrap_or(0);
    let priority = stat_parts[15].parse::<i64>().unwrap_or(0);
    let nice = stat_parts[16].parse::<i64>().unwrap_or(0);
    let num_threads = stat_parts[17].parse::<u64>().unwrap_or(0);
    let start_time = stat_parts[19].parse::<u64>().unwrap_or(0);
    let vsize = stat_parts[20].parse::<u64>().unwrap_or(0);
    let rss = stat_parts[21].parse::<u64>().unwrap_or(0);

    // Get additional info
    let command = get_command_line(&pid.to_string()).unwrap_or_else(|_| name.to_string());
    let memory_kb = get_memory_usage(&pid.to_string()).unwrap_or(0);
    let user =
        get_process_user(file_pid, user_cache).unwrap_or_else(|| format!("uid:{}", file_pid));
    let terminal = get_terminal_name(tty_nr);

    let cpu_time_total = utime + stime;
    let virtual_memory_kb = vsize / 1024;

    // Still need to calculate CPU percentage (requires sampling over time)
    let cpu_percent = 0.0;

    Ok(ProcessInfo {
        pid: file_pid,
        ppid,
        name: name.to_string(),
        command,
        cpu_percent,
        memory_kb,
        start_time,
        state,
        user,
        priority,
        nice,
        num_threads,
        virtual_memory_kb,
        cpu_time_total,
        session_id,
        terminal,
    })
}

fn get_terminal_name(tty_nr: u64) -> String {
    if tty_nr == 0 {
        return "?".to_string();
    }

    // Decode device number: major (upper bits) + minor (lower bits)
    let major = (tty_nr >> 8) & 0xfff;
    let minor = tty_nr & 0xff;

    match major {
        4 => {
            // TTY devices
            if minor == 0 {
                "tty0".to_string()
            } else if minor <= 63 {
                format!("tty{}", minor)
            } else {
                format!("tty{}", minor)
            }
        }
        5 => {
            // Console devices
            match minor {
                0 => "tty".to_string(),
                1 => "console".to_string(),
                2 => "ptmx".to_string(),
                _ => format!("tty{}", minor),
            }
        }
        136..=143 => {
            // Unix98 PTY slaves (pts)
            let pts_minor = ((major - 136) << 8) + minor;
            format!("pts/{}", pts_minor)
        }
        _ => {
            // Unknown or other device types
            format!("{}:{}", major, minor)
        }
    }
}

fn get_process_user(pid: u64, user_cache: &mut UsersCache) -> Option<String> {
    use std::os::unix::fs::MetadataExt;
    let stat_path = format!("/proc/{pid}/stat");
    let meta_data = std::fs::metadata(stat_path).ok()?;
    let uid = meta_data.uid();
    //this works out het boks
    if let Some(user) = user_cache.get_user_by_uid(uid) {
        return Some(user.name().to_string_lossy().to_string());
    } else {
        return Some(format!("uid:{uid}"));
    }
}

fn get_memory_usage(pid: &str) -> Option<u64> {
    let status_content = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    for line in status_content.lines() {
        //VmRSS: 13484 kB
        if line.starts_with("VmRSS:") {
            return line.split_whitespace().nth(1)?.parse().ok();
        }
    }
    None
}

pub fn get_command_line(pid: &str) -> Result<String> {
    let cmd = std::fs::read_to_string(format!("/proc/{pid}/cmdline"))?;

    Ok(cmd)
}
