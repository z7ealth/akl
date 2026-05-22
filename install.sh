#!/usr/bin/env bash
set -euo pipefail

APP_NAME="akl"
AKL_DIR="$HOME/.config/akl"
SERVICE_NAME="akl"
SERVICE_FILE="${SERVICE_NAME}.service"
SERVICE_DIR="$HOME/.config/systemd/user"
EXEC_PATH="$HOME/.config/akl/bin/akl"

log() {
  echo "[+] $1"
}

command -v cargo >/dev/null 2>&1 || {
  echo "Error: cargo command not found. Please install Rust and Cargo."
  exit 1
}

log "Building executable in release mode..."
cargo build --release

if [ -d "$AKL_DIR" ]; then
  echo -e "The directory $AKL_DIR already exists. Existing configuration will be lost.\n"
  read -r -p "Do you want to proceed? (y/n): " choice
  case "$choice" in
    y|Y) rm -rf "$AKL_DIR" ;;
    n|N) echo "Exiting."; exit 1 ;;
    *) echo "Invalid choice. Exiting."; exit 1 ;;
  esac
fi

log "Creating app directory..."
mkdir -p "$AKL_DIR/bin"
mkdir -p "$AKL_DIR/assets/images"

log "Installing executable and assets..."
cp -f ./target/release/akl "$EXEC_PATH"
cp -f ./assets/images/akl_logo.png "$AKL_DIR/assets/images/akl_logo.png"

echo -e "Select your DeepCool CPU Cooler model:\n1. AK500 Digital\n2. AK620 Digital\n"
read -r -p "Pick a number: " model_choice

case "$model_choice" in
  1) PRODUCT="AK500" ;;
  2) PRODUCT="AK620" ;;
  *) echo "Invalid choice. Exiting."; exit 1 ;;
esac

log "Creating config..."
cat > "$AKL_DIR/config.toml" <<EOF
# AKL Configuration File

product = "$PRODUCT"
mode = "temp"
EOF

log "Creating user systemd service..."

mkdir -p "$SERVICE_DIR"

if [ -f "$SERVICE_DIR/$SERVICE_FILE" ]; then
  systemctl --user disable "$SERVICE_NAME" || true
  systemctl --user stop "$SERVICE_NAME" || true
  rm -f "$SERVICE_DIR/$SERVICE_FILE"
fi

cat > "$SERVICE_DIR/$SERVICE_FILE" <<EOF
[Unit]
Description=AK Digital for Linux
After=graphical-session.target
PartOf=graphical-session.target

[Service]
Type=simple
ExecStartPre=/bin/sh -c 'systemctl --user import-environment DISPLAY WAYLAND_DISPLAY XDG_RUNTIME_DIR DBUS_SESSION_BUS_ADDRESS XDG_CURRENT_DESKTOP || true'
ExecStart=$EXEC_PATH
Restart=on-failure
RestartSec=5s

Environment=AKL_CONFIG_DIR=$AKL_DIR
Environment=GTK_USE_PORTAL=1

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=default.target
EOF

systemctl --user daemon-reexec
systemctl --user daemon-reload
systemctl --user enable --now "$SERVICE_NAME"

log "Installing XDG autostart fallback..."

mkdir -p "$HOME/.config/autostart"

cat > "$HOME/.config/autostart/akl.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=AK Digital for Linux
Comment=DeepCool AK Digital tray icon
Exec=$EXEC_PATH
Terminal=false
X-GNOME-Autostart-enabled=true
EOF

echo -e "\nInstallation finished!"
echo ""
echo "If using Hyprland, add this to ~/.config/hypr/hyprland.conf:"
echo ""
echo "exec-once = systemctl --user import-environment DISPLAY WAYLAND_DISPLAY XDG_RUNTIME_DIR DBUS_SESSION_BUS_ADDRESS XDG_CURRENT_DESKTOP"
echo "exec-once = systemctl --user restart akl.service"
