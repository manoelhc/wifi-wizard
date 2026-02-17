use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, Entry, Label, ListBox, 
    ListBoxRow, Orientation, ScrolledWindow, Stack, StackSwitcher,
};
use glib;
use std::cell::RefCell;
use std::rc::Rc;

use crate::network_manager::{NetworkManagerClient, WifiNetwork};

pub struct WifiWizardApp {
    app: Application,
    nm_client: Rc<NetworkManagerClient>,
}

impl WifiWizardApp {
    pub fn new(nm_client: NetworkManagerClient) -> Self {
        let app = Application::builder()
            .application_id("com.example.wifi-wizard")
            .build();

        Self {
            app,
            nm_client: Rc::new(nm_client),
        }
    }

    pub fn run(&self) {
        let nm_client = self.nm_client.clone();
        
        self.app.connect_activate(move |app| {
            let nm_client = nm_client.clone();
            build_ui(app, nm_client);
        });

        self.app.run();
    }
}

fn build_ui(app: &Application, nm_client: Rc<NetworkManagerClient>) {
    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WiFi Wizard")
        .default_width(800)
        .default_height(600)
        .build();

    // Make window fullscreen
    window.fullscreen();

    // Create main container
    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    // Create header
    let header_label = Label::new(Some("WiFi Wizard"));
    header_label.add_css_class("title-1");
    main_box.append(&header_label);

    // Create stack for switching between views
    let stack = Stack::new();
    let stack_switcher = StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack));
    main_box.append(&stack_switcher);

    // Create available networks view
    let networks_view = create_networks_view(nm_client.clone(), window.clone());
    stack.add_titled(&networks_view, Some("available"), "Available Networks");

    // Create hidden network view
    let hidden_view = create_hidden_network_view(nm_client.clone(), window.clone());
    stack.add_titled(&hidden_view, Some("hidden"), "Hidden Network");

    main_box.append(&stack);

    // Create exit button
    let exit_button = Button::with_label("Exit");
    let window_clone = window.clone();
    exit_button.connect_clicked(move |_| {
        window_clone.close();
    });
    main_box.append(&exit_button);

    window.set_child(Some(&main_box));
    window.present();
}

fn create_networks_view(nm_client: Rc<NetworkManagerClient>, window: ApplicationWindow) -> Box {
    let view_box = Box::new(Orientation::Vertical, 10);

    // Create scrolled window for network list
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);
    scrolled.set_min_content_height(400);

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    scrolled.set_child(Some(&list_box));

    // Status label
    let status_label = Label::new(Some("Click 'Scan Networks' to discover WiFi networks"));
    view_box.append(&status_label);

    // Scan button
    let scan_button = Button::with_label("Scan Networks");
    let nm_client_clone = nm_client.clone();
    let list_box_clone = list_box.clone();
    let status_label_clone = status_label.clone();
    let window_clone = window.clone();
    
    scan_button.connect_clicked(move |button| {
        button.set_sensitive(false);
        status_label_clone.set_text("Scanning for networks...");
        
        let nm_client = nm_client_clone.clone();
        let list_box = list_box_clone.clone();
        let status_label = status_label_clone.clone();
        let button_clone = button.clone();
        let window = window_clone.clone();

        glib::spawn_future_local(async move {
            match nm_client.scan_networks().await {
                Ok(networks) => {
                    // Clear existing items
                    while let Some(child) = list_box.first_child() {
                        list_box.remove(&child);
                    }

                    if networks.is_empty() {
                        status_label.set_text("No networks found");
                    } else {
                        status_label.set_text(&format!("Found {} network(s)", networks.len()));
                        
                        for network in networks {
                            let row = create_network_row(network, nm_client.clone(), window.clone());
                            list_box.append(&row);
                        }
                    }
                }
                Err(e) => {
                    status_label.set_text(&format!("Error scanning networks: {}", e));
                }
            }
            button_clone.set_sensitive(true);
        });
    });

    view_box.append(&scan_button);
    view_box.append(&scrolled);

    view_box
}

