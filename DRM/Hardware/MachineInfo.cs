using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Threading.Tasks;

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
                components.Add(Environment.MachineName);
                components.Add(Environment.UserName);
                components.Add(Environment.OSVersion.ToString());
                components.Add(Environment.ProcessorCount.ToString());
                
                var fingerprint = string.Join("|", components);
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
        
        public static string GetMachineId()
        {
            return GetMachineFingerprint().Substring(0, 16);
        }
    }
}
