use crate::avif::lock;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Condvar, Mutex};
use std::thread::JoinHandle;

type Task = Box<dyn FnOnce() + Send>;

struct State {
    task: Option<Task>,
    stopping: bool,
    thread: Option<JoinHandle<()>>,
}

static STATE: Mutex<State> = Mutex::new(State {
    task: None,
    stopping: false,
    thread: None,
});
static WAKE: Condvar = Condvar::new();

/// Runs `task` on the background thread, starting the thread on first use
pub fn request(task: impl FnOnce() + Send + 'static) {
    let mut state = lock(&STATE);
    if state.stopping {
        return;
    }

    if state.thread.is_none() {
        let spawned = std::thread::Builder::new()
            .name("avif-prefetch".into())
            .spawn(run);
        match spawned {
            Ok(thread) => state.thread = Some(thread),
            // decoding ahead is only an optimization, so it is okay if the thread fails to start
            Err(_) => return,
        }
    }

    state.task = Some(Box::new(task));
    WAKE.notify_one();
}

/// Stops the thread and waits for it, the host unloads the library right after shutdown
pub fn stop() {
    let thread = {
        let mut state = lock(&STATE);
        state.stopping = true;
        state.task = None;
        state.thread.take()
    };

    WAKE.notify_all();
    if let Some(thread) = thread {
        let _ = thread.join();
    }

    lock(&STATE).stopping = false;
}

fn run() {
    loop {
        let task = {
            let mut state = lock(&STATE);
            loop {
                if state.stopping {
                    return;
                }
                if let Some(task) = state.task.take() {
                    break task;
                }
                state = WAKE.wait(state).unwrap_or_else(|e| e.into_inner());
            }
        };

        let _ = catch_unwind(AssertUnwindSafe(task));
    }
}
