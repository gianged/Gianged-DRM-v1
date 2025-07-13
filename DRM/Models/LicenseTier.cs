namespace DRM.Models
{
    public enum LicenseTier
    {
        Trial = 0,
        Premium = 1
    }
    
    internal static class LicenseTierExtensions
    {
        public static string GetDisplayName(this LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => "Trial",
                LicenseTier.Premium => "Premium",
                _ => "Unknown"
            };
        }
        
        public static int GetMaxFeatures(this LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => 2,
                LicenseTier.Premium => 10,
                _ => 0
            };
        }
        
        public static TimeSpan GetDefaultDuration(this LicenseTier tier)
        {
            return tier switch
            {
                LicenseTier.Trial => TimeSpan.FromDays(7),
                LicenseTier.Premium => TimeSpan.FromDays(365),
                _ => TimeSpan.Zero
            };
        }
    }
}