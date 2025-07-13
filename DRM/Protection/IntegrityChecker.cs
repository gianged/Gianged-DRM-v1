using System.Reflection;
using System.Security.Cryptography;
using System.Text;

namespace DRM.Protection
{
    public class IntegrityChecker
    {
        private static readonly Dictionary<string, string> _knownHashes = new();
        private static bool _isInitialized = false;

        public static void Initialize()
        {
            if (_isInitialized) return;

            try
            {
                var currentAssembly = Assembly.GetExecutingAssembly();
                var assemblyPath = currentAssembly.Location;
                
                if (!string.IsNullOrEmpty(assemblyPath) && File.Exists(assemblyPath))
                {
                    var hash = ComputeFileHash(assemblyPath);
                    _knownHashes[assemblyPath] = hash;
                }

                // Skip loading referenced assemblies to avoid issues
                _isInitialized = true;
            }
            catch
            {
                // If initialization fails, mark as initialized to prevent hanging
                _isInitialized = true;
            }
        }

        public static bool VerifyAssemblyIntegrity()
        {
            if (!_isInitialized)
            {
                Initialize();
            }

            try
            {
                foreach (var kvp in _knownHashes)
                {
                    var filePath = kvp.Key;
                    var expectedHash = kvp.Value;

                    if (!File.Exists(filePath))
                    {
                        return false; // File has been deleted
                    }

                    var currentHash = ComputeFileHash(filePath);
                    if (currentHash != expectedHash)
                    {
                        return false; // File has been modified
                    }
                }

                return true;
            }
            catch
            {
                return false; // Error during verification
            }
        }

        public static bool VerifyCodeIntegrity()
        {
            try
            {
                // Check if critical methods have been tampered with
                var currentAssembly = Assembly.GetExecutingAssembly();
                var types = currentAssembly.GetTypes();

                foreach (var type in types)
                {
                    if (type.Namespace?.StartsWith("DRM") == true)
                    {
                        var methods = type.GetMethods(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Static | BindingFlags.Instance);
                        
                        foreach (var method in methods)
                        {
                            if (IsMethodTampered(method))
                            {
                                return false;
                            }
                        }
                    }
                }

                return true;
            }
            catch
            {
                return false;
            }
        }

        private static bool IsMethodTampered(MethodInfo method)
        {
            try
            {
                // Basic check for method body presence
                if (method.GetMethodBody() == null && !method.IsAbstract && 
                    (method.Attributes & MethodAttributes.PinvokeImpl) == 0)
                {
                    return true; // Method body missing unexpectedly
                }

                // Check for suspicious attributes that might indicate tampering
                var attributes = method.GetCustomAttributes();
                foreach (var attr in attributes)
                {
                    var attrName = attr.GetType().Name.ToLower();
                    if (attrName.Contains("unmanaged") || attrName.Contains("unsafe"))
                    {
                        // Suspicious attributes detected
                        return true;
                    }
                }

                return false;
            }
            catch
            {
                return true; // If we can't analyze the method, assume tampering
            }
        }

        public static string ComputeFileHash(string filePath)
        {
            try
            {
                using var sha256 = SHA256.Create();
                using var stream = File.OpenRead(filePath);
                var hashBytes = sha256.ComputeHash(stream);
                return Convert.ToBase64String(hashBytes);
            }
            catch
            {
                return string.Empty;
            }
        }

        public static string ComputeStringHash(string input)
        {
            try
            {
                using var sha256 = SHA256.Create();
                var inputBytes = Encoding.UTF8.GetBytes(input);
                var hashBytes = sha256.ComputeHash(inputBytes);
                return Convert.ToBase64String(hashBytes);
            }
            catch
            {
                return string.Empty;
            }
        }

        public static bool VerifyChecksum(string data, string expectedChecksum)
        {
            var actualChecksum = ComputeStringHash(data);
            return string.Equals(actualChecksum, expectedChecksum, StringComparison.Ordinal);
        }

        public static void PerformIntegrityCheck()
        {
            try
            {
                if (!VerifyAssemblyIntegrity() || !VerifyCodeIntegrity())
                {
                    // Integrity check failed - exit immediately
                    Environment.Exit(2);
                }
            }
            catch (Exception)
            {
                // If integrity check fails, continue (don't break the application)
                // In production, you might want to be more strict
            }
        }

        public static bool IsRunningFromExpectedLocation()
        {
            try
            {
                var currentAssembly = Assembly.GetExecutingAssembly();
                var assemblyPath = currentAssembly.Location;
                
                // Check if running from a temporary directory (potential unpacking)
                var tempPath = Path.GetTempPath();
                if (assemblyPath.StartsWith(tempPath, StringComparison.OrdinalIgnoreCase))
                {
                    return false;
                }

                // Check for common debugging/analysis directories
                string[] suspiciousPaths = {
                    "\\temp\\", "\\tmp\\", "\\debug\\", "\\analysis\\",
                    "\\sandbox\\", "\\virtual\\", "\\vm\\", "\\vbox\\"
                };

                var pathLower = assemblyPath.ToLower();
                return !suspiciousPaths.Any(suspicious => pathLower.Contains(suspicious));
            }
            catch
            {
                return false;
            }
        }

        public static void StartIntegrityMonitoring()
        {
            Task.Run(async () =>
            {
                Initialize();
                
                while (true)
                {
                    if (!VerifyAssemblyIntegrity() || !VerifyCodeIntegrity() || !IsRunningFromExpectedLocation())
                    {
                        Environment.Exit(2);
                    }
                    await Task.Delay(5000); // Check every 5 seconds
                }
            });
        }
    }
}
