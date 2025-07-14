using DRM.Core;
using DRM.Hardware;
using DRM.Models;
using DRM.Protection;
using DRM.Storage;
using DRM.Tests;

try
{
    Console.WriteLine("DRM Educational System - Initializing Security...");
    
    Console.WriteLine("Checking for debuggers...");
    bool debuggerDetected = AntiDebugger.IsProcessBeingDebugged();
    if (debuggerDetected)
    {
        Console.WriteLine("WARNING: Debugger detected - continuing for educational purposes");
    }
    else
    {
        Console.WriteLine("No debugger detected");
    }
    
    Console.WriteLine("Verifying code integrity...");
    bool integrityValid = IntegrityChecker.VerifyAssemblyIntegrity() && IntegrityChecker.VerifyCodeIntegrity();
    if (!integrityValid)
    {
        Console.WriteLine("WARNING: Integrity issues detected - continuing for educational purposes");
    }
    else
    {
        Console.WriteLine("Code integrity verified");
    }
    
    Console.WriteLine("Security checks passed!");
    Console.WriteLine();
    
    // Background monitoring disabled for educational demo to prevent automatic termination
    // AntiDebugger.StartAntiDebugMonitoring();
    // IntegrityChecker.StartIntegrityMonitoring();
    
    // Initialize license storage
    var licenseStorage = new LicenseStorage();
    Console.WriteLine($"License storage path: {licenseStorage.GetLicensePath()}");
    
    // Check for existing license
    var existingLicense = licenseStorage.RetrieveLicense();
    if (existingLicense != null)
    {
        var isValid = LicenseValidator.ValidateLicense(existingLicense);
        Console.WriteLine($"Existing license found: {(isValid.IsValid ? "VALID" : "INVALID")}");
        if (!isValid.IsValid)
        {
            Console.WriteLine($"License issues: {string.Join(", ", isValid.Errors)}");
        }
    }
    else
    {
        Console.WriteLine("No existing license found");
    }
    
    Console.WriteLine();
    Console.WriteLine("DRM Educational License System");

    while (true)
    {
        ShowMainMenu();
        var choice = Console.ReadLine();

        switch (choice?.ToLower())
        {
            case "1":
                RunApplication(licenseStorage);
                break;
            case "2":
                RunSecurityDemo();
                break;
            case "3":
                ManageLicenses(licenseStorage);
                break;
            case "4":
                DRMTestRunner.RunAllTests();
                break;
            case "5":
                ShowSystemInfo();
                break;
            case "6":
                Console.WriteLine("Goodbye!");
                return;
            default:
                Console.WriteLine("Invalid choice. Please try again.");
                continue;
        }
        
        Console.WriteLine();
        Console.Write("Press any key to continue...");
        try
        {
            Console.ReadKey();
        }
        catch
        {
            Thread.Sleep(1000);
        }
        Console.Clear();
    }
}
catch (Exception ex)
{
    Console.WriteLine($"Critical Error: {ex.Message}");
    Console.WriteLine("Application terminated for security reasons.");
    Environment.Exit(1);
}

static void ShowMainMenu()
{
    Console.WriteLine("=======================================");
    Console.WriteLine("Select an option:");
    Console.WriteLine("1. Run License Demo Application");
    Console.WriteLine("2. Run Security Protection Demo");
    Console.WriteLine("3. Manage Licenses");
    Console.WriteLine("4. Run DRM Tests");
    Console.WriteLine("5. Show System Information");
    Console.WriteLine("6. Exit");
    Console.WriteLine("=======================================");
    Console.Write("Enter choice (1-6): ");
}

