using System.Text;

namespace DRM.Utils
{
    public static class Encoder
    {
        private const string CUSTOM_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        private const string OBFUSCATED_ALPHABET = "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm0987654321+/";

        public static string ToBase64(string input)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            var bytes = Encoding.UTF8.GetBytes(input);
            return Convert.ToBase64String(bytes);
        }

        public static string FromBase64(string input)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            try
            {
                var bytes = Convert.FromBase64String(input);
                return Encoding.UTF8.GetString(bytes);
            }
            catch
            {
                return string.Empty;
            }
        }

        public static string ToBase64UrlSafe(string input)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            var base64 = ToBase64(input);
            return base64.Replace('+', '-').Replace('/', '_').TrimEnd('=');
        }

        public static string FromBase64UrlSafe(string input)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            var base64 = input.Replace('-', '+').Replace('_', '/');
            
            // Add padding if needed
            var padding = base64.Length % 4;
            if (padding > 0)
                base64 = base64.PadRight(base64.Length + (4 - padding), '=');

            return FromBase64(base64);
        }

        public static string ToCustomEncoding(string input)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            var base64 = ToBase64(input);
            var result = new StringBuilder(base64.Length);

            foreach (var c in base64)
            {
                var index = CUSTOM_ALPHABET.IndexOf(c);
                if (index >= 0)
                    result.Append(OBFUSCATED_ALPHABET[index]);
                else
                    result.Append(c);
            }

            return result.ToString();
        }

        public static string FromCustomEncoding(string input)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            var result = new StringBuilder(input.Length);

            foreach (var c in input)
            {
                var index = OBFUSCATED_ALPHABET.IndexOf(c);
                if (index >= 0)
                    result.Append(CUSTOM_ALPHABET[index]);
                else
                    result.Append(c);
            }

            return FromBase64(result.ToString());
        }

        public static string ToHex(byte[] bytes)
        {
            if (bytes == null || bytes.Length == 0)
                return string.Empty;

            return Convert.ToHexString(bytes).ToLowerInvariant();
        }

        public static byte[] FromHex(string hex)
        {
            if (string.IsNullOrEmpty(hex) || hex.Length % 2 != 0)
                return Array.Empty<byte>();

            try
            {
                return Convert.FromHexString(hex);
            }
            catch
            {
                return Array.Empty<byte>();
            }
        }

        public static string ObfuscateString(string input, int shift = 7)
        {
            if (string.IsNullOrEmpty(input))
                return string.Empty;

            var result = new StringBuilder(input.Length);
            
            foreach (var c in input)
            {
                if (char.IsLetter(c))
                {
                    var offset = char.IsUpper(c) ? 'A' : 'a';
                    var shifted = (c - offset + shift) % 26 + offset;
                    result.Append((char)shifted);
                }
                else if (char.IsDigit(c))
                {
                    var shifted = (c - '0' + shift) % 10 + '0';
                    result.Append((char)shifted);
                }
                else
                {
                    result.Append(c);
                }
            }

            return result.ToString();
        }

        public static string DeobfuscateString(string input, int shift = 7)
        {
            return ObfuscateString(input, 26 - shift);
        }
    }
}
