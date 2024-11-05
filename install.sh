#!/usr/bin/sh

# List of environment variables to check
ENV_VARS=("XAUTHORITY" "DISPLAY")

# Function to check if an environment variable is set
check_env_var() {
  local var_name=$1
  local var_value=$(printenv "$var_name")

  if [ -z "$var_value" ]; then
    echo "Environment variable $var_name is not set."
    exit 1
  else
    echo "Environment variable $var_name is set to: $var_value"
  fi
}

# Iterate over the list of environment variables and check each one
for var in "${ENV_VARS[@]}"; do
  check_env_var "$var"
done

echo -e "All required environment variables are set.\n"

echo -e "Script will prompt for sudo privileges in order to copy files to /usr/bin /etc/akl and /etc/systemd/system\n"

AKL_DIR="/etc/akl"

# Check if the directory exists
if [ -d "$AKL_DIR" ]; then
  # Directory is not empty
  echo -e "The directory $AKL_DIR is not empty. Existing configuration will be lost.\n"
  read -p "Do you want to proceed? (y/n):" choice
  case "$choice" in 
    y|Y ) echo -e "Deleting current configuration\n"; sudo rm -rf $AKL_DIR;;
    n|N ) echo "Exiting."; exit 1;;
    * ) echo "Invalid choice. Exiting."; exit 1;;
  esac
fi

echo -e "Creating directory /etc/akl\n"
sudo mkdir $AKL_DIR

echo -e "Building executable in release mode...\n"
cargo build --release
echo -e "Moving akl executable to /usr/bin\n"
sudo cp -rf ./target/release/akl /usr/bin
sudo cp -rf ./assets/images/deepcool.png /etc/akl

echo -e "Select your DeepCool CPU Cooler model:\n1. AK500 Digital\n2. AK620 Digital\n"

read -p "Pick a number: " model_choice
case "$model_choice" in 
  1 ) PRODUCT="AK500";;
  2 ) PRODUCT="AK620";;
  * ) echo "Invalid choice. Exiting."; exit 1;;
esac

echo -e "Creating configuration file $AKL_DIR/config.toml\n"

OUTPUT_CONFIG_FILE="config.toml"

cat <<EOF > "$OUTPUT_CONFIG_FILE"
# AKL Configuration File

# DeepCool CPU cooler's model (AK500 or AK620). Defaults to "AK500".
product = "$PRODUCT"

# Default display mode (temp, temp_f or util). Defaults to "temp".
mode = "temp"
EOF

sudo mv $OUTPUT_CONFIG_FILE $AKL_DIR

echo -e "Copying service to /etc/systemd/system\n"

if [ -f "/etc/systemd/system/akl.service" ]; then
  sudo systemctl disable akl.service
  sudo systemctl stop akl.service
  echo "Deleting previous service file..."
  sudo rm "/etc/systemd/system/akl.service"
  echo "File akl.service has been deleted."
  sudo systemctl daemon-reload
fi

OUTPUT_SERVICE_FILE="akl.service"

cat <<EOF > "$OUTPUT_SERVICE_FILE"
[Unit]
Description=DeepCool AK Digital for Linux service.
After=graphical.target

[Service]
Type=simple
User=root
Restart=on-failure
RestartSec=5s
ExecStart=/usr/bin/akl
StandardOutput=append:/var/log/akl.log
StandardError=append:/var/log/akl.log
Environment=DISPLAY=$DISPLAY
Environment=XAUTHORITY=$XAUTHORITY

[Install]
WantedBy=default.target
EOF

sudo mv akl.service /etc/systemd/system
sudo chown root:root /etc/systemd/system/akl.service
sudo systemctl daemon-reload
sudo systemctl enable --now akl.service

echo -e "\nInstallation finished!"