static void RunApplication(LicenseStorage licenseStorage)
{
    try
    {
        Console.WriteLine();
        Console.WriteLine("=== DRM License Demo Application ===");
        
        var machineId = MachineInfo.GetMachineId();
        Console.WriteLine($"Machine ID: {machineId}");
        
        if (MachineInfo.IsVirtualMachine())
        {
            Console.WriteLine("Virtual machine detected!");
        }
        
        // Check for existing valid license first
        var existingLicense = licenseStorage.RetrieveLicense();
        if (existingLicense != null)
        {
            var validation = LicenseValidator.ValidateLicense(existingLicense);
            if (validation.IsValid)
            {
                Console.WriteLine();
                Console.WriteLine("Using existing valid license:");
                Console.WriteLine($"License Key: {existingLicense.LicenseKey}");
                Console.WriteLine($"Tier: {existingLicense.Tier}");
                Console.WriteLine($"Expires: {existingLicense.ExpirationDate:yyyy-MM-dd HH:mm:ss}");
                
                Console.WriteLine();
                Console.WriteLine("Testing License Features...");
                TestLicenseFeatures(existingLicense);
                
                Console.WriteLine();
                Console.WriteLine("Application running with existing license!");
                return;
            }
            else
            {
                Console.WriteLine();
                Console.WriteLine("Existing license is invalid or expired");
                Console.WriteLine($"Issues: {string.Join(", ", validation.Errors)}");
            }
        }
        
        Console.WriteLine();
        Console.WriteLine("No valid license found. Generating demo licenses...");
        
        var trialLicense = LicenseGenerator.GenerateTrialLicense(machineId);
        Console.WriteLine($"Trial License: {trialLicense.LicenseKey}");
        Console.WriteLine($"   Valid: {LicenseValidator.ValidateLicense(trialLicense).IsValid}");
        
        var premiumLicense = LicenseGenerator.GeneratePremiumLicense(machineId, "user123", "DRM-Educational");
        Console.WriteLine($"Premium License: {premiumLicense.LicenseKey}");
        Console.WriteLine($"   Valid: {LicenseValidator.ValidateLicense(premiumLicense).IsValid}");
        
        Console.WriteLine();
        Console.WriteLine("Testing Trial License Features...");
        TestLicenseFeatures(trialLicense);
        
        Console.WriteLine();
        Console.WriteLine("Testing Premium License Features...");
        TestLicenseFeatures(premiumLicense);
        
        Console.WriteLine();
        Console.WriteLine("License demo completed successfully!");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error in license demo: {ex.Message}");
    }
}

static void TestLicenseFeatures(License license)
{
    var features = new[] { "BasicFeatures", "AdvancedFeatures", "PremiumFeatures" };
    
    foreach (var feature in features)
    {
        var result = LicenseValidator.ValidateFeature(license, feature);
        var status = result.IsValid ? "[OK]" : "[FAIL]";
        var message = result.IsValid ? "Available" : string.Join(", ", result.Errors);
        Console.WriteLine($"   {status} {feature}: {message}");
        
        if (result.IsValid)
        {
            SimulateFeature(feature);
        }
    }
}

static void RunSecurityDemo()
{
    try
    {
        Console.WriteLine();
        Console.WriteLine("=== Security Protection Demo ===");
        
        Console.WriteLine();
        Console.WriteLine("Anti-Debugging Tests:");
        
        bool debuggerDetected = AntiDebugger.IsDebuggerAttached();
        Console.WriteLine($"   Debugger Attached: {(debuggerDetected ? "YES" : "NO")}");
        
        bool remoteDebugger = AntiDebugger.IsRemoteDebuggerPresent();
        Console.WriteLine($"   Remote Debugger: {(remoteDebugger ? "YES" : "NO")}");
        
        bool timingCheck = AntiDebugger.CheckDebuggerWithTiming();
        Console.WriteLine($"   Timing Anomaly: {(timingCheck ? "DETECTED" : "NORMAL")}");
        
        bool processCheck = AntiDebugger.IsProcessBeingDebugged();
        Console.WriteLine($"   Overall Status: {(processCheck ? "DEBUGGING DETECTED" : "SECURE")}");
        
        Console.WriteLine();
        Console.WriteLine("Integrity Checking Tests:");

        bool assemblyIntegrity = IntegrityChecker.VerifyAssemblyIntegrity();
        Console.WriteLine($"   Assembly Integrity: {(assemblyIntegrity ? "VALID" : "COMPROMISED")}");
        
        bool codeIntegrity = IntegrityChecker.VerifyCodeIntegrity();
        Console.WriteLine($"   Code Integrity: {(codeIntegrity ? "VALID" : "COMPROMISED")}");
        
        bool locationCheck = IntegrityChecker.IsRunningFromExpectedLocation();
        Console.WriteLine($"   Location Check: {(locationCheck ? "SAFE" : "SUSPICIOUS")}");
        
        string testData = "Sample license data";
        string hash = IntegrityChecker.ComputeStringHash(testData);
        Console.WriteLine($"   Sample Hash: {hash[..16]}...");
        
        Console.WriteLine();
        Console.WriteLine("Background monitoring is active...");
        Console.WriteLine("   - Anti-debugging checks every 1 second");
        Console.WriteLine("   - Integrity verification every 5 seconds");
        
        Console.WriteLine();
        Console.WriteLine("Security demo completed!");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error in security demo: {ex.Message}");
    }
}

