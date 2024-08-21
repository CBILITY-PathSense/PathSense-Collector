#!/bin/bash

echo "Removing PathSense-Collector..."

# Remove ./pathsense-collector from root
sudo rm -d -r /pathsense-collector
echo "PathSense-Collector files are removed"

# Check if pathsense-collector is in boot
if grep -Fxq "/pathsense-collector/run.sh &" /etc/rc.local; then
  # Remove pathsense-system from boot
  sudo sed -i '\/pathsense-collector\/run.sh &/d' /etc/rc.local
  echo "PathSense-Collector is removed from boot"
fi

echo "PathSense-Collector removal complete"
