#!/usr/bin/env bash
set -euo pipefail

APP_NAME="akl"
AKL_DIR="$HOME/.config/akl"
SERVICE_NAME="akl"
SERVICE_FILE="${SERVICE_NAME}.service"
SERVICE_DIR="$HOME/.config/systemd/user"
AUTOSTART_FILE="$HOME/.config/autostart/akl.desktop"

log() {
  echo "[+] $1"
}

echo "This will uninstall AK Digital for Linux."
read -r -p "Do you want to proceed? (y/n): " choice

case "$choice" in
  y|Y) ;;
  n|N) echo "Exiting."; exit 0 ;;
  *) echo "Invalid choice."; exit 1 ;;
esac

log "Stopping and disabling user service..."

if systemctl --user list-unit-files | grep -q "^${SERVICE_NAME}.service"; then
  systemctl --user stop "$SERVICE_NAME" || true
  systemctl --user disable "$SERVICE_NAME" || true
fi

log "Removing systemd user service..."

if [ -f "$SERVICE_DIR/$SERVICE_FILE" ]; then
  rm -f "$SERVICE_DIR/$SERVICE_FILE"
fi

systemctl --user daemon-reload
systemctl --user daemon-reexec

log "Removing autostart entry..."

if [ -f "$AUTOSTART_FILE" ]; then
  rm -f "$AUTOSTART_FILE"
fi

log "Removing application files..."

if [ -d "$AKL_DIR" ]; then
  rm -rf "$AKL_DIR"
fi

log "Removing old legacy system service..."

if [ -f "/etc/systemd/system/akl.service" ]; then
  sudo systemctl stop akl.service || true
  sudo systemctl disable akl.service || true
  sudo rm -f /etc/systemd/system/akl.service
  sudo systemctl daemon-reload
fi

log "Removing old legacy files..."

sudo rm -f /usr/bin/akl || true
sudo rm -rf /etc/akl || true

echo ""
echo "AK Digital for Linux has been uninstalled."
