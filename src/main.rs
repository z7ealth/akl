use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

mod display;
mod systray;

fn main() {
    let mode: Arc<Mutex<String>> = Arc::new(Mutex::new("temp".to_string()));

    let mode_systray = Arc::clone(&mode);
    let mode_display = Arc::clone(&mode);

    // Start the systray in a separate task
    thread::spawn(|| {
        systray::start(mode_systray);
    });

    // Start the display in another task
    thread::spawn(|| {
        display::start(mode_display);
    });

    // Keep the main function alive
    loop {
        thread::sleep(Duration::from_secs(1));

        /*
        let current_mode = *mode.lock().await;

        *mode.lock().await = match current_mode {
            "temp" => "util",
            _ => "temp"
        };
        */
    }
}
