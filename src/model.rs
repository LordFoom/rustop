///Possible states of a process
pub enum ProcessState {
    Running,
    Sleeping,
    Zombie,
    Stopped,
    Unknown(char),
}
///Struct to hold information about processes
pub struct ProcessInfo {
    pub pid: u64,
    pub ppid: u64,
    pub name: String,
    pub command: String,
    pub cpu_percent: f64,
    pub memory_kb: u64,
    pub start_time: u64,
    pub state: ProcessState,
    pub user: String,
}
