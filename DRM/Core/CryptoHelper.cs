using System.Security.Cryptography;
using System.Text;

namespace DRM.Core
{
    public class CryptoHelper
    {
        protected CryptoHelper() { }

        private static readonly byte[] DefaultSalt = Encoding.UTF8.GetBytes("DRMSalt2024");

        public static string GenerateAESKey()
        {
            using var aes = Aes.Create();
            aes.GenerateKey();
            return Convert.ToBase64String(aes.Key);
        }

        public static string EncryptAES(string plainText, string key)
        {
            var keyBytes = Convert.FromBase64String(key);
            using var aes = Aes.Create();
            aes.Key = keyBytes;
            aes.GenerateIV();

            using var encryptor = aes.CreateEncryptor(aes.Key, aes.IV);
            using var msEncrypt = new MemoryStream();
            using var csEncrypt = new CryptoStream(msEncrypt, encryptor, CryptoStreamMode.Write);
            using var swEncrypt = new StreamWriter(csEncrypt);
            
            swEncrypt.Write(plainText);
            csEncrypt.FlushFinalBlock();
            
            var encrypted = msEncrypt.ToArray();
            var result = new byte[aes.IV.Length + encrypted.Length];
            Buffer.BlockCopy(aes.IV, 0, result, 0, aes.IV.Length);
            Buffer.BlockCopy(encrypted, 0, result, aes.IV.Length, encrypted.Length);
            
            return Convert.ToBase64String(result);
        }

        public static string DecryptAES(string cipherText, string key)
        {
            var fullCipher = Convert.FromBase64String(cipherText);
            var keyBytes = Convert.FromBase64String(key);
            
            using var aes = Aes.Create();
            aes.Key = keyBytes;
            
            var iv = new byte[aes.BlockSize / 8];
            var cipher = new byte[fullCipher.Length - iv.Length];
            
            Buffer.BlockCopy(fullCipher, 0, iv, 0, iv.Length);
            Buffer.BlockCopy(fullCipher, iv.Length, cipher, 0, cipher.Length);
            
            aes.IV = iv;

            using var decryptor = aes.CreateDecryptor(aes.Key, aes.IV);
            using var msDecrypt = new MemoryStream(cipher);
            using var csDecrypt = new CryptoStream(msDecrypt, decryptor, CryptoStreamMode.Read);
            using var srDecrypt = new StreamReader(csDecrypt);
            
            return srDecrypt.ReadToEnd();
        }

        public static (string publicKey, string privateKey) GenerateRSAKeyPair(int keySize = 2048)
        {
            using var rsa = RSA.Create(keySize);
            var publicKey = Convert.ToBase64String(rsa.ExportRSAPublicKey());
            var privateKey = Convert.ToBase64String(rsa.ExportRSAPrivateKey());
            return (publicKey, privateKey);
        }

        public static byte[] SignDataRSA(byte[] data, string privateKey)
        {
            using var rsa = RSA.Create();
            rsa.ImportRSAPrivateKey(Convert.FromBase64String(privateKey), out _);
            return rsa.SignData(data, HashAlgorithmName.SHA256, RSASignaturePadding.Pkcs1);
        }

        public static bool VerifySignatureRSA(byte[] data, byte[] signature, string publicKey)
        {
            try
            {
                using var rsa = RSA.Create();
                rsa.ImportRSAPublicKey(Convert.FromBase64String(publicKey), out _);
                return rsa.VerifyData(data, signature, HashAlgorithmName.SHA256, RSASignaturePadding.Pkcs1);
            }
            catch
            {
                return false;
            }
        }

        public static byte[] GenerateSecureRandomBytes(int length)
        {
            using var rng = RandomNumberGenerator.Create();
            var randomBytes = new byte[length];
            rng.GetBytes(randomBytes);
            return randomBytes;
        }

        public static string GenerateSecureRandomString(int length)
        {
            const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            var random = GenerateSecureRandomBytes(length);
            var result = new StringBuilder(length);
            
            for (int i = 0; i < length; i++)
            {
                result.Append(chars[random[i] % chars.Length]);
            }
            
            return result.ToString();
        }

        public static string ComputeSHA256Hash(string input)
        {
            using var sha256 = SHA256.Create();
            var hashedBytes = sha256.ComputeHash(Encoding.UTF8.GetBytes(input));
            return Convert.ToBase64String(hashedBytes);
        }

        public static string ComputeSHA256Hash(byte[] input)
        {
            using var sha256 = SHA256.Create();
            var hashedBytes = sha256.ComputeHash(input);
            return Convert.ToBase64String(hashedBytes);
        }

        public static byte[] DeriveKey(string password, byte[]? salt = null, int keyLength = 32, int iterations = 10000)
        {
            salt ??= DefaultSalt;
            using var rfc2898 = new Rfc2898DeriveBytes(password, salt, iterations, HashAlgorithmName.SHA256);
            return rfc2898.GetBytes(keyLength);
        }

        public static string DeriveKeyBase64(string password, byte[]? salt = null, int keyLength = 32, int iterations = 10000)
        {
            var keyBytes = DeriveKey(password, salt, keyLength, iterations);
            return Convert.ToBase64String(keyBytes);
        }

        public static bool SecureCompare(byte[] a, byte[] b)
        {
            if (a.Length != b.Length)
                return false;

            int result = 0;
            for (int i = 0; i < a.Length; i++)
            {
                result |= a[i] ^ b[i];
            }
            return result == 0;
        }

        public static bool SecureCompare(string a, string b)
        {
            if (a.Length != b.Length)
                return false;

            int result = 0;
            for (int i = 0; i < a.Length; i++)
            {
                result |= a[i] ^ b[i];
            }
            return result == 0;
        }
    }
}
