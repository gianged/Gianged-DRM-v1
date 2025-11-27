use crate::core::CryptoHelper;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntegrityError {
    #[error("Integrity check failed: {0}")]
    IntegrityViolation(String),

    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid location: {0}")]
    InvalidLocation(String),

    #[error("Integrity check error: {0}")]
    CheckError(String),
}

pub type IntegrityResult<T> = Result<T, IntegrityError>;

/// Integrity checker for detecting tampering
pub struct IntegrityChecker {
    original_hash: Option<String>,
    executable_path: PathBuf,
}

impl IntegrityChecker {
    /// Create a new integrity checker
    pub fn new() -> IntegrityResult<Self> {
        let executable_path = env::current_exe()?;

        Ok(Self {
            original_hash: None,
            executable_path,
        })
    }

    /// Create with a specific executable path
    pub fn with_path(path: impl AsRef<Path>) -> IntegrityResult<Self> {
        Ok(Self {
            original_hash: None,
            executable_path: path.as_ref().to_path_buf(),
        })
    }

    /// Initialize by computing the current hash
    pub fn initialize(&mut self) -> IntegrityResult<()> {
        let hash = self.compute_current_hash()?;
        self.original_hash = Some(hash);
        Ok(())
    }

    /// Set the expected hash manually
    pub fn set_expected_hash(&mut self, hash: String) {
        self.original_hash = Some(hash);
    }

    /// Compute the current hash of the executable
    fn compute_current_hash(&self) -> IntegrityResult<String> {
        let data = fs::read(&self.executable_path)?;
        Ok(CryptoHelper::compute_sha256_hash_hex(&data))
    }

    /// Check if the executable has been modified
    pub fn verify_integrity(&self) -> IntegrityResult<()> {
        if let Some(ref original_hash) = self.original_hash {
            let current_hash = self.compute_current_hash()?;

            if &current_hash != original_hash {
                return Err(IntegrityError::IntegrityViolation(
                    "Executable has been modified".to_string(),
                ));
            }
        } else {
            return Err(IntegrityError::CheckError(
                "Integrity checker not initialized".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if the executable is running from a suspicious location
    pub fn verify_location(&self) -> IntegrityResult<()> {
        let path_str = self.executable_path.to_string_lossy().to_lowercase();

        // Check for suspicious directories
        let suspicious_dirs = [
            "temp",
            "tmp",
            "debug",
            "downloads",
            "desktop",
            "appdata\\local\\temp",
            "/tmp/",
            "/var/tmp/",
        ];

        for suspicious in &suspicious_dirs {
            if path_str.contains(suspicious) {
                return Err(IntegrityError::InvalidLocation(format!(
                    "Executable running from suspicious location: {}",
                    suspicious
                )));
            }
        }

        Ok(())
    }

    /// Perform all integrity checks
    pub fn check_all(&self) -> IntegrityResult<()> {
        self.verify_integrity()?;
        self.verify_location()?;
        Ok(())
    }

    /// Get the executable path
    pub fn get_executable_path(&self) -> &Path {
        &self.executable_path
    }

    /// Get the current hash
    pub fn get_current_hash(&self) -> IntegrityResult<String> {
        self.compute_current_hash()
    }

    /// Get the original (expected) hash
    pub fn get_expected_hash(&self) -> Option<&str> {
        self.original_hash.as_deref()
    }

    /// Check file integrity for any file
    pub fn check_file_integrity(
        &self,
        file_path: impl AsRef<Path>,
        expected_hash: &str,
    ) -> IntegrityResult<bool> {
        let data = fs::read(file_path.as_ref())?;
        let hash = CryptoHelper::compute_sha256_hash_hex(&data);
        Ok(hash == expected_hash)
    }

    /// Compute hash for any file
    pub fn compute_file_hash(file_path: impl AsRef<Path>) -> IntegrityResult<String> {
        let data = fs::read(file_path.as_ref())?;
        Ok(CryptoHelper::compute_sha256_hash_hex(&data))
    }

    /// Get detailed integrity report
    pub fn get_integrity_report(&self) -> String {
        let mut report = Vec::new();

        report.push("=== Integrity Check Report ===".to_string());
        report.push(format!("Executable: {}", self.executable_path.display()));
        report.push(String::new());

        // Hash check
        if let Some(ref expected_hash) = self.original_hash {
            match self.compute_current_hash() {
                Ok(current_hash) => {
                    if &current_hash == expected_hash {
                        report.push("✓ Hash Integrity: VALID".to_string());
                        report.push(format!("  Hash: {}", current_hash));
                    } else {
                        report.push("✗ Hash Integrity: VIOLATED".to_string());
                        report.push(format!("  Expected: {}", expected_hash));
                        report.push(format!("  Current:  {}", current_hash));
                    }
                }
                Err(e) => {
                    report.push(format!("✗ Hash Check: ERROR - {}", e));
                }
            }
        } else {
            report.push("⚠ Hash Integrity: NOT INITIALIZED".to_string());
        }

        report.push(String::new());

        // Location check
        match self.verify_location() {
            Ok(_) => report.push("✓ Location: VALID".to_string()),
            Err(e) => report.push(format!("✗ Location: {}", e)),
        }

        report.join("\n")
    }

    /// Exit if integrity check fails
    pub fn exit_if_violated(&self) {
        if let Err(e) = self.check_all() {
            eprintln!("Security Alert: {}", e);
            std::process::exit(1);
        }
    }
}

impl Default for IntegrityChecker {
    fn default() -> Self {
        Self::new().expect("Failed to create integrity checker")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_integrity_checker_creation() {
        let checker = IntegrityChecker::new();
        assert!(checker.is_ok());
    }

    #[test]
    fn test_file_hash_computation() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "test content").unwrap();

        let hash = IntegrityChecker::compute_file_hash(temp_file.path()).unwrap();
        assert_eq!(hash.len(), 64); // SHA-256 hex is 64 characters
    }

    #[test]
    fn test_file_integrity_check() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "test content").unwrap();

        let checker = IntegrityChecker::new().unwrap();
        let hash = IntegrityChecker::compute_file_hash(temp_file.path()).unwrap();

        let is_valid = checker.check_file_integrity(temp_file.path(), &hash).unwrap();
        assert!(is_valid);

        let is_invalid = checker
            .check_file_integrity(temp_file.path(), "wrong_hash")
            .unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_initialize_and_verify() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "test content").unwrap();

        let mut checker = IntegrityChecker::with_path(temp_file.path()).unwrap();
        checker.initialize().unwrap();

        let result = checker.verify_integrity();
        assert!(result.is_ok());
    }

    #[test]
    fn test_location_verification() {
        let checker = IntegrityChecker::new().unwrap();
        // This test depends on where the test is run from
        // Just ensure it doesn't panic
        let _ = checker.verify_location();
    }

    #[test]
    fn test_hash_consistency() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "consistent content").unwrap();

        let hash1 = IntegrityChecker::compute_file_hash(temp_file.path()).unwrap();
        let hash2 = IntegrityChecker::compute_file_hash(temp_file.path()).unwrap();

        assert_eq!(hash1, hash2);
    }
}
