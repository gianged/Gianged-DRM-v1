namespace DRM.Models
{
    public class LicenseFeature
    {
        public string Name { get; set; }
        public bool IsEnabled { get; set; }
        public DateTime? ExpirationDate { get; set; }
        public string? Value { get; set; }
        
        public LicenseFeature(string name, bool isEnabled = true, DateTime? expirationDate = null, string? value = null)
        {
            Name = name;
            IsEnabled = isEnabled;
            ExpirationDate = expirationDate;
            Value = value;
        }
    }
}
