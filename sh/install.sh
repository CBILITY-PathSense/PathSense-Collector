#!/bin/bash

echo "Installing PathSense-Collector..."

sudo apt-get -y update
sudo apt-get -y install \
  pkg-config \
  build-essential \
  v4l-utils \
  libopencv-dev \
  libclang-dev \
  clang

# Copy the bin folder to the .pathsense-collector directory
sudo mkdir -p /pathsense-collector
sudo cp -r bin/* /pathsense-collector/
echo "PathSense-Collector files are copied to /pathsense-collector"

# Copy the run script to the .pathsense-collector directory
sydo cp -r sh/run.sh /pathsense-collector/
echo "PathSense-Collector run script is copied to /pathsense-collector"

# Check if run command is already in the boot file
if grep -Fxq "/pathsense-collector/run.sh &" /etc/rc.local; then
  echo "PathSense-Collector is already set to run on boot"
else
  # If not, add the command to the boot file
  sudo sed -i -e '$i \/pathsense-collector\/run.sh &' /etc/rc.local
  echo "PathSense-Collector is now set to run on boot"
fi

echo "PathSense-Collector installation complete"
echo "Reboot to start the collector"
