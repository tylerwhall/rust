use crate::sync::atomic::{AtomicUsize, Ordering};

pub type Key = usize;

static KEY_INDEX: AtomicUsize = AtomicUsize::new(0);
const MAX_KEYS: usize = 32;

#[inline]
pub fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let ret = KEY_INDEX.fetch_add(1, Ordering::Relaxed);
    assert!(ret < MAX_KEYS);
    ret
}

#[inline]
unsafe fn get_or_create_tls_area() -> *mut [*mut u8; MAX_KEYS] {
    let space = zephyr_core::any::k_thread_custom_data_get();
    let ptr = if space.is_null() {
        let boxed = Box::new([core::ptr::null::<u8>(); MAX_KEYS]);
        let ptr: *mut u8 = Box::into_raw(boxed) as *mut _ as *mut u8;
        zephyr_core::any::k_thread_custom_data_set(ptr);
        ptr
    } else {
        space
    };
    // SAFETY: the pointer is this thread's boxed TLS table, allocated above.
    let mem: &mut [*mut u8; MAX_KEYS] = unsafe { &mut *(ptr as *mut [*mut u8; MAX_KEYS]) };
    mem
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    // SAFETY: the caller supplies a valid key for this thread's TLS table.
    unsafe {
        let mem = get_or_create_tls_area();
        (*mem)[key] = value;
    }
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    // SAFETY: the caller supplies a valid key for this thread's TLS table.
    unsafe {
        let mem = get_or_create_tls_area();
        (*mem)[key]
    }
}

#[inline]
pub unsafe fn destroy(_key: Key) {
    // nothing for now
}
