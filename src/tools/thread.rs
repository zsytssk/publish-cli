use std::sync::OnceLock;

use super::{
    no_sync_wrap::NoSyncWrap,
    thread_pool::{PoolJoinHandle, ThreadPool},
};

static THREAD_POOL: OnceLock<NoSyncWrap<ThreadPool>> = OnceLock::new();

pub fn run_in_thread<F, T>(f: F) -> PoolJoinHandle<T>
where
    F: FnOnce() -> T + 'static + Send,
    T: 'static + Send,
{
    let pool = THREAD_POOL.get_or_init(|| NoSyncWrap::new(ThreadPool::new(3)));

    pool.get().unwrap().execute(move || {
        // Send the result back to the main thread
        f()
    })
    // thread::spawn(move || {
    //     // Send the result back to the main thread
    //     f()
    // })
}
