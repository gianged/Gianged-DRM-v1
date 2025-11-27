use crate::core::CryptoHelper;
use crate::models::License;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Crypto error: {0}")]
    CryptoError(#[from] crate::core::crypto_helper::CryptoError),

    #[error("License not found")]
    NotFound,

    #[error("Storage error: {0}")]
    StorageError(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

/// Manager for license file storage with encryption
pub struct LicenseStorage {
    storage_dir: PathBuf,
    license_file: PathBuf,
}

impl LicenseStorage {
    /// Create a new license storage manager
    pub fn new() -> StorageResult<Self> {
        let storage_dir = Self::get_storage_directory()?;
        let license_file = storage_dir.join("license.drm");

        Ok(Self {
            storage_dir,
            license_file,
        })
    }

    /// Create a license storage with a custom directory
    pub fn with_directory(dir: impl AsRef<Path>) -> StorageResult<Self> {
        let storage_dir = dir.as_ref().to_path_buf();
        let license_file = storage_dir.join("license.drm");

        // Create directory if it doesn't exist
        fs::create_dir_all(&storage_dir)?;

        Ok(Self {
            storage_dir,
            license_file,
        })
    }

    /// Save a license to encrypted storage
    pub fn save_license(&self, license: &License) -> StorageResult<()> {
        // Ensure storage directory exists
        fs::create_dir_all(&self.storage_dir)?;

        // Serialize license to JSON
        let json = serde_json::to_string_pretty(license)?;

        // Encrypt the JSON data
        let encrypted = CryptoHelper::encrypt_string(&json)?;

        // Write to file
        fs::write(&self.license_file, encrypted)?;

        Ok(())
    }

    /// Load a license from encrypted storage
    pub fn load_license(&self) -> StorageResult<License> {
        // Check if file exists
        if !self.license_file.exists() {
            return Err(StorageError::NotFound);
        }

        // Read encrypted data
        let encrypted = fs::read_to_string(&self.license_file)?;

        // Decrypt the data
        let json = CryptoHelper::decrypt_string(&encrypted)?;

        // Deserialize from JSON
        let license: License = serde_json::from_str(&json)?;

        Ok(license)
    }

    /// Delete the stored license
    pub fn delete_license(&self) -> StorageResult<()> {
        if !self.license_file.exists() {
            return Err(StorageError::NotFound);
        }

        // Secure deletion: overwrite with random data first
        self.secure_delete(&self.license_file)?;

        Ok(())
    }

    /// Check if a license file exists
    pub fn license_exists(&self) -> bool {
        self.license_file.exists()
    }

    /// Get the license file path
    pub fn get_license_path(&self) -> &Path {
        &self.license_file
    }

    /// Get the storage directory path
    pub fn get_storage_directory() -> StorageResult<PathBuf> {
        let app_data_dir = dirs::data_dir()
            .ok_or_else(|| StorageError::StorageError("Could not determine data directory".to_string()))?;

        Ok(app_data_dir.join("DRM"))
    }

    /// Export license to a file (encrypted)
    pub fn export_license(&self, license: &License, export_path: impl AsRef<Path>) -> StorageResult<()> {
        let json = serde_json::to_string_pretty(license)?;
        let encrypted = CryptoHelper::encrypt_string(&json)?;
        fs::write(export_path.as_ref(), encrypted)?;
        Ok(())
    }

    /// Import license from a file (encrypted)
    pub fn import_license(&self, import_path: impl AsRef<Path>) -> StorageResult<License> {
        let encrypted = fs::read_to_string(import_path.as_ref())?;
        let json = CryptoHelper::decrypt_string(&encrypted)?;
        let license: License = serde_json::from_str(&json)?;
        Ok(license)
    }

    /// Save license to a specific file
    pub fn save_license_to_file(&self, license: &License, file_path: impl AsRef<Path>) -> StorageResult<()> {
        let json = serde_json::to_string_pretty(license)?;
        let encrypted = CryptoHelper::encrypt_string(&json)?;

        // Ensure parent directory exists
        if let Some(parent) = file_path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(file_path.as_ref(), encrypted)?;
        Ok(())
    }

    /// Load license from a specific file
    pub fn load_license_from_file(&self, file_path: impl AsRef<Path>) -> StorageResult<License> {
        let encrypted = fs::read_to_string(file_path.as_ref())?;
        let json = CryptoHelper::decrypt_string(&encrypted)?;
        let license: License = serde_json::from_str(&json)?;
        Ok(license)
    }

    /// Securely delete a file by overwriting with random data
    fn secure_delete(&self, path: &Path) -> StorageResult<()> {
        if !path.exists() {
            return Ok(());
        }

        // Get file size
        let metadata = fs::metadata(path)?;
        let file_size = metadata.len() as usize;

        // Overwrite with random data 3 times
        for _ in 0..3 {
            let random_data = CryptoHelper::generate_secure_random_bytes(file_size)?;
            fs::write(path, &random_data)?;
        }

        // Finally delete the file
        fs::remove_file(path)?;

        Ok(())
    }

    /// Clear all license data
    pub fn clear_all(&self) -> StorageResult<()> {
        if self.license_file.exists() {
            self.secure_delete(&self.license_file)?;
        }
        Ok(())
    }

    /// Get storage statistics
    pub fn get_storage_info(&self) -> StorageResult<StorageInfo> {
        let license_exists = self.license_file.exists();
        let file_size = if license_exists {
            Some(fs::metadata(&self.license_file)?.len())
        } else {
            None
        };

        Ok(StorageInfo {
            storage_dir: self.storage_dir.clone(),
            license_file: self.license_file.clone(),
            license_exists,
            file_size,
        })
    }
}

impl Default for LicenseStorage {
    fn default() -> Self {
        Self::new().expect("Failed to create default license storage")
    }
}

/// Information about license storage
#[derive(Debug, Clone)]
pub struct StorageInfo {
    pub storage_dir: PathBuf,
    pub license_file: PathBuf,
    pub license_exists: bool,
    pub file_size: Option<u64>,
}

impl StorageInfo {
    pub fn display(&self) -> String {
        format!(
            "Storage Information:\n\
             - Directory: {}\n\
             - License File: {}\n\
             - License Exists: {}\n\
             - File Size: {}",
            self.storage_dir.display(),
            self.license_file.display(),
            self.license_exists,
            self.file_size
                .map(|s| format!("{} bytes", s))
                .unwrap_or_else(|| "N/A".to_string())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::LicenseGenerator;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load_license() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LicenseStorage::with_directory(temp_dir.path()).unwrap();

        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine1", "Product")
            .unwrap();

        // Save license
        storage.save_license(&license).unwrap();
        assert!(storage.license_exists());

        // Load license
        let loaded_license = storage.load_license().unwrap();
        assert_eq!(license.license_key, loaded_license.license_key);
        assert_eq!(license.user_id, loaded_license.user_id);
    }

    #[test]
    fn test_delete_license() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LicenseStorage::with_directory(temp_dir.path()).unwrap();

        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine1", "Product")
            .unwrap();

        storage.save_license(&license).unwrap();
        assert!(storage.license_exists());

        storage.delete_license().unwrap();
        assert!(!storage.license_exists());
    }

    #[test]
    fn test_load_nonexistent_license() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LicenseStorage::with_directory(temp_dir.path()).unwrap();

        let result = storage.load_license();
        assert!(result.is_err());
        if let Err(StorageError::NotFound) = result {
            // Expected
        } else {
            panic!("Expected NotFound error");
        }
    }

    #[test]
    fn test_export_import_license() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LicenseStorage::with_directory(temp_dir.path()).unwrap();

        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine1", "Product")
            .unwrap();

        let export_path = temp_dir.path().join("exported_license.drm");

        // Export license
        storage.export_license(&license, &export_path).unwrap();
        assert!(export_path.exists());

        // Import license
        let imported_license = storage.import_license(&export_path).unwrap();
        assert_eq!(license.license_key, imported_license.license_key);
    }

    #[test]
    fn test_storage_info() {
        let temp_dir = TempDir::new().unwrap();
        let storage = LicenseStorage::with_directory(temp_dir.path()).unwrap();

        let info = storage.get_storage_info().unwrap();
        assert!(!info.license_exists);
        assert_eq!(info.file_size, None);

        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine1", "Product")
            .unwrap();

        storage.save_license(&license).unwrap();

        let info = storage.get_storage_info().unwrap();
        assert!(info.license_exists);
        assert!(info.file_size.is_some());
    }
}
