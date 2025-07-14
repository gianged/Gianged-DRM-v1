using System.Text;
using DRM.Models;
using DRM.Core;
using Newtonsoft.Json;

namespace DRM.Storage
{
    public class LicenseStorage
    {
        private const string LICENSE_FILE_NAME = "license.drm";
        private readonly string _licensePath;
        private readonly CryptoHelper _cryptoHelper;

        public LicenseStorage()
        {
            _cryptoHelper = new CryptoHelper();
            var appDataPath = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);
            var drmFolder = Path.Combine(appDataPath, "DRM");
            _licensePath = Path.Combine(drmFolder, LICENSE_FILE_NAME);
            
            // Ensure directory exists
            Directory.CreateDirectory(drmFolder);
        }

        public void StoreLicense(License license)
        {
            if (license == null)
                throw new ArgumentNullException(nameof(license));

            var jsonData = JsonConvert.SerializeObject(license, Formatting.Indented);
            var encryptedData = _cryptoHelper.EncryptString(jsonData);
            
            File.WriteAllText(_licensePath, encryptedData, Encoding.UTF8);
        }

        public License? RetrieveLicense()
        {
            if (!File.Exists(_licensePath))
                return null;

            try
            {
                var encryptedData = File.ReadAllText(_licensePath, Encoding.UTF8);
                var decryptedJson = _cryptoHelper.DecryptString(encryptedData);
                return JsonConvert.DeserializeObject<License>(decryptedJson);
            }
            catch
            {
                return null;
            }
        }

        public void DeleteLicense()
        {
            try
            {
                if (File.Exists(_licensePath))
                {
                    // Overwrite file with random data before deletion for security
                    var randomData = new byte[new FileInfo(_licensePath).Length];
                    Random.Shared.NextBytes(randomData);
                    File.WriteAllBytes(_licensePath, randomData);
                    
                    File.Delete(_licensePath);
                }
            }
            catch { }
        }

        public bool HasValidLicense()
        {
            var license = RetrieveLicense();
            return license != null && !license.IsExpired();
        }

        public string GetLicensePath() => _licensePath;

        public bool LicenseFileExists() => File.Exists(_licensePath);
    }
}
