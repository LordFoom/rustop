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
    name: String,
    state: String,
}
