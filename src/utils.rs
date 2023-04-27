use std::thread;

#[allow(dead_code)]
pub fn run_async<F, R>(f: F) -> thread::JoinHandle<R>
where
    F: FnOnce() -> R,
    F: Send + 'static,
    R: Send + 'static,
{
    thread::spawn(f)
}
