#!/bin/bash

echo "Installing PathSense collector script..."

sudo apt-get update
sudo apt-get upgrade

sudo apt-get install v4l-utils -y
sudo apt-get install python3-dev -y
sudo apt-get install python3-opencv -y
sudo apt-get install python3-requests -y
sudo apt-get install unzip -y
sudo apt-get install wget -y

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
    echo "PathSense collector script is now set to run at boot"
fi

echo "PathSense collector script installation complete"
echo "Reboot to start the collector script"
