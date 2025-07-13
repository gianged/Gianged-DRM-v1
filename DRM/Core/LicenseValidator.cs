using DRM.Models;
using DRM.Hardware;
using System.Text.RegularExpressions;

namespace DRM.Core
{
    public class LicenseValidator
    {
        private static readonly Regex LicenseKeyPattern = new Regex(@"^[A-Z]+-[A-Z0-9]+-[0-9]+-[A-Z0-9]+$", RegexOptions.Compiled);
        private static readonly TimeSpan ClockSkewTolerance = TimeSpan.FromMinutes(5);

        protected LicenseValidator() { }

        public static ValidationResult ValidateLicense(License license, bool skipSignatureValidation = false)
        {
            var result = new ValidationResult();

            if (license == null)
            {
                result.AddError("No license provided");
                return result;
            }

            result.Combine(ValidateLicenseFormat(license));
            result.Combine(ValidateExpirationDate(license));
            result.Combine(ValidateHardwareFingerprint(license));
            
            if (!skipSignatureValidation)
            {
                result.Combine(ValidateDigitalSignature(license));
            }

            result.Combine(ValidateFeatures(license));

            return result;
        }

        public static ValidationResult ValidateLicenseFormat(License license)
        {
            var result = new ValidationResult();

            if (license == null)
            {
                result.AddError("License object is null");
                return result;
            }

            if (string.IsNullOrWhiteSpace(license.LicenseKey))
            {
                result.AddError("License key is empty or null");
            }
            else if (!LicenseKeyPattern.IsMatch(license.LicenseKey))
            {
                result.AddError("License key format is invalid. Expected format: PREFIX-RANDOM-TIMESTAMP-CHECKSUM");
            }
            else
            {
                var keyChecksum = ValidateLicenseKeyChecksum(license.LicenseKey);
                if (!keyChecksum.IsValid)
                {
                    result.AddError("License key checksum verification failed");
                }
            }

            if (string.IsNullOrWhiteSpace(license.MachineId))
            {
                result.AddError("Machine ID is required");
            }

            if (license.IssueDate > DateTime.UtcNow.Add(ClockSkewTolerance))
            {
                result.AddError("License issue date is in the future");
            }

            if (license.ExpirationDate <= license.IssueDate)
            {
                result.AddError("License expiration date must be after issue date");
            }

            if (!license.IsValid)
            {
                result.AddError("License is marked as invalid");
            }

            return result;
        }

        public static ValidationResult ValidateExpirationDate(License license)
        {
            var result = new ValidationResult();

            if (license == null)
            {
                result.AddError("License object is null");
                return result;
            }

            var currentTime = DateTime.UtcNow;
            var expirationTime = license.ExpirationDate.ToUniversalTime();

            if (expirationTime < currentTime.Subtract(ClockSkewTolerance))
            {
                var timeExpired = currentTime - expirationTime;
                result.AddError($"License expired {timeExpired.Days} days, {timeExpired.Hours} hours ago");
            }
            else if (expirationTime < currentTime.Add(TimeSpan.FromDays(7)))
            {
                var timeRemaining = expirationTime - currentTime;
                result.AddWarning($"License expires in {timeRemaining.Days} days, {timeRemaining.Hours} hours");
            }

            return result;
        }

        public static ValidationResult ValidateHardwareFingerprint(License license)
        {
            var result = new ValidationResult();

            if (license == null)
            {
                result.AddError("License object is null");
                return result;
            }

            try
            {
                var currentMachineId = MachineInfo.GetMachineId();
                
                if (string.IsNullOrWhiteSpace(currentMachineId))
                {
                    result.AddError("Unable to generate machine fingerprint");
                    return result;
                }

                if (!CryptoHelper.SecureCompare(license.MachineId, currentMachineId))
                {
                    result.AddError("License is not valid for this machine. Hardware fingerprint mismatch.");
                }
            }
            catch (Exception ex)
            {
                result.AddError($"Hardware fingerprint validation failed: {ex.Message}");
            }

            return result;
        }

        public static ValidationResult ValidateDigitalSignature(License license)
        {
            var result = new ValidationResult();

            if (license == null)
            {
                result.AddError("License object is null");
                return result;
            }

            if (string.IsNullOrWhiteSpace(license.Signature))
            {
                result.AddError("License signature is missing");
                return result;
            }

            try
            {
                var isValidSignature = LicenseGenerator.VerifyLicenseSignature(license, license.Signature);
                if (!isValidSignature)
                {
                    result.AddError("License signature verification failed. License may be tampered with.");
                }
            }
            catch (Exception ex)
            {
                result.AddError($"Signature validation error: {ex.Message}");
            }

            return result;
        }

        public static ValidationResult ValidateFeatures(License license)
        {
            var result = new ValidationResult();

            if (license == null)
            {
                result.AddError("License object is null");
                return result;
            }

            foreach (var feature in license.Features)
            {
                var featureResult = ValidateFeature(license, feature.Name);
                result.Combine(featureResult);
            }

            var tierValidation = ValidateTierLimits(license);
            result.Combine(tierValidation);

            return result;
        }

