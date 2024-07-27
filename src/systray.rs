use gtk::prelude::{CheckMenuItemExt, GtkMenuItemExt, MenuShellExt, RadioMenuItemExt, WidgetExt};
use gtk::{Menu, MenuItem, RadioMenuItem};
use libappindicator::{AppIndicator, AppIndicatorStatus};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

pub fn start(mode: Arc<Mutex<String>>) {
    loop {
        match gtk::init() {
            Ok(_) => {
                println!("GTK Initialized");

                let mut indicator = AppIndicator::new("libappindicator test application", "");
                indicator.set_status(AppIndicatorStatus::Active);

                let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/images");
                indicator.set_icon_theme_path(icon_path.to_str().unwrap());
                indicator.set_icon_full("deepcool", "icon");

                let mut menu = build_menu(mode);

                indicator.set_menu(&mut menu);

                menu.show_all();

                gtk::main();
                break;
            }
            Err(_) => {
                println!("Waiting for GTK to initialize");
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}

fn build_menu(mode: Arc<Mutex<String>>) -> Menu {
    let menu = Menu::new();

    let device_item = get_device_item();
    let display_item = get_display_switch_item(mode);

    menu.append(&device_item);
    menu.append(&display_item);

    menu
}

fn get_device_item() -> MenuItem {
    let device_radio_button = RadioMenuItem::with_label("AK500 Digital");
    let device_submenu = Menu::new();
    let device_menu_item = MenuItem::with_label("Device");
    device_radio_button.set_sensitive(false);
    device_submenu.append(&device_radio_button);
    device_menu_item.set_submenu(Some(&device_submenu));

    device_menu_item
}

fn get_display_switch_item(mode: Arc<Mutex<String>>) -> MenuItem {
    let temperature_radio_button = RadioMenuItem::with_label("Temperature");
    let util_radio_button = RadioMenuItem::with_label("Util");
    temperature_radio_button.join_group(Some(&temperature_radio_button));
    util_radio_button.join_group(Some(&temperature_radio_button));

    temperature_radio_button.connect_toggled(move |button| {
        let group = button.group();

        let active_radio = group.iter().find(|item| item.is_active()).unwrap();

        match active_radio.label().unwrap().as_str() {
            "Util" => {
                let mut write_mode = mode.lock().unwrap();
                *write_mode = "util".to_string();
            }
            _ => {
                let mut write_mode = mode.lock().unwrap();
                *write_mode = "temp".to_string();
            }
        }
    });

    let display_switch_submenu = Menu::new();
    display_switch_submenu.append(&temperature_radio_button);
    display_switch_submenu.append(&util_radio_button);

    let display_switch_menu_item = MenuItem::with_label("Display Switch");
    display_switch_menu_item.set_submenu(Some(&display_switch_submenu));

    display_switch_menu_item
}
