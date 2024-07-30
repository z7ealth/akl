use std::{
    sync::{Arc, Mutex},
    thread
};

use config::get_config;

mod display;
mod systray;
mod config;

fn main() {
    let akl_config = get_config().unwrap();

    let mode: Arc<Mutex<String>> = Arc::new(Mutex::new(akl_config.mode.to_string()));

    let mode_systray = Arc::clone(&mode);
    let mode_display = Arc::clone(&mode);

    // Start the systray in a separate task
    let systray_handle = thread::spawn(|| {
        systray::start(mode_systray);
    });

    // Start the display in another task
    let display_hanlde = thread::spawn(|| {
        display::start(mode_display);
    });

    systray_handle.join().unwrap();
    display_hanlde.join().unwrap();
}
