pub type PID = usize;

pub enum State {
    Runnable,
    Waiting,
    Running,
    Terminated
}

pub struct Process {
    // stack
    pid_: PID,
    state_: State
}

impl Process {
    pub fn new() -> Process {
        Process {
            pid_: 0,
            state_: State::Runnable
        }
    }

    pub fn get_state() {
        return state_;
    }

    pub fn get_pid() -> PID{
        return pid_;
    }
}