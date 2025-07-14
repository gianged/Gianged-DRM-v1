namespace DRM.Models
{
    public class License
    {
        public string LicenseKey { get; set; } = string.Empty;
        public string MachineId { get; set; } = string.Empty;
        public string UserId { get; set; } = string.Empty;
        public string ProductName { get; set; } = string.Empty;
        public DateTime ExpirationDate { get; set; }
        public DateTime IssueDate { get; set; }
        public bool IsValid { get; set; }
        public LicenseTier Tier { get; set; }
        public List<LicenseFeature> Features { get; set; }
        public string Signature { get; set; } = string.Empty;
        
        public License()
        {
            Features = new List<LicenseFeature>();
            IsValid = false;
            IssueDate = DateTime.UtcNow;
        }
        
        public License(string licenseKey, string machineId, DateTime expirationDate, LicenseTier tier = LicenseTier.Trial)
        {
            LicenseKey = licenseKey;
            MachineId = machineId;
            ExpirationDate = expirationDate;
            Tier = tier;
            IsValid = true;
            Features = new List<LicenseFeature>();
            IssueDate = DateTime.UtcNow;
        }

        public bool IsExpired()
        {
            return DateTime.UtcNow > ExpirationDate;
        }

        public bool IsValidLicense()
        {
            return IsValid && !IsExpired();
        }
    }
}
