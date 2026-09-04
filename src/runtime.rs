use std::fs;
use std::os::unix::fs::DirBuilderExt;
use std::path::PathBuf;
use std::process::Command;

unsafe extern "C" {
    fn getuid() -> u32;
    fn kill(pid: i32, sig: i32) -> i32;
}

pub const SIGINT: i32 = 2;
pub const SIGTERM: i32 = 15;

/// Returns current process user ID via direct POSIX getuid syscall.
#[inline]
pub fn current_uid() -> u32 {
    unsafe { getuid() }
}

/// Sends a POSIX signal to a process using the native kill syscall.
#[inline]
pub fn send_signal(pid: u32, sig: i32) -> bool {
    unsafe { kill(pid as i32, sig) == 0 }
}

/// Returns a private, secure runtime directory isolated to the current user (mode 0700).
pub fn get_runtime_dir() -> PathBuf {
    let base = std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_default();
            PathBuf::from(home).join(".cache/shotdock/runtime")
        });
    let dir = base.join("shotdock");
    let _ = fs::DirBuilder::new()
        .mode(0o700)
        .recursive(true)
        .create(&dir);
    dir
}

/// Path to the shotdock GUI instance PID file.
pub fn get_dock_pid_file() -> PathBuf {
    get_runtime_dir().join("shotdock.pid")
}

/// Registers the current process PID in the secure runtime directory.
pub fn write_dock_pid() {
    let _ = fs::write(get_dock_pid_file(), std::process::id().to_string());
}

/// Removes the active shotdock PID file.
pub fn cleanup_dock_pid() {
    let _ = fs::remove_file(get_dock_pid_file());
}

/// Verifies that a given PID is currently active and matches the expected executable name.
pub fn is_process_running_with_comm(pid: u32, expected_comm: &str) -> bool {
    let comm_path = format!("/proc/{}/comm", pid);
    if let Ok(comm) = fs::read_to_string(comm_path) {
        return comm.trim() == expected_comm;
    }
    false
}

/// Sends signal to all user-owned processes matching command name.
pub fn pkill_user_process(comm: &str, sig: &str) {
    let uid = current_uid().to_string();
    let _ = Command::new("pkill")
        .args(["-u", &uid, sig, "-x", comm])
        .status();
}

/// Checks if any user-owned process with the given name is running.
pub fn is_user_process_running(comm: &str) -> bool {
    let uid = current_uid().to_string();
    Command::new("pgrep")
        .args(["-u", &uid, "-x", comm])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_uid() {
        assert!(current_uid() < 65536 * 100);
    }

    #[test]
    fn test_runtime_dir_creation() {
        let dir = get_runtime_dir();
        assert!(dir.exists());
        assert!(dir.ends_with("shotdock"));
    }
}
