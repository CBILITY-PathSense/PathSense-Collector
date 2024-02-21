# PathSense-collector
This repository contains all binaries required to run the PathSense collector on an OrangePi. The collector is designed to be installed on OrangePi Zero3 devices running Debian. Other devices and operating system may work, but are not officially supported.

## Automatic Installation (OrangePi only)
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
To update, pull the latest changes and run the update command
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

## Manual Installation (for non-OrangePi / non-aarch64 devices)
For non-OrangePi or non-aarch64 devices, you might have to manually build the binary from source instead.
To do this, make sure that:
- Your device is running a Debian-based operating system (e.g. Ubuntu, Raspbian, etc.)
- You have Rust installed on your device

Then, clone this repository, navigate to the 'source' directory, and run the build script.
```sh
git clone https://github.com/JiraPit/CMKL-PathSense-DataCollector.git
cd CMKL-PathSense-DataCollector/source
./build.sh
cd ..
```

After that, you should be able to install the collector using the automatic installation instructions above.


