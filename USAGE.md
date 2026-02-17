# Usage Guide

This guide provides detailed instructions on how to use the WiFi Wizard application.

## Starting the Application

Run the application from the terminal:
```bash
wifi-wizard
```

Or if running from the build directory:
```bash
cargo run --release
```

The application will start in full-screen mode.

## Main Interface

The WiFi Wizard interface has two main tabs:
1. **Available Networks** - Scan and connect to visible WiFi networks
2. **Hidden Network** - Connect to hidden networks by SSID

### Navigation
- Use the tab switcher at the top to switch between views
- Click "Exit" button to close the application
- Press `F11` or `Esc` to exit full-screen mode (if supported by window manager)

## Connecting to Visible Networks

### Step 1: Scan for Networks
1. Switch to the "Available Networks" tab
2. Click the "Scan Networks" button
3. Wait a few seconds for the scan to complete

### Step 2: View Available Networks
After scanning, you'll see a list of networks showing:
- **Network Name (SSID)** - The name of the WiFi network
- **Signal Strength** - Percentage (0-100%)
- **Security Status** - Lock icon (🔒) indicates a secured network

### Step 3: Connect
1. Find the network you want to connect to
2. Click the "Connect" button next to the network name
3. If the network is secured:
   - A password dialog will appear
   - Enter the WiFi password
   - Click "Connect"
4. Wait for the connection to establish
5. A success message will confirm when connected

## Connecting to Hidden Networks

Hidden networks don't broadcast their SSID and won't appear in the scan results.

### Step 1: Access Hidden Network View
1. Switch to the "Hidden Network" tab

### Step 2: Enter Network Details
1. **Network Name (SSID)**: Enter the exact name of the hidden network
   - Case-sensitive
   - Maximum 32 characters
   - No wildcards or special formatting

2. **Password**: Enter the network password
   - Leave empty for open (unsecured) networks
   - Characters are masked for security

### Step 3: Connect
1. Click "Connect to Hidden Network"
2. Wait for the connection to establish
3. Status messages will show connection progress
4. A success dialog confirms when connected

## Tips and Best Practices

### Successful Connections
- **Strong Signal**: Connect to networks with signal strength above 50% for best performance
- **Correct Password**: Double-check password before connecting
- **Network Name**: For hidden networks, ensure SSID is spelled exactly as configured

### Troubleshooting

#### "No networks found"
- Ensure WiFi adapter is enabled
- Check if airplane mode is off
- Move closer to WiFi access points
- Try scanning again

#### "Failed to connect"
- Verify password is correct
- Check if network is in range
- Ensure WiFi adapter is working: `nmcli device status`
- Check NetworkManager status: `sudo systemctl status NetworkManager`

#### "Permission denied"
- Ensure you have NetworkManager permissions
- Add user to netdev group: `sudo usermod -aG netdev $USER`
- Log out and log back in

#### "Failed to create NetworkManager client"
- Verify NetworkManager is running: `sudo systemctl start NetworkManager`
- Check D-Bus is available: `systemctl status dbus`

## Connection Management

### Saved Connections
Once connected, NetworkManager saves the network configuration. Your system will automatically reconnect to known networks.

### Managing Saved Networks
Use NetworkManager tools to manage saved connections:
```bash
# List all connections
nmcli connection show

# Delete a connection
nmcli connection delete "NetworkName"

# Modify a connection
nmcli connection modify "NetworkName" <settings>
```

### View Connection Details
```bash
# Show active connections
nmcli connection show --active

# Show connection details
nmcli connection show "NetworkName"
```

## Keyboard Shortcuts

- **Tab** - Navigate between UI elements
- **Enter** - Activate focused button
- **Esc** - Close dialogs (in some window managers, also exits full-screen)
- **Arrow Keys** - Navigate through network list

## Security Notes

### Password Safety
- Passwords are never logged or displayed in plain text
- Passwords are securely stored by NetworkManager
- Connection details are saved in `/etc/NetworkManager/system-connections/`

### Open Networks
- Connecting to unsecured (open) networks is less safe
- Data transmitted over open networks can be intercepted
- Use VPN on public WiFi networks

### Password Requirements
- WPA/WPA2: Minimum 8 characters
- WEP: 5 or 13 characters (or 10/26 hexadecimal digits)
- Open networks: No password required

## Command Line Alternatives

If you prefer command line tools, NetworkManager provides:

### nmcli - NetworkManager CLI
```bash
# Scan for networks
nmcli device wifi list

# Connect to network
nmcli device wifi connect "SSID" password "PASSWORD"

# Connect to hidden network
nmcli device wifi connect "SSID" password "PASSWORD" hidden yes

# Disconnect
nmcli device disconnect wlan0
```

### nmtui - NetworkManager Text UI
```bash
# Launch text-based interface
nmtui
```

## Advanced Features

### Priority Networks
NetworkManager automatically connects to known networks based on priority. Higher priority networks are preferred.

### Automatic Connection
By default, saved networks are set to connect automatically. This can be modified in NetworkManager settings.

### VPN Integration
After connecting to WiFi, you can establish VPN connections through NetworkManager for additional security.

## Getting Help

If you encounter issues:
1. Check this usage guide
2. Review INSTALL.md for system requirements
3. Check SECURITY.md for permission issues
4. Check NetworkManager status and logs
5. Open an issue in the project repository

## Example Workflow

### First Time Setup
1. Launch WiFi Wizard
2. Click "Scan Networks"
3. Find your home network
4. Click "Connect"
5. Enter password
6. Connection saved - automatic reconnection in future

### Mobile Hotspot Connection
1. Enable mobile hotspot on phone
2. Launch WiFi Wizard
3. Click "Scan Networks"
4. Find your phone's hotspot
5. Click "Connect"
6. Enter hotspot password

### Hidden Enterprise Network
1. Switch to "Hidden Network" tab
2. Enter enterprise SSID (e.g., "CompanyWiFi-Internal")
3. Enter authentication password
4. Click "Connect to Hidden Network"
5. System saves for future use
