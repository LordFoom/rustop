use std::time::Instant;

///Possible states of a process
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Running,       // R - Running or runnable (on run queue)
    Sleeping,      // S - Interruptible sleep (waiting for an event to complete)
    DiskSleep,     // D - Uninterruptible sleep (usually IO)
    Zombie,        // Z - Zombie (terminated but not reaped by parent)
    Stopped,       // T - Stopped (on a signal or because it is being traced)
    TracingStop,   // t - Tracing stop (Linux 2.6.33 onward)
    Dead,          // X - Dead (should never be seen)
    WakeKill,      // K - Wakekill (Linux 2.6.33 to 3.13 only)
    Waking,        // W - Waking (Linux 2.6.33 to 3.13 only)
    Parked,        // P - Parked (Linux 3.9 to 3.13 only)
    Unknown(char), // Any unrecognized state
}

impl From<char> for ProcessState {
    fn from(c: char) -> Self {
        match c {
            'R' => ProcessState::Running,
            'S' => ProcessState::Sleeping,
            'D' => ProcessState::DiskSleep,
            'Z' => ProcessState::Zombie,
            'T' => ProcessState::Stopped,
            't' => ProcessState::TracingStop,
            'X' => ProcessState::Dead,
            'K' => ProcessState::WakeKill,
            'W' => ProcessState::Waking,
            'P' => ProcessState::Parked,
            other => ProcessState::Unknown(other),
        }
    }
}

impl ProcessState {
    pub fn as_char(&self) -> char {
        match self {
            ProcessState::Running => 'R',
            ProcessState::Sleeping => 'S',
            ProcessState::DiskSleep => 'D',
            ProcessState::Zombie => 'Z',
            ProcessState::Stopped => 'T',
            ProcessState::TracingStop => 't',
            ProcessState::Dead => 'X',
            ProcessState::WakeKill => 'K',
            ProcessState::Waking => 'W',
            ProcessState::Parked => 'P',
            ProcessState::Unknown(c) => *c,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ProcessState::Running => "Running",
            ProcessState::Sleeping => "Sleeping",
            ProcessState::DiskSleep => "Disk Sleep",
            ProcessState::Zombie => "Zombie",
            ProcessState::Stopped => "Stopped",
            ProcessState::TracingStop => "Tracing",
            ProcessState::Dead => "Dead",
            ProcessState::WakeKill => "Wake Kill",
            ProcessState::Waking => "Waking",
            ProcessState::Parked => "Parked",
            ProcessState::Unknown(_) => "Unknown",
        }
    }
}
///
///Struct to hold information about processes
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u64,
    pub ppid: u64,
    pub name: String,
    pub command: String,
    pub cpu_percent: f64,
    ///how much cpu time has been used
    pub cpu_time_total: u64,
    ///previous cpu time measurement
    pub last_cpu_time: Option<u64>,
    ///When, if we did, did we last measure the cpu time
    pub last_measurement: Option<Instant>,
    pub memory_kb: u64,
    pub start_time: u64,
    pub state: ProcessState,
    pub user: String,

    // Additional useful fields for a process monitor
    pub priority: i64,          // Process priority
    pub nice: i64,              // Nice value (-20 to 19)
    pub num_threads: u64,       // Number of threads
    pub virtual_memory_kb: u64, // Virtual memory size
    pub session_id: u64,        // Session ID
    pub terminal: String,       // Controlling terminal (e.g., "pts/0", "tty1")
}

impl ProcessInfo {
    pub fn new() -> Self {
        ProcessInfo {
            pid: 0,
            ppid: 0,
            name: String::new(),
            command: String::new(),
            cpu_percent: 0.0,
            cpu_time_total: 0,
            last_cpu_time: None,
            last_measurement: None,
            memory_kb: 0,
            start_time: 0,
            state: ProcessState::Unknown('?'),
            user: String::new(),
            priority: 0,
            nice: 0,
            num_threads: 0,
            virtual_memory_kb: 0,
            session_id: 0,
            terminal: String::new(),
        }
    }

    /// Get formatted memory usage (e.g., "1.2M", "3.4G")
    pub fn formatted_memory(&self) -> String {
        format_bytes(self.memory_kb * 1024)
    }

    /// Get formatted virtual memory usage
    pub fn formatted_virtual_memory(&self) -> String {
        format_bytes(self.virtual_memory_kb * 1024)
    }

    /// Get process age since start_time (in seconds)
    pub fn age_seconds(&self) -> u64 {
        // You'll need to implement this based on system uptime
        // and convert start_time from clock ticks
        0 // Placeholder
    }

    /// Check if process is a kernel thread (usually indicated by brackets)
    pub fn is_kernel_thread(&self) -> bool {
        self.name.starts_with('[') && self.name.ends_with(']')
    }

    /// Get short command name (without path and arguments)
    pub fn short_command(&self) -> &str {
        self.command
            .split_whitespace()
            .next()
            .and_then(|cmd| cmd.split('/').next_back())
            .unwrap_or(&self.name)
    }

    pub fn update_cpu_percent(&mut self) {
        let current_time = Instant::now();
        let current_cpu_time = self.cpu_time_total;
        if let (Some(last_cpu), Some(last_time)) = (self.last_cpu_time, self.last_measurement) {
            let time_delta = current_time.duration_since(last_time).as_secs_f64();
            let cpu_delta = current_cpu_time.saturating_sub(last_cpu) as f64;
        }
    }
}

impl Default for ProcessInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to format bytes in human-readable format
fn format_bytes(bytes: u64) -> String {
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

impl std::fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>8} {:>8} {:>6.1}% {:>8} {} {} {}",
            self.pid,
            self.ppid,
            self.cpu_percent,
            self.formatted_memory(),
            self.state.as_char(),
            self.user,
            self.short_command()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Cpu,
    Memory,
    Pid,
    Name,
    Command,
}
