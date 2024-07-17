mod display;
mod systray;

fn main() {
    systray::start();
    display::start();
}