#[path = "../unsupported/os.rs"]
pub mod os;
pub mod thread;
pub mod thread_local_key;
pub mod time;

#[path = "../unsupported/common.rs"]
#[deny(unsafe_op_in_unsafe_fn)]
#[allow(unused)]
mod common;
pub use common::{abort_internal, cleanup, init, unsupported, unsupported_err};

// Could import user-only here with a feature
use zephyr_core::any as zephyr;

pub use libc::strlen;
