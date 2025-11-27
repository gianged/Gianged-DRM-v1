use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{LicenseFeature, LicenseTier};

/// Represents a software license
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct License {
    /// Unique license key (format: PREFIX-RANDOM-TIMESTAMP-CHECKSUM)
    pub license_key: String,

    /// Machine ID (hardware fingerprint)
    pub machine_id: String,

    /// User identifier
    pub user_id: String,

    /// Product name
    pub product_name: String,

    /// License expiration date
    pub expiration_date: DateTime<Utc>,

    /// License issue date
    pub issue_date: DateTime<Utc>,

    /// Whether the license is currently valid
    pub is_valid: bool,

    /// License tier level
    pub tier: LicenseTier,

    /// List of features included in this license
    pub features: Vec<LicenseFeature>,

    /// RSA digital signature (Base64 encoded)
    pub signature: String,
}

impl License {
    /// Create a new license with the given parameters
    pub fn new(
        license_key: String,
        machine_id: String,
        user_id: String,
        product_name: String,
        expiration_date: DateTime<Utc>,
        tier: LicenseTier,
        features: Vec<LicenseFeature>,
    ) -> Self {
        Self {
            license_key,
            machine_id,
            user_id,
            product_name,
            expiration_date,
            issue_date: Utc::now(),
            is_valid: true,
            tier,
            features,
            signature: String::new(),
        }
    }

    /// Check if the license has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expiration_date
    }

    /// Check if a specific feature is available and valid
    pub fn has_feature(&self, feature_name: &str) -> bool {
        self.features
            .iter()
            .any(|f| f.name == feature_name && f.is_valid())
    }

    /// Get a feature by name
    pub fn get_feature(&self, feature_name: &str) -> Option<&LicenseFeature> {
        self.features.iter().find(|f| f.name == feature_name)
    }

    /// Get a mutable reference to a feature by name
    pub fn get_feature_mut(&mut self, feature_name: &str) -> Option<&mut LicenseFeature> {
        self.features.iter_mut().find(|f| f.name == feature_name)
    }

    /// Add a new feature to the license
    pub fn add_feature(&mut self, feature: LicenseFeature) {
        self.features.push(feature);
    }

    /// Remove a feature from the license
    pub fn remove_feature(&mut self, feature_name: &str) {
        self.features.retain(|f| f.name != feature_name);
    }

    /// Get the number of days until expiration
    pub fn days_until_expiration(&self) -> i64 {
        let duration = self.expiration_date.signed_duration_since(Utc::now());
        duration.num_days()
    }

    /// Check if the license tier allows a certain number of features
    pub fn validate_feature_count(&self) -> bool {
        self.features.len() <= self.tier.max_features()
    }

    /// Get a data string for signing (excludes signature field)
    pub fn get_signing_data(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            self.license_key,
            self.machine_id,
            self.user_id,
            self.product_name,
            self.expiration_date.to_rfc3339(),
            self.issue_date.to_rfc3339(),
            self.tier.name(),
            self.features.len()
        )
    }

    /// Set the signature for this license
    pub fn set_signature(&mut self, signature: String) {
        self.signature = signature;
    }

    /// Get a summary of the license
    pub fn summary(&self) -> String {
        format!(
            "License: {}\nTier: {}\nUser: {}\nProduct: {}\nExpires: {}\nFeatures: {}\nValid: {}",
            self.license_key,
            self.tier,
            self.user_id,
            self.product_name,
            self.expiration_date.format("%Y-%m-%d %H:%M:%S UTC"),
            self.features.len(),
            self.is_valid && !self.is_expired()
        )
    }
}

impl Default for License {
    fn default() -> Self {
        Self {
            license_key: String::new(),
            machine_id: String::new(),
            user_id: String::new(),
            product_name: String::from("Gianged DRM"),
            expiration_date: Utc::now(),
            issue_date: Utc::now(),
            is_valid: false,
            tier: LicenseTier::default(),
            features: Vec::new(),
            signature: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_license_creation() {
        let license = License::new(
            "TEST-1234-5678-90AB".to_string(),
            "machine123".to_string(),
            "user123".to_string(),
            "Test Product".to_string(),
            Utc::now() + Duration::days(30),
            LicenseTier::Trial,
            vec![],
        );

        assert_eq!(license.license_key, "TEST-1234-5678-90AB");
        assert!(!license.is_expired());
        assert!(license.is_valid);
    }

    #[test]
    fn test_license_expiration() {
        let license = License::new(
            "TEST-1234-5678-90AB".to_string(),
            "machine123".to_string(),
            "user123".to_string(),
            "Test Product".to_string(),
            Utc::now() - Duration::days(1),
            LicenseTier::Trial,
            vec![],
        );

        assert!(license.is_expired());
    }

    #[test]
    fn test_feature_management() {
        let mut license = License::default();
        let feature = LicenseFeature::new("TestFeature");

        license.add_feature(feature);
        assert!(license.has_feature("TestFeature"));

        license.remove_feature("TestFeature");
        assert!(!license.has_feature("TestFeature"));
    }
}
