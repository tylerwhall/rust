pub struct RWLock {}

unsafe impl Send for RWLock {}
unsafe impl Sync for RWLock {} // no threads on wasm

impl RWLock {
    pub const fn new() -> RWLock {
        RWLock {}
    }

    #[inline]
    pub unsafe fn read(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn write(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        unimplemented!()
    }

    #[inline]
    pub unsafe fn destroy(&self) {
    }
}
