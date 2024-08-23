use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub const LOCK_FILE_NAME: &str = "centerpiece.lock";
pub const XDG_RUNTIME_DIR_ENV: &str = "XDG_RUNTIME_DIR";

/// Queries the environment for the $XDG_RUNTIME_DIR
fn get_xdg_runtime_dir() -> Option<String> {
    env::var(XDG_RUNTIME_DIR_ENV).ok()
}

#[derive(Debug)]
pub struct LockFile(PathBuf);

impl LockFile {
    pub fn get_or_init() -> &'static Option<Self> {
        static LOCK_FILE: OnceLock<Option<LockFile>> = OnceLock::new();
        LOCK_FILE.get_or_init(Self::init)
    }

    fn init() -> Option<Self> {
        Some(Self(Self::get_lock_file_path()?))
    }

    fn path(&self) -> &Path {
        self.0.as_path()
    }

    fn get_lock_file_path() -> Option<PathBuf> {
        let xdg_runtime_dir = get_xdg_runtime_dir()?;
        Some(Path::new(&xdg_runtime_dir).join(LOCK_FILE_NAME))
    }

    fn try_lock(&self) -> std::io::Result<()> {
        if self.path().is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File found",
            ));
        } else {
            let _ = File::create(self.path())?;
        };
        Ok(())
    }

    pub fn unlock() -> std::io::Result<()> {
        if let Some(lock_file) = Self::get_or_init() {
            std::fs::remove_file(&lock_file.path())?;
        }
        Ok(())
    }
    /// Attempts to hold an exclusive lock in the runtime dir.
    /// If we can't find the XDG_RUNTIME_DIR, we don't hold a lock.
    /// If the lock is not successful, then exit `centerpiece`.
    pub fn run_exclusive() {
        if let Some(lock_file) = Self::get_or_init() {
            if lock_file.try_lock().is_err() {
                eprintln!(
                    "Could not hold an exclusive lock in {lock_file:?} stopping centerpiece."
                );
                std::process::exit(1);
            }
        }
    }
}
