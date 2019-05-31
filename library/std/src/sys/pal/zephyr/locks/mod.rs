#[path = "../../../sync/condvar/no_threads.rs"]
mod condvar;
mod mutex;
#[path = "../../../sync/rwlock/no_threads.rs"]
mod rwlock;
pub use condvar::Condvar;
pub use mutex::Mutex;
pub use rwlock::RwLock;
