#!/bin/bash

echo "Setting PathSense collector script to run on boot..."

# Copy the CollectorScript folder to the root directory
sudo cp -r CollectorScript /

echo "PathSense collector script added to root directory"

# Check if main.py is already in the boot file
if grep -Fxq "cd /CollectorScript && sudo python main.py &" /etc/rc.local
then
    echo "PathSense collector script is already set to run at boot"
else
    # If not, add the command to the boot file
    sed -i '$i cd /CollectorScript && sudo python main.py &' /etc/rc.local
    echo "PathSense system script is now set to run at boot"
fi

echo "Reboot to start the collector script"
