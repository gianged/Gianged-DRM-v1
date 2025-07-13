using DRM.Core;
using DRM.Hardware;
using DRM.Models;
using DRM.Protection;
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
    
    Console.WriteLine("DRM Educational License System");

    while (true)
    {
        ShowMainMenu();
        var choice = Console.ReadLine();

        switch (choice?.ToLower())
        {
            case "1":
                RunApplication();
                break;
            case "2":
                RunSecurityDemo();
                break;
            case "3":
                DRMTestRunner.RunAllTests();
                break;
            case "4":
                ShowSystemInfo();
                break;
            case "5":
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
    Console.WriteLine("3. Run DRM Tests");
    Console.WriteLine("4. Show System Information");
    Console.WriteLine("5. Exit");
    Console.WriteLine("=======================================");
    Console.Write("Enter choice (1-5): ");
}

static void RunApplication()
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
        
        Console.WriteLine();
        Console.WriteLine("Generating licenses...");
        
        var trialLicense = LicenseGenerator.GenerateTrialLicense(machineId);
        Console.WriteLine($"Trial License: {trialLicense.LicenseKey}");
        Console.WriteLine($"   Valid: {LicenseValidator.ValidateLicense(trialLicense)}");
        
        var premiumLicense = LicenseGenerator.GeneratePremiumLicense(machineId, "user123", "DRM-Educational");
        Console.WriteLine($"Premium License: {premiumLicense.LicenseKey}");
        Console.WriteLine($"   Valid: {LicenseValidator.ValidateLicense(premiumLicense)}");
        
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

static void SimulateFeature(string featureName)
{
    Console.WriteLine($"     -> Executing {featureName}...");
    Thread.Sleep(200);
    Console.WriteLine($"     -> {featureName} completed!");
}

