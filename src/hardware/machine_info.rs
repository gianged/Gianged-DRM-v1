use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use sysinfo::System;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MachineInfoError {
    #[error("Failed to retrieve machine information: {0}")]
    RetrievalError(String),

    #[error("Failed to get MAC address: {0}")]
    MacAddressError(String),

    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),
}

pub type MachineInfoResult<T> = Result<T, MachineInfoError>;

/// Information about the machine hardware and environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineInfo {
    /// CPU identifier
    pub cpu_id: String,

    /// Motherboard serial number (platform-specific)
    pub motherboard_serial: String,

    /// MAC addresses of network interfaces
    pub mac_addresses: Vec<String>,

    /// Machine name/hostname
    pub machine_name: String,

    /// Operating system version
    pub os_version: String,

    /// Whether running in a virtual machine
    pub is_virtual_machine: bool,

    /// Composite hardware fingerprint (SHA-256 hash)
    pub fingerprint: String,
}

impl MachineInfo {
    /// Collect machine information and generate fingerprint
    pub fn collect() -> MachineInfoResult<Self> {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_id = Self::get_cpu_id(&sys)?;
        let motherboard_serial = Self::get_motherboard_serial()?;
        let mac_addresses = Self::get_mac_addresses()?;
        let machine_name = Self::get_machine_name(&sys);
        let os_version = Self::get_os_version(&sys);
        let is_virtual_machine = Self::detect_virtual_machine(&sys);

        // Generate composite fingerprint
        let fingerprint = Self::generate_fingerprint(
            &cpu_id,
            &motherboard_serial,
            &mac_addresses,
            &machine_name,
            &os_version,
        );

        Ok(Self {
            cpu_id,
            motherboard_serial,
            mac_addresses,
            machine_name,
            os_version,
            is_virtual_machine,
            fingerprint,
        })
    }

    /// Get a short machine ID (first 16 characters of fingerprint)
    pub fn get_short_id(&self) -> String {
        self.fingerprint.chars().take(16).collect()
    }

    /// Check if the current machine matches this fingerprint
    pub fn matches(&self, other_fingerprint: &str) -> bool {
        self.fingerprint == other_fingerprint
    }

    /// Display machine information
    pub fn display(&self) -> String {
        format!(
            "Machine Information:\n\
             - CPU ID: {}\n\
             - Motherboard: {}\n\
             - MAC Addresses: {}\n\
             - Machine Name: {}\n\
             - OS Version: {}\n\
             - Virtual Machine: {}\n\
             - Fingerprint: {}",
            self.cpu_id,
            self.motherboard_serial,
            self.mac_addresses.join(", "),
            self.machine_name,
            self.os_version,
            self.is_virtual_machine,
            self.fingerprint
        )
    }

    /// Get CPU ID from system information
    fn get_cpu_id(sys: &System) -> MachineInfoResult<String> {
        if let Some(cpu) = sys.cpus().first() {
            let brand = cpu.brand();
            Ok(brand.to_string())
        } else {
            Ok("UNKNOWN_CPU".to_string())
        }
    }

    /// Get motherboard serial number (platform-specific)
    fn get_motherboard_serial() -> MachineInfoResult<String> {
        #[cfg(target_os = "windows")]
        {
            Self::get_motherboard_serial_windows()
        }

        #[cfg(target_os = "linux")]
        {
            Self::get_motherboard_serial_linux()
        }

        #[cfg(target_os = "macos")]
        {
            Self::get_motherboard_serial_macos()
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Ok("UNKNOWN_BOARD".to_string())
        }
    }

    #[cfg(target_os = "windows")]
    fn get_motherboard_serial_windows() -> MachineInfoResult<String> {
        use std::process::Command;

        let output = Command::new("wmic")
            .args(&["baseboard", "get", "serialnumber"])
            .output()
            .map_err(|e| MachineInfoError::RetrievalError(format!("WMIC command failed: {}", e)))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            if lines.len() > 1 {
                let serial = lines[1].trim();
                if !serial.is_empty() {
                    return Ok(serial.to_string());
                }
            }
        }

