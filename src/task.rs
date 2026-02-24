use std::time::Instant;

pub type TaskId = u64;

#[derive(Debug, Clone, Copy)]
pub enum TaskKind {
    Timer,
    Callback,
    Microtask,
}
pub struct Task {
    pub id: TaskId,
    pub kind: TaskKind,
    pub created_at: Instant,
    pub due_at: Option<Instant>,
    pub callback: Box<dyn FnOnce() + Send>,
}
