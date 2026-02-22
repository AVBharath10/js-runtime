mod runtime;
mod task;
use runtime::Runtime;

fn main() {
    let mut runtime = Runtime::new();
    println!("start");
    runtime.q_callbackq(Box::new(|| println!("timeout")));
    runtime.q_microtask(Box::new(|| println!("promise")));
    println!("end");
    runtime.run();
}
