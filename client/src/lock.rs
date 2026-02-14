use std::env;
use std::fmt;
use std::fs::File;
use std::fs::TryLockError;
use std::path::{Path, PathBuf};

const LOCK_FILE_NAME: &str = "centerpiece.lock";
const XDG_RUNTIME_DIR_ENV: &str = "XDG_RUNTIME_DIR";

#[derive(Debug)]
pub enum LockFileError {
    AlreadyLocked(PathBuf),
    IoError(std::io::Error),
    UnlockError(PathBuf, std::io::Error),
}

type LockFileResult<T> = Result<T, LockFileError>;

impl fmt::Display for LockFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_path = |path: &PathBuf| path.to_string_lossy().to_string();
        match self {
            LockFileError::AlreadyLocked(path) => {
                write!(
                    f,
                    "Another instance is already running (lock: {})\n\
                    If you think this is an error, please remove the lockfile manually",
                    display_path(path)
                )
            }
            LockFileError::IoError(err) => write!(f, "IoError: {}", err),
            LockFileError::UnlockError(path, err) => {
                write!(
                    f,
                    "Failed to remove the lockfile: {} - IoError: {}",
                    display_path(path),
                    err
                )
            }
        }
    }
}

impl From<std::io::Error> for LockFileError {
    fn from(err: std::io::Error) -> Self {
        LockFileError::IoError(err)
    }
}

fn get_xdg_runtime_dir() -> Option<String> {
    env::var(XDG_RUNTIME_DIR_ENV).ok()
}

#[derive(Debug)]
pub struct LockFile {
    path: PathBuf,
    file: Option<File>,
}

impl LockFile {
    fn new() -> Option<Self> {
        Some(Self {
            path: Self::get_lock_file_path()?,
            file: None,
        })
    }

    fn path(&self) -> &Path {
        self.path.as_ref()
    }

    fn path_buf(&self) -> PathBuf {
        self.path().to_path_buf()
    }

    fn get_lock_file_path() -> Option<PathBuf> {
        let xdg_runtime_dir = get_xdg_runtime_dir()?;
        Some(Path::new(&xdg_runtime_dir).join(LOCK_FILE_NAME))
    }

    fn try_lock(&mut self) -> LockFileResult<()> {
        let file = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.path())?;

        match file.try_lock() {
            Ok(()) => {
                self.file = Some(file);
                Ok(())
            }
            Err(TryLockError::WouldBlock) => Err(LockFileError::AlreadyLocked(self.path_buf())),
            Err(TryLockError::Error(e)) => Err(LockFileError::IoError(e)),
        }
    }

    fn remove_lockfile() -> LockFileResult<()> {
        if let Some(lock_file) = Self::new() {
            std::fs::remove_file(lock_file.path())
                .map_err(|err| LockFileError::UnlockError(lock_file.path_buf(), err))?
        }
        Ok(())
    }

    pub fn unlock() {
        if let Err(err) = Self::remove_lockfile() {
            log::error!("{err}");
        }
    }

    pub fn acquire_exclusive_lock() -> LockFileResult<Self> {
        match Self::new() {
            Some(mut lock_file) => {
                lock_file.try_lock()?;
                Ok(lock_file)
            }
            None => {
                log::warn!("XDG_RUNTIME_DIR not found, running without file lock");
                Err(LockFileError::IoError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "XDG_RUNTIME_DIR environment variable not set",
                )))
            }
        }
    }

    pub fn run_exclusive() -> Self {
        match Self::acquire_exclusive_lock() {
            Ok(lock_file) => {
                log::info!("Successfully acquired exclusive lock");
                lock_file
            }
            Err(err) => {
                log::warn!("{err}");
                std::process::exit(0);
            }
        }
    }
}
