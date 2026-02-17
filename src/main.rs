mod network_manager;
mod ui;

use network_manager::NetworkManagerClient;
use ui::WifiWizardApp;

#[tokio::main]
async fn main() {
    // Initialize GTK
    gtk4::init().expect("Failed to initialize GTK");

    // Create NetworkManager client
    let nm_client = NetworkManagerClient::new()
        .await
        .expect("Failed to create NetworkManager client. Make sure NetworkManager is running and you have the necessary permissions.");

    // Create and run the app
    let app = WifiWizardApp::new(nm_client);
    app.run();
}
