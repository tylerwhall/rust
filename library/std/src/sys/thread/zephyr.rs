use crate::convert::TryInto;
use crate::io;
use crate::num::NonZero;
use crate::thread::ThreadInit;
use crate::time::Duration;

pub struct Thread(!);

pub const DEFAULT_MIN_STACK_SIZE: usize = 1024;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, _init: Box<ThreadInit>) -> io::Result<Thread> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }

    pub fn join(self) {
        self.0
    }
}

pub fn yield_now() {
    // do nothing
}

pub fn sleep(dur: Duration) {
    zephyr_core::any::k_sleep((&dur).try_into().unwrap());
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    Ok(unsafe { NonZero::new_unchecked(zephyr_sys::raw::CONFIG_MP_NUM_CPUS as usize) })
}
