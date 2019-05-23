pub fn errno() -> i32 {
    0
}

pub fn decode_error_kind(errno: i32) -> crate::io::ErrorKind {
    use crate::io::ErrorKind;

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

pub fn error_string(_errno: i32) -> String {
    "operation successful".to_string()
}
