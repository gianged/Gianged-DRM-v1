use aes::Aes256;
use cbc::{Decryptor, Encryptor};
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::Rng;
use rsa::{Pkcs1v15Sign, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::signature::{SignatureEncoding, Signer, Verifier};
use sha2::{Digest, Sha256};
use thiserror::Error;

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

const DEFAULT_KEY: &str = "DRMDefaultKey2024!@#$%^&*()_+";
const DEFAULT_SALT: &str = "DRMSalt2024";
const PBKDF2_ITERATIONS: u32 = 10_000;
const AES_KEY_SIZE: usize = 32; // 256 bits
const AES_IV_SIZE: usize = 16;  // 128 bits

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),

    #[error("Decryption failed: {0}")]
    DecryptionError(String),

    #[error("Key generation failed: {0}")]
    KeyGenerationError(String),

    #[error("Signing failed: {0}")]
    SigningError(String),

    #[error("Verification failed: {0}")]
    VerificationError(String),

    #[error("Invalid key or data: {0}")]
    InvalidData(String),

    #[error("Base64 error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("RSA error: {0}")]
    RsaError(String),
}

pub type CryptoResult<T> = Result<T, CryptoError>;

/// Helper struct for cryptographic operations
pub struct CryptoHelper;

impl CryptoHelper {
    /// Encrypt a string using AES-256-CBC with the default key
    pub fn encrypt_string(plain_text: &str) -> CryptoResult<String> {
        Self::encrypt_string_with_key(plain_text, DEFAULT_KEY)
    }

    /// Decrypt a string using AES-256-CBC with the default key
    pub fn decrypt_string(cipher_text: &str) -> CryptoResult<String> {
        Self::decrypt_string_with_key(cipher_text, DEFAULT_KEY)
    }

    /// Encrypt a string using AES-256-CBC with a custom key
    pub fn encrypt_string_with_key(plain_text: &str, key: &str) -> CryptoResult<String> {
        let key_bytes = Self::derive_key(key.as_bytes(), DEFAULT_SALT.as_bytes())?;
        let iv = Self::generate_secure_random_bytes(AES_IV_SIZE)?;

        let encrypted = Self::encrypt_aes(&key_bytes, &iv, plain_text.as_bytes())?;

        // Prepend IV to encrypted data
        let mut result = iv.clone();
        result.extend_from_slice(&encrypted);

        Ok(base64::encode(&result))
    }

    /// Decrypt a string using AES-256-CBC with a custom key
    pub fn decrypt_string_with_key(cipher_text: &str, key: &str) -> CryptoResult<String> {
        let key_bytes = Self::derive_key(key.as_bytes(), DEFAULT_SALT.as_bytes())?;
        let encrypted_data = base64::decode(cipher_text)?;

        if encrypted_data.len() < AES_IV_SIZE {
            return Err(CryptoError::DecryptionError(
                "Invalid encrypted data length".to_string(),
            ));
        }

        // Extract IV from the beginning
        let (iv, encrypted) = encrypted_data.split_at(AES_IV_SIZE);

        let decrypted = Self::decrypt_aes(&key_bytes, iv, encrypted)?;

        String::from_utf8(decrypted)
            .map_err(|e| CryptoError::DecryptionError(format!("UTF-8 conversion failed: {}", e)))
    }

    /// Generate a new AES-256 key
    pub fn generate_aes_key() -> CryptoResult<Vec<u8>> {
        Self::generate_secure_random_bytes(AES_KEY_SIZE)
    }

