use std::env;
use std::fs::{File, TryLockError};
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

const LOCK_FILE_NAME: &str = "centerpiece.lock";

pub struct LockFile {
    _file: File,
}

impl LockFile {
    pub fn acquire() -> Option<Self> {
        let path = lock_file_path()?;
        let mut file = open_lock_file(&path)?;

        match file.try_lock() {
            Ok(()) => {
                write_pid(&mut file);
                log::info!("Acquired exclusive lock");
                Some(Self { _file: file })
            }
            Err(TryLockError::WouldBlock) => {
                kill_existing_instance(&mut file);
                // Blocks until the old process exits and releases the lock
                file.lock().ok()?;
                write_pid(&mut file);
                log::info!("Acquired exclusive lock after killing previous instance");
                Some(Self { _file: file })
            }
            Err(TryLockError::Error(e)) => {
                log::error!("Failed to acquire lock: {e}");
                None
            }
        }
    }

    pub fn cleanup() {
        if let Some(path) = lock_file_path() {
            let _ = std::fs::remove_file(path);
        }
    }
}

fn lock_file_path() -> Option<PathBuf> {
    let dir = env::var("XDG_RUNTIME_DIR").ok();
    if dir.is_none() {
        log::warn!("XDG_RUNTIME_DIR not set, running without file lock");
    }
    Some(Path::new(&dir?).join(LOCK_FILE_NAME))
}

fn open_lock_file(path: &Path) -> Option<File> {
    File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .map_err(|e| log::error!("Failed to open lock file: {e}"))
        .ok()
}

fn write_pid(file: &mut File) {
    let _ = file.set_len(0);
    let _ = file.seek(std::io::SeekFrom::Start(0));
    let _ = write!(file, "{}", std::process::id());
    let _ = file.flush();
}

fn read_pid(file: &mut File) -> Option<u32> {
    let mut contents = String::new();
    file.seek(std::io::SeekFrom::Start(0)).ok()?;
    file.read_to_string(&mut contents).ok()?;
    contents.trim().parse().ok()
}

fn kill_existing_instance(file: &mut File) {
    let Some(pid) = read_pid(file) else {
        log::warn!("Lock held but no PID in lock file");
        return;
    };
    log::info!("Killing existing instance with PID {pid}");
    // SAFETY: sending SIGTERM to a known PID
    unsafe {
        libc::kill(pid as i32, libc::SIGTERM);
    }
}
