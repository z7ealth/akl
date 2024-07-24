use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task};

mod display;
mod systray;

#[tokio::main]
async fn main() {

    let mode: Arc<Mutex<&str>> = Arc::new(Mutex::new("temp"));

    let mode_systray = Arc::clone(&mode);
    let mode_display = Arc::clone(&mode);

    // Start the systray in a separate task
    task::spawn(async move {
        systray::start(mode_systray).await;
    });

    // Start the display in another task
    task::spawn(async move {
        display::start(mode_display).await;
    });

    // Keep the main function alive
    loop {
        tokio::time::sleep(Duration::from_secs(6)).await;

        let current_mode = *mode.lock().await;

        *mode.lock().await = match current_mode {
            "temp" => "util",
            _ => "temp"
        };
    }
}