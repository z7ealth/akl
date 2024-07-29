use std::{
    sync::{Arc, Mutex},
    thread
};

mod display;
mod systray;

fn main() {
    let mode: Arc<Mutex<String>> = Arc::new(Mutex::new("temp".to_string()));

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