        Ok("UNKNOWN_BOARD".to_string())
    }

    #[cfg(target_os = "linux")]
    fn get_motherboard_serial_linux() -> MachineInfoResult<String> {
        use std::fs;

        // Try to read from DMI
        let paths = [
            "/sys/class/dmi/id/board_serial",
            "/sys/class/dmi/id/product_serial",
        ];

        for path in &paths {
            if let Ok(serial) = fs::read_to_string(path) {
                let serial = serial.trim();
                if !serial.is_empty() && serial != "None" {
                    return Ok(serial.to_string());
                }
            }
        }

        Ok("UNKNOWN_BOARD".to_string())
    }

    #[cfg(target_os = "macos")]
    fn get_motherboard_serial_macos() -> MachineInfoResult<String> {
        use std::process::Command;

        let output = Command::new("system_profiler")
            .args(&["SPHardwareDataType"])
            .output()
            .map_err(|e| MachineInfoError::RetrievalError(format!("system_profiler failed: {}", e)))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("Serial Number") {
                    if let Some(serial) = line.split(':').nth(1) {
                        return Ok(serial.trim().to_string());
                    }
                }
            }
        }

        Ok("UNKNOWN_BOARD".to_string())
    }

    /// Get all MAC addresses from network interfaces
    fn get_mac_addresses() -> MachineInfoResult<Vec<String>> {
        let mut mac_addrs = Vec::new();

        match mac_address::get_mac_address() {
            Ok(Some(addr)) => {
                mac_addrs.push(addr.to_string());
            }
            _ => {}
        }

        // Try to get all MAC addresses
        if let Ok(interfaces) = mac_address::mac_address_by_name("") {
            if let Some(addr) = interfaces {
                let addr_str = addr.to_string();
                if !mac_addrs.contains(&addr_str) {
                    mac_addrs.push(addr_str);
                }
            }
        }

        if mac_addrs.is_empty() {
            mac_addrs.push("00:00:00:00:00:00".to_string());
        }

        Ok(mac_addrs)
    }

    /// Get machine name/hostname
    fn get_machine_name(sys: &System) -> String {
        sys.host_name().unwrap_or_else(|| "UNKNOWN_HOST".to_string())
    }

    /// Get operating system version
    fn get_os_version(sys: &System) -> String {
        let os_name = sys.name().unwrap_or_else(|| "Unknown OS".to_string());
        let os_version = sys.os_version().unwrap_or_else(|| "Unknown Version".to_string());
        let kernel_version = sys.kernel_version().unwrap_or_else(|| "Unknown Kernel".to_string());

        format!("{} {} ({})", os_name, os_version, kernel_version)
    }

    /// Detect if running in a virtual machine
    fn detect_virtual_machine(sys: &System) -> bool {
        let mut is_vm = false;

        // Check CPU brand for VM indicators
        for cpu in sys.cpus() {
            let brand = cpu.brand().to_uppercase();
            if brand.contains("VBOX")
                || brand.contains("VMWARE")
                || brand.contains("VIRTUAL")
                || brand.contains("XEN")
                || brand.contains("QEMU")
                || brand.contains("HYPER-V")
                || brand.contains("PARALLELS")
            {
                is_vm = true;
                break;
            }
        }

        // Check system information
        if !is_vm {
            if let Some(name) = sys.name() {
                let name_upper = name.to_uppercase();
                if name_upper.contains("VIRTUAL")
                    || name_upper.contains("VMWARE")
                    || name_upper.contains("VBOX")
                {
                    is_vm = true;
                }
            }
        }

        // Platform-specific checks
        #[cfg(target_os = "windows")]
        {
            is_vm = is_vm || Self::detect_vm_windows();
        }

        #[cfg(target_os = "linux")]
        {
            is_vm = is_vm || Self::detect_vm_linux();
        }

        is_vm
    }

    #[cfg(target_os = "windows")]
    fn detect_vm_windows() -> bool {
        use std::process::Command;

        // Check using systeminfo command
        if let Ok(output) = Command::new("systeminfo").output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_uppercase();
                if stdout.contains("VIRTUAL")
                    || stdout.contains("VMWARE")
                    || stdout.contains("VBOX")
                    || stdout.contains("HYPER-V")
                {
                    return true;
                }
            }
        }

        false
    }

    #[cfg(target_os = "linux")]
    fn detect_vm_linux() -> bool {
        use std::fs;

        // Check DMI information
        if let Ok(product_name) = fs::read_to_string("/sys/class/dmi/id/product_name") {
            let name_upper = product_name.to_uppercase();
            if name_upper.contains("VIRTUAL")
                || name_upper.contains("VMWARE")
                || name_upper.contains("VBOX")
                || name_upper.contains("KVM")
                || name_upper.contains("QEMU")
            {
                return true;
            }
        }

        // Check for hypervisor CPU flag
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            if cpuinfo.contains("hypervisor") {
                return true;
            }
        }

        false
    }

    /// Generate a composite fingerprint from all hardware identifiers
    fn generate_fingerprint(
        cpu_id: &str,
        motherboard: &str,
        mac_addresses: &[String],
        machine_name: &str,
        os_version: &str,
    ) -> String {
        let mut hasher = Sha256::new();

        hasher.update(cpu_id.as_bytes());
        hasher.update(motherboard.as_bytes());

        for mac in mac_addresses {
            hasher.update(mac.as_bytes());
        }

        hasher.update(machine_name.as_bytes());
        hasher.update(os_version.as_bytes());

        let result = hasher.finalize();
        hex::encode(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_info_collection() {
        let info = MachineInfo::collect();
        assert!(info.is_ok());

        let info = info.unwrap();
        assert!(!info.cpu_id.is_empty());
        assert!(!info.fingerprint.is_empty());
        assert_eq!(info.fingerprint.len(), 64); // SHA-256 hex = 64 chars
    }

    #[test]
    fn test_short_id() {
        let info = MachineInfo::collect().unwrap();
        let short_id = info.get_short_id();
        assert_eq!(short_id.len(), 16);
    }

    #[test]
    fn test_fingerprint_consistency() {
        let info1 = MachineInfo::collect().unwrap();
        let info2 = MachineInfo::collect().unwrap();

        // Fingerprints should be consistent
        assert_eq!(info1.fingerprint, info2.fingerprint);
    }

    #[test]
    fn test_fingerprint_matching() {
        let info = MachineInfo::collect().unwrap();
        assert!(info.matches(&info.fingerprint));
        assert!(!info.matches("invalid_fingerprint"));
    }
}
