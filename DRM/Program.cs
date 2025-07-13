using DRM.Core;
using DRM.Hardware;
using DRM.Models;
using DRM.Tests;

Console.WriteLine("=== DRM Protection System - Phase 1.2 ===");
Console.WriteLine("Trial and Premium License System");
Console.WriteLine();

Console.WriteLine("Select an option:");
Console.WriteLine("1. Run Application");
Console.WriteLine("2. Run Tests");
Console.WriteLine("3. Exit");
Console.Write("Enter choice (1-3): ");

var choice = Console.ReadLine();

switch (choice)
{
    case "1":
        RunApplication();
        break;
    case "2":
        DRMTestRunner.RunAllTests();
        break;
    case "3":
        Console.WriteLine("Goodbye!");
        return;
    default:
        Console.WriteLine("Invalid choice. Running application...");
        RunApplication();
        break;
}

Console.WriteLine();
Console.WriteLine("Press any key to exit...");
Console.ReadKey();

static void RunApplication()
{
    Console.WriteLine();
    Console.WriteLine("=== DRM Application Running ===");
    
    var machineId = MachineInfo.GetMachineId();
    Console.WriteLine($"Machine ID: {machineId}");
    
    var trialLicense = LicenseGenerator.GenerateTrialLicense(machineId);
    Console.WriteLine($"Generated Trial License: {trialLicense.LicenseKey}");
    Console.WriteLine($"License Valid: {LicenseValidator.ValidateLicense(trialLicense)}");
    
    Console.WriteLine();
    Console.WriteLine("Checking available features...");
    
    if (LicenseValidator.ValidateFeature(trialLicense, "BasicFeatures").IsValid)
    {
        Console.WriteLine("✓ Basic Features - Available");
        SimulateBasicFeatures();
    }
    else
    {
        Console.WriteLine("✗ Basic Features - Not Available");
    }
    
    if (LicenseValidator.ValidateFeature(trialLicense, "AdvancedFeatures").IsValid)
    {
        Console.WriteLine("✓ Advanced Features - Available");
        SimulateAdvancedFeatures();
    }
    else
    {
        Console.WriteLine("✗ Advanced Features - Requires Premium License");
    }
    
    Console.WriteLine();
    Console.WriteLine("Application completed successfully!");
}

static void SimulateBasicFeatures()
{
    Console.WriteLine("  → Running basic functionality...");
    Thread.Sleep(500);
    Console.WriteLine("  → Basic features executed successfully!");
}

static void SimulateAdvancedFeatures()
{
    Console.WriteLine("  → Running advanced functionality...");
    Thread.Sleep(500);
    Console.WriteLine("  → Advanced features executed successfully!");
}