    /// Encrypt data using AES-256-CBC
    pub fn encrypt_aes(key: &[u8], iv: &[u8], data: &[u8]) -> CryptoResult<Vec<u8>> {
        if key.len() != AES_KEY_SIZE {
            return Err(CryptoError::InvalidData(format!(
                "Invalid key size: expected {}, got {}",
                AES_KEY_SIZE,
                key.len()
            )));
        }

        if iv.len() != AES_IV_SIZE {
            return Err(CryptoError::InvalidData(format!(
                "Invalid IV size: expected {}, got {}",
                AES_IV_SIZE,
                iv.len()
            )));
        }

        // Apply PKCS7 padding
        let padded_data = Self::pkcs7_pad(data, 16);

        let cipher = Aes256CbcEnc::new(key.into(), iv.into());
        cipher
            .encrypt_padded_vec_mut::<cbc::cipher::block_padding::NoPadding>(&padded_data)
            .map_err(|e| CryptoError::EncryptionError(format!("AES encryption failed: {}", e)))
    }

    /// Decrypt data using AES-256-CBC
    pub fn decrypt_aes(key: &[u8], iv: &[u8], data: &[u8]) -> CryptoResult<Vec<u8>> {
        if key.len() != AES_KEY_SIZE {
            return Err(CryptoError::InvalidData(format!(
                "Invalid key size: expected {}, got {}",
                AES_KEY_SIZE,
                key.len()
            )));
        }

        if iv.len() != AES_IV_SIZE {
            return Err(CryptoError::InvalidData(format!(
                "Invalid IV size: expected {}, got {}",
                AES_IV_SIZE,
                iv.len()
            )));
        }

        let cipher = Aes256CbcDec::new(key.into(), iv.into());
        let decrypted = cipher
            .decrypt_padded_vec_mut::<cbc::cipher::block_padding::NoPadding>(data)
            .map_err(|e| CryptoError::DecryptionError(format!("AES decryption failed: {}", e)))?;

        // Remove PKCS7 padding
        Self::pkcs7_unpad(&decrypted)
    }

    /// Generate an RSA key pair (2048-bit)
    pub fn generate_rsa_key_pair() -> CryptoResult<(String, String)> {
        let mut rng = rand::thread_rng();

        let private_key = RsaPrivateKey::new(&mut rng, 2048)
            .map_err(|e| CryptoError::KeyGenerationError(format!("RSA key generation failed: {}", e)))?;

        let public_key = RsaPublicKey::from(&private_key);

        let private_pem = private_key
            .to_pkcs8_pem(LineEnding::LF)
            .map_err(|e| CryptoError::KeyGenerationError(format!("Failed to encode private key: {}", e)))?
            .to_string();

        let public_pem = public_key
            .to_public_key_pem(LineEnding::LF)
            .map_err(|e| CryptoError::KeyGenerationError(format!("Failed to encode public key: {}", e)))?;

        Ok((private_pem, public_pem))
    }

    /// Sign data using RSA private key with SHA-256
    pub fn sign_data_rsa(data: &[u8], private_key_pem: &str) -> CryptoResult<Vec<u8>> {
        let private_key = RsaPrivateKey::from_pkcs8_pem(private_key_pem)
            .map_err(|e| CryptoError::SigningError(format!("Failed to parse private key: {}", e)))?;

        let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(private_key);
        let signature = signing_key.sign(data);

        Ok(signature.to_vec())
    }

    /// Verify RSA signature with SHA-256
    pub fn verify_signature_rsa(
        data: &[u8],
        signature: &[u8],
        public_key_pem: &str,
    ) -> CryptoResult<bool> {
        let public_key = RsaPublicKey::from_public_key_pem(public_key_pem)
            .map_err(|e| CryptoError::VerificationError(format!("Failed to parse public key: {}", e)))?;

        let verifying_key = rsa::pkcs1v15::VerifyingKey::<Sha256>::new(public_key);

        let sig = rsa::pkcs1v15::Signature::try_from(signature)
            .map_err(|e| CryptoError::VerificationError(format!("Invalid signature format: {}", e)))?;

        Ok(verifying_key.verify(data, &sig).is_ok())
    }

