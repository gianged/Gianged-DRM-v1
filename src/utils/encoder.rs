use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncoderError {
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("Hex decode error: {0}")]
    HexError(#[from] hex::FromHexError),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),
}

pub type EncoderResult<T> = Result<T, EncoderError>;

/// Utility for various encoding and decoding operations
pub struct Encoder;

impl Encoder {
    /// Encode bytes to Base64
    pub fn base64_encode(data: &[u8]) -> String {
        base64::encode(data)
    }

    /// Decode Base64 to bytes
    pub fn base64_decode(encoded: &str) -> EncoderResult<Vec<u8>> {
        Ok(base64::decode(encoded)?)
    }

    /// Encode string to Base64
    pub fn base64_encode_string(text: &str) -> String {
        Self::base64_encode(text.as_bytes())
    }

    /// Decode Base64 to string
    pub fn base64_decode_string(encoded: &str) -> EncoderResult<String> {
        let bytes = Self::base64_decode(encoded)?;
        String::from_utf8(bytes).map_err(|e| EncoderError::Utf8Error(e.to_string()))
    }

    /// Encode bytes to Base64 URL-safe variant
    pub fn base64_url_encode(data: &[u8]) -> String {
        base64::encode_config(data, base64::URL_SAFE_NO_PAD)
    }

    /// Decode Base64 URL-safe variant
    pub fn base64_url_decode(encoded: &str) -> EncoderResult<Vec<u8>> {
        Ok(base64::decode_config(encoded, base64::URL_SAFE_NO_PAD)?)
    }

    /// Encode bytes to hexadecimal
    pub fn hex_encode(data: &[u8]) -> String {
        hex::encode(data)
    }

    /// Decode hexadecimal to bytes
    pub fn hex_decode(encoded: &str) -> EncoderResult<Vec<u8>> {
        Ok(hex::decode(encoded)?)
    }

    /// Encode string to hexadecimal
    pub fn hex_encode_string(text: &str) -> String {
        Self::hex_encode(text.as_bytes())
    }

    /// Decode hexadecimal to string
    pub fn hex_decode_string(encoded: &str) -> EncoderResult<String> {
        let bytes = Self::hex_decode(encoded)?;
        String::from_utf8(bytes).map_err(|e| EncoderError::Utf8Error(e.to_string()))
    }

    /// Custom alphabet substitution encoding
    pub fn custom_alphabet_encode(text: &str, alphabet: &str) -> EncoderResult<String> {
        if alphabet.len() != 64 {
            return Err(EncoderError::EncodingError(
                "Alphabet must be 64 characters".to_string(),
            ));
        }

        let standard_base64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let encoded = Self::base64_encode_string(text);

        let result: String = encoded
            .chars()
            .map(|c| {
                if let Some(pos) = standard_base64.find(c) {
                    alphabet.chars().nth(pos).unwrap_or(c)
                } else {
                    c // Keep padding characters
                }
            })
            .collect();

        Ok(result)
    }

    /// Custom alphabet substitution decoding
    pub fn custom_alphabet_decode(encoded: &str, alphabet: &str) -> EncoderResult<String> {
        if alphabet.len() != 64 {
            return Err(EncoderError::EncodingError(
                "Alphabet must be 64 characters".to_string(),
            ));
        }

        let standard_base64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let decoded_base64: String = encoded
            .chars()
            .map(|c| {
                if let Some(pos) = alphabet.find(c) {
                    standard_base64.chars().nth(pos).unwrap_or(c)
                } else {
                    c // Keep padding characters
                }
            })
            .collect();

        Self::base64_decode_string(&decoded_base64)
    }

    /// Caesar cipher encoding
    pub fn caesar_encode(text: &str, shift: u8) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let offset = (c as u8 - base + shift) % 26;
                    (base + offset) as char
                } else {
                    c
                }
            })
            .collect()
    }

    /// Caesar cipher decoding
    pub fn caesar_decode(text: &str, shift: u8) -> String {
        Self::caesar_encode(text, 26 - (shift % 26))
    }

    /// ROT13 encoding/decoding
    pub fn rot13(text: &str) -> String {
        Self::caesar_encode(text, 13)
    }

    /// URL encoding
    pub fn url_encode(text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
                    c.to_string()
                } else {
                    format!("%{:02X}", c as u8)
                }
            })
            .collect()
    }

    /// URL decoding
    pub fn url_decode(encoded: &str) -> EncoderResult<String> {
        let mut result = String::new();
        let mut chars = encoded.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                let hex: String = chars.by_ref().take(2).collect();
                if hex.len() == 2 {
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte as char);
                    } else {
                        return Err(EncoderError::EncodingError(
                            "Invalid URL encoding".to_string(),
                        ));
                    }
                } else {
                    return Err(EncoderError::EncodingError(
                        "Incomplete URL encoding".to_string(),
                    ));
                }
            } else if c == '+' {
                result.push(' ');
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }

    /// Binary to string (showing 1s and 0s)
    pub fn binary_encode(data: &[u8]) -> String {
        data.iter()
            .map(|byte| format!("{:08b}", byte))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Parse binary string back to bytes
    pub fn binary_decode(binary: &str) -> EncoderResult<Vec<u8>> {
        binary
            .split_whitespace()
            .map(|byte_str| {
                u8::from_str_radix(byte_str, 2).map_err(|e| {
                    EncoderError::EncodingError(format!("Invalid binary string: {}", e))
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encoding() {
        let original = "Hello, World!";
        let encoded = Encoder::base64_encode_string(original);
        let decoded = Encoder::base64_decode_string(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_hex_encoding() {
        let original = "Test Data";
        let encoded = Encoder::hex_encode_string(original);
        let decoded = Encoder::hex_decode_string(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_caesar_cipher() {
        let original = "HelloWorld";
        let encoded = Encoder::caesar_encode(original, 5);
        let decoded = Encoder::caesar_decode(&encoded, 5);
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_rot13() {
        let original = "Test123";
        let encoded = Encoder::rot13(original);
        let decoded = Encoder::rot13(&encoded);
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_url_encoding() {
        let original = "Hello World!";
        let encoded = Encoder::url_encode(original);
        let decoded = Encoder::url_decode(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_binary_encoding() {
        let data = b"AB";
        let encoded = Encoder::binary_encode(data);
        let decoded = Encoder::binary_decode(&encoded).unwrap();
        assert_eq!(data.to_vec(), decoded);
    }

    #[test]
    fn test_base64_url_safe() {
        let data = b"Test with special chars: +++///";
        let encoded = Encoder::base64_url_encode(data);
        let decoded = Encoder::base64_url_decode(&encoded).unwrap();
        assert_eq!(data.to_vec(), decoded);
        assert!(!encoded.contains('+'));
        assert!(!encoded.contains('/'));
    }
}