fn create_network_row(
    network: WifiNetwork,
    nm_client: Rc<NetworkManagerClient>,
    window: ApplicationWindow,
) -> ListBoxRow {
    let row = ListBoxRow::new();
    let row_box = Box::new(Orientation::Horizontal, 10);
    row_box.set_margin_top(5);
    row_box.set_margin_bottom(5);
    row_box.set_margin_start(5);
    row_box.set_margin_end(5);

    // Network name
    let name_label = Label::new(Some(&network.ssid));
    name_label.set_hexpand(true);
    name_label.set_halign(gtk4::Align::Start);
    row_box.append(&name_label);

    // Signal strength
    let strength_label = Label::new(Some(&format!("{}%", network.strength)));
    row_box.append(&strength_label);

    // Security indicator
    let security_label = Label::new(Some(if network.secured { "🔒" } else { "" }));
    row_box.append(&security_label);

    // Connect button
    let connect_button = Button::with_label("Connect");
    let network_clone = network.clone();
    let nm_client_clone = nm_client.clone();
    let window_clone = window.clone();
    
    connect_button.connect_clicked(move |_| {
        let network = network_clone.clone();
        let nm_client = nm_client_clone.clone();
        let window = window_clone.clone();
        
        if network.secured {
            show_password_dialog(&network.ssid, nm_client, &window, false);
        } else {
            connect_to_network(&network.ssid, None, false, nm_client, &window);
        }
    });
    
    row_box.append(&connect_button);
    row.set_child(Some(&row_box));

    row
}

fn create_hidden_network_view(nm_client: Rc<NetworkManagerClient>, window: ApplicationWindow) -> Box {
    let view_box = Box::new(Orientation::Vertical, 10);

    let info_label = Label::new(Some("Connect to a hidden WiFi network"));
    view_box.append(&info_label);

    // SSID input
    let ssid_label = Label::new(Some("Network Name (SSID):"));
    ssid_label.set_halign(gtk4::Align::Start);
    view_box.append(&ssid_label);

    let ssid_entry = Entry::new();
    ssid_entry.set_placeholder_text(Some("Enter network name"));
    view_box.append(&ssid_entry);

    // Password input
    let password_label = Label::new(Some("Password (leave empty for open network):"));
    password_label.set_halign(gtk4::Align::Start);
    view_box.append(&password_label);

    let password_entry = Entry::new();
    password_entry.set_placeholder_text(Some("Enter password"));
    password_entry.set_visibility(false);
    password_entry.set_input_purpose(gtk4::InputPurpose::Password);
    view_box.append(&password_entry);

    // Status label
    let status_label = Label::new(None);
    view_box.append(&status_label);

    // Connect button
    let connect_button = Button::with_label("Connect to Hidden Network");
    let ssid_entry_clone = ssid_entry.clone();
    let password_entry_clone = password_entry.clone();
    let status_label_clone = status_label.clone();
    let nm_client_clone = nm_client.clone();
    let window_clone = window.clone();
    
    connect_button.connect_clicked(move |button| {
        let ssid = ssid_entry_clone.text().to_string();
        
        if ssid.is_empty() {
            status_label_clone.set_text("Please enter a network name");
            return;
        }

        let password_text = password_entry_clone.text().to_string();
        let password = if password_text.is_empty() {
            None
        } else {
            Some(password_text)
        };

        button.set_sensitive(false);
        status_label_clone.set_text("Connecting...");

        let nm_client = nm_client_clone.clone();
        let status_label = status_label_clone.clone();
        let button_clone = button.clone();
        let window = window_clone.clone();

        glib::spawn_future_local(async move {
            match nm_client.connect_to_network(&ssid, password.as_deref(), true).await {
                Ok(_) => {
                    show_success_dialog(&format!("Successfully connected to {}", ssid), &window);
                    status_label.set_text("Connection successful!");
                }
                Err(e) => {
                    show_error_dialog(&format!("Failed to connect: {}", e), &window);
                    status_label.set_text(&format!("Connection failed: {}", e));
                }
            }
            button_clone.set_sensitive(true);
        });
    });

    view_box.append(&connect_button);

    view_box
}

