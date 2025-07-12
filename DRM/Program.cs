using DRM.Core;
using DRM.Hardware;
using DRM.Models;

Console.WriteLine("=== DRM Protection System - Phase 1.1 ===");
Console.WriteLine();

var machineId = MachineInfo.GetMachineId();
Console.WriteLine($"Machine ID: {machineId}");
Console.WriteLine($"Machine Fingerprint: {MachineInfo.GetMachineFingerprint()}");
Console.WriteLine();

var validLicense = new License(
    "DEMO-LICENSE-KEY-12345",
    machineId,
    DateTime.Now.AddDays(30)
);
validLicense.Features.Add(new LicenseFeature("BasicFeatures", true));
validLicense.Features.Add(new LicenseFeature("AdvancedFeatures", false));

Console.WriteLine("=== License Validation Test ===");
Console.WriteLine($"License Key: {validLicense.LicenseKey}");
Console.WriteLine($"Expiration: {validLicense.ExpirationDate:yyyy-MM-dd}");
Console.WriteLine($"Validation Result: {LicenseValidator.ValidateLicense(validLicense)}");
Console.WriteLine($"Validation Message: {LicenseValidator.GetValidationMessage(validLicense)}");
Console.WriteLine();

Console.WriteLine("=== Feature Validation Test ===");
Console.WriteLine($"BasicFeatures: {LicenseValidator.ValidateFeature(validLicense, "BasicFeatures")}");
Console.WriteLine($"AdvancedFeatures: {LicenseValidator.ValidateFeature(validLicense, "AdvancedFeatures")}");
Console.WriteLine($"NonExistentFeature: {LicenseValidator.ValidateFeature(validLicense, "NonExistentFeature")}");
Console.WriteLine();

var invalidLicense = new License(
    "INVALID-LICENSE-KEY",
    "WRONG-MACHINE-ID",
    DateTime.Now.AddDays(-1)
);

Console.WriteLine("=== Invalid License Test ===");
Console.WriteLine($"Invalid License Validation: {LicenseValidator.ValidateLicense(invalidLicense)}");
Console.WriteLine($"Invalid License Message: {LicenseValidator.GetValidationMessage(invalidLicense)}");
Console.WriteLine();

Console.WriteLine("DRM Phase 1.1 Complete - Basic PC checking implemented!");
Console.WriteLine("Press any key to exit...");
Console.ReadKey();
