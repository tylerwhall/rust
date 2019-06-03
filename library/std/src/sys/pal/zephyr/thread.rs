use super::unsupported;
use crate::convert::TryInto;
use crate::ffi::CStr;
use crate::io;
use crate::num::NonZeroUsize;
use crate::time::{Duration, Instant};

pub struct Thread(!);

pub const DEFAULT_MIN_STACK_SIZE: usize = 1024;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(
        _stack: usize,
        _name: Option<&str>,
        _p: Box<dyn FnOnce()>,
    ) -> io::Result<Thread> {
        unsupported()
    }

    pub fn yield_now() {
        // do nothing
    }

    pub fn set_name(_name: &CStr) {
        // nope
    }

    pub fn sleep(dur: Duration) {
        zephyr_core::any::k_sleep((&dur).try_into().unwrap());
    }

    // Zephyr has no absolute sleep primitive; like the platforms
    // documented as "Other", derive the remaining duration and sleep
    // relatively.
    pub fn sleep_until(deadline: Instant) {
        if let Some(dur) = deadline.checked_duration_since(Instant::now()) {
            Self::sleep(dur)
        }
    }

    pub fn join(self) {
        self.0
    }
}

pub fn available_parallelism() -> io::Result<NonZeroUsize> {
    Ok(unsafe { NonZeroUsize::new_unchecked(zephyr_sys::raw::CONFIG_MP_NUM_CPUS as usize) })
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
