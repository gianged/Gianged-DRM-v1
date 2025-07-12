using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using DRM.Models;
using DRM.Hardware;

namespace DRM.Core
{
    internal class LicenseValidator
    {
        public static bool ValidateLicense(License license)
        {
            if (license == null)
                return false;
                
            if (string.IsNullOrEmpty(license.LicenseKey))
                return false;
                
            if (!license.IsValid)
                return false;
                
            if (license.ExpirationDate < DateTime.Now)
                return false;
                
            var currentMachineId = MachineInfo.GetMachineId();
            if (license.MachineId != currentMachineId)
                return false;
                
            return true;
        }
        
        public static bool ValidateFeature(License license, string featureName)
        {
            if (!ValidateLicense(license))
                return false;
                
            var feature = license.Features.FirstOrDefault(f => f.Name == featureName);
            if (feature == null)
                return false;
                
            if (!feature.IsEnabled)
                return false;
                
            if (feature.ExpirationDate.HasValue && feature.ExpirationDate < DateTime.Now)
                return false;
                
            return true;
        }
        
        public static string GetValidationMessage(License license)
        {
            if (license == null)
                return "No license provided";
                
            if (string.IsNullOrEmpty(license.LicenseKey))
                return "Invalid license key";
                
            if (!license.IsValid)
                return "License is marked as invalid";
                
            if (license.ExpirationDate < DateTime.Now)
                return "License has expired";
                
            var currentMachineId = MachineInfo.GetMachineId();
            if (license.MachineId != currentMachineId)
                return "License is not valid for this machine";
                
            return "License is valid";
        }
    }
}
