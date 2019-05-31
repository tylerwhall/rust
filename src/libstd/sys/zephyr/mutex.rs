pub struct Mutex(zephyr::kernel::KMutex);

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    pub const fn new() -> Mutex {
        // Only safe because std boxes this and if we're using k_malloc for box
        unsafe { Mutex(zephyr::kernel::KMutex::uninit()) }
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        self.0.init()
    }

    #[inline]
    pub unsafe fn lock(&self) {
        self.0.lock()
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        self.0.unlock()
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        self.0.try_lock()
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
