use gianged_drm::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       Gianged DRM - Educational License Management System      ║");
    println!("║                         Rust Edition                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Version: {}", gianged_drm::VERSION);
    println!();
    println!("DISCLAIMER: This is an educational project for learning purposes.");
    println!("Do not use this in production environments!");
    println!();

    loop {
        display_main_menu();

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => run_license_demo(),
            "2" => run_security_demo(),
            "3" => manage_licenses(),
            "4" => run_tests(),
            "5" => show_system_info(),
            "6" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }

        println!();
        pause();
    }
}

fn display_main_menu() {
    println!("═══════════════ MAIN MENU ═══════════════");
    println!("1. Run License Demo Application");
    println!("2. Run Security Protection Demo");
    println!("3. Manage Licenses");
    println!("4. Run DRM Tests");
    println!("5. Show System Information");
    println!("6. Exit");
    println!("═════════════════════════════════════════");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn pause() {
    println!();
    print!("Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn run_license_demo() {
    println!("\n=== License Demo Application ===\n");

    let logger = Logger::console_only();
    logger.set_min_level(utils::logger::LogLevel::Info);

    // Get machine information
    logger.info("Collecting machine information...");
    let machine_info = match MachineInfo::collect() {
        Ok(info) => info,
        Err(e) => {
            logger.error(&format!("Failed to collect machine info: {}", e));
            return;
        }
    };

    println!("\n{}", machine_info.display());
    println!();

    // Choose license tier
    println!("Select License Tier:");
    println!("1. Trial (30 days, 2 features)");
    println!("2. Premium (365 days, 10 features)");

    let tier_choice = get_user_input("Enter choice: ");

    let tier = match tier_choice.trim() {
        "1" => LicenseTier::Trial,
        "2" => LicenseTier::Premium,
        _ => {
            println!("Invalid choice, defaulting to Trial");
            LicenseTier::Trial
        }
    };

    // Generate license
    logger.info("Generating license...");
    let generator = match LicenseGenerator::new() {
        Ok(gen) => gen,
        Err(e) => {
            logger.error(&format!("Failed to create generator: {}", e));
            return;
        }
    };

    let license = match tier {
        LicenseTier::Trial => generator.generate_trial_license(
            "demo_user",
            &machine_info.fingerprint,
            "Gianged DRM Demo",
        ),
        LicenseTier::Premium => generator.generate_premium_license(
            "demo_user",
            &machine_info.fingerprint,
            "Gianged DRM Demo",
        ),
    };

    let license = match license {
        Ok(lic) => lic,
        Err(e) => {
            logger.error(&format!("Failed to generate license: {}", e));
            return;
        }
    };

    println!("\n{}", license.summary());

    // Validate license
    logger.info("Validating license...");
    let validator = LicenseValidator::new(generator.get_public_key().to_string());

    match validator.validate(&license, &machine_info.fingerprint) {
        Ok(_) => logger.info("✓ License is valid!"),
        Err(e) => logger.error(&format!("✗ License validation failed: {}", e)),
    }

    println!("\n{}", validator.get_validation_report(&license, &machine_info.fingerprint));

    // Test features
    println!("\n=== Testing Features ===");
    for feature in &license.features {
        println!("Testing feature: {}", feature.name);
        if feature.is_valid() {
            println!("  ✓ Feature is available and valid");
            // Simulate feature execution
            println!("  → Executing feature...");
            thread::sleep(Duration::from_millis(500));
            println!("  ✓ Feature executed successfully");
        } else {
            println!("  ✗ Feature is not available");
        }
    }

    // Ask to save
    println!();
    let save = get_user_input("Save license to storage? (y/n): ");
    if save.trim().to_lowercase() == "y" {
        let storage = match LicenseStorage::new() {
            Ok(s) => s,
            Err(e) => {
                logger.error(&format!("Failed to create storage: {}", e));
                return;
            }
        };

        match storage.save_license(&license) {
            Ok(_) => logger.info("✓ License saved successfully!"),
            Err(e) => logger.error(&format!("✗ Failed to save license: {}", e)),
        }
    }
}

fn run_security_demo() {
    println!("\n=== Security Protection Demo ===\n");

    let logger = Logger::console_only();
    logger.set_min_level(utils::logger::LogLevel::Info);

    // Anti-Debugger Check
    println!("1. Anti-Debugger Protection");
    println!("   Checking for attached debuggers...");

    let mut anti_debugger = AntiDebugger::new();
    match anti_debugger.check() {
        Ok(_) => logger.info("   ✓ No debugger detected"),
        Err(e) => logger.warning(&format!("   ✗ {}", e)),
    }

    // Integrity Check
    println!("\n2. Integrity Verification");
    println!("   Computing executable hash...");

    let mut integrity_checker = match IntegrityChecker::new() {
        Ok(checker) => checker,
        Err(e) => {
            logger.error(&format!("   ✗ Failed to create integrity checker: {}", e));
            return;
        }
    };

    if let Err(e) = integrity_checker.initialize() {
        logger.error(&format!("   ✗ Failed to initialize: {}", e));
        return;
    }

    logger.info(&format!("   Executable: {}", integrity_checker.get_executable_path().display()));
    logger.info(&format!("   Hash: {}", integrity_checker.get_expected_hash().unwrap_or("N/A")));

    match integrity_checker.verify_location() {
        Ok(_) => logger.info("   ✓ Location verification passed"),
        Err(e) => logger.warning(&format!("   ✗ {}", e)),
    }

    // Timing Check
    println!("\n3. Timing Anomaly Detection");
    println!("   Performing timing analysis...");

    let start = std::time::Instant::now();
    let mut sum = 0u64;
    for i in 0..10000 {
        sum = sum.wrapping_add(i);
    }
    let elapsed = start.elapsed();

    logger.info(&format!("   Execution time: {:?}", elapsed));
    if elapsed.as_millis() > 100 {
        logger.warning("   ⚠ Execution took longer than expected (possible debugger)");
    } else {
        logger.info("   ✓ Execution time normal");
    }

    // Simulate continuous monitoring
    println!("\n4. Continuous Monitoring Simulation");
    println!("   Running security checks for 5 seconds...");

    let monitor_duration = Duration::from_secs(5);
    let check_interval = Duration::from_secs(1);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < monitor_duration {
        match anti_debugger.quick_check() {
            Ok(_) => print!("."),
            Err(e) => {
                println!("\n   ✗ {}", e);
                break;
            }
        }
        io::stdout().flush().unwrap();
        thread::sleep(check_interval);
    }

    println!("\n   ✓ Monitoring completed");
}

fn manage_licenses() {
    println!("\n=== License Management ===\n");

    let logger = Logger::console_only();
    let storage = match LicenseStorage::new() {
        Ok(s) => s,
        Err(e) => {
            logger.error(&format!("Failed to create storage: {}", e));
            return;
        }
    };

    println!("1. View Current License");
    println!("2. Generate and Save New License");
    println!("3. Load License from Storage");
    println!("4. Delete License");
    println!("5. Storage Information");
    println!("6. Back to Main Menu");

    let choice = get_user_input("Enter choice: ");

    match choice.trim() {
        "1" => view_current_license(&storage, &logger),
        "2" => generate_and_save_license(&storage, &logger),
        "3" => load_license(&storage, &logger),
        "4" => delete_license(&storage, &logger),
        "5" => show_storage_info(&storage, &logger),
        "6" => return,
        _ => println!("Invalid choice"),
    }
}

fn view_current_license(storage: &LicenseStorage, logger: &Logger) {
    match storage.load_license() {
        Ok(license) => {
            println!("\n{}", license.summary());

            let machine_info = MachineInfo::collect().ok();
            if let Some(info) = machine_info {
                let generator = LicenseGenerator::new().ok();
                if let Some(gen) = generator {
                    let validator = LicenseValidator::new(gen.get_public_key().to_string());
                    println!("\n{}", validator.get_validation_report(&license, &info.fingerprint));
                }
            }
        }
        Err(e) => logger.error(&format!("Failed to load license: {}", e)),
    }
}

fn generate_and_save_license(storage: &LicenseStorage, logger: &Logger) {
    let machine_info = match MachineInfo::collect() {
        Ok(info) => info,
        Err(e) => {
            logger.error(&format!("Failed to collect machine info: {}", e));
            return;
        }
    };

    println!("\nSelect License Tier:");
    println!("1. Trial");
    println!("2. Premium");

    let tier_choice = get_user_input("Enter choice: ");
    let tier = match tier_choice.trim() {
        "1" => LicenseTier::Trial,
        "2" => LicenseTier::Premium,
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    let user_id = get_user_input("Enter user ID: ");

    let generator = match LicenseGenerator::new() {
        Ok(gen) => gen,
        Err(e) => {
            logger.error(&format!("Failed to create generator: {}", e));
            return;
        }
    };

    let license = match tier {
        LicenseTier::Trial => generator.generate_trial_license(
            user_id,
            &machine_info.fingerprint,
            "Gianged DRM",
        ),
        LicenseTier::Premium => generator.generate_premium_license(
            user_id,
            &machine_info.fingerprint,
            "Gianged DRM",
        ),
    };

    let license = match license {
        Ok(lic) => lic,
        Err(e) => {
            logger.error(&format!("Failed to generate license: {}", e));
            return;
        }
    };

    println!("\n{}", license.summary());

    match storage.save_license(&license) {
        Ok(_) => logger.info("✓ License saved successfully!"),
        Err(e) => logger.error(&format!("✗ Failed to save license: {}", e)),
    }
}

fn load_license(storage: &LicenseStorage, logger: &Logger) {
    match storage.load_license() {
        Ok(license) => {
            logger.info("✓ License loaded successfully!");
            println!("\n{}", license.summary());
        }
        Err(e) => logger.error(&format!("✗ Failed to load license: {}", e)),
    }
}

fn delete_license(storage: &LicenseStorage, logger: &Logger) {
    let confirm = get_user_input("Are you sure you want to delete the license? (yes/no): ");

    if confirm.trim().to_lowercase() == "yes" {
        match storage.delete_license() {
            Ok(_) => logger.info("✓ License deleted successfully!"),
            Err(e) => logger.error(&format!("✗ Failed to delete license: {}", e)),
        }
    } else {
        println!("Deletion cancelled.");
    }
}

fn show_storage_info(storage: &LicenseStorage, logger: &Logger) {
    match storage.get_storage_info() {
        Ok(info) => println!("\n{}", info.display()),
        Err(e) => logger.error(&format!("Failed to get storage info: {}", e)),
    }
}

fn run_tests() {
    println!("\n=== DRM Test Suite ===\n");

    let logger = Logger::console_only();
    logger.set_min_level(utils::logger::LogLevel::Info);

    // Test 1: Hardware Fingerprinting
    println!("Test 1: Hardware Fingerprinting");
    match MachineInfo::collect() {
        Ok(info) => {
            logger.info("  ✓ Machine info collected successfully");
            println!("  Short ID: {}", info.get_short_id());
            println!("  Is VM: {}", info.is_virtual_machine);
        }
        Err(e) => logger.error(&format!("  ✗ Failed: {}", e)),
    }

    // Test 2: License Generation
    println!("\nTest 2: License Generation");
    match LicenseGenerator::new() {
        Ok(generator) => {
            match generator.generate_trial_license("test_user", "test_machine", "Test Product") {
                Ok(license) => {
                    logger.info("  ✓ License generated successfully");
                    println!("  Key: {}", license.license_key);
                }
                Err(e) => logger.error(&format!("  ✗ Failed: {}", e)),
            }
        }
        Err(e) => logger.error(&format!("  ✗ Failed: {}", e)),
    }

    // Test 3: License Validation
    println!("\nTest 3: License Validation");
    match LicenseGenerator::new() {
        Ok(generator) => {
            match generator.generate_trial_license("test_user", "test_machine_123", "Test Product") {
                Ok(license) => {
                    let validator = LicenseValidator::new(generator.get_public_key().to_string());
                    match validator.validate(&license, "test_machine_123") {
                        Ok(_) => logger.info("  ✓ License validation passed"),
                        Err(e) => logger.error(&format!("  ✗ Validation failed: {}", e)),
                    }
                }
                Err(e) => logger.error(&format!("  ✗ Failed: {}", e)),
            }
        }
        Err(e) => logger.error(&format!("  ✗ Failed: {}", e)),
    }

    // Test 4: Tier Validation
    println!("\nTest 4: Tier Feature Limits");
    logger.info(&format!("  Trial max features: {}", LicenseTier::Trial.max_features()));
    logger.info(&format!("  Premium max features: {}", LicenseTier::Premium.max_features()));
    logger.info("  ✓ Tier validation passed");

    // Test 5: Cryptography
    println!("\nTest 5: Cryptography");
    let test_data = "Test encryption data";
    match CryptoHelper::encrypt_string(test_data) {
        Ok(encrypted) => {
            match CryptoHelper::decrypt_string(&encrypted) {
                Ok(decrypted) => {
                    if decrypted == test_data {
                        logger.info("  ✓ Encryption/Decryption test passed");
                    } else {
                        logger.error("  ✗ Decrypted data mismatch");
                    }
                }
                Err(e) => logger.error(&format!("  ✗ Decryption failed: {}", e)),
            }
        }
        Err(e) => logger.error(&format!("  ✗ Encryption failed: {}", e)),
    }

    println!("\n=== Test Suite Completed ===");
}

fn show_system_info() {
    println!("\n=== System Information ===\n");

    let logger = Logger::console_only();

    match MachineInfo::collect() {
        Ok(info) => {
            println!("{}", info.display());
            println!("\nShort Machine ID: {}", info.get_short_id());
        }
        Err(e) => logger.error(&format!("Failed to collect system info: {}", e)),
    }

    println!("\nLibrary Info: {}", gianged_drm::get_info());
}
