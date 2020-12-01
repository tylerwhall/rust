use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ptr;

use zephyr::context::Any as Context;
use zephyr::mutex::*;
use zephyr::mutex_alloc::DynMutex;

pub struct Mutex(UnsafeCell<MaybeUninit<DynMutex>>);

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
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
    pub unsafe fn init(&mut self) {
        self.0 = UnsafeCell::new(MaybeUninit::new(DynMutex::new::<Context>().expect("mutex alloc")))
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
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex {}
    }

    #[inline]
    pub unsafe fn init(&mut self) {}

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
