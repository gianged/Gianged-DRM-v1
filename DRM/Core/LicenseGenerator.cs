using DRM.Models;
using DRM.Hardware;
using Newtonsoft.Json;
using System.Text;

namespace DRM.Core
{
    public class LicenseGenerator
    {
        private static readonly string DefaultPrivateKey = "MIIEpAIBAAKCAQEA2+5VR8YfwqX1..."; // Placeholder - should be securely managed
        private static readonly string DefaultPublicKey = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA2+5V..."; // Placeholder

        protected LicenseGenerator() { }
        
        public static License GenerateTrialLicense(string machineId, string userId = "", string productName = "Educational DRM")
        {
            var licenseKey = GenerateLicenseKey("TRIAL");
            var expirationDate = DateTime.UtcNow.AddDays(30); // 30 day trial
            
            var license = new License(licenseKey, machineId, expirationDate, LicenseTier.Trial);
            license.UserId = userId;
            license.ProductName = productName;
            license.IssueDate = DateTime.UtcNow;
            
            license.Features.Add(new LicenseFeature("BasicFeatures", true));
            license.Features.Add(new LicenseFeature("LimitedExport", true));
            license.Features.Add(new LicenseFeature("MaxUsers", true, "1"));
            
            return license;
        }
        
        public static License GeneratePremiumLicense(string machineId, string userId = "", string productName = "Educational DRM", DateTime? customExpiration = null)
        {
            var licenseKey = GenerateLicenseKey("PREMIUM");
            var expirationDate = customExpiration ?? DateTime.UtcNow.AddYears(1); // 1 year premium
            
            var license = new License(licenseKey, machineId, expirationDate, LicenseTier.Premium);
            license.UserId = userId;
            license.ProductName = productName;
            license.IssueDate = DateTime.UtcNow;
            
            license.Features.Add(new LicenseFeature("BasicFeatures", true));
            license.Features.Add(new LicenseFeature("AdvancedFeatures", true));
            license.Features.Add(new LicenseFeature("UnlimitedExport", true));
            license.Features.Add(new LicenseFeature("PrioritySupport", true));
            license.Features.Add(new LicenseFeature("CustomizationTools", true));
            license.Features.Add(new LicenseFeature("MaxUsers", true, "unlimited"));
            
            return license;
        }
        
        public static License GenerateLicenseForTier(LicenseTier tier, string machineId, string userId = "", string productName = "Educational DRM", DateTime? customExpiration = null)
        {
            return tier switch
            {
                LicenseTier.Trial => GenerateTrialLicense(machineId, userId, productName),
                LicenseTier.Premium => GeneratePremiumLicense(machineId, userId, productName, customExpiration),
                _ => throw new ArgumentException($"Unsupported license tier: {tier}")
            };
        }

        public static string GenerateLicenseKey(string prefix = "LIC")
        {
            var randomPart = CryptoHelper.GenerateSecureRandomString(8).ToUpper();
            var timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds().ToString()[^6..];
            var checksum = CalculateKeyChecksum($"{prefix}-{randomPart}-{timestamp}");
            
            return $"{prefix}-{randomPart}-{timestamp}-{checksum}";
        }

        public static string CreateEncryptedLicenseFile(License license, string encryptionKey = null)
        {
            try
            {
                var licenseJson = JsonConvert.SerializeObject(license, Formatting.Indented);
                
                encryptionKey ??= CryptoHelper.GenerateAESKey();
                
                var encryptedLicense = CryptoHelper.EncryptAES(licenseJson, encryptionKey);
                
                var licenseData = new
                {
                    EncryptedLicense = encryptedLicense,
                    KeyHash = CryptoHelper.ComputeSHA256Hash(encryptionKey),
                    Timestamp = DateTimeOffset.UtcNow.ToUnixTimeSeconds(),
                    Version = "1.0"
                };
                
                return JsonConvert.SerializeObject(licenseData, Formatting.Indented);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to create encrypted license file: {ex.Message}", ex);
            }
        }

