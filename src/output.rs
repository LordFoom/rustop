use std::cmp::Ordering;

use anyhow::Result;

use crate::model::{ProcessInfo, SortBy};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

///Utility method to display timestamp
pub fn display_timestamp() {
    println!("rustop - {}", chrono::Local::now().format("%H::%M::%S"));
}
//Display the process info brought in
pub fn display_processes(processes: &[ProcessInfo]) -> Result<()> {
    print!(
        "{:>8} {:>8} {:>8} {:>8} {:>6} {:>4} {:>8} {:>8} {} {:>8} {}\r\n",
        "PID", "PPID", "USER", "NICE", "CPU%", "STATE", "MEM", "VMEM", "TTY", "THREADS", "COMMAND"
    );
    println!("{}", "-".repeat(100));

    for process in processes {
        print!(
            "{:>8} {:>8} {:>8} {:>8} {:>6.1} {:>4} {:>8} {:>8} {} {:>8} {}\r\n",
            process.pid,
            process.ppid,
            truncate_string(&process.user, 8),
            process.nice,
            process.cpu_percent,
            process.state.as_char(),
            format_memory(process.memory_kb),
            format_memory(process.virtual_memory_kb),
            truncate_string(&process.terminal, 8),
            process.num_threads,
            truncate_string(&process.command, 40)
        );
    }
    Ok(())
}

/// If optional SortBy is supplied, sort processes appropriately
pub fn display_processes_sorted(
    processes: &mut [ProcessInfo],
    maybe_sort_by: &Option<SortBy>,
) -> Result<()> {
    if let Some(sort_by) = maybe_sort_by {
        match sort_by {
            SortBy::Cpu => processes.sort_by(|a, b| {
                b.cpu_percent
                    .partial_cmp(&a.cpu_percent)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            SortBy::Memory => processes.sort_by(|a, b| {
                b.memory_kb
                    .partial_cmp(&a.memory_kb)
                    .unwrap_or(Ordering::Equal)
            }),
            SortBy::Pid => {
                processes.sort_by(|a, b| b.pid.partial_cmp(&a.pid).unwrap_or(Ordering::Equal))
            }
            SortBy::Name => {
                processes.sort_by(|a, b| b.name.partial_cmp(&a.name).unwrap_or(Ordering::Equal));
            }
        }
    }
    display_processes(processes)
}

/// Helper function to format memory in human-readable format
fn format_memory(kb: u64) -> String {
    if kb == 0 {
        return "0".to_string();
    }

    let bytes = kb * 1024;
    const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
    const THRESHOLD: u64 = 1024;

    if bytes < THRESHOLD {
        return format!("{}B", bytes);
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD as f64;
        unit_index += 1;
    }

    format!("{:.1}{}", size, UNITS[unit_index])
}

// Helper function to truncate strings to fit in columns
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len.saturating_sub(1)])
    }
}

#[cfg(test)]
mod test {
    use crate::processes::get_process_info;

    use super::*;
    use anyhow::Result;
    use users::UsersCache;

    #[test]
    pub fn test_display_processes() -> Result<()> {
        let mut user_cache = UsersCache::new();
        let processes = get_process_info(&mut user_cache)?;
        display_processes(&processes)?;
        Ok(())
    }
}
