use rand::Rng;

/// Helper for code obfuscation techniques (educational purposes)
pub struct ObfuscationHelper;

impl ObfuscationHelper {
    /// Obfuscate a string using Caesar cipher (ROT13)
    pub fn caesar_cipher(text: &str, shift: u8) -> String {
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

    /// ROT13 encoding (Caesar cipher with shift 13)
    pub fn rot13(text: &str) -> String {
        Self::caesar_cipher(text, 13)
    }

    /// Base64 obfuscation
    pub fn base64_obfuscate(text: &str) -> String {
        base64::encode(text)
    }

    /// Base64 deobfuscation
    pub fn base64_deobfuscate(text: &str) -> Result<String, base64::DecodeError> {
        let bytes = base64::decode(text)?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    /// Custom alphabet substitution
    pub fn alphabet_substitution(text: &str, key: &str) -> String {
        if key.len() != 26 {
            return text.to_string();
        }

        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let is_lowercase = c.is_ascii_lowercase();
                    let upper_c = c.to_ascii_uppercase();

                    if let Some(pos) = alphabet.find(upper_c) {
                        let substituted = key.chars().nth(pos).unwrap_or(c);
                        if is_lowercase {
                            substituted.to_ascii_lowercase()
                        } else {
                            substituted
                        }
                    } else {
                        c
                    }
                } else {
                    c
                }
            })
            .collect()
    }

    /// XOR obfuscation with a key
    pub fn xor_obfuscate(data: &[u8], key: &[u8]) -> Vec<u8> {
        if key.is_empty() {
            return data.to_vec();
        }

        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key[i % key.len()])
            .collect()
    }

    /// XOR deobfuscation (same as obfuscation for XOR)
    pub fn xor_deobfuscate(data: &[u8], key: &[u8]) -> Vec<u8> {
        Self::xor_obfuscate(data, key)
    }

    /// String reversal obfuscation
    pub fn reverse_string(text: &str) -> String {
        text.chars().rev().collect()
    }

    /// Generate a decoy string of random characters
    pub fn generate_decoy_string(length: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..62);
                if idx < 10 {
                    (b'0' + idx as u8) as char
                } else if idx < 36 {
                    (b'A' + (idx - 10) as u8) as char
                } else {
                    (b'a' + (idx - 36) as u8) as char
                }
            })
            .collect()
    }

    /// Generate noisy code (random identifiers)
    pub fn generate_noise_identifiers(count: usize) -> Vec<String> {
        (0..count)
            .map(|_| {
                let length = rand::thread_rng().gen_range(8..16);
                Self::generate_decoy_string(length)
            })
            .collect()
    }

    /// Generate fake method signatures
    pub fn generate_fake_methods(count: usize) -> Vec<String> {
        let return_types = vec!["void", "int", "bool", "String", "Vec<u8>", "Result<(), Error>"];
        let method_names = vec![
            "validate", "process", "compute", "check", "verify",
            "calculate", "analyze", "transform", "execute", "handle",
        ];

        let mut rng = rand::thread_rng();
        (0..count)
            .map(|_| {
                let return_type = return_types[rng.gen_range(0..return_types.len())];
                let method_name = method_names[rng.gen_range(0..method_names.len())];
                let suffix = Self::generate_decoy_string(4);

                format!("fn {}_{}_{}() -> {} {{ /* ... */ }}",
                    method_name, suffix, rng.gen_range(100..999), return_type)
            })
            .collect()
    }

    /// Simple control flow obfuscation (add dummy branches)
    pub fn obfuscate_control_flow<T, F>(value: T, computation: F) -> T
    where
        F: FnOnce(T) -> T,
    {
        let mut rng = rand::thread_rng();
        let dummy_condition = rng.gen_range(0..100);

        // Always false branch (never executed)
        if dummy_condition > 1000 {
            panic!("This should never happen");
        }

        // Real computation
        let result = computation(value);

        // Another dummy check
        if dummy_condition < -1000 {
            panic!("This should never happen either");
        }

        result
    }

    /// Scramble byte order (simple obfuscation)
    pub fn scramble_bytes(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(data.len() + 1);

        // Add length as first byte (for unscrambling)
        result.push(data.len() as u8);

        // Reverse order
        result.extend(data.iter().rev());

        result
    }

    /// Unscramble bytes
    pub fn unscramble_bytes(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        // Skip length byte and reverse
        data[1..].iter().rev().copied().collect()
    }

    /// Apply multiple layers of obfuscation
    pub fn multi_layer_obfuscate(text: &str) -> String {
        let layer1 = Self::rot13(text);
        let layer2 = Self::base64_obfuscate(&layer1);
        let layer3 = Self::reverse_string(&layer2);
        layer3
    }

    /// Remove multiple layers of obfuscation
    pub fn multi_layer_deobfuscate(text: &str) -> Result<String, String> {
        let layer1 = Self::reverse_string(text);
        let layer2 = Self::base64_deobfuscate(&layer1)
            .map_err(|e| format!("Base64 decode error: {}", e))?;
        let layer3 = Self::rot13(&layer2);
        Ok(layer3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher() {
        let original = "HelloWorld";
        let encrypted = ObfuscationHelper::caesar_cipher(original, 3);
        let decrypted = ObfuscationHelper::caesar_cipher(&encrypted, 23); // Reverse shift

        assert_ne!(original, encrypted);
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_rot13() {
        let original = "Test123";
        let encoded = ObfuscationHelper::rot13(original);
        let decoded = ObfuscationHelper::rot13(&encoded);

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_xor_obfuscation() {
        let data = b"Secret Message";
        let key = b"key123";

        let obfuscated = ObfuscationHelper::xor_obfuscate(data, key);
        let deobfuscated = ObfuscationHelper::xor_deobfuscate(&obfuscated, key);

        assert_ne!(data.to_vec(), obfuscated);
        assert_eq!(data.to_vec(), deobfuscated);
    }

    #[test]
    fn test_string_reversal() {
        let original = "Hello";
        let reversed = ObfuscationHelper::reverse_string(original);
        let restored = ObfuscationHelper::reverse_string(&reversed);

        assert_eq!(reversed, "olleH");
        assert_eq!(restored, original);
    }

    #[test]
    fn test_scramble_bytes() {
        let data = b"TestData";
        let scrambled = ObfuscationHelper::scramble_bytes(data);
        let unscrambled = ObfuscationHelper::unscramble_bytes(&scrambled);

        assert_ne!(data.to_vec(), scrambled);
        assert_eq!(data.to_vec(), unscrambled);
    }

    #[test]
    fn test_multi_layer_obfuscation() {
        let original = "SecretMessage";
        let obfuscated = ObfuscationHelper::multi_layer_obfuscate(original);
        let deobfuscated = ObfuscationHelper::multi_layer_deobfuscate(&obfuscated).unwrap();

        assert_ne!(original, obfuscated);
        assert_eq!(original, deobfuscated);
    }

    #[test]
    fn test_generate_decoy() {
        let decoy1 = ObfuscationHelper::generate_decoy_string(20);
        let decoy2 = ObfuscationHelper::generate_decoy_string(20);

        assert_eq!(decoy1.len(), 20);
        assert_eq!(decoy2.len(), 20);
        assert_ne!(decoy1, decoy2);
    }

    #[test]
    fn test_fake_methods() {
        let methods = ObfuscationHelper::generate_fake_methods(5);
        assert_eq!(methods.len(), 5);

        for method in methods {
            assert!(method.starts_with("fn "));
            assert!(method.contains("->"));
        }
    }
}
