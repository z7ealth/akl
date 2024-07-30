use gtk::gdk_pixbuf::{InterpType, Pixbuf};
use gtk::glib::Propagation;
use gtk::prelude::{
    AboutDialogExt, CheckMenuItemExt, GtkMenuItemExt, GtkWindowExt, MenuShellExt, WidgetExt,
};
use gtk::{AboutDialog, Menu, MenuItem, RadioMenuItem, WindowPosition};
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
                indicator.set_label("AKL", "");

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
    let about_item = get_about_item();

    menu.append(&device_item);
    menu.append(&display_item);
    menu.append(&about_item);

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
    let temperature_radio_button = RadioMenuItem::with_label("Temperature C°");
    let temperaturef_radio_button =
        RadioMenuItem::with_label_from_widget(&temperature_radio_button, Some("Temperature F°"));
    let util_radio_button =
        RadioMenuItem::with_label_from_widget(&temperature_radio_button, Some("Util"));

    let temp_mode = Arc::clone(&mode);

    temperature_radio_button.connect_toggled(move |_| {
        let mut write_mode = temp_mode.lock().unwrap();
        *write_mode = "temp".to_string();
    });

    let temp_f_mode = Arc::clone(&mode);

    temperaturef_radio_button.connect_toggled(move |_| {
        let mut write_mode = temp_f_mode.lock().unwrap();
        *write_mode = "temp_f".to_string();
    });

    let util_mode = Arc::clone(&mode);

    util_radio_button.connect_toggled(move |_| {
        let mut write_mode = util_mode.lock().unwrap();
        *write_mode = "util".to_string();
    });

    let display_switch_submenu = Menu::new();
    display_switch_submenu.append(&temperature_radio_button);
    display_switch_submenu.append(&temperaturef_radio_button);
    display_switch_submenu.append(&util_radio_button);

    let display_switch_menu_item = MenuItem::with_label("Display Switch");
    display_switch_menu_item.set_submenu(Some(&display_switch_submenu));

    display_switch_menu_item
}

fn get_about_item() -> MenuItem {
    let about = MenuItem::with_label("About");

    about.connect_button_press_event(|_, _| {
        let window = AboutDialog::new();
        let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/images/deepcool.png");
        let icon = Pixbuf::from_file(icon_path.clone())
            .unwrap()
            .scale_simple(50, 50, InterpType::Bilinear)
            .unwrap();

        window.set_icon_from_file(icon_path.clone()).unwrap();
        window.set_logo(Some(&icon));
        window.set_program_name("AK Digital for Linux");
        window.set_version(Some(env!("CARGO_PKG_VERSION")));
        window.set_default_width(400);
        window.set_default_height(200);
        window.set_copyright(Some(format!("{}  {}", "\u{F09B}", "z7ealth").as_str()));
        window.set_comments(Some(
            "Unofficial Linux version of DeepCool's AK Digital Software.",
        ));
        window.set_resizable(false);
        window.set_window_position(WindowPosition::Center);
        window.set_title("About");

        window.connect_button_release_event(|dialog, _| {
            dialog.close();

            Propagation::Proceed
        });

        window.show();

        Propagation::Stop
    });

    about
}
