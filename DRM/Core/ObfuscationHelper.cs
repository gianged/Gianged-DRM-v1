using System.Text;
using DRM.Utils;

namespace DRM.Core
{
    public static class ObfuscationHelper
    {
        private static readonly Random _random = new Random();
        
        public static string ObfuscateString(string input, ObfuscationMethod method = ObfuscationMethod.Caesar)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            return method switch
            {
                ObfuscationMethod.Caesar => CaesarCipher(input, 13),
                ObfuscationMethod.Base64 => Utils.Encoder.ToBase64(input),
                ObfuscationMethod.Custom => Utils.Encoder.ToCustomEncoding(input),
                ObfuscationMethod.XOR => XorObfuscation(input),
                ObfuscationMethod.Reverse => ReverseString(input),
                _ => input
            };
        }

        public static string DeobfuscateString(string input, ObfuscationMethod method = ObfuscationMethod.Caesar)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            return method switch
            {
                ObfuscationMethod.Caesar => CaesarCipher(input, -13),
                ObfuscationMethod.Base64 => Utils.Encoder.FromBase64(input),
                ObfuscationMethod.Custom => Utils.Encoder.FromCustomEncoding(input),
                ObfuscationMethod.XOR => XorObfuscation(input),
                ObfuscationMethod.Reverse => ReverseString(input),
                _ => input
            };
        }

        public static string[] GenerateDecoyStrings(int count = 5)
        {
            var decoys = new string[count];
            var templates = new[]
            {
                "System.Security.Cryptography.{0}",
                "Microsoft.{0}.Authentication",
                "Windows.{0}.Registry",
                "{0}ValidationService",
                "Anti{0}Protection"
            };

            for (int i = 0; i < count; i++)
            {
                var randomWord = GenerateRandomString(8);
                var template = templates[_random.Next(templates.Length)];
                decoys[i] = string.Format(template, randomWord);
            }

            return decoys;
        }

        public static string GenerateNoisyCode(int lines = 10)
        {
            var noise = new StringBuilder();
            var noisyStatements = new[]
            {
                "var temp{0} = DateTime.Now.Ticks;",
                "var dummy{0} = new Random().Next();",
                "Thread.Sleep(0);",
                "GC.Collect(0, GCCollectionMode.Optimized);",
                "var hash{0} = \"{1}\".GetHashCode();",
                "Console.Write(\"\");",
                "var check{0} = Environment.TickCount;"
            };

            for (int i = 0; i < lines; i++)
            {
                var statement = noisyStatements[_random.Next(noisyStatements.Length)];
                if (statement.Contains("{0}"))
                {
                    statement = string.Format(statement, i, GenerateRandomString(6));
                }
                noise.AppendLine($"            {statement}");
            }

            return noise.ToString();
        }

        public static string CreateFakeMethodSignature()
        {
            var returnTypes = new[] { "void", "bool", "int", "string", "object" };
            var methodNames = new[] { "ValidateSystem", "CheckIntegrity", "ProcessData", "CalculateHash", "VerifySignature" };
            var paramTypes = new[] { "string", "int", "bool", "byte[]", "object" };

            var returnType = returnTypes[_random.Next(returnTypes.Length)];
            var methodName = methodNames[_random.Next(methodNames.Length)] + _random.Next(100, 999);
            var paramCount = _random.Next(0, 4);

            var parameters = new List<string>();
            for (int i = 0; i < paramCount; i++)
            {
                var paramType = paramTypes[_random.Next(paramTypes.Length)];
                var paramName = "param" + i;
                parameters.Add($"{paramType} {paramName}");
            }

            return $"private {returnType} {methodName}({string.Join(", ", parameters)})";
        }

        public static string ScrambleControlFlow(string originalCode)
        {
            if (string.IsNullOrEmpty(originalCode))
                return originalCode;

            var lines = originalCode.Split('\n');
            var scrambled = new StringBuilder();

            // Add some fake conditional checks
            scrambled.AppendLine("            if (DateTime.Now.Millisecond % 2 == 0) { }");
            
            foreach (var line in lines)
            {
                scrambled.AppendLine(line);
                
                // Randomly insert noise
                if (_random.Next(0, 3) == 0)
                {
                    scrambled.AppendLine("            var _temp = Thread.CurrentThread.ManagedThreadId;");
                }
            }

            scrambled.AppendLine("            if (Environment.TickCount < 0) return;");
            
            return scrambled.ToString();
        }

        public static string HideStringLiteral(string literal)
        {
            if (string.IsNullOrEmpty(literal))
                return "\"\"";

            var method = (ObfuscationMethod)_random.Next(0, 5);
            var obfuscated = ObfuscateString(literal, method);
            
            return method switch
            {
                ObfuscationMethod.Caesar => $"ObfuscationHelper.DeobfuscateString(\"{obfuscated}\", ObfuscationMethod.Caesar)",
                ObfuscationMethod.Base64 => $"Utils.Encoder.FromBase64(\"{obfuscated}\")",
                ObfuscationMethod.Custom => $"Utils.Encoder.FromCustomEncoding(\"{obfuscated}\")",
                ObfuscationMethod.XOR => $"ObfuscationHelper.DeobfuscateString(\"{obfuscated}\", ObfuscationMethod.XOR)",
                ObfuscationMethod.Reverse => $"ObfuscationHelper.DeobfuscateString(\"{obfuscated}\", ObfuscationMethod.Reverse)",
                _ => $"\"{literal}\""
            };
        }

        private static string CaesarCipher(string input, int shift)
        {
            var result = new StringBuilder();
            
            foreach (char c in input)
            {
                if (char.IsLetter(c))
                {
                    var offset = char.IsUpper(c) ? 'A' : 'a';
                    var shifted = (char)((c - offset + shift + 26) % 26 + offset);
                    result.Append(shifted);
                }
                else
                {
                    result.Append(c);
                }
            }
            
            return result.ToString();
        }

        private static string XorObfuscation(string input)
        {
            const byte key = 0x42;
            var bytes = Encoding.UTF8.GetBytes(input);
            
            for (int i = 0; i < bytes.Length; i++)
            {
                bytes[i] ^= key;
            }
            
            return Convert.ToBase64String(bytes);
        }

        private static string ReverseString(string input)
        {
            var chars = input.ToCharArray();
            Array.Reverse(chars);
            return new string(chars);
        }

        private static string GenerateRandomString(int length)
        {
            const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
            var result = new StringBuilder(length);
            
            for (int i = 0; i < length; i++)
            {
                result.Append(chars[_random.Next(chars.Length)]);
            }
            
            return result.ToString();
        }
    }

    public enum ObfuscationMethod
    {
        Caesar,
        Base64,
        Custom,
        XOR,
        Reverse
    }
}
