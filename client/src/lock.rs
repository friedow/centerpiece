use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

pub const LOCK_FILE_NAME: &str = "centerpiece.lock";
pub const XDG_RUNTIME_DIR_ENV: &str = "XDG_RUNTIME_DIR";

/// Queries the environment for the $XDG_RUNTIME_DIR
fn get_xdg_runtime_dir() -> Option<String> {
    env::var(XDG_RUNTIME_DIR_ENV).ok()
}

#[derive(Debug)]
pub struct LockFile(PathBuf);

impl LockFile {
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
                "file not found",
            ));
        } else {
            let _ = File::create(self.path())?;
        };
        Ok(())
    }

    pub fn unlock(&self) -> std::io::Result<()> {
        std::fs::remove_file(&self.path())?;
        Ok(())
    }

    /// Attempts to hold an exclusive lock in the runtime dir.
    /// If we can't find the XDG_RUNTIME_DIR, we don't hold a lock.
    /// If the lock is not successful, then exit `centerpiece`.
    pub fn run_exclusive() {
        if let Some(lock_file) = Self::init() {
            if lock_file.try_lock().is_err() {
                eprintln!(
                    "Could not hold an exclusive lock in {lock_file:?} stopping centerpiece."
                );
                std::process::exit(1);
            }
        }
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        let _ = self.unlock();
    }
}