    /// Generate cryptographically secure random bytes
    pub fn generate_secure_random_bytes(length: usize) -> CryptoResult<Vec<u8>> {
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; length];
        rng.fill(&mut bytes[..]);
        Ok(bytes)
    }

    /// Compute SHA-256 hash of data
    pub fn compute_sha256_hash(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Compute SHA-256 hash and return as hex string
    pub fn compute_sha256_hash_hex(data: &[u8]) -> String {
        hex::encode(Self::compute_sha256_hash(data))
    }

    /// Derive a key using PBKDF2 with SHA-256
    pub fn derive_key(password: &[u8], salt: &[u8]) -> CryptoResult<Vec<u8>> {
        let mut key = vec![0u8; AES_KEY_SIZE];
        pbkdf2::<Hmac<Sha256>>(password, salt, PBKDF2_ITERATIONS, &mut key)
            .map_err(|e| CryptoError::KeyGenerationError(format!("PBKDF2 failed: {}", e)))?;
        Ok(key)
    }

    /// Timing-safe comparison of two byte slices
    pub fn secure_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }

        result == 0
    }

    /// Apply PKCS#7 padding
    fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
        let padding_len = block_size - (data.len() % block_size);
        let mut padded = data.to_vec();
        padded.extend(std::iter::repeat(padding_len as u8).take(padding_len));
        padded
    }

    /// Remove PKCS#7 padding
    fn pkcs7_unpad(data: &[u8]) -> CryptoResult<Vec<u8>> {
        if data.is_empty() {
            return Err(CryptoError::DecryptionError(
                "Cannot unpad empty data".to_string(),
            ));
        }

        let padding_len = data[data.len() - 1] as usize;

        if padding_len == 0 || padding_len > data.len() {
            return Err(CryptoError::DecryptionError(
                "Invalid padding".to_string(),
            ));
        }

        // Verify all padding bytes are correct
        for i in (data.len() - padding_len)..data.len() {
            if data[i] != padding_len as u8 {
                return Err(CryptoError::DecryptionError(
                    "Invalid padding bytes".to_string(),
                ));
            }
        }

        Ok(data[..data.len() - padding_len].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_encryption_decryption() {
        let original = "Hello, World! This is a test message.";
        let encrypted = CryptoHelper::encrypt_string(original).unwrap();
        let decrypted = CryptoHelper::decrypt_string(&encrypted).unwrap();

        assert_eq!(original, decrypted);
        assert_ne!(original, encrypted);
    }

    #[test]
    fn test_sha256_hash() {
        let data = b"test data";
        let hash1 = CryptoHelper::compute_sha256_hash(data);
        let hash2 = CryptoHelper::compute_sha256_hash(data);

        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32); // SHA-256 produces 32 bytes
    }

    #[test]
    fn test_rsa_signing_verification() {
        let (private_key, public_key) = CryptoHelper::generate_rsa_key_pair().unwrap();
        let data = b"Important message to sign";

        let signature = CryptoHelper::sign_data_rsa(data, &private_key).unwrap();
        let is_valid = CryptoHelper::verify_signature_rsa(data, &signature, &public_key).unwrap();

        assert!(is_valid);

        // Test with modified data
        let modified_data = b"Modified message";
        let is_valid_modified =
            CryptoHelper::verify_signature_rsa(modified_data, &signature, &public_key).unwrap();

        assert!(!is_valid_modified);
    }

    #[test]
    fn test_secure_compare() {
        let a = b"test";
        let b = b"test";
        let c = b"fail";

        assert!(CryptoHelper::secure_compare(a, b));
        assert!(!CryptoHelper::secure_compare(a, c));
    }

    #[test]
    fn test_random_bytes() {
        let bytes1 = CryptoHelper::generate_secure_random_bytes(32).unwrap();
        let bytes2 = CryptoHelper::generate_secure_random_bytes(32).unwrap();

        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2);
    }

    #[test]
    fn test_key_derivation() {
        let password = b"my_password";
        let salt = b"my_salt";

        let key1 = CryptoHelper::derive_key(password, salt).unwrap();
        let key2 = CryptoHelper::derive_key(password, salt).unwrap();

        assert_eq!(key1, key2);
        assert_eq!(key1.len(), AES_KEY_SIZE);
    }
}
