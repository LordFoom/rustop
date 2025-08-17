use anyhow::Result;

use crate::model::ProcessInfo;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
pub fn display_processes(processes: Vec<ProcessInfo>) -> Result<()> {
    println!(
        "{:>8} {:>8} {:>8} {:>8} {:>6} {:>4} {:>8} {:>8} {} {:>8} {}",
        "PID", "PPID", "USER", "NICE", "CPU%", "STATE", "MEM", "VMEM", "TTY", "THREADS", "COMMAND"
    );
    println!("{}", "-".repeat(100));

    for process in processes {
        println!(
            "{:>8} {:>8} {:>8} {:>8} {:>6.1} {:>4} {:>8} {:>8} {} {:>8} {}",
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
