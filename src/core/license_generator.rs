use crate::core::CryptoHelper;
use crate::models::{License, LicenseFeature, LicenseTier, license_feature::features};
use chrono::{Duration, Utc};
use rand::Rng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LicenseGeneratorError {
    #[error("Failed to generate license: {0}")]
    GenerationError(String),

    #[error("Crypto error: {0}")]
    CryptoError(#[from] crate::core::crypto_helper::CryptoError),

    #[error("Invalid tier configuration: {0}")]
    InvalidTier(String),
}

pub type LicenseGeneratorResult<T> = Result<T, LicenseGeneratorError>;

/// Generator for creating software licenses
pub struct LicenseGenerator {
    private_key: String,
    public_key: String,
}

impl LicenseGenerator {
    /// Create a new license generator with RSA key pair
    pub fn new() -> LicenseGeneratorResult<Self> {
        let (private_key, public_key) = CryptoHelper::generate_rsa_key_pair()?;

        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// Create a new license generator with existing keys
    pub fn with_keys(private_key: String, public_key: String) -> Self {
        Self {
            private_key,
            public_key,
        }
    }

    /// Generate a trial license
    pub fn generate_trial_license(
        &self,
        user_id: impl Into<String>,
        machine_id: impl Into<String>,
        product_name: impl Into<String>,
    ) -> LicenseGeneratorResult<License> {
        let features = vec![
            LicenseFeature::new(features::BASIC_FEATURES),
            LicenseFeature::with_details(
                features::MAX_USERS,
                true,
                None,
                Some("1".to_string()),
            ),
        ];

        self.generate_license(
            user_id,
            machine_id,
            product_name,
            LicenseTier::Trial,
            features,
            Some(Duration::days(LicenseTier::Trial.default_duration_days())),
        )
    }

    /// Generate a premium license
    pub fn generate_premium_license(
        &self,
        user_id: impl Into<String>,
        machine_id: impl Into<String>,
        product_name: impl Into<String>,
    ) -> LicenseGeneratorResult<License> {
        let features = vec![
            LicenseFeature::new(features::BASIC_FEATURES),
            LicenseFeature::new(features::ADVANCED_FEATURES),
            LicenseFeature::new(features::UNLIMITED_EXPORT),
            LicenseFeature::new(features::PRIORITY_SUPPORT),
            LicenseFeature::new(features::CUSTOMIZATION_TOOLS),
            LicenseFeature::with_details(
                features::MAX_USERS,
                true,
                None,
                Some("unlimited".to_string()),
            ),
        ];

        self.generate_license(
            user_id,
            machine_id,
            product_name,
            LicenseTier::Premium,
            features,
            Some(Duration::days(LicenseTier::Premium.default_duration_days())),
        )
    }

    /// Generate a license with custom parameters
    pub fn generate_license(
        &self,
        user_id: impl Into<String>,
        machine_id: impl Into<String>,
        product_name: impl Into<String>,
        tier: LicenseTier,
        features: Vec<LicenseFeature>,
        duration: Option<Duration>,
    ) -> LicenseGeneratorResult<License> {
        // Validate feature count
        if features.len() > tier.max_features() {
            return Err(LicenseGeneratorError::InvalidTier(format!(
                "Too many features for {} tier: {} > {}",
                tier.name(),
                features.len(),
                tier.max_features()
            )));
        }

        let machine_id_str = machine_id.into();
        let user_id_str = user_id.into();
        let product_name_str = product_name.into();

        // Generate license key
        let license_key = Self::generate_license_key(&tier, &machine_id_str)?;

        // Calculate expiration date
        let expiration_date = if let Some(dur) = duration {
            Utc::now() + dur
        } else {
            Utc::now() + Duration::days(tier.default_duration_days())
        };

        // Create license
        let mut license = License::new(
            license_key,
            machine_id_str,
            user_id_str,
            product_name_str,
            expiration_date,
            tier,
            features,
        );

        // Sign the license
        let signing_data = license.get_signing_data();
        let signature = CryptoHelper::sign_data_rsa(signing_data.as_bytes(), &self.private_key)?;
        let signature_base64 = base64::encode(&signature);

        license.set_signature(signature_base64);

        Ok(license)
    }

    /// Generate a license key with format: PREFIX-RANDOM-TIMESTAMP-CHECKSUM
    fn generate_license_key(
        tier: &LicenseTier,
        machine_id: &str,
    ) -> LicenseGeneratorResult<String> {
        let mut rng = rand::thread_rng();

        // Generate random component (8 characters)
        let random_part: String = (0..8)
            .map(|_| {
                let idx = rng.gen_range(0..36);
                if idx < 10 {
                    char::from_digit(idx, 10).unwrap()
                } else {
                    (b'A' + (idx - 10) as u8) as char
                }
            })
            .collect();

        // Generate timestamp component (current timestamp % 1000000)
        let timestamp = Utc::now().timestamp() % 1_000_000;
        let timestamp_part = format!("{:06}", timestamp.abs());

        // Generate checksum from prefix, random, timestamp, and machine_id
        let checksum_data = format!(
            "{}{}{}{}",
            tier.prefix(),
            random_part,
            timestamp_part,
            machine_id
        );
        let checksum_hash = CryptoHelper::compute_sha256_hash(checksum_data.as_bytes());
        let checksum_part = hex::encode(&checksum_hash[..2]); // First 2 bytes as hex

        // Combine all parts
        let license_key = format!(
            "{}-{}-{}-{}",
            tier.prefix(),
            random_part,
            timestamp_part,
            checksum_part
        );

        Ok(license_key)
    }

    /// Get the public key for verification
    pub fn get_public_key(&self) -> &str {
        &self.public_key
    }

    /// Get the private key (use with caution!)
    pub fn get_private_key(&self) -> &str {
        &self.private_key
    }
}

impl Default for LicenseGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default license generator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_generation() {
        let generator = LicenseGenerator::new().unwrap();

        let license = generator
            .generate_trial_license("user123", "machine456", "Test Product")
            .unwrap();

        assert!(license.license_key.starts_with("TRIAL-"));
        assert_eq!(license.user_id, "user123");
        assert_eq!(license.machine_id, "machine456");
        assert_eq!(license.tier, LicenseTier::Trial);
        assert!(!license.signature.is_empty());
    }

    #[test]
    fn test_trial_license() {
        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine1", "Product")
            .unwrap();

        assert_eq!(license.tier, LicenseTier::Trial);
        assert!(license.features.len() <= LicenseTier::Trial.max_features());
    }

    #[test]
    fn test_premium_license() {
        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_premium_license("user1", "machine1", "Product")
            .unwrap();

        assert_eq!(license.tier, LicenseTier::Premium);
        assert!(license.features.len() <= LicenseTier::Premium.max_features());
    }

    #[test]
    fn test_license_key_format() {
        let generator = LicenseGenerator::new().unwrap();
        let license = generator
            .generate_trial_license("user1", "machine1", "Product")
            .unwrap();

        let parts: Vec<&str> = license.license_key.split('-').collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], "TRIAL");
        assert_eq!(parts[1].len(), 8);
        assert_eq!(parts[2].len(), 6);
        assert_eq!(parts[3].len(), 4);
    }

    #[test]
    fn test_feature_count_validation() {
        let generator = LicenseGenerator::new().unwrap();

        // Try to create a trial license with too many features
        let too_many_features: Vec<LicenseFeature> = (0..10)
            .map(|i| LicenseFeature::new(format!("Feature{}", i)))
            .collect();

        let result = generator.generate_license(
            "user1",
            "machine1",
            "Product",
            LicenseTier::Trial,
            too_many_features,
            None,
        );

        assert!(result.is_err());
    }
}
