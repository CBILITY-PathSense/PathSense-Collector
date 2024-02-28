#!/bin/bash

echo "Removing PathSense-collector..."

# Remove ./pathsense-collector from root
sudo rm -d -r /pathsense-collector
echo "PathSense-collector files is removed"

# Check if pathsense-collector is in boot
if grep -Fxq "cd /pathsense-collector && ./pathsense_collector &" /etc/rc.local; then
    # Remove pathsense-collector from boot
    sudo sed -i '/cd /pathsense-collector && .\/pathsense_collector &/d' /etc/rc.local
    echo "PathSense-collector is removed from boot"
fi

echo "PathSense-collector removal complete"
