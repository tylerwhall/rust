use zephyr::context::Kernel as Context;
use zephyr::mutex::*;
use zephyr::mutex::global::k_mutex;

pub struct Mutex(k_mutex);

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    pub const fn new() -> Mutex {
        // Only safe because std boxes this and if we're using k_malloc for box
        unsafe { Mutex(k_mutex::uninit()) }
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        if zephyr::CONFIG_USERSPACE {
            // These need to be allocated with k_object_alloc(). Main mutex.rs
            // puts this in a Box which in not kernel memory when user space is
            // enabled.
            unimplemented!()
        }
        self.0.init::<Context>()
    }

    #[inline]
    pub unsafe fn lock(&self) {
        self.0.lock::<Context>()
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        self.0.unlock::<Context>()
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        self.0.try_lock::<Context>()
    }

    #[inline]
    pub unsafe fn destroy(&self) {
    }
}

pub struct ReentrantMutex {}

impl ReentrantMutex {
    pub unsafe fn uninitialized() -> ReentrantMutex {
        ReentrantMutex { }
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
