# PathSense-Collector

This repository contains all binaries required to run the PathSense Collector on an OrangePi. The collector is designed to be installed on OrangePi Zero3 devices running Debian. Other devices and operating system may work, but are not officially supported.

## Installation

If automatic installation fails or you are not using an OrangePi device running Debian, you can manually build the system instead.
To do this, make sure that:

- You have **Rust** version **1.75 or higher** installed.
- Your device has at least **4GB** of RAM.
- Your device is running a **Debian-based** operating system (e.g. Debian, Ubuntu, Raspbian, Pop! etc.) of any version. This is because the build scripts depend on apt. If your device uses other package managers, you will need to manually install all of the required dependencies.

Then, navigate to the 'source' directory, and run the build script.

```sh
cd source
./build.sh
cd ..
```

After that, you should be able to install the system using the **pathsense-collector** commands.

```sh
cd sh
./install.sh
```


