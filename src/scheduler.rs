use process::{Process, PID};

struct Scheduler {
    processes: Vec<Process>,
    current: Option<PID>,
    previous: Option<PID>
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler {
            processes: VecDeque::new(),
            current: None,
            previous: None
        }
    }

    fn add(&mut self, mut process: Process) -> Option<PID> {
        let pid = match self.previous {
            Some(previous) => previous.checked_add(1)?,
            None => 0
        };

        self.processes.push_back(process);

        if let None = self.current {
            self.current = Some(id);
        }

        self.last_id = Some(pid);
        return self.last_id
    }

    fn switch_to() {
        //TODO
    }
}