#!/bin/bash

echo "Stopping PathSense-collector..."

# Check if pathsense-collector is in boot
if grep -Fxq "cd /pathsense-collector && ./pathsense_collector" /etc/rc.local; then
    # Remove pathsense-collector from boot
    sudo sed -i '/cd /pathsense-collector && .\/pathsense_collector/d' /etc/rc.local
    echo "PathSense-collector is removed from boot"
else
    echo "PathSense-collector is already not in boot"
fi

echo "PathSense-collector stop complete"
echo "To start the collector again, run this command: pathsense-tools update"
