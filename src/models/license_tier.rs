use serde::{Deserialize, Serialize};

/// Represents the tier level of a license
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseTier {
    /// Trial license with limited features (30 days)
    Trial = 0,
    /// Premium license with full features (1 year)
    Premium = 1,
}

impl LicenseTier {
    /// Get the maximum number of features allowed for this tier
    pub fn max_features(&self) -> usize {
        match self {
            LicenseTier::Trial => 2,
            LicenseTier::Premium => 10,
        }
    }

    /// Get the maximum number of users allowed for this tier
    pub fn max_users(&self) -> Option<usize> {
        match self {
            LicenseTier::Trial => Some(1),
            LicenseTier::Premium => None, // Unlimited
        }
    }

    /// Get the default duration in days for this tier
    pub fn default_duration_days(&self) -> i64 {
        match self {
            LicenseTier::Trial => 30,
            LicenseTier::Premium => 365,
        }
    }

    /// Get the tier name as a string
    pub fn name(&self) -> &str {
        match self {
            LicenseTier::Trial => "Trial",
            LicenseTier::Premium => "Premium",
        }
    }

    /// Get the tier prefix for license keys
    pub fn prefix(&self) -> &str {
        match self {
            LicenseTier::Trial => "TRIAL",
            LicenseTier::Premium => "PREM",
        }
    }
}

impl Default for LicenseTier {
    fn default() -> Self {
        LicenseTier::Trial
    }
}

impl std::fmt::Display for LicenseTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
