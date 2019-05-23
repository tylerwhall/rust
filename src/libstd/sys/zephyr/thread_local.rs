pub type Key = usize;

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern fn(*mut u8)>) -> Key {
    unimplemented!()
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    unimplemented!()
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    unimplemented!()
}

#[inline]
pub unsafe fn destroy(key: Key) {
    unimplemented!()
}

#[inline]
pub fn requires_synchronized_create() -> bool {
    false
}