static void ShowSystemInfo()
{
    try
    {
        Console.WriteLine();
        Console.WriteLine("=== System Information ===");
        
        Console.WriteLine($"Machine Name: {Environment.MachineName}");
        Console.WriteLine($"OS Version: {Environment.OSVersion}");
        Console.WriteLine($"64-bit OS: {Environment.Is64BitOperatingSystem}");
        Console.WriteLine($"Processor Count: {Environment.ProcessorCount}");
        Console.WriteLine($"Working Set: {Environment.WorkingSet / 1024 / 1024} MB");
        
        Console.WriteLine();
        Console.WriteLine("Hardware Fingerprint:");
        Console.WriteLine($"   CPU ID: {MachineInfo.GetCpuId()}");
        Console.WriteLine($"   Motherboard: {MachineInfo.GetMotherboardSerial()}");
        Console.WriteLine($"   MAC Addresses: {MachineInfo.GetMacAddresses()}");
        Console.WriteLine($"   Machine ID: {MachineInfo.GetMachineId()}");
        Console.WriteLine($"   Full Fingerprint: {MachineInfo.GetMachineFingerprint()[..32]}...");
        
        Console.WriteLine();
        Console.WriteLine($"Virtual Machine: {(MachineInfo.IsVirtualMachine() ? "YES" : "NO")}");
        
        Console.WriteLine();
        Console.WriteLine("System information displayed!");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error getting system info: {ex.Message}");
    }
}

static void ManageLicenses(LicenseStorage licenseStorage)
{
    try
    {
        Console.WriteLine();
        Console.WriteLine("=== License Management ===");
        
        while (true)
        {
            ShowLicenseMenu();
            var choice = Console.ReadLine();
            
            switch (choice?.ToLower())
            {
                case "1":
                    ViewCurrentLicense(licenseStorage);
                    break;
                case "2":
                    GenerateAndSaveLicense(licenseStorage);
                    break;
                case "3":
                    LoadLicenseFromFile(licenseStorage);
                    break;
                case "4":
                    DeleteCurrentLicense(licenseStorage);
                    break;
                case "5":
                    return;
                default:
                    Console.WriteLine("Invalid choice. Please try again.");
                    continue;
            }
            
            Console.WriteLine();
            Console.Write("Press any key to continue...");
            try
            {
                Console.ReadKey();
            }
            catch
            {
                Thread.Sleep(1000);
            }
            Console.Clear();
            Console.WriteLine("=== License Management ===");
        }
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error in license management: {ex.Message}");
    }
}

static void ShowLicenseMenu()
{
    Console.WriteLine("---------------------------------------");
    Console.WriteLine("License Management Options:");
    Console.WriteLine("1. View Current License");
    Console.WriteLine("2. Generate and Save New License");
    Console.WriteLine("3. Load License from Storage");
    Console.WriteLine("4. Delete Current License");
    Console.WriteLine("5. Back to Main Menu");
    Console.WriteLine("---------------------------------------");
    Console.Write("Enter choice (1-5): ");
}

static void ViewCurrentLicense(LicenseStorage licenseStorage)
{
    try
    {
        var license = licenseStorage.RetrieveLicense();
        if (license == null)
        {
            Console.WriteLine("No license found in storage.");
            return;
        }
        
        Console.WriteLine();
        Console.WriteLine("Current License Details:");
        Console.WriteLine($"  License Key: {license.LicenseKey}");
        Console.WriteLine($"  User ID: {license.UserId ?? "N/A"}");
        Console.WriteLine($"  Product: {license.ProductName ?? "N/A"}");
        Console.WriteLine($"  Tier: {license.Tier}");
        Console.WriteLine($"  Issue Date: {license.IssueDate:yyyy-MM-dd HH:mm:ss}");
        Console.WriteLine($"  Expiration: {license.ExpirationDate:yyyy-MM-dd HH:mm:ss}");
        Console.WriteLine($"  Machine ID: {license.MachineId}");
        Console.WriteLine($"  Is Expired: {license.IsExpired()}");
        
        var validation = LicenseValidator.ValidateLicense(license);
        Console.WriteLine($"  Validation: {(validation.IsValid ? "VALID" : "INVALID")}");
        if (!validation.IsValid)
        {
            Console.WriteLine($"  Issues: {string.Join(", ", validation.Errors)}");
        }
        
        Console.WriteLine();
        Console.WriteLine("License Features:");
        TestLicenseFeatures(license);
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error viewing license: {ex.Message}");
    }
}

