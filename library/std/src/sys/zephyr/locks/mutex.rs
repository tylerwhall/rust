use crate::ptr::null_mut;
use crate::sync::atomic::{
    AtomicPtr,
    Ordering::{AcqRel, Acquire},
};

use zephyr_core::context::Any as Context;
use zephyr_core::mutex::*;
use zephyr_core::mutex_alloc::DynMutex;

pub struct Mutex(AtomicPtr<KMutex>);

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    #[rustc_const_stable(feature = "const_sys_mutex_new", since = "1.0.0")]
    pub const fn new() -> Mutex {
        Mutex(AtomicPtr::new(null_mut()))
    }

    fn get_pointer(&self) -> *mut KMutex {
        let ptr = self.0.load(Acquire);
        if ptr.is_null() { self.initialize() } else { ptr }
    }

    unsafe fn as_ref(&self) -> &KMutex {
        &*self.get_pointer()
    }

    fn initialize(&self) -> *mut KMutex {
        // Copied from library/std/src/sys_common/lazy_box.rs
        let new_ptr = DynMutex::new::<Context>().expect("mutex alloc").into_raw();
        match self.0.compare_exchange(null_mut(), new_ptr, AcqRel, Acquire) {
            Ok(_) => new_ptr,
            Err(ptr) =>  {
                // Lost the race to another thread.
                // Drop the new one and use the existing one.
                drop(unsafe { DynMutex::from_raw(new_ptr) });
                ptr
            }
        }
    }

    #[inline]
    pub unsafe fn lock(&self) {
        self.as_ref().lock::<Context>()
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        self.as_ref().unlock::<Context>()
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        self.as_ref().try_lock::<Context>()
    }
}
