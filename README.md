# PathSense-collector

This repository contains all binaries required to run the PathSense collector on an Orange Pi. The collector is designed to be installed on Orange Pi Zero 3 devices running Debian. Other devices and operating collectors may work, but are not officially supported.

## Installation

Clone this repository and navigate to the directory
```sh
git clone git@github.com:JiraPit/CMKL-PathSense-DataCollector.git
cd CMKL-PathSense-DataCollector
```

Run pathsense-collector install
```sh
./pathsense-collector install
```

## Updating

To update, pool the latest changes and run the update command
```sh
git pull
./pathsense-collector update
```

## Removing and Stopping

To remove the collector, run the remove command.
```sh
./pathsense-collector remove
```
This will remove all installed collector files and stop the collector from booting.

You can also stop the collector from booting without removing the collector files. To do this, run the stop command.
```sh
./pathsense-collector stop
```
To start the collector again, simply run the update command.
