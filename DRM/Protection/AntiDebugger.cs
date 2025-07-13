using System.Diagnostics;
using System.Runtime.InteropServices;

namespace DRM.Protection
{
    public class AntiDebugger
    {
        [DllImport("kernel32.dll", SetLastError = true, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool CheckRemoteDebuggerPresent(IntPtr hProcess, [MarshalAs(UnmanagedType.Bool)] ref bool isDebuggerPresent);

        [DllImport("kernel32.dll", SetLastError = true, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        private static extern bool IsDebuggerPresent();

        [DllImport("kernel32.dll")]
        private static extern IntPtr GetCurrentProcess();

        public static bool IsDebuggerAttached()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                return Debugger.IsAttached || IsDebuggerPresent();
            }
            return Debugger.IsAttached;
        }

        public static bool IsRemoteDebuggerPresent()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                try
                {
                    bool isDebuggerPresent = false;
                    CheckRemoteDebuggerPresent(GetCurrentProcess(), ref isDebuggerPresent);
                    return isDebuggerPresent;
                }
                catch
                {
                    return false;
                }
            }
            return false;
        }

        public static bool IsProcessBeingDebugged()
        {
            return IsDebuggerAttached() || IsRemoteDebuggerPresent() || IsDebuggerProcessRunning();
        }

        private static bool IsDebuggerProcessRunning()
        {
            string[] debuggerProcessNames = {
                "ollydbg", "ida", "ida64", "idag", "idag64", "idaw", "idaw64",
                "idaq", "idaq64", "idau", "idau64", "scylla", "scylla_x64",
                "scylla_x86", "protection_id", "x64dbg", "x32dbg", "windbg",
                "reshacker", "ImportREC", "IMMUNITYDEBUGGER", "MegaDumper"
            };

            try
            {
                Process[] processes = Process.GetProcesses();
                foreach (Process process in processes)
                {
                    try
                    {
                        string processName = process.ProcessName.ToLower();
                        if (debuggerProcessNames.Any(debugger => processName.Contains(debugger)))
                        {
                            return true;
                        }
                    }
                    catch
                    {
                        // Ignore access denied exceptions
                    }
                }
            }
            catch
            {
                // If we can't enumerate processes, assume no debugger
            }

            return false;
        }

        public static void PerformAntiDebugCheck()
        {
            try
            {
                if (IsProcessBeingDebugged())
                {
                    // Exit immediately if debugger is detected
                    Environment.Exit(1);
                }
            }
            catch (Exception)
            {
                // If anti-debug check fails, continue (don't break the application)
                // In production, you might want to be more strict
            }
        }

        public static bool CheckDebuggerWithTiming()
        {
            var sw = Stopwatch.StartNew();
            
            // Perform a simple operation
            int dummy = 0;
            for (int i = 0; i < 1000; i++)
            {
                dummy += i;
            }
            
            sw.Stop();
            
            // If execution took too long, likely being debugged
            return sw.ElapsedMilliseconds > 10;
        }

        public static void StartAntiDebugMonitoring()
        {
            Task.Run(async () =>
            {
                while (true)
                {
                    if (IsProcessBeingDebugged() || CheckDebuggerWithTiming())
                    {
                        Environment.Exit(1);
                    }
                    await Task.Delay(1000); // Check every second
                }
            });
        }
    }
}
