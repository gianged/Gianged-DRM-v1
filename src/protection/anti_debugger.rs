use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DebuggerError {
    #[error("Debugger detected: {0}")]
    DebuggerDetected(String),

    #[error("Detection check failed: {0}")]
    CheckFailed(String),
}

pub type DebuggerResult<T> = Result<T, DebuggerError>;

/// Anti-debugger protection mechanism
pub struct AntiDebugger {
    last_check: Instant,
    check_interval: Duration,
}

impl AntiDebugger {
    /// Create a new anti-debugger instance
    pub fn new() -> Self {
        Self {
            last_check: Instant::now(),
            check_interval: Duration::from_secs(1),
        }
    }

    /// Set the check interval
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.check_interval = interval;
        self
    }

    /// Check if a debugger is attached
    pub fn check(&mut self) -> DebuggerResult<()> {
        self.last_check = Instant::now();

        // Check for attached debugger
        if Self::is_debugger_present()? {
            return Err(DebuggerError::DebuggerDetected(
                "Debugger is attached to the process".to_string(),
            ));
        }

        // Check for debugger processes
        if Self::check_debugger_processes()? {
            return Err(DebuggerError::DebuggerDetected(
                "Debugger process detected".to_string(),
            ));
        }

        // Timing-based detection
        if Self::timing_check()? {
            return Err(DebuggerError::DebuggerDetected(
                "Timing anomaly detected (possible debugger)".to_string(),
            ));
        }

        Ok(())
    }

    /// Quick check without timing tests
    pub fn quick_check(&self) -> DebuggerResult<()> {
        if Self::is_debugger_present()? {
            return Err(DebuggerError::DebuggerDetected(
                "Debugger is attached".to_string(),
            ));
        }
        Ok(())
    }

    /// Platform-specific check for attached debugger
    fn is_debugger_present() -> DebuggerResult<bool> {
        #[cfg(target_os = "windows")]
        {
            Self::is_debugger_present_windows()
        }

        #[cfg(target_os = "linux")]
        {
            Self::is_debugger_present_linux()
        }

        #[cfg(target_os = "macos")]
        {
            Self::is_debugger_present_macos()
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Ok(false)
        }
    }

    #[cfg(target_os = "windows")]
    fn is_debugger_present_windows() -> DebuggerResult<bool> {
        use windows::Win32::System::Diagnostics::Debug::IsDebuggerPresent;

        unsafe {
            Ok(IsDebuggerPresent().as_bool())
        }
    }

    #[cfg(target_os = "linux")]
    fn is_debugger_present_linux() -> DebuggerResult<bool> {
        use std::fs;

        // Check /proc/self/status for TracerPid
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("TracerPid:") {
                    if let Some(pid_str) = line.split_whitespace().nth(1) {
                        if let Ok(pid) = pid_str.parse::<i32>() {
                            return Ok(pid != 0);
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    #[cfg(target_os = "macos")]
    fn is_debugger_present_macos() -> DebuggerResult<bool> {
        use std::process::Command;

        // Check using sysctl
        if let Ok(output) = Command::new("sysctl")
            .args(&["-n", "kern.proc.pid.self"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // This is a simplified check; real implementation would be more complex
            if stdout.contains("debug") {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check for known debugger process names
    fn check_debugger_processes() -> DebuggerResult<bool> {
        let debugger_names = [
            "gdb",
            "lldb",
            "x64dbg",
            "x32dbg",
            "ollydbg",
            "windbg",
            "ida",
            "ida64",
            "idaq",
            "idaq64",
            "idaw",
            "idaw64",
            "scylla",
            "protection_id",
            "megadumper",
            "dumpcap",
            "wireshark",
            "dbg",
            "debugger",
        ];

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;

            if let Ok(output) = Command::new("tasklist").output() {
                let process_list = String::from_utf8_lossy(&output.stdout).to_lowercase();

                for debugger in &debugger_names {
                    if process_list.contains(debugger) {
                        return Ok(true);
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;

            if let Ok(entries) = fs::read_dir("/proc") {
                for entry in entries.flatten() {
                    if let Ok(file_name) = entry.file_name().into_string() {
                        if file_name.parse::<u32>().is_ok() {
                            let cmdline_path = entry.path().join("cmdline");
                            if let Ok(cmdline) = fs::read_to_string(&cmdline_path) {
                                let cmdline_lower = cmdline.to_lowercase();
                                for debugger in &debugger_names {
                                    if cmdline_lower.contains(debugger) {
                                        return Ok(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;

            if let Ok(output) = Command::new("ps").args(&["-A", "-o", "comm="]).output() {
                let process_list = String::from_utf8_lossy(&output.stdout).to_lowercase();

                for debugger in &debugger_names {
                    if process_list.contains(debugger) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Timing-based debugger detection
    fn timing_check() -> DebuggerResult<bool> {
        let start = Instant::now();

        // Perform a simple operation
        let mut sum = 0u64;
        for i in 0..1000 {
            sum = sum.wrapping_add(i);
        }

        let elapsed = start.elapsed();

        // If execution took longer than expected, might be debugged
        // This is a heuristic and can have false positives
        let threshold = Duration::from_millis(10);

        // Prevent compiler optimization
        if sum == u64::MAX {
            println!("Impossible");
        }

        Ok(elapsed > threshold)
    }

    /// Exit if debugger is detected
    pub fn exit_if_detected(&mut self) {
        if let Err(e) = self.check() {
            eprintln!("Security Alert: {}", e);
            std::process::exit(1);
        }
    }

    /// Continuous monitoring (blocking)
    pub fn start_monitoring(&mut self, duration: Duration) {
        let end_time = Instant::now() + duration;

        println!("Starting anti-debugger monitoring...");

        while Instant::now() < end_time {
            if let Err(e) = self.check() {
                eprintln!("Security Alert: {}", e);
                std::process::exit(1);
            }

            std::thread::sleep(self.check_interval);
        }

        println!("Monitoring completed.");
    }

    /// Get time since last check
    pub fn time_since_last_check(&self) -> Duration {
        self.last_check.elapsed()
    }

    /// Get status report
    pub fn get_status(&mut self) -> String {
        match self.check() {
            Ok(_) => "No debugger detected".to_string(),
            Err(e) => format!("WARNING: {}", e),
        }
    }
}

impl Default for AntiDebugger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anti_debugger_creation() {
        let debugger = AntiDebugger::new();
        assert_eq!(debugger.check_interval, Duration::from_secs(1));
    }

    #[test]
    fn test_custom_interval() {
        let debugger = AntiDebugger::new().with_interval(Duration::from_secs(5));
        assert_eq!(debugger.check_interval, Duration::from_secs(5));
    }

    #[test]
    fn test_quick_check() {
        let debugger = AntiDebugger::new();
        // This should pass unless actually being debugged
        let result = debugger.quick_check();
        // We can't assert success because it depends on environment
        println!("Quick check result: {:?}", result);
    }

    #[test]
    fn test_timing_check() {
        // Basic test that timing check runs without panicking
        let result = AntiDebugger::timing_check();
        println!("Timing check result: {:?}", result);
        assert!(result.is_ok());
    }
}
