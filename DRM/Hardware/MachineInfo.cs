using System.Management;
using System.Net.NetworkInformation;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using System.Text;

namespace DRM.Hardware
{
    internal class MachineInfo
    {
        protected MachineInfo() { }

        public static string GetMachineFingerprint()
        {
            var components = new List<string>();
            
            try
            {
                components.Add(GetCpuId());
                components.Add(GetMotherboardSerial());
                components.Add(GetMacAddresses());
                components.Add(Environment.MachineName);
                components.Add(Environment.OSVersion.ToString());
                
                var fingerprint = string.Join("|", components.Where(c => !string.IsNullOrEmpty(c)));
                return ComputeHash(fingerprint);
            }
            catch (Exception)
            {
                return "UNKNOWN_MACHINE";
            }
        }
        
        private static string ComputeHash(string input)
        {
            using (var sha256 = SHA256.Create())
            {
                var bytes = Encoding.UTF8.GetBytes(input);
                var hash = sha256.ComputeHash(bytes);
                return Convert.ToBase64String(hash);
            }
        }
        
        public static string GetCpuId()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                try
                {
                    using (var searcher = new ManagementObjectSearcher("SELECT ProcessorId FROM Win32_Processor"))
                    {
                        foreach (ManagementObject obj in searcher.Get())
                        {
                            var processorId = obj["ProcessorId"]?.ToString();
                            if (!string.IsNullOrEmpty(processorId))
                                return processorId;
                        }
                    }
                }
                catch (Exception)
                {
                    try
                    {
                        using (var searcher = new ManagementObjectSearcher("SELECT UniqueId FROM Win32_Processor"))
                        {
                            foreach (ManagementObject obj in searcher.Get())
                            {
                                var uniqueId = obj["UniqueId"]?.ToString();
                                if (!string.IsNullOrEmpty(uniqueId))
                                    return uniqueId;
                            }
                        }
                    }
                    catch (Exception) { }
                }
            }
            
            return Environment.ProcessorCount.ToString() + "_" + Environment.Is64BitOperatingSystem;
        }
        
        public static string GetMotherboardSerial()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                try
                {
                    using (var searcher = new ManagementObjectSearcher("SELECT SerialNumber FROM Win32_BaseBoard"))
                    {
                        foreach (ManagementObject obj in searcher.Get())
                        {
                            var serial = obj["SerialNumber"]?.ToString();
                            if (!string.IsNullOrEmpty(serial) && serial != "Base Board Serial Number")
                                return serial;
                        }
                    }
                }
                catch (Exception) { }
                
                try
                {
                    using (var searcher = new ManagementObjectSearcher("SELECT UUID FROM Win32_ComputerSystemProduct"))
                    {
                        foreach (ManagementObject obj in searcher.Get())
                        {
                            var uuid = obj["UUID"]?.ToString();
                            if (!string.IsNullOrEmpty(uuid))
                                return uuid;
                        }
                    }
                }
                catch (Exception) { }
            }
            
            return Environment.MachineName;
        }
        
        public static string GetMacAddresses()
        {
            try
            {
                var macAddresses = new List<string>();
                
                foreach (var nic in NetworkInterface.GetAllNetworkInterfaces())
                {
                    if (nic.OperationalStatus == OperationalStatus.Up &&
                        nic.NetworkInterfaceType != NetworkInterfaceType.Loopback &&
                        nic.NetworkInterfaceType != NetworkInterfaceType.Tunnel)
                    {
                        var mac = nic.GetPhysicalAddress().ToString();
                        if (!string.IsNullOrEmpty(mac) && mac != "000000000000")
                        {
                            macAddresses.Add(mac);
                        }
                    }
                }
                
                return string.Join(",", macAddresses.OrderBy(m => m).Take(3));
            }
            catch (Exception)
            {
                return string.Empty;
            }
        }
        
        public static bool IsVirtualMachine()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                try
                {
                    var virtualIndicators = new List<string>
                    {
                        "VBOX", "VMWARE", "VIRTUAL", "XEN", "QEMU", "HYPER-V", "PARALLELS"
                    };
                    
                    using (var searcher = new ManagementObjectSearcher("SELECT * FROM Win32_ComputerSystem"))
                    {
                        foreach (ManagementObject obj in searcher.Get())
                        {
                            var manufacturer = obj["Manufacturer"]?.ToString()?.ToUpper() ?? "";
                            var model = obj["Model"]?.ToString()?.ToUpper() ?? "";
                            
                            if (virtualIndicators.Any(indicator => 
                                manufacturer.Contains(indicator) || model.Contains(indicator)))
                            {
                                return true;
                            }
                        }
                    }
                    
                    using (var searcher = new ManagementObjectSearcher("SELECT * FROM Win32_BIOS"))
                    {
                        foreach (ManagementObject obj in searcher.Get())
                        {
                            var version = obj["Version"]?.ToString()?.ToUpper() ?? "";
                            var manufacturer = obj["Manufacturer"]?.ToString()?.ToUpper() ?? "";
                            
                            if (virtualIndicators.Any(indicator => 
                                version.Contains(indicator) || manufacturer.Contains(indicator)))
                            {
                                return true;
                            }
                        }
                    }
                }
                catch (Exception) { }
            }
            
            return false;
        }
        
        public static string GetMachineId()
        {
            return GetMachineFingerprint().Substring(0, 16);
        }
    }
}
