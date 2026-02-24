mod runtime;
mod task;

use runtime::Runtime;
use std::time::Duration;

fn main() {
    let mut runtime = Runtime::new();

    println!("start");

    runtime.set_timeout(Duration::from_millis(100000), || {
        println!("timeout");
    });

    runtime.q_microtask(crate::task::Task {
        id: 999,
        kind: crate::task::TaskKind::Microtask,
        created_at: std::time::Instant::now(),
        due_at: None,
        callback: Box::new(|| println!("promise")),
    });

    println!("end");

    runtime.run();
}