fn show_password_dialog(
    ssid: &str,
    nm_client: Rc<NetworkManagerClient>,
    window: &ApplicationWindow,
    hidden: bool,
) {
    let dialog = gtk4::Window::new();
    dialog.set_title(Some("Enter Password"));
    dialog.set_transient_for(Some(window));
    dialog.set_modal(true);
    dialog.set_default_width(400);
    dialog.set_default_height(200);

    let dialog_box = Box::new(Orientation::Vertical, 10);
    dialog_box.set_margin_top(20);
    dialog_box.set_margin_bottom(20);
    dialog_box.set_margin_start(20);
    dialog_box.set_margin_end(20);

    let label = Label::new(Some(&format!("Enter password for: {}", ssid)));
    dialog_box.append(&label);

    let password_entry = Entry::new();
    password_entry.set_placeholder_text(Some("Password"));
    password_entry.set_visibility(false);
    password_entry.set_input_purpose(gtk4::InputPurpose::Password);
    dialog_box.append(&password_entry);

    let button_box = Box::new(Orientation::Horizontal, 10);
    button_box.set_halign(gtk4::Align::Center);

    let connect_button = Button::with_label("Connect");
    let cancel_button = Button::with_label("Cancel");

    let ssid_string = ssid.to_string();
    let password_entry_clone = password_entry.clone();
    let dialog_clone = dialog.clone();
    let nm_client_clone = nm_client.clone();
    let window_clone = window.clone();
    
    connect_button.connect_clicked(move |_| {
        let password = password_entry_clone.text().to_string();
        if !password.is_empty() {
            connect_to_network(
                &ssid_string,
                Some(password.as_str()),
                hidden,
                nm_client_clone.clone(),
                &window_clone,
            );
        }
        dialog_clone.close();
    });

    let dialog_clone = dialog.clone();
    cancel_button.connect_clicked(move |_| {
        dialog_clone.close();
    });

    button_box.append(&connect_button);
    button_box.append(&cancel_button);
    dialog_box.append(&button_box);

    dialog.set_child(Some(&dialog_box));
    dialog.present();
}

fn connect_to_network(
    ssid: &str,
    password: Option<&str>,
    hidden: bool,
    nm_client: Rc<NetworkManagerClient>,
    window: &ApplicationWindow,
) {
    let ssid_string = ssid.to_string();
    let password_string = password.map(|s| s.to_string());
    let window_clone = window.clone();

    glib::spawn_future_local(async move {
        match nm_client
            .connect_to_network(&ssid_string, password_string.as_deref(), hidden)
            .await
        {
            Ok(_) => {
                show_success_dialog(
                    &format!("Successfully connected to {}", ssid_string),
                    &window_clone,
                );
            }
            Err(e) => {
                show_error_dialog(&format!("Failed to connect: {}", e), &window_clone);
            }
        }
    });
}

fn show_success_dialog(message: &str, window: &ApplicationWindow) {
    let dialog = gtk4::Window::new();
    dialog.set_title(Some("Success"));
    dialog.set_transient_for(Some(window));
    dialog.set_modal(true);
    dialog.set_default_width(300);
    dialog.set_default_height(150);

    let dialog_box = Box::new(Orientation::Vertical, 10);
    dialog_box.set_margin_top(20);
    dialog_box.set_margin_bottom(20);
    dialog_box.set_margin_start(20);
    dialog_box.set_margin_end(20);

    let label = Label::new(Some(message));
    dialog_box.append(&label);

    let ok_button = Button::with_label("OK");
    let dialog_clone = dialog.clone();
    ok_button.connect_clicked(move |_| {
        dialog_clone.close();
    });
    dialog_box.append(&ok_button);

    dialog.set_child(Some(&dialog_box));
    dialog.present();
}

fn show_error_dialog(message: &str, window: &ApplicationWindow) {
    let dialog = gtk4::Window::new();
    dialog.set_title(Some("Error"));
    dialog.set_transient_for(Some(window));
    dialog.set_modal(true);
    dialog.set_default_width(300);
    dialog.set_default_height(150);

    let dialog_box = Box::new(Orientation::Vertical, 10);
    dialog_box.set_margin_top(20);
    dialog_box.set_margin_bottom(20);
    dialog_box.set_margin_start(20);
    dialog_box.set_margin_end(20);

    let label = Label::new(Some(message));
    dialog_box.append(&label);

    let ok_button = Button::with_label("OK");
    let dialog_clone = dialog.clone();
    ok_button.connect_clicked(move |_| {
        dialog_clone.close();
    });
    dialog_box.append(&ok_button);

    dialog.set_child(Some(&dialog_box));
    dialog.present();
}
