use crate::io::ErrorKind;

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod condvar;
#[path = "../wasm/fs.rs"]
pub mod fs;
#[path = "../wasm/io.rs"]
pub mod io;
#[path = "../unix/memchr.rs"]
pub mod memchr;
pub mod mutex;
#[path = "../wasm/net.rs"]
pub mod net;
pub mod os;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../wasm/pipe.rs"]
pub mod pipe;
#[path = "../wasm/process.rs"]
pub mod process;
pub mod rwlock;
#[path = "../wasm/stack_overflow.rs"]
pub mod stack_overflow;
pub mod stdio;
pub mod thread;
pub mod thread_local;
pub mod time;

// Could import user-only here with a feature
use ::zephyr::any as zephyr;

pub mod env {
    pub mod os {
        pub const FAMILY: &str = "zephyr";
        pub const OS: &str = "zephyr";
        pub const DLL_PREFIX: &str = "";
        pub const DLL_SUFFIX: &str = "";
        pub const DLL_EXTENSION: &str = "";
        pub const EXE_SUFFIX: &str = "";
        pub const EXE_EXTENSION: &str = "";
    }
}

pub use crate::sys_common::os_str_bytes as os_str;
pub use libc::strlen;

pub fn hashmap_random_keys() -> (u64, u64) {
    (0, 0)
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

#[cfg(not(test))]
pub fn init() {
}

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::Error::new(crate::io::ErrorKind::Other,
                   "operation not supported on zephyr yet")
}

pub fn decode_error_kind(errno: i32) -> crate::io::ErrorKind {
    match errno as u32 {
        zephyr_sys::raw::ECONNREFUSED => ErrorKind::ConnectionRefused,
        zephyr_sys::raw::ECONNRESET => ErrorKind::ConnectionReset,
        zephyr_sys::raw::EPERM | zephyr_sys::raw::EACCES => ErrorKind::PermissionDenied,
        zephyr_sys::raw::EPIPE => ErrorKind::BrokenPipe,
        zephyr_sys::raw::ENOTCONN => ErrorKind::NotConnected,
        zephyr_sys::raw::ECONNABORTED => ErrorKind::ConnectionAborted,
        zephyr_sys::raw::EADDRNOTAVAIL => ErrorKind::AddrNotAvailable,
        zephyr_sys::raw::EADDRINUSE => ErrorKind::AddrInUse,
        zephyr_sys::raw::ENOENT => ErrorKind::NotFound,
        zephyr_sys::raw::EINTR => ErrorKind::Interrupted,
        zephyr_sys::raw::EINVAL => ErrorKind::InvalidInput,
        zephyr_sys::raw::ETIMEDOUT => ErrorKind::TimedOut,
        zephyr_sys::raw::EEXIST => ErrorKind::AlreadyExists,

        // These two constants can have the same value on some systems,
        // but different values on others, so we can't use a match
        // clause
        x if x == zephyr_sys::raw::EAGAIN || x == zephyr_sys::raw::EWOULDBLOCK =>
            ErrorKind::WouldBlock,

        _ => ErrorKind::Other,
    }
}

pub unsafe fn abort_internal() -> ! {
    core::intrinsics::abort();
}
