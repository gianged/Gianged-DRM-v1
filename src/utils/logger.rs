use colored::Colorize;
use regex::Regex;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Log levels for the logger
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Critical = 4,
}

impl LogLevel {
    fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRIT",
        }
    }
}

/// Thread-safe logger with file output and sanitization
pub struct Logger {
    inner: Arc<Mutex<LoggerInner>>,
}

struct LoggerInner {
    min_level: LogLevel,
    log_file: Option<File>,
    log_dir: PathBuf,
    current_log_index: usize,
    max_log_files: usize,
    max_file_size: u64,
}

impl Logger {
    /// Create a new logger instance
    pub fn new() -> Result<Self, std::io::Error> {
        let log_dir = Self::get_log_directory()?;
        fs::create_dir_all(&log_dir)?;

        let log_file_path = log_dir.join("drm.log");
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)?;

        Ok(Self {
            inner: Arc::new(Mutex::new(LoggerInner {
                min_level: LogLevel::Info,
                log_file: Some(log_file),
                log_dir,
                current_log_index: 0,
                max_log_files: 10,
                max_file_size: 10 * 1024 * 1024, // 10MB
            })),
        })
    }

    /// Create a logger without file output
    pub fn console_only() -> Self {
        let log_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("DRM")
            .join("Logs");

        Self {
            inner: Arc::new(Mutex::new(LoggerInner {
                min_level: LogLevel::Info,
                log_file: None,
                log_dir,
                current_log_index: 0,
                max_log_files: 10,
                max_file_size: 10 * 1024 * 1024,
            })),
        }
    }

    /// Set the minimum log level
    pub fn set_min_level(&self, level: LogLevel) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.min_level = level;
        }
    }

    /// Log a debug message
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Log an info message
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Log a warning message
    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    /// Log an error message
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    /// Log a critical message
    pub fn critical(&self, message: &str) {
        self.log(LogLevel::Critical, message);
    }

    /// Main logging function
    fn log(&self, level: LogLevel, message: &str) {
        if let Ok(mut inner) = self.inner.lock() {
            if level < inner.min_level {
                return;
            }

            let sanitized_message = Self::sanitize_message(message);
            let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let log_entry = format!("[{}] [{}] {}", timestamp, level.as_str(), sanitized_message);

            // Console output with color
            let colored_level = match level {
                LogLevel::Debug => level.as_str().cyan().bold(),
                LogLevel::Info => level.as_str().green().bold(),
                LogLevel::Warning => level.as_str().yellow().bold(),
                LogLevel::Error => level.as_str().red().bold(),
                LogLevel::Critical => level.as_str().magenta().bold(),
            };
            println!("[{}] [{}] {}", timestamp, colored_level, sanitized_message);

            // File output
            if let Some(ref mut file) = inner.log_file {
                if let Err(e) = writeln!(file, "{}", log_entry) {
                    eprintln!("Failed to write to log file: {}", e);
                }

                // Check file size and rotate if needed
                if let Ok(metadata) = file.metadata() {
                    if metadata.len() > inner.max_file_size {
                        let _ = inner.rotate_logs();
                    }
                }
            }
        }
    }

    /// Sanitize sensitive data from log messages
    fn sanitize_message(message: &str) -> String {
        let mut sanitized = message.to_string();

        // Sanitize patterns
        let patterns = vec![
            // Base64 patterns (likely to be keys or encrypted data)
            (Regex::new(r"[A-Za-z0-9+/]{32,}={0,2}").unwrap(), "[REDACTED_BASE64]"),
            // Hex patterns (likely to be keys or hashes)
            (Regex::new(r"[0-9a-fA-F]{32,}").unwrap(), "[REDACTED_HEX]"),
            // License key patterns
            (Regex::new(r"(TRIAL|PREM)-[A-Z0-9]{8}-\d{6}-[a-f0-9]{4}").unwrap(), "[REDACTED_LICENSE]"),
            // Common sensitive keywords
            (Regex::new(r"(?i)(password|secret|token|key)\s*[:=]\s*\S+").unwrap(), "$1: [REDACTED]"),
        ];

        for (pattern, replacement) in patterns {
            sanitized = pattern.replace_all(&sanitized, replacement).to_string();
        }

        sanitized
    }

    /// Get the log directory path
    fn get_log_directory() -> Result<PathBuf, std::io::Error> {
        let app_data_dir = dirs::data_dir()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine data directory"
            ))?;

        Ok(app_data_dir.join("DRM").join("Logs"))
    }
}

impl LoggerInner {
    /// Rotate log files
    fn rotate_logs(&mut self) -> Result<(), std::io::Error> {
        // Close current file
        self.log_file = None;

        // Rotate existing files
        for i in (1..self.max_log_files).rev() {
            let old_path = self.log_dir.join(format!("drm.log.{}", i));
            let new_path = self.log_dir.join(format!("drm.log.{}", i + 1));

            if old_path.exists() {
                if i + 1 >= self.max_log_files {
                    // Delete oldest file
                    fs::remove_file(&old_path)?;
                } else {
                    fs::rename(&old_path, &new_path)?;
                }
            }
        }

        // Rename current log file
        let current_path = self.log_dir.join("drm.log");
        let rotated_path = self.log_dir.join("drm.log.1");

        if current_path.exists() {
            fs::rename(&current_path, &rotated_path)?;
        }

        // Open new log file
        self.log_file = Some(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&current_path)?
        );

        self.current_log_index += 1;

        Ok(())
    }
}

impl Clone for Logger {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::console_only()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let logger = Logger::console_only();
        logger.info("Test message");
    }

    #[test]
    fn test_log_levels() {
        let logger = Logger::console_only();
        logger.set_min_level(LogLevel::Debug);

        logger.debug("Debug message");
        logger.info("Info message");
        logger.warning("Warning message");
        logger.error("Error message");
        logger.critical("Critical message");
    }

    #[test]
    fn test_sanitization() {
        let logger = Logger::console_only();

        // These should be sanitized
        logger.info("License key: TRIAL-ABCD1234-123456-abcd");
        logger.info("Password: secret123");
        logger.info("Base64 data: SGVsbG8gV29ybGQhIFRoaXMgaXMgYSB0ZXN0");
        logger.info("Hex: 1234567890abcdef1234567890abcdef12345678");
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warning);
        assert!(LogLevel::Warning < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Critical);
    }

    #[test]
    fn test_min_level_filtering() {
        let logger = Logger::console_only();
        logger.set_min_level(LogLevel::Warning);

        // These should not be logged (but we can't easily test that)
        logger.debug("This should not appear");
        logger.info("This should not appear");

        // These should be logged
        logger.warning("This should appear");
        logger.error("This should appear");
    }
}
