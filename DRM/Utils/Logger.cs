using System.Text;
using System.Text.RegularExpressions;

namespace DRM.Utils
{
    public enum LogLevel
    {
        Debug,
        Info,
        Warning,
        Error,
        Critical
    }

    public class Logger
    {
        private static readonly object _lockObject = new object();
        private static Logger? _instance;
        private readonly string _logDirectory;
        private readonly string _logFileName;
        private readonly int _maxLogFiles;
        private readonly long _maxFileSize;

        private readonly string[] _sensitivePatterns = new[]
        {
            @"\b[A-Za-z0-9+/]{20,}={0,2}\b",  // Base64 patterns
            @"\b[0-9a-fA-F]{32,}\b",          // Hex patterns (likely keys/hashes)
            @"password|key|secret|token",      // Sensitive keywords
            @"\b\d{4}-\d{4}-\d{4}-\d{4}\b"    // License key patterns
        };

        private Logger(string logDirectory = "", int maxLogFiles = 10, long maxFileSize = 10 * 1024 * 1024)
        {
            _logDirectory = string.IsNullOrEmpty(logDirectory) 
                ? Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), "DRM", "Logs")
                : logDirectory;
            
            _logFileName = "drm_{0:yyyyMMdd}.log";
            _maxLogFiles = maxLogFiles;
            _maxFileSize = maxFileSize;

            Directory.CreateDirectory(_logDirectory);
        }

        public static Logger Instance
        {
            get
            {
                if (_instance == null)
                {
                    lock (_lockObject)
                    {
                        _instance ??= new Logger();
                    }
                }
                return _instance;
            }
        }

        public void Log(LogLevel level, string message, Exception? exception = null)
        {
            var sanitizedMessage = SanitizeMessage(message);
            var logEntry = FormatLogEntry(level, sanitizedMessage, exception);

            lock (_lockObject)
            {
                WriteToFile(logEntry);
                WriteToConsole(level, sanitizedMessage);
            }
        }

        public void Debug(string message) => Log(LogLevel.Debug, message);
        public void Info(string message) => Log(LogLevel.Info, message);
        public void Warning(string message) => Log(LogLevel.Warning, message);
        public void Error(string message, Exception? exception = null) => Log(LogLevel.Error, message, exception);
        public void Critical(string message, Exception? exception = null) => Log(LogLevel.Critical, message, exception);

        private string SanitizeMessage(string message)
        {
            if (string.IsNullOrEmpty(message))
                return message;

            var sanitized = message;

            foreach (var pattern in _sensitivePatterns)
            {
                sanitized = Regex.Replace(sanitized, pattern, 
                    match => new string('*', Math.Min(match.Length, 8)) + "[REDACTED]", 
                    RegexOptions.IgnoreCase);
            }

            return sanitized;
        }

        private string FormatLogEntry(LogLevel level, string message, Exception? exception)
        {
            var timestamp = DateTime.UtcNow.ToString("yyyy-MM-dd HH:mm:ss.fff");
            var threadId = Thread.CurrentThread.ManagedThreadId;
            
            var logBuilder = new StringBuilder();
            logBuilder.AppendLine($"[{timestamp}] [{level}] [Thread:{threadId}] {message}");

            if (exception != null)
            {
                logBuilder.AppendLine($"Exception: {exception.GetType().Name}: {exception.Message}");
                logBuilder.AppendLine($"StackTrace: {exception.StackTrace}");
                
                var inner = exception.InnerException;
                while (inner != null)
                {
                    logBuilder.AppendLine($"Inner Exception: {inner.GetType().Name}: {inner.Message}");
                    inner = inner.InnerException;
                }
            }

            return logBuilder.ToString();
        }

        private void WriteToFile(string logEntry)
        {
            try
            {
                var currentLogFile = Path.Combine(_logDirectory, string.Format(_logFileName, DateTime.UtcNow));
                
                // Check if file needs rotation
                if (File.Exists(currentLogFile) && new FileInfo(currentLogFile).Length > _maxFileSize)
                {
                    RotateLogFiles();
                }

                File.AppendAllText(currentLogFile, logEntry, Encoding.UTF8);
            }
            catch
            {
                // Silent failure for logging to prevent infinite loops
            }
        }

        private void WriteToConsole(LogLevel level, string message)
        {
            var originalColor = Console.ForegroundColor;
            
            Console.ForegroundColor = level switch
            {
                LogLevel.Debug => ConsoleColor.Gray,
                LogLevel.Info => ConsoleColor.White,
                LogLevel.Warning => ConsoleColor.Yellow,
                LogLevel.Error => ConsoleColor.Red,
                LogLevel.Critical => ConsoleColor.Magenta,
                _ => ConsoleColor.White
            };

            Console.WriteLine($"[{DateTime.Now:HH:mm:ss}] [{level}] {message}");
            Console.ForegroundColor = originalColor;
        }

        private void RotateLogFiles()
        {
            try
            {
                var logFiles = Directory.GetFiles(_logDirectory, "drm_*.log")
                    .OrderByDescending(f => new FileInfo(f).CreationTime)
                    .Skip(_maxLogFiles - 1)
                    .ToArray();

                foreach (var file in logFiles)
                {
                    File.Delete(file);
                }
            }
            catch
            {
                // Silent failure for log rotation
            }
        }

        public void ClearLogs()
        {
            lock (_lockObject)
            {
                try
                {
                    var logFiles = Directory.GetFiles(_logDirectory, "drm_*.log");
                    foreach (var file in logFiles)
                    {
                        File.Delete(file);
                    }
                }
                catch
                {
                    // Silent failure
                }
            }
        }

        public string[] GetRecentLogs(int count = 100)
        {
            lock (_lockObject)
            {
                try
                {
                    var currentLogFile = Path.Combine(_logDirectory, string.Format(_logFileName, DateTime.UtcNow));
                    if (!File.Exists(currentLogFile))
                        return Array.Empty<string>();

                    var lines = File.ReadAllLines(currentLogFile);
                    return lines.TakeLast(count).ToArray();
                }
                catch
                {
                    return Array.Empty<string>();
                }
            }
        }
    }
}
