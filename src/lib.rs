//! Gianged DRM - Educational Digital Rights Management System
//!
//! This is an educational project demonstrating software licensing and protection mechanisms.
//! It showcases various security concepts including cryptography, hardware fingerprinting,
//! anti-tampering, and license validation.
//!
//! **IMPORTANT**: This is for educational purposes only and should not be used in production.

pub mod core;
pub mod hardware;
pub mod models;
pub mod protection;
pub mod storage;
pub mod utils;

// Re-export commonly used types
pub use core::{CryptoHelper, LicenseGenerator, LicenseValidator, ObfuscationHelper};
pub use hardware::MachineInfo;
pub use models::{License, LicenseFeature, LicenseTier};
pub use protection::{AntiDebugger, IntegrityChecker};
pub use storage::LicenseStorage;
pub use utils::{Encoder, Logger};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Get library information
pub fn get_info() -> String {
    format!("{} v{}", NAME, VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_info() {
        let info = get_info();
        assert!(info.contains(VERSION));
    }
}
