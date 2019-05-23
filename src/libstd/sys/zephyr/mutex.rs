pub struct Mutex {}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    pub const fn new() -> Mutex {
        Mutex {}
    }

    #[inline]
    pub unsafe fn init(&mut self) {
    }

    #[inline]
    pub unsafe fn lock(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        unimplemented!()
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
