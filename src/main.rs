use std::time::Duration;
use tokio::task;

mod display;
mod systray;

#[tokio::main]
async fn main() {
    // Start the systray in a separate task
    task::spawn(async {
        systray::start().await;
    });

    // Start the display in another task
    task::spawn(async {
        display::start().await;
    });

    // Keep the main function alive
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}