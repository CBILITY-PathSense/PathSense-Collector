#!/bin/bash

echo "Removing PathSense collector script from boot..."

# Check if the command is present in rc.local
if grep -Fxq "cd /CollectorScript && sudo python main.py &" /etc/rc.local
then
    # Remove the line containing the command
    sudo sed -i '/cd \/CollectorScript && sudo python main.py &/d' /etc/rc.local
    echo "PathSense collector script removed from boot"
else
    echo "PathSense collector script is not set to run at boot"
fi
