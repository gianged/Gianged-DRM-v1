namespace DRM.Models
{
    internal class LicenseFeature
    {
        public string Name { get; set; }
        public bool IsEnabled { get; set; }
        public DateTime? ExpirationDate { get; set; }
        
        public LicenseFeature(string name, bool isEnabled = true, DateTime? expirationDate = null)
        {
            Name = name;
            IsEnabled = isEnabled;
            ExpirationDate = expirationDate;
        }
    }
}