        public static ValidationResult ValidateFeature(License license, string featureName)
        {
            var result = new ValidationResult();

            if (license == null)
            {
                result.AddError("License object is null");
                return result;
            }

            if (string.IsNullOrWhiteSpace(featureName))
            {
                result.AddError("Feature name cannot be empty");
                return result;
            }

            var feature = license.Features.FirstOrDefault(f => f.Name.Equals(featureName, StringComparison.OrdinalIgnoreCase));
            if (feature == null)
            {
                result.AddError($"Feature '{featureName}' not found in license");
                return result;
            }

            if (!feature.IsEnabled)
            {
                result.AddError($"Feature '{featureName}' is disabled");
            }

            if (feature.ExpirationDate.HasValue)
            {
                var featureExpiration = feature.ExpirationDate.Value.ToUniversalTime();
                var currentTime = DateTime.UtcNow;

                if (featureExpiration < currentTime.Subtract(ClockSkewTolerance))
                {
                    result.AddError($"Feature '{featureName}' has expired");
                }
            }

            if (!ValidateFeatureForTier(license.Tier, featureName))
            {
                result.AddError($"Feature '{featureName}' is not available for {license.Tier} tier");
            }

            return result;
        }

        public static bool ValidateFeatureForTier(LicenseTier tier, string featureName)
        {
            var allowedFeatures = GetAllowedFeaturesForTier(tier);
            return allowedFeatures.Contains(featureName, StringComparer.OrdinalIgnoreCase);
        }

        public static ValidationResult ValidateTierLimits(License license)
        {
            var result = new ValidationResult();

            if (license == null)
            {
                result.AddError("License object is null");
                return result;
            }

            var maxUsers = GetMaxUsersForTier(license.Tier);
            var maxUsersFeature = license.Features.FirstOrDefault(f => f.Name.Equals("MaxUsers", StringComparison.OrdinalIgnoreCase));
            
            if (maxUsersFeature != null && maxUsersFeature.Value != "unlimited")
            {
                if (int.TryParse(maxUsersFeature.Value, out int userLimit))
                {
                    if (userLimit > maxUsers)
                    {
                        result.AddError($"User limit ({userLimit}) exceeds maximum for {license.Tier} tier ({maxUsers})");
                    }
                }
            }

            var enabledFeatures = license.Features.Count(f => f.IsEnabled);
            var maxFeatures = GetMaxFeaturesForTier(license.Tier);
            
            if (enabledFeatures > maxFeatures)
            {
                result.AddError($"Number of enabled features ({enabledFeatures}) exceeds maximum for {license.Tier} tier ({maxFeatures})");
            }

            return result;
        }

        public static bool HasFeature(License license, string featureName)
        {
            if (license == null || string.IsNullOrWhiteSpace(featureName))
                return false;

            var validationResult = ValidateFeature(license, featureName);
            return validationResult.IsValid;
        }

        public static string GetFeatureValue(License license, string featureName, string defaultValue = "")
        {
            if (license == null || string.IsNullOrWhiteSpace(featureName))
                return defaultValue;

            var feature = license.Features.FirstOrDefault(f => f.Name.Equals(featureName, StringComparison.OrdinalIgnoreCase));
            return feature?.Value ?? defaultValue;
        }

        private static ValidationResult ValidateLicenseKeyChecksum(string licenseKey)
        {
            var result = new ValidationResult();

            try
            {
                var parts = licenseKey.Split('-');
                if (parts.Length != 4)
                {
                    result.AddError("License key must have 4 parts separated by hyphens");
                    return result;
                }

                var keyWithoutChecksum = string.Join("-", parts[0], parts[1], parts[2]);
                var expectedChecksum = CryptoHelper.ComputeSHA256Hash(keyWithoutChecksum)[..4].ToUpper();
                
                if (!CryptoHelper.SecureCompare(parts[3], expectedChecksum))
                {
                    result.AddError("License key checksum is invalid");
                }
            }
            catch (Exception ex)
            {
                result.AddError($"Checksum validation failed: {ex.Message}");
            }

            return result;
        }

        private static string[] GetAllowedFeaturesForTier(LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => new[] { "BasicFeatures", "LimitedExport", "MaxUsers" },
                LicenseTier.Premium => new[] { "BasicFeatures", "AdvancedFeatures", "UnlimitedExport", "PrioritySupport", "CustomizationTools", "MaxUsers" },
                _ => new[] { "BasicFeatures" }
            };
        }

        private static int GetMaxUsersForTier(LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => 1,
                LicenseTier.Premium => int.MaxValue,
                _ => 1
            };
        }

        private static int GetMaxFeaturesForTier(LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => 3,
                LicenseTier.Premium => 10,
                _ => 1
            };
        }

        public class ValidationResult
        {
            public bool IsValid => !Errors.Any();
            public List<string> Errors { get; } = new List<string>();
            public List<string> Warnings { get; } = new List<string>();

            public void AddError(string error)
            {
                if (!string.IsNullOrWhiteSpace(error))
                    Errors.Add(error);
            }

            public void AddWarning(string warning)
            {
                if (!string.IsNullOrWhiteSpace(warning))
                    Warnings.Add(warning);
            }

            public void Combine(ValidationResult other)
            {
                if (other != null)
                {
                    Errors.AddRange(other.Errors);
                    Warnings.AddRange(other.Warnings);
                }
            }

            public string GetSummary()
            {
                if (IsValid)
                {
                    return Warnings.Any() ? $"Valid with {Warnings.Count} warning(s)" : "Valid";
                }
                
                return $"{Errors.Count} error(s)" + (Warnings.Any() ? $", {Warnings.Count} warning(s)" : "");
            }

            public override string ToString()
            {
                var messages = new List<string>();
                
                if (Errors.Any())
                {
                    messages.Add("ERRORS:");
                    messages.AddRange(Errors.Select(e => $"  - {e}"));
                }
                
                if (Warnings.Any())
                {
                    messages.Add("WARNINGS:");
                    messages.AddRange(Warnings.Select(w => $"  - {w}"));
                }
                
                return messages.Any() ? string.Join(Environment.NewLine, messages) : "No issues found";
            }
        }
    }
}
