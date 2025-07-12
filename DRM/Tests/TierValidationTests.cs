using System;
using DRM.Core;
using DRM.Models;

namespace DRM.Tests
{
    internal class TierValidationTests
    {
        protected TierValidationTests() { }

        public static void RunTierValidationTests(string machineId)
        {
            Console.WriteLine("=== Tier Validation Tests ===");
            
            TestTrialLicense(machineId);
            TestPremiumLicense(machineId);
            TestFeatureRestrictions(machineId);
            TestTierLimits(machineId);
            
            Console.WriteLine();
        }
        
        private static void TestTrialLicense(string machineId)
        {
            Console.WriteLine("--- Trial License Test ---");
            var trialLicense = LicenseGenerator.GenerateTrialLicense(machineId);
            
            Console.WriteLine($"License Key: {trialLicense.LicenseKey}");
            Console.WriteLine($"Tier: {trialLicense.Tier.GetDisplayName()}");
            Console.WriteLine($"Expires: {trialLicense.ExpirationDate:yyyy-MM-dd HH:mm}");
            Console.WriteLine($"Valid: {LicenseValidator.ValidateLicense(trialLicense)}");
            Console.WriteLine($"Feature Count: {trialLicense.Features.Count}");
            Console.WriteLine();
        }
        
        private static void TestPremiumLicense(string machineId)
        {
            Console.WriteLine("--- Premium License Test ---");
            var premiumLicense = LicenseGenerator.GeneratePremiumLicense(machineId);
            
            Console.WriteLine($"License Key: {premiumLicense.LicenseKey}");
            Console.WriteLine($"Tier: {premiumLicense.Tier.GetDisplayName()}");
            Console.WriteLine($"Expires: {premiumLicense.ExpirationDate:yyyy-MM-dd HH:mm}");
            Console.WriteLine($"Valid: {LicenseValidator.ValidateLicense(premiumLicense)}");
            Console.WriteLine($"Feature Count: {premiumLicense.Features.Count}");
            Console.WriteLine();
        }
        
        private static void TestFeatureRestrictions(string machineId)
        {
            Console.WriteLine("--- Feature Restriction Tests ---");
            var trialLicense = LicenseGenerator.GenerateTrialLicense(machineId);
            var premiumLicense = LicenseGenerator.GeneratePremiumLicense(machineId);
            
            Console.WriteLine("Trial License Features:");
            Console.WriteLine($"  BasicFeatures: {LicenseValidator.ValidateFeature(trialLicense, "BasicFeatures")}");
            Console.WriteLine($"  LimitedExport: {LicenseValidator.ValidateFeature(trialLicense, "LimitedExport")}");
            Console.WriteLine($"  AdvancedFeatures: {LicenseValidator.ValidateFeature(trialLicense, "AdvancedFeatures")} (Expected: False)");
            Console.WriteLine($"  UnlimitedExport: {LicenseValidator.ValidateFeature(trialLicense, "UnlimitedExport")} (Expected: False)");
            
            Console.WriteLine("Premium License Features:");
            Console.WriteLine($"  BasicFeatures: {LicenseValidator.ValidateFeature(premiumLicense, "BasicFeatures")}");
            Console.WriteLine($"  AdvancedFeatures: {LicenseValidator.ValidateFeature(premiumLicense, "AdvancedFeatures")}");
            Console.WriteLine($"  UnlimitedExport: {LicenseValidator.ValidateFeature(premiumLicense, "UnlimitedExport")}");
            Console.WriteLine($"  PrioritySupport: {LicenseValidator.ValidateFeature(premiumLicense, "PrioritySupport")}");
            Console.WriteLine();
        }
        
        private static void TestTierLimits(string machineId)
        {
            Console.WriteLine("--- Tier Limits Tests ---");
            var trialLicense = LicenseGenerator.GenerateTrialLicense(machineId);
            var premiumLicense = LicenseGenerator.GeneratePremiumLicense(machineId);
            
            Console.WriteLine($"Trial Max Features: {trialLicense.Tier.GetMaxFeatures()}");
            Console.WriteLine($"Trial Current Features: {trialLicense.Features.Count}");
            Console.WriteLine($"Trial Limits Valid: {LicenseValidator.ValidateTierLimits(trialLicense)}");
            
            Console.WriteLine($"Premium Max Features: {premiumLicense.Tier.GetMaxFeatures()}");
            Console.WriteLine($"Premium Current Features: {premiumLicense.Features.Count}");
            Console.WriteLine($"Premium Limits Valid: {LicenseValidator.ValidateTierLimits(premiumLicense)}");
            Console.WriteLine();
        }
    }
}