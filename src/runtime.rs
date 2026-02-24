use crate::task::{Task, TaskKind};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::time::{Duration, Instant};

pub struct Runtime {
    pub micro_taskq: VecDeque<Task>,
    pub callback_q: VecDeque<Task>,
    timers: BinaryHeap<Reverse<TimerEntry>>,
    timer_tasks: HashMap<u64, Task>,
    next_id: u64,
}

#[derive(Eq)]
pub struct TimerEntry {
    pub due_at: Instant,
    pub id: u64,
}

impl Ord for TimerEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.due_at.cmp(&other.due_at).then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimerEntry {
    fn eq(&self, other: &Self) -> bool {
        self.due_at == other.due_at && self.id == other.id
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            micro_taskq: VecDeque::new(),
            callback_q: VecDeque::new(),
            timers: BinaryHeap::new(),
            timer_tasks: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn q_microtask(&mut self, task: Task) {
        self.micro_taskq.push_back(task);
    }

    pub fn q_callbackq(&mut self, task: Task) {
        self.callback_q.push_back(task);
    }

    pub fn set_timeout<F>(&mut self, duration: Duration, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;

        let now = Instant::now();
        let due = now + duration;

        let task = Task {
            id,
            kind: TaskKind::Timer,
            created_at: now,
            due_at: Some(due),
            callback: Box::new(f),
        };

        self.timer_tasks.insert(id, task);
        self.timers.push(Reverse(TimerEntry { due_at: due, id }));
    }

    fn move_expired_timers(&mut self) {
        let now = Instant::now();

        while let Some(Reverse(entry)) = self.timers.peek() {
            if entry.due_at <= now {
                let Reverse(entry) = self.timers.pop().unwrap();
                if let Some(task) = self.timer_tasks.remove(&entry.id) {
                    self.callback_q.push_back(task);
                }
            } else {
                break;
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            self.move_expired_timers();

            while let Some(task) = self.micro_taskq.pop_front() {
                (task.callback)();
            }

            if let Some(task) = self.callback_q.pop_front() {
                (task.callback)();
                continue;
            }

            if let Some(Reverse(entry)) = self.timers.peek() {
                let now = Instant::now();
                if entry.due_at > now {
                    std::thread::sleep(entry.due_at - now);
                }
                continue;
            }

            break;
        }
    }
}