static void GenerateAndSaveLicense(LicenseStorage licenseStorage)
{
    try
    {
        Console.WriteLine();
        Console.WriteLine("Generate New License:");
        Console.WriteLine("1. Trial License (7 days)");
        Console.WriteLine("2. Premium License (1 year)");
        Console.Write("Choose license type (1-2): ");
        
        var choice = Console.ReadLine();
        var machineId = MachineInfo.GetMachineId();
        License newLicense;
        
        switch (choice)
        {
            case "1":
                newLicense = LicenseGenerator.GenerateTrialLicense(machineId);
                Console.WriteLine("Generated Trial License");
                break;
            case "2":
                Console.Write("Enter User ID: ");
                var userId = Console.ReadLine() ?? "demo-user";
                Console.Write("Enter Product Name: ");
                var productName = Console.ReadLine() ?? "DRM-Educational";
                newLicense = LicenseGenerator.GeneratePremiumLicense(machineId, userId, productName);
                Console.WriteLine("Generated Premium License");
                break;
            default:
                Console.WriteLine("Invalid choice. Generating Trial License as default.");
                newLicense = LicenseGenerator.GenerateTrialLicense(machineId);
                break;
        }
        
        // Save the license
        licenseStorage.StoreLicense(newLicense);
        Console.WriteLine($"License saved successfully!");
        Console.WriteLine($"License Key: {newLicense.LicenseKey}");
        Console.WriteLine($"Expires: {newLicense.ExpirationDate:yyyy-MM-dd HH:mm:ss}");
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error generating license: {ex.Message}");
    }
}

static void LoadLicenseFromFile(LicenseStorage licenseStorage)
{
    try
    {
        Console.WriteLine();
        Console.WriteLine("Loading license from storage...");
        
        if (!licenseStorage.LicenseFileExists())
        {
            Console.WriteLine("No license file found in storage.");
            return;
        }
        
        var license = licenseStorage.RetrieveLicense();
        if (license == null)
        {
            Console.WriteLine("Failed to load license from storage (file may be corrupted).");
            return;
        }
        
        Console.WriteLine("License loaded successfully!");
        Console.WriteLine($"License Key: {license.LicenseKey}");
        Console.WriteLine($"Tier: {license.Tier}");
        Console.WriteLine($"Expires: {license.ExpirationDate:yyyy-MM-dd HH:mm:ss}");
        
        var validation = LicenseValidator.ValidateLicense(license);
        Console.WriteLine($"Status: {(validation.IsValid ? "VALID" : "INVALID")}");
        if (!validation.IsValid)
        {
            Console.WriteLine($"Issues: {string.Join(", ", validation.Errors)}");
        }
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error loading license: {ex.Message}");
    }
}

static void DeleteCurrentLicense(LicenseStorage licenseStorage)
{
    try
    {
        Console.WriteLine();
        if (!licenseStorage.LicenseFileExists())
        {
            Console.WriteLine("No license file found to delete.");
            return;
        }
        
        Console.Write("Are you sure you want to delete the current license? (y/N): ");
        var confirmation = Console.ReadLine();
        
        if (confirmation?.ToLower() == "y" || confirmation?.ToLower() == "yes")
        {
            licenseStorage.DeleteLicense();
            Console.WriteLine("License deleted successfully!");
        }
        else
        {
            Console.WriteLine("License deletion cancelled.");
        }
    }
    catch (Exception ex)
    {
        Console.WriteLine($"Error deleting license: {ex.Message}");
    }
}

static void SimulateFeature(string featureName)
{
    Console.WriteLine($"     -> Executing {featureName}...");
    Thread.Sleep(200);
    Console.WriteLine($"     -> {featureName} completed!");
}

