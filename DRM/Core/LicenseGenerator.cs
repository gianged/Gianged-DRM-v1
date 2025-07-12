using DRM.Models;
using DRM.Hardware;

namespace DRM.Core
{
    internal class LicenseGenerator
    {
        protected LicenseGenerator() { }
        
        public static License GenerateTrialLicense(string machineId)
        {
            var licenseKey = GenerateLicenseKey("TRIAL");
            var expirationDate = DateTime.Now.Add(LicenseTier.Trial.GetDefaultDuration());
            
            var license = new License(licenseKey, machineId, expirationDate, LicenseTier.Trial);
            
            license.Features.Add(new LicenseFeature("BasicFeatures", true));
            license.Features.Add(new LicenseFeature("LimitedExport", true));
            
            return license;
        }
        
        public static License GeneratePremiumLicense(string machineId)
        {
            var licenseKey = GenerateLicenseKey("PREMIUM");
            var expirationDate = DateTime.Now.Add(LicenseTier.Premium.GetDefaultDuration());
            
            var license = new License(licenseKey, machineId, expirationDate, LicenseTier.Premium);
            
            license.Features.Add(new LicenseFeature("BasicFeatures", true));
            license.Features.Add(new LicenseFeature("AdvancedFeatures", true));
            license.Features.Add(new LicenseFeature("UnlimitedExport", true));
            license.Features.Add(new LicenseFeature("PrioritySupport", true));
            license.Features.Add(new LicenseFeature("CustomizationTools", true));
            
            return license;
        }
        
        public static License GenerateLicenseForTier(LicenseTier tier, string machineId)
        {
            return tier switch
            {
                LicenseTier.Trial => GenerateTrialLicense(machineId),
                LicenseTier.Premium => GeneratePremiumLicense(machineId),
                _ => throw new ArgumentException($"Unsupported license tier: {tier}")
            };
        }
        
        private static string GenerateLicenseKey(string prefix)
        {
            var guid = Guid.NewGuid().ToString("N")[..8].ToUpper();
            var timestamp = DateTimeOffset.Now.ToUnixTimeSeconds().ToString()[^6..];
            
            return $"{prefix}-{guid}-{timestamp}";
        }
    }
}
