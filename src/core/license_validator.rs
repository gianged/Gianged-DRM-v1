use crate::core::CryptoHelper;
use crate::models::{License, LicenseTier};
use chrono::{Duration, Utc};
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("License has expired")]
    Expired,

    #[error("Invalid license format")]
    InvalidFormat,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Machine ID mismatch")]
    MachineIdMismatch,

    #[error("Feature not available: {0}")]
    FeatureNotAvailable(String),

    #[error("Too many features for tier: {0}")]
    TooManyFeatures(String),

    #[error("Crypto error: {0}")]
    CryptoError(#[from] crate::core::crypto_helper::CryptoError),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validator for verifying software licenses
pub struct LicenseValidator {
    public_key: String,
    clock_skew_tolerance: Duration,
}

impl LicenseValidator {
    /// Create a new license validator with a public key
    pub fn new(public_key: String) -> Self {
        Self {
            public_key,
            clock_skew_tolerance: Duration::minutes(5),
        }
    }

    /// Set the clock skew tolerance for expiration checks
    pub fn with_clock_skew_tolerance(mut self, tolerance: Duration) -> Self {
        self.clock_skew_tolerance = tolerance;
        self
    }

    /// Validate a license completely
    pub fn validate(&self, license: &License, current_machine_id: &str) -> ValidationResult<()> {
        // Validate format
        self.validate_format(&license.license_key)?;

        // Validate expiration
        self.validate_expiration(license)?;

        // Validate machine ID
        self.validate_machine_id(license, current_machine_id)?;

        // Validate signature
        self.validate_signature(license)?;

        // Validate feature count
        self.validate_feature_count(license)?;

        Ok(())
    }

    /// Validate license key format
    pub fn validate_format(&self, license_key: &str) -> ValidationResult<()> {
        // Expected format: PREFIX-RANDOM-TIMESTAMP-CHECKSUM
        // Example: TRIAL-ABCD1234-123456-7890
        let pattern = r"^(TRIAL|PREM)-[A-Z0-9]{8}-\d{6}-[a-f0-9]{4}$";
        let regex = Regex::new(pattern).unwrap();

        if regex.is_match(license_key) {
            Ok(())
        } else {
            Err(ValidationError::InvalidFormat)
        }
    }

    /// Validate license expiration with clock skew tolerance
    pub fn validate_expiration(&self, license: &License) -> ValidationResult<()> {
        let now = Utc::now();
        let expiration_with_tolerance = license.expiration_date + self.clock_skew_tolerance;

        if now > expiration_with_tolerance {
            Err(ValidationError::Expired)
        } else {
            Ok(())
        }
    }

    /// Validate machine ID matches
    pub fn validate_machine_id(
        &self,
        license: &License,
        current_machine_id: &str,
    ) -> ValidationResult<()> {
        // Compare first 16 characters (short ID)
        let license_short_id: String = license.machine_id.chars().take(16).collect();
        let current_short_id: String = current_machine_id.chars().take(16).collect();

        if license_short_id == current_short_id {
            Ok(())
        } else {
            Err(ValidationError::MachineIdMismatch)
        }
    }

    /// Validate digital signature
    pub fn validate_signature(&self, license: &License) -> ValidationResult<()> {
        if license.signature.is_empty() {
            return Err(ValidationError::InvalidSignature);
        }

        let signing_data = license.get_signing_data();
        let signature_bytes = base64::decode(&license.signature)
            .map_err(|e| ValidationError::ValidationError(format!("Invalid signature encoding: {}", e)))?;

        let is_valid = CryptoHelper::verify_signature_rsa(
            signing_data.as_bytes(),
            &signature_bytes,
            &self.public_key,
        )?;

        if is_valid {
            Ok(())
        } else {
            Err(ValidationError::InvalidSignature)
        }
    }

    /// Validate feature count against tier limits
    pub fn validate_feature_count(&self, license: &License) -> ValidationResult<()> {
        let max_features = license.tier.max_features();

        if license.features.len() > max_features {
            Err(ValidationError::TooManyFeatures(format!(
                "{} features exceeds {} tier limit of {}",
                license.features.len(),
                license.tier.name(),
                max_features
            )))
        } else {
            Ok(())
        }
    }

    /// Check if a specific feature is available and valid
    pub fn check_feature(&self, license: &License, feature_name: &str) -> ValidationResult<()> {
        if license.has_feature(feature_name) {
            Ok(())
        } else {
            Err(ValidationError::FeatureNotAvailable(
                feature_name.to_string(),
            ))
        }
    }

    /// Validate a feature with custom logic
    pub fn validate_feature_value(
        &self,
        license: &License,
        feature_name: &str,
        expected_value: &str,
    ) -> ValidationResult<()> {
        if let Some(feature) = license.get_feature(feature_name) {
            if let Some(value) = feature.get_value() {
                if value == expected_value || value == "unlimited" {
                    Ok(())
                } else {
                    Err(ValidationError::FeatureNotAvailable(format!(
                        "{} value mismatch: expected {}, got {}",
                        feature_name, expected_value, value
                    )))
                }
            } else {
                Err(ValidationError::FeatureNotAvailable(format!(
                    "{} has no value",
                    feature_name
                )))
            }
        } else {
            Err(ValidationError::FeatureNotAvailable(
                feature_name.to_string(),
            ))
        }
    }

    /// Get validation report
    pub fn get_validation_report(&self, license: &License, current_machine_id: &str) -> String {
        let mut report = Vec::new();

        report.push("=== License Validation Report ===".to_string());
        report.push(format!("License Key: {}", license.license_key));
        report.push(format!("Tier: {}", license.tier));
        report.push(format!("User: {}", license.user_id));
        report.push(format!("Product: {}", license.product_name));
        report.push(String::new());

        // Format validation
        match self.validate_format(&license.license_key) {
            Ok(_) => report.push("✓ Format: VALID".to_string()),
            Err(e) => report.push(format!("✗ Format: INVALID - {}", e)),
        }

        // Expiration validation
        match self.validate_expiration(license) {
            Ok(_) => {
                let days = license.days_until_expiration();
                report.push(format!("✓ Expiration: VALID ({} days remaining)", days));
            }
            Err(e) => report.push(format!("✗ Expiration: INVALID - {}", e)),
        }

        // Machine ID validation
        match self.validate_machine_id(license, current_machine_id) {
            Ok(_) => report.push("✓ Machine ID: VALID".to_string()),
            Err(e) => report.push(format!("✗ Machine ID: INVALID - {}", e)),
        }

        // Signature validation
        match self.validate_signature(license) {
            Ok(_) => report.push("✓ Signature: VALID".to_string()),
            Err(e) => report.push(format!("✗ Signature: INVALID - {}", e)),
        }

        // Feature count validation
        match self.validate_feature_count(license) {
            Ok(_) => report.push(format!(
                "✓ Feature Count: VALID ({}/{})",
                license.features.len(),
                license.tier.max_features()
            )),
            Err(e) => report.push(format!("✗ Feature Count: INVALID - {}", e)),
        }

        report.push(String::new());
        report.push("=== Features ===".to_string());
        for feature in &license.features {
            let status = if feature.is_valid() { "✓" } else { "✗" };
            report.push(format!("{} {}: {}", status, feature.name, feature.is_enabled));
        }

        report.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::LicenseGenerator;

    #[test]
    fn test_valid_license() {
        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine123", "Product")
            .unwrap();

        let validator = LicenseValidator::new(generator.get_public_key().to_string());
        let result = validator.validate(&license, "machine123");

        assert!(result.is_ok());
    }

    #[test]
    fn test_expired_license() {
        let generator = LicenseGenerator::new().unwrap();
        let mut license = generator
            .generate_trial_license("user1", "machine123", "Product")
            .unwrap();

        // Set expiration to the past
        license.expiration_date = Utc::now() - Duration::days(1);

        let validator = LicenseValidator::new(generator.get_public_key().to_string());
        let result = validator.validate(&license, "machine123");

        assert!(result.is_err());
        if let Err(ValidationError::Expired) = result {
            // Expected
        } else {
            panic!("Expected Expired error");
        }
    }

    #[test]
    fn test_machine_id_mismatch() {
        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine123", "Product")
            .unwrap();

        let validator = LicenseValidator::new(generator.get_public_key().to_string());
        let result = validator.validate(&license, "different_machine");

        assert!(result.is_err());
    }

    #[test]
    fn test_format_validation() {
        let validator = LicenseValidator::new("dummy_key".to_string());

        assert!(validator.validate_format("TRIAL-ABCD1234-123456-abcd").is_ok());
        assert!(validator.validate_format("PREM-XYZA9876-654321-ef01").is_ok());
        assert!(validator.validate_format("INVALID-FORMAT").is_err());
        assert!(validator.validate_format("TRIAL-ABC-123-4567").is_err());
    }

    #[test]
    fn test_feature_check() {
        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine123", "Product")
            .unwrap();

        let validator = LicenseValidator::new(generator.get_public_key().to_string());

        assert!(validator.check_feature(&license, "BasicFeatures").is_ok());
        assert!(validator.check_feature(&license, "NonExistentFeature").is_err());
    }
}
