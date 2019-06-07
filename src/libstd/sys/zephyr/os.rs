use crate::error::Error as StdError;
use crate::ffi::{OsString, OsStr};
use crate::fmt;
use crate::io;
use crate::path::{Path, PathBuf};
use crate::sys::{unsupported, Void};

pub fn errno() -> i32 {
    //(*z_errno())
    unimplemented!()
}

pub fn error_string(errno: i32) -> String {
    format!("errno {}", errno)
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub fn getcwd() -> io::Result<PathBuf> {
    unsupported()
}

pub fn chdir(_: &Path) -> io::Result<()> {
    unsupported()
}

pub struct SplitPaths<'a>(&'a Void);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    panic!("unsupported")
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        match *self.0 {}
    }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
    where I: Iterator<Item=T>, T: AsRef<OsStr>
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "not supported on wasm yet".fmt(f)
    }
}

impl StdError for JoinPathsError {
    fn description(&self) -> &str {
        "not supported on wasm yet"
    }
}


pub struct Env(());

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        None
    }
}

pub fn env() -> Env {
    panic!("not supported on zephyr")
}

pub fn getenv(_k: &OsStr) -> io::Result<Option<OsString>> {
    unsupported()
}

pub fn setenv(_k: &OsStr, _v: &OsStr) -> io::Result<()> {
    unsupported()
}

pub fn unsetenv(_k: &OsStr) -> io::Result<()> {
    unsupported()
}

pub fn temp_dir() -> PathBuf {
    panic!("not supported on zephyr")
}

pub fn home_dir() -> Option<PathBuf> {
    None
}

pub fn exit(_code: i32) -> ! {
    panic!("not supported on zephyr")
}

pub fn getpid() -> u32 {
    panic!("not supported on zephyr")
}
