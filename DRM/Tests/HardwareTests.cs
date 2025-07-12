using System;
using DRM.Hardware;

namespace DRM.Tests
{
    internal class HardwareTests
    {
        protected HardwareTests() { }

        public static void RunHardwareTests()
        {
            Console.WriteLine("=== Hardware Fingerprinting Tests ===");
            
            var machineId = MachineInfo.GetMachineId();
            var fingerprint = MachineInfo.GetMachineFingerprint();
            
            Console.WriteLine($"Machine ID: {machineId}");
            Console.WriteLine($"Machine Fingerprint: {fingerprint}");
            Console.WriteLine($"Machine ID Length: {machineId.Length} (Expected: 16)");
            Console.WriteLine($"Fingerprint Consistency: {TestFingerprintConsistency()}");
            Console.WriteLine();
        }
        
        private static bool TestFingerprintConsistency()
        {
            var fingerprint1 = MachineInfo.GetMachineFingerprint();
            var fingerprint2 = MachineInfo.GetMachineFingerprint();
            var machineId1 = MachineInfo.GetMachineId();
            var machineId2 = MachineInfo.GetMachineId();
            
            return fingerprint1 == fingerprint2 && machineId1 == machineId2;
        }
    }
}