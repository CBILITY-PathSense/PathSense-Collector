#!/bin/bash

echo "Installing PathSense-collector..."

sudo apt-get update
sudo apt-get install -y \
    build-essential \
    libopencv-dev \
    libclang-dev \
    clang

# Copy the bin folder to the .pathsense-collector directory
sudo mkdir -p /pathsense-collector
sudo cp -r bin/* /pathsense-collector/
echo "PathSense-collector files is copied to /pathsense-collector"

# Check if pathsense-collector is already in the boot file
if grep -Fxq "cd /pathsense-collector && ./pathsense_collector" /etc/rc.local; then
    echo "PathSense-collector is already set to run at boot"
else
    # If not, add the command to the boot file
    sudo sed -i -e '$i \cd /pathsense-collector && ./pathsense_collector\n' /etc/rc.local
    echo "PathSense-collector is now set to run at boot"
fi

echo "PathSense-collector installation complete"
echo "Reboot to start the collector"
