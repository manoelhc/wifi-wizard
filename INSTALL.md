# Build and Installation Guide

## System Requirements

### Operating System
- Linux (Ubuntu 20.04+, Fedora 35+, Arch Linux, or other modern distributions)
- SystemD-based system with NetworkManager

### Required System Packages

#### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libgtk-4-dev \
    libglib2.0-dev \
    network-manager \
    dbus
```

#### Fedora
```bash
sudo dnf install -y \
    gcc \
    pkg-config \
    gtk4-devel \
    glib2-devel \
    NetworkManager \
    dbus
```

#### Arch Linux
```bash
sudo pacman -S --needed \
    base-devel \
    gtk4 \
    glib2 \
    networkmanager \
    dbus
```

### Rust Toolchain

Install Rust using rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Building the Application

1. Clone the repository:
```bash
git clone <your-repository-url>
cd wifi-wizard
```

2. Build the application:
```bash
cargo build --release
```

The compiled binary will be located at `target/release/wifi-wizard`.

## Running the Application

### Prerequisites
1. Ensure NetworkManager is running:
```bash
sudo systemctl status NetworkManager
```

2. Ensure your user has D-Bus access to NetworkManager. Most desktop users have this by default. You can verify by running:
```bash
dbus-send --system --print-reply --dest=org.freedesktop.NetworkManager \
    /org/freedesktop/NetworkManager \
    org.freedesktop.DBus.Properties.Get \
    string:org.freedesktop.NetworkManager \
    string:Version
```

### Running

From the build directory:
```bash
cargo run --release
```

Or directly execute the binary:
```bash
./target/release/wifi-wizard
```

## User Permissions

If you encounter permission errors when trying to connect to networks, you may need to:

1. Add your user to the `netdev` group (Debian/Ubuntu):
```bash
sudo usermod -aG netdev $USER
```

2. Or add to the `network` group (some distributions):
```bash
sudo usermod -aG network $USER
```

3. Log out and log back in for group changes to take effect.

## Troubleshooting

### "Failed to connect to system D-Bus"
- Ensure D-Bus service is running: `sudo systemctl status dbus`
- Check if you have permission to access the system bus

### "Failed to create NetworkManager client"
- Verify NetworkManager is running: `sudo systemctl status NetworkManager`
- Check NetworkManager version: `nmcli --version` (should be 1.0 or later)

### "No WiFi device found"
- Ensure your WiFi adapter is detected: `nmcli device status`
- Check if the adapter is managed by NetworkManager

### Build errors related to GTK4
- Ensure all system development packages are installed
- Update pkg-config cache: `sudo ldconfig`

## Testing Without Full-Screen

If you want to test the application without full-screen mode during development, you can modify `src/ui.rs` and comment out the line:
```rust
// window.fullscreen();
```

## Uninstallation

To remove the application:
```bash
cargo clean
```

Or simply delete the project directory.
