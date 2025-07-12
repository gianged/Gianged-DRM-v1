using DRM.Core;
using DRM.Models;

namespace DRM.Tests
{
    internal class LicenseValidationTests
    {
        protected LicenseValidationTests() { }

        public static void RunLicenseValidationTests(string machineId)
        {
            Console.WriteLine("=== License Validation Tests ===");
            
            TestValidLicense(machineId);
            TestExpiredLicense(machineId);
            TestWrongMachineLicense();
            TestInvalidLicense();
            
            Console.WriteLine();
        }
        
        private static void TestValidLicense(string machineId)
        {
            Console.WriteLine("--- Valid License Test ---");
            var validLicense = new License(
                "VALID-LICENSE-KEY-12345",
                machineId,
                DateTime.Now.AddDays(30),
                LicenseTier.Premium
            );
            validLicense.Features.Add(new LicenseFeature("BasicFeatures", true));
            
            var isValid = LicenseValidator.ValidateLicense(validLicense);
            var message = LicenseValidator.GetValidationMessage(validLicense);
            
            Console.WriteLine($"Valid License Test: {isValid} (Expected: True)");
            Console.WriteLine($"Message: {message}");
        }
        
        private static void TestExpiredLicense(string machineId)
        {
            Console.WriteLine("--- Expired License Test ---");
            var expiredLicense = new License(
                "EXPIRED-LICENSE-KEY",
                machineId,
                DateTime.Now.AddDays(-1),
                LicenseTier.Trial
            );
            
            var isValid = LicenseValidator.ValidateLicense(expiredLicense);
            var message = LicenseValidator.GetValidationMessage(expiredLicense);
            
            Console.WriteLine($"Expired License Test: {isValid} (Expected: False)");
            Console.WriteLine($"Message: {message}");
        }
        
        private static void TestWrongMachineLicense()
        {
            Console.WriteLine("--- Wrong Machine License Test ---");
            var wrongMachineLicense = new License(
                "WRONG-MACHINE-LICENSE",
                "DIFFERENT-MACHINE-ID",
                DateTime.Now.AddDays(30),
                LicenseTier.Premium
            );
            
            var isValid = LicenseValidator.ValidateLicense(wrongMachineLicense);
            var message = LicenseValidator.GetValidationMessage(wrongMachineLicense);
            
            Console.WriteLine($"Wrong Machine Test: {isValid} (Expected: False)");
            Console.WriteLine($"Message: {message}");
        }
        
        private static void TestInvalidLicense()
        {
            Console.WriteLine("--- Invalid License Test ---");
            var invalidLicense = new License();
            
            var isValid = LicenseValidator.ValidateLicense(invalidLicense);
            var message = LicenseValidator.GetValidationMessage(invalidLicense);
            
            Console.WriteLine($"Invalid License Test: {isValid} (Expected: False)");
            Console.WriteLine($"Message: {message}");
        }
    }
}