# Wi-Fi Password Brute-Force Tester

A command-line tool to test the strength of Wi-Fi passwords using a list of common weak passwords.

### Prerequisites

A Linux system with NetworkManager installed.
The nmcli command-line tool.
Access to the terminal.

### Installation

Clone this repository:

```
git clone <repository-url>
cd <repository-folder>
```

Ensure you have a file named passwordList.txt located in the src directory containing the list of common weak passwords. Each password should be on a new line.

### Usage

Open a Terminal:

Open your terminal application on your Linux system.

Navigate to the Project Directory:

```
cd <path-to-your-project>
```

Run the Tool: Execute the following command:

```
cargo run
```

Scan for Available Wi-Fi Networks: The tool will automatically scan for available Wi-Fi networks and display them.

Enter the Wi-Fi SSID: When prompted, enter the SSID (name) of the Wi-Fi network you want to test.

Wait for Results: The tool will attempt to connect to the Wi-Fi network using passwords from the list. It will display attempts and notify you of any successful connections or incorrect passwords.

### Notes

This tool is for educational purposes only. Do not use it against networks without permission.
Ensure you have the necessary permissions to connect to the Wi-Fi network being tested.

### Troubleshooting

If you encounter issues, ensure you have sudo privileges to allow network modifications.
Check if nmcli is installed by running:

```
nmcli --version
```
