# Quick Start Guide

Get WiFi Wizard up and running in minutes!

## 🚀 Quick Installation (Ubuntu/Debian)

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y libgtk-4-dev build-essential pkg-config

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone <repository-url>
cd wifi-wizard
cargo build --release

# Run
./target/release/wifi-wizard
```

## 🎯 Quick Usage

### Connect to a visible network:
1. Click **"Scan Networks"**
2. Find your network in the list
3. Click **"Connect"**
4. Enter password (if secured)
5. Done! ✓

### Connect to a hidden network:
1. Switch to **"Hidden Network"** tab
2. Enter network name (SSID)
3. Enter password (if needed)
4. Click **"Connect to Hidden Network"**
5. Done! ✓

## 📋 Requirements

- **OS**: Linux with NetworkManager
- **Packages**: GTK4, GLib2, NetworkManager
- **Rust**: 1.70 or later

## 🛠️ Troubleshooting

### Permission issues?
```bash
sudo usermod -aG netdev $USER
# Log out and back in
```

### NetworkManager not running?
```bash
sudo systemctl start NetworkManager
```

### Can't build?
Make sure all dependencies are installed:
```bash
# Ubuntu/Debian
sudo apt-get install libgtk-4-dev libglib2.0-dev

# Fedora
sudo dnf install gtk4-devel glib2-devel

# Arch
sudo pacman -S gtk4 glib2
```

## 📚 More Information

- Full installation guide: `INSTALL.md`
- Usage documentation: `USAGE.md`
- Security info: `SECURITY.md`
- Contributing: `CONTRIBUTING.md`

## ⚡ Features

✓ Full-screen GTK4 interface  
✓ Scan and display WiFi networks  
✓ Connect to secured networks  
✓ Support for hidden networks  
✓ Signal strength indicators  
✓ Persistent connection storage  
✓ Linux/NetworkManager integration  

Enjoy your WiFi Wizard! 🧙‍♂️
