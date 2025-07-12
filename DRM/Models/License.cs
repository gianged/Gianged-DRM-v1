using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace DRM.Models
{
    internal class License
    {
        public string LicenseKey { get; set; }
        public string MachineId { get; set; }
        public DateTime ExpirationDate { get; set; }
        public bool IsValid { get; set; }
        public List<LicenseFeature> Features { get; set; }
        
        public License()
        {
            Features = new List<LicenseFeature>();
            IsValid = false;
        }
        
        public License(string licenseKey, string machineId, DateTime expirationDate)
        {
            LicenseKey = licenseKey;
            MachineId = machineId;
            ExpirationDate = expirationDate;
            IsValid = true;
            Features = new List<LicenseFeature>();
        }
    }
}
