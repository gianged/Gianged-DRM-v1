use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a feature within a license
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LicenseFeature {
    /// Name of the feature
    pub name: String,
    /// Whether the feature is enabled
    pub is_enabled: bool,
    /// Optional expiration date for the feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<DateTime<Utc>>,
    /// Optional value associated with the feature (e.g., user count)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LicenseFeature {
    /// Create a new enabled feature without expiration
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_enabled: true,
            expiration_date: None,
            value: None,
        }
    }

    /// Create a new feature with all properties
    pub fn with_details(
        name: impl Into<String>,
        is_enabled: bool,
        expiration_date: Option<DateTime<Utc>>,
        value: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            is_enabled,
            expiration_date,
            value,
        }
    }

    /// Check if the feature is currently valid (not expired)
    pub fn is_valid(&self) -> bool {
        if !self.is_enabled {
            return false;
        }

        if let Some(expiration) = self.expiration_date {
            Utc::now() <= expiration
        } else {
            true
        }
    }

    /// Set the feature value
    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = Some(value.into());
    }

    /// Get the feature value
    pub fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

// Common feature names as constants
pub mod features {
    pub const BASIC_FEATURES: &str = "BasicFeatures";
    pub const ADVANCED_FEATURES: &str = "AdvancedFeatures";
    pub const LIMITED_EXPORT: &str = "LimitedExport";
    pub const UNLIMITED_EXPORT: &str = "UnlimitedExport";
    pub const PRIORITY_SUPPORT: &str = "PrioritySupport";
    pub const CUSTOMIZATION_TOOLS: &str = "CustomizationTools";
    pub const MAX_USERS: &str = "MaxUsers";
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_feature_creation() {
        let feature = LicenseFeature::new("TestFeature");
        assert_eq!(feature.name, "TestFeature");
        assert!(feature.is_enabled);
        assert!(feature.is_valid());
    }

    #[test]
    fn test_feature_expiration() {
        let past = Utc::now() - Duration::days(1);
        let feature = LicenseFeature::with_details("ExpiredFeature", true, Some(past), None);
        assert!(!feature.is_valid());
    }

    #[test]
    fn test_feature_value() {
        let mut feature = LicenseFeature::new("TestFeature");
        feature.set_value("100");
        assert_eq!(feature.get_value(), Some("100"));
    }
}
