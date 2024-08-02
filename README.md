### DeepCool AK Digital for Linux (AKL).

#### About

Unofficial Linux version of DeepCool's AK Digital Software.

Based on <a href="https://github.com/raghulkrishna/deepcool-ak620-digital-linux" target="_blank">raghulkrishnadeepcool-ak620-digital-linux</a>

This project aims to provide the same functionality as DeepCool's Windows version.

> [!NOTE]
> As far as I know, this should work on any Linux distro.

#### Supported CPU Coolers

    DeepCool AK500 Digital
    DeepCool AK620 Digital

#### Requirements

 * libudev
 * libayatana-appindicator
 * libxdo

#### Fedora

`sudo dnf -y install systemd-devel libayatana-appindicator-gtk3-devel libxdo-devel`

> For any other distro, search for the package names and use the native package manager (e.g., apt, pacman, zypper) to install them.

<hr>

#### Install

1. `git clone https://github.com/z7ealth/akl.git && cd akl`

2. `chmod +x ./install.sh`

3. `./install.sh`

<hr>

#### Gallery

<img src="./assets/images/akl_screenshot_1.png" alt="Systray icon" width="60%">

<img src="./assets/images/akl_screenshot_2.png" alt="Display" width="60%">
