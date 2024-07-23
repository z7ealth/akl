### DeepCool AK Digital for Linux (AKL).

#### About

Unofficial Linux version of DeepCool's AK Digital Software.

Based on <a href="https://github.com/raghulkrishna/deepcool-ak620-digital-linux" target="_blank">raghulkrishnadeepcool-ak620-digital-linux</a>

This project aims to provide the same functionality as DeepCool's Windows version.

#### Requirements

 * libudev
 * libayatana-appindicator
 * libxdo

#### Fedora

`sudo dnf -y install systemd-devel libayatana-appindicator-gtk3-devel libxdo-devel`

<hr>

#### Install

1. `sudo cp akl_example.service /etc/systemd/system/akl.service`

2. Edit /etc/systemd/system/akl.service according to your system config.

3. `sudo systemctl daemon-reload`
4. `cargo build --release`
5. `sudo cp ./target/release/akl /usr/bin/akl`
6. `sudo systemctl enable akl.service`
7. `sudo systemctl start akl.service`

<hr>

#### Gallery

<img src="./assets/images/akl_screenshot_1.png" alt="Systray icon" width="60%">

<img src="./assets/images/akl_screenshot_2.png" alt="Display" width="60%">