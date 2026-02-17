# WiFi Wizard

A full-screen GTK4 application for managing WiFi connections on Linux.

## Features

- **List Available Networks**: Scan and display all available WiFi networks with signal strength indicators
- **Hidden Network Support**: Connect to hidden WiFi networks by manually entering the SSID
- **Secure Connections**: Support for password-protected networks (WPA/WPA2)
- **Full-Screen Interface**: Immersive full-screen GTK4 user interface
- **NetworkManager Integration**: Uses NetworkManager D-Bus API to manage connections
- **Linux-Only**: Designed specifically for Linux systems with NetworkManager

## Requirements

- Linux operating system
- NetworkManager installed and running
- GTK4 development libraries
- Rust toolchain (1.70 or later)
- D-Bus access to NetworkManager

### System Dependencies

Install the required system dependencies:

**Ubuntu/Debian:**
```bash
sudo apt-get install libgtk-4-dev build-essential pkg-config network-manager
```

**Fedora:**
```bash
sudo dnf install gtk4-devel gcc pkg-config NetworkManager
```

**Arch Linux:**
```bash
sudo pacman -S gtk4 base-devel networkmanager
```

## Building

```bash
cargo build --release
```

## Running

The application requires appropriate permissions to interact with NetworkManager. You can run it with:

```bash
cargo run --release
```

Or run the compiled binary:

```bash
./target/release/wifi-wizard
```

**Note:** If you encounter permission issues, you may need to:
1. Ensure your user is in the `netdev` or `network` group
2. Or run with appropriate privileges (though this is not recommended for production use)

## Usage

1. **Scanning Networks**: Click the "Scan Networks" button in the "Available Networks" tab to discover nearby WiFi networks
2. **Connecting to Visible Networks**: Click "Connect" next to any network. If the network is secured, you'll be prompted for a password
3. **Connecting to Hidden Networks**: Switch to the "Hidden Network" tab, enter the SSID and password (if required), then click "Connect to Hidden Network"
4. **Exit**: Click the "Exit" button to close the application

## Architecture

The application is built with:
- **GTK4**: Modern GNOME toolkit for the user interface
- **zbus**: D-Bus communication with NetworkManager
- **tokio**: Asynchronous runtime for non-blocking operations
- **NetworkManager**: System service for managing network connections

## Security

- Passwords are transmitted securely to NetworkManager via D-Bus
- Connection settings are saved by NetworkManager in the system configuration
- The application follows Linux security best practices for network management

## License

See LICENSE file for details.

## Contributing

Contributions are welcome! Please ensure that your code:
- Follows Rust best practices
- Maintains Linux-only compatibility
- Works with NetworkManager
- Preserves the full-screen GTK4 interface