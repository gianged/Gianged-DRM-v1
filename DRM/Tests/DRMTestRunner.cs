using DRM.Core;
using DRM.Hardware;
using DRM.Models;

namespace DRM.Tests
{
    internal class DRMTestRunner
    {
        protected DRMTestRunner() { }

        public static void RunAllTests()
        {
            Console.WriteLine("=== DRM Test Suite ===");
            Console.WriteLine();
            
            var machineId = MachineInfo.GetMachineId();
            Console.WriteLine($"Test Environment - Machine ID: {machineId}");
            Console.WriteLine();
            
            HardwareTests.RunHardwareTests();
            LicenseValidationTests.RunLicenseValidationTests(machineId);
            TierValidationTests.RunTierValidationTests(machineId);
            
            Console.WriteLine("=== All Tests Complete ===");
        }
    }
}