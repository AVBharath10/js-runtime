use crate::task::Task;
use std::collections::VecDeque;

pub struct Runtime {
    pub call_stack: Vec<String>,
    pub micro_taskq: VecDeque<Task>,
    pub callback_q: VecDeque<Task>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            call_stack: Vec::new(),
            micro_taskq: VecDeque::new(),
            callback_q: VecDeque::new(),
        }
    }
    pub fn push_stack(&mut self, name: &str) {
        self.call_stack.push(name.to_string());
    }
    pub fn pop_stack(&mut self) {
        self.call_stack.pop();
    }
    pub fn q_microtask(&mut self, task: Task) {
        self.micro_taskq.push_back(task);
    }
    pub fn q_callbackq(&mut self, task: Task) {
        self.callback_q.push_back(task);
    }
    pub fn run(&mut self) {
        while !self.micro_taskq.is_empty() || !self.callback_q.is_empty() {
            if self.call_stack.is_empty() {
                while let Some(task) = self.micro_taskq.pop_front() {
                    task();
                }
                if let Some(task) = self.callback_q.pop_front() {
                    task();
                }
            }
        }
    }
}
