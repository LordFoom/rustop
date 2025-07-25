///Possible states of a process
enum ProcessState {
    Running,
    Sleeping,
    Zombie,
    Stopped,
    Unknown(char),
}
///Struct to hold information about processes
struct ProcessInfo {
    pid: u64,
    ppid: u64,
    name: String,
    command: String,
    cpu_percent: f64,
    memory_kb: u64,
    start_time: u64,
    state: ProcessState,
    user: String,
}
