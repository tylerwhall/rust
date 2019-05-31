use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ptr;

use zephyr_core::context::Any as Context;
use zephyr_core::mutex::*;
use zephyr_core::mutex_alloc::DynMutex;

pub struct Mutex(UnsafeCell<MaybeUninit<DynMutex>>);

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    #[rustc_const_stable(feature = "const_sys_mutex_new", since = "1.0.0")]
    pub const fn new() -> Mutex {
        Mutex(UnsafeCell::new(MaybeUninit::uninit()))
    }

    unsafe fn as_ref(&self) -> &DynMutex {
        &*(*self.0.get()).as_ptr()
    }

    unsafe fn as_mut_ptr(&self) -> *mut DynMutex {
        (*self.0.get()).as_mut_ptr()
    }

    #[inline]
    pub unsafe fn init(&self) {
        *self.0.get() = MaybeUninit::new(DynMutex::new::<Context>().expect("mutex alloc"))
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

    #[inline]
    pub unsafe fn destroy(&self) {
        ptr::drop_in_place(self.as_mut_ptr());
    }
}

pub struct ReentrantMutex {}

impl ReentrantMutex {
    pub const fn uninitialized() -> ReentrantMutex {
        ReentrantMutex {}
    }

    #[inline]
    pub unsafe fn init(&self) {}

    #[inline]
    pub unsafe fn lock(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn unlock(&self) {}

    #[inline]
    pub unsafe fn destroy(&self) {}
}
