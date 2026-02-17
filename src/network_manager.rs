use anyhow::{Context, Result};
use std::collections::HashMap;
use zbus::{Connection, dbus_proxy};

#[derive(Debug, Clone)]
pub struct WifiNetwork {
    pub ssid: String,
    pub strength: u8,
    pub secured: bool,
    pub path: String,
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    fn get_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
    fn add_and_activate_connection(
        &self,
        connection: HashMap<String, HashMap<String, zbus::zvariant::Value>>,
        device: &zbus::zvariant::ObjectPath,
        specific_object: &zbus::zvariant::ObjectPath,
    ) -> zbus::Result<(zbus::zvariant::OwnedObjectPath, zbus::zvariant::OwnedObjectPath)>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Device.Wireless",
    default_service = "org.freedesktop.NetworkManager"
)]
trait WirelessDevice {
    fn get_access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
    fn request_scan(&self, options: HashMap<String, zbus::zvariant::Value>) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.AccessPoint",
    default_service = "org.freedesktop.NetworkManager"
)]
trait AccessPoint {
    #[dbus_proxy(property)]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;
    
    #[dbus_proxy(property)]
    fn strength(&self) -> zbus::Result<u8>;
    
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::Result<u32>;
    
    #[dbus_proxy(property)]
    fn wpa_flags(&self) -> zbus::Result<u32>;
    
    #[dbus_proxy(property)]
    fn rsn_flags(&self) -> zbus::Result<u32>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Device",
    default_service = "org.freedesktop.NetworkManager"
)]
trait Device {
    #[dbus_proxy(property)]
    fn device_type(&self) -> zbus::Result<u32>;
}

pub struct NetworkManagerClient {
    connection: Connection,
}

impl NetworkManagerClient {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system()
            .await
            .context("Failed to connect to system D-Bus")?;
        Ok(Self { connection })
    }

    pub async fn scan_networks(&self) -> Result<Vec<WifiNetwork>> {
        let nm_proxy = NetworkManagerProxy::new(&self.connection)
            .await
            .context("Failed to create NetworkManager proxy")?;

        let devices = nm_proxy
            .get_devices()
            .await
            .context("Failed to get devices")?;

        let mut networks = Vec::new();

        for device_path in devices {
            let device_proxy = DeviceProxy::builder(&self.connection)
                .path(device_path.clone())
                .context("Invalid device path")?
                .build()
                .await
                .context("Failed to create Device proxy")?;

            let device_type = device_proxy
                .device_type()
                .await
                .context("Failed to get device type")?;

            // Device type 2 is WiFi
            if device_type != 2 {
                continue;
            }

            let wireless_proxy = WirelessDeviceProxy::builder(&self.connection)
                .path(device_path.clone())
                .context("Invalid device path")?
                .build()
                .await
                .context("Failed to create WirelessDevice proxy")?;

            // Request a scan
            let _ = wireless_proxy
                .request_scan(HashMap::new())
                .await;

            // Give the scan a moment to complete
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            let access_points = wireless_proxy
                .get_access_points()
                .await
                .context("Failed to get access points")?;

            for ap_path in access_points {
                let ap_proxy = AccessPointProxy::builder(&self.connection)
                    .path(ap_path.clone())
                    .context("Invalid access point path")?
                    .build()
                    .await
                    .context("Failed to create AccessPoint proxy")?;

                let ssid_bytes = ap_proxy.ssid().await.unwrap_or_default();
                let ssid = String::from_utf8_lossy(&ssid_bytes).to_string();

                if ssid.is_empty() {
                    continue;
                }

                let strength = ap_proxy.strength().await.unwrap_or(0);
                let flags = ap_proxy.flags().await.unwrap_or(0);
                let wpa_flags = ap_proxy.wpa_flags().await.unwrap_or(0);
                let rsn_flags = ap_proxy.rsn_flags().await.unwrap_or(0);

                let secured = flags != 0 || wpa_flags != 0 || rsn_flags != 0;

                networks.push(WifiNetwork {
                    ssid,
                    strength,
                    secured,
                    path: ap_path.to_string(),
                });
            }

            break; // Use the first WiFi device found
        }

        // Remove duplicates based on SSID
        networks.sort_by(|a, b| b.strength.cmp(&a.strength));
        networks.dedup_by(|a, b| a.ssid == b.ssid);

        Ok(networks)
    }

    pub async fn connect_to_network(
        &self,
        ssid: &str,
        password: Option<&str>,
        hidden: bool,
    ) -> Result<()> {
        let nm_proxy = NetworkManagerProxy::new(&self.connection)
            .await
            .context("Failed to create NetworkManager proxy")?;

        let devices = nm_proxy
            .get_devices()
            .await
            .context("Failed to get devices")?;

        let mut wifi_device_path = None;

        for device_path in devices {
            let device_proxy = DeviceProxy::builder(&self.connection)
                .path(device_path.clone())
                .context("Invalid device path")?
                .build()
                .await
                .context("Failed to create Device proxy")?;

            let device_type = device_proxy
                .device_type()
                .await
                .context("Failed to get device type")?;

            if device_type == 2 {
                wifi_device_path = Some(device_path);
                break;
            }
        }

        let wifi_device_path = wifi_device_path
            .context("No WiFi device found")?;

        let mut connection_settings: HashMap<String, HashMap<String, zbus::zvariant::Value>> =
            HashMap::new();

        // Connection settings
        let mut connection = HashMap::new();
        connection.insert(
            "type".to_string(),
            zbus::zvariant::Value::new("802-11-wireless"),
        );
        connection.insert("id".to_string(), zbus::zvariant::Value::new(ssid));
        connection.insert("uuid".to_string(), zbus::zvariant::Value::new(uuid::Uuid::new_v4().to_string()));
        connection_settings.insert("connection".to_string(), connection);

        // WiFi settings
        let mut wireless = HashMap::new();
        wireless.insert(
            "ssid".to_string(),
            zbus::zvariant::Value::new(ssid.as_bytes()),
        );
        wireless.insert("mode".to_string(), zbus::zvariant::Value::new("infrastructure"));
        if hidden {
            wireless.insert("hidden".to_string(), zbus::zvariant::Value::new(true));
        }
        connection_settings.insert("802-11-wireless".to_string(), wireless);

        // Security settings
        if let Some(pwd) = password {
            let mut wireless_security = HashMap::new();
            wireless_security.insert(
                "key-mgmt".to_string(),
                zbus::zvariant::Value::new("wpa-psk"),
            );
            wireless_security.insert("psk".to_string(), zbus::zvariant::Value::new(pwd));
            connection_settings.insert("802-11-wireless-security".to_string(), wireless_security);
        }

        // IPv4 settings
        let mut ipv4 = HashMap::new();
        ipv4.insert("method".to_string(), zbus::zvariant::Value::new("auto"));
        connection_settings.insert("ipv4".to_string(), ipv4);

        // IPv6 settings
        let mut ipv6 = HashMap::new();
        ipv6.insert("method".to_string(), zbus::zvariant::Value::new("auto"));
        connection_settings.insert("ipv6".to_string(), ipv6);

        let empty_path = zbus::zvariant::ObjectPath::try_from("/").unwrap();
        nm_proxy
            .add_and_activate_connection(
                connection_settings,
                &wifi_device_path.as_ref(),
                &empty_path,
            )
            .await
            .context("Failed to add and activate connection")?;

        Ok(())
    }
}
