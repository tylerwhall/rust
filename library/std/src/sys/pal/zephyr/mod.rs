use crate::io::ErrorKind;

pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../../cmath.rs"]
pub mod cmath;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../../os_str/mod.rs"]
pub mod os_str;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
pub mod thread;
pub mod thread_parking;
pub mod time;

#[path = "../unsupported/common.rs"]
#[deny(unsafe_op_in_unsafe_fn)]
#[allow(unused)]
mod common;
pub use common::{abort_internal, cleanup, init, unsupported, unsupported_err};

// Could import user-only here with a feature
use zephyr_core::any as zephyr;

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

pub use libc::strlen;

#[inline]
pub(crate) fn is_interrupted(errno: i32) -> bool {
    errno as u32 == zephyr_sys::raw::EINTR
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
        x if x == zephyr_sys::raw::EAGAIN || x == zephyr_sys::raw::EWOULDBLOCK => {
            ErrorKind::WouldBlock
        }

        _ => ErrorKind::Other,
    }
}