        public static License DecryptLicenseFile(string encryptedLicenseData, string encryptionKey)
        {
            try
            {
                var licenseData = JsonConvert.DeserializeObject<dynamic>(encryptedLicenseData);
                var encryptedLicense = licenseData.EncryptedLicense.ToString();
                var keyHash = licenseData.KeyHash.ToString();
                
                if (!CryptoHelper.SecureCompare(CryptoHelper.ComputeSHA256Hash(encryptionKey), keyHash))
                {
                    throw new UnauthorizedAccessException("Invalid decryption key");
                }
                
                var decryptedJson = CryptoHelper.DecryptAES(encryptedLicense, encryptionKey);
                return JsonConvert.DeserializeObject<License>(decryptedJson);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to decrypt license file: {ex.Message}", ex);
            }
        }

        public static string SignLicense(License license, string privateKey = null)
        {
            try
            {
                privateKey ??= DefaultPrivateKey;
                
                var licenseJson = JsonConvert.SerializeObject(license, Formatting.None);
                var licenseBytes = Encoding.UTF8.GetBytes(licenseJson);
                var signature = CryptoHelper.SignDataRSA(licenseBytes, privateKey);
                
                return Convert.ToBase64String(signature);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to sign license: {ex.Message}", ex);
            }
        }

        public static bool VerifyLicenseSignature(License license, string signature, string publicKey = null)
        {
            try
            {
                publicKey ??= DefaultPublicKey;
                
                var licenseJson = JsonConvert.SerializeObject(license, Formatting.None);
                var licenseBytes = Encoding.UTF8.GetBytes(licenseJson);
                var signatureBytes = Convert.FromBase64String(signature);
                
                return CryptoHelper.VerifySignatureRSA(licenseBytes, signatureBytes, publicKey);
            }
            catch
            {
                return false;
            }
        }

        public static License CreateSignedLicense(LicenseTier tier, string machineId, string userId = "", string productName = "Educational DRM", DateTime? customExpiration = null, string privateKey = null)
        {
            var license = GenerateLicenseForTier(tier, machineId, userId, productName, customExpiration);
            license.Signature = SignLicense(license, privateKey);
            return license;
        }

        public static string CreateLicenseTemplate(LicenseTier tier)
        {
            var template = new
            {
                Tier = tier.ToString(),
                DefaultDuration = GetDefaultDurationForTier(tier),
                RequiredFields = new[] { "MachineId", "UserId", "ProductName" },
                OptionalFields = new[] { "CustomExpiration", "AdditionalFeatures" },
                Features = GetDefaultFeaturesForTier(tier),
                Restrictions = GetRestrictionsForTier(tier)
            };
            
            return JsonConvert.SerializeObject(template, Formatting.Indented);
        }

        public static License CreateLicenseFromTemplate(string templateJson, Dictionary<string, object> parameters)
        {
            try
            {
                var template = JsonConvert.DeserializeObject<dynamic>(templateJson);
                var tier = Enum.Parse<LicenseTier>(template.Tier.ToString());
                
                var machineId = parameters.GetValueOrDefault("MachineId", "").ToString();
                var userId = parameters.GetValueOrDefault("UserId", "").ToString();
                var productName = parameters.GetValueOrDefault("ProductName", "Educational DRM").ToString();
                
                DateTime? customExpiration = null;
                if (parameters.ContainsKey("CustomExpiration") && parameters["CustomExpiration"] is DateTime exp)
                {
                    customExpiration = exp;
                }
                
                return GenerateLicenseForTier(tier, machineId, userId, productName, customExpiration);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to create license from template: {ex.Message}", ex);
            }
        }

        private static string CalculateKeyChecksum(string key)
        {
            var hash = CryptoHelper.ComputeSHA256Hash(key);
            return hash[..4].ToUpper();
        }

        private static TimeSpan GetDefaultDurationForTier(LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => TimeSpan.FromDays(30),
                LicenseTier.Premium => TimeSpan.FromDays(365),
                _ => TimeSpan.FromDays(30)
            };
        }

        private static string[] GetDefaultFeaturesForTier(LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => new[] { "BasicFeatures", "LimitedExport", "MaxUsers:1" },
                LicenseTier.Premium => new[] { "BasicFeatures", "AdvancedFeatures", "UnlimitedExport", "PrioritySupport", "CustomizationTools", "MaxUsers:unlimited" },
                _ => new[] { "BasicFeatures" }
            };
        }

        private static string[] GetRestrictionsForTier(LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => new[] { "30 day limit", "Single user", "Limited export options" },
                LicenseTier.Premium => new[] { "Annual renewal required" },
                _ => new[] { "Basic features only" }
            };
        }
    }
}
