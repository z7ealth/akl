use std::{thread, time::Duration};

use tray_icon::menu::{Menu, MenuId, CheckMenuItem, SubmenuBuilder};
use tray_icon::TrayIconBuilder;

pub async fn start() {
    loop {
        match gtk::init() {
            Ok(_) => {
                println!("GTK Initialized");

                let path = concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/images/deepcool.ico"
                );
                let icon: tray_icon::Icon = load_icon(std::path::Path::new(path));

                let _tray_icon = TrayIconBuilder::new()
                    .with_tooltip("DeepCool AK Digital for Linux")
                    .with_icon(icon)
                    .with_title("DeepCool AK Digital")
                    .with_menu(Box::new(build_menu()))
                    .build()
                    .unwrap();

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

fn build_menu() -> Menu {
    let menu = Menu::new();

    let device_info = CheckMenuItem::new("AK500 Digital", false, true, None);

    let device_submenu = SubmenuBuilder::new()
        .id(MenuId::new("device_submenu"))
        .text("Device")
        .enabled(true)
        .build()
        .unwrap();

    device_submenu.append(&device_info).unwrap();

    menu.append_items(&[&device_submenu]).unwrap();

    menu
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
