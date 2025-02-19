# PathSense-Collector  

This repository contains all the necessary binaries to run the PathSense Collector on an OrangePi running Debian.  

## Running the System Manually (Recommended Before Installation)  

1. Navigate to the `bin` directory and execute the binary:  

   ```sh
   cd bin
   ./pathsense_collector
   ```  

2. Upon successful execution, a new directory will be created at:  

   ```
   $HOME/pathsense_images_xxxxxx
   ```  

   where `xxxxxx` is a randomly generated number.  

### Troubleshooting: Processor Architecture Mismatch  

If the binary fails to run, it may be due to a processor architecture mismatch. In this case, you can manually build the system instead. See the [Building the System Manually](#building-the-system-manually) section.

## Installation  

Before proceeding with the installation, ensure that the system runs successfully in manual mode.  

1. Navigate to the `sh` directory and run the installation script:  

   ```sh
   cd sh
   ./install.sh
   ```  

This will install the necessary components and set up the PathSense Collector for automated execution.  

## Building the System Manually  

1. Ensure your system meets the following requirements:  

  - **Rust** version **1.75 or higher** installed.  
  - At least **4GB of RAM** available.  
  - A **Debian-based** operating system (e.g., Debian, Ubuntu, Raspbian, Pop!_OS, etc.).  
  - The build scripts rely on `apt`, so if your system uses a different package manager, you will need to install dependencies manually.
    
2. Navigate to the `source` directory and execute the build script:  

   ```sh
   cd source
   ./build.sh
   cd ..
   ```  
