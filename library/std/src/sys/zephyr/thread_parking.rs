use crate::time::Duration;

pub use zephyr_core::thread::ThreadId;

use zephyr_core::thread::ThreadSyscalls;
use zephyr_core::context::Any as C;

pub fn current() -> ThreadId {
    C::k_current_get()
}

pub fn park(_hint: usize) {
    C::k_current_get().k_thread_suspend::<C>()
}

pub fn park_timeout(dur: Duration, _hint: usize) {
    crate::thread::sleep(dur)
}

pub fn unpark(tid: ThreadId, _hint: usize) {
    tid.k_wakeup::<C>()
}
