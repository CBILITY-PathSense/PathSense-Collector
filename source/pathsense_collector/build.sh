rm -d -r ../bin
mkdir -p ../bin
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install \
    build-essential \
    libopencv-dev \
    libclang-dev \
    clang
cargo build --release
cp target/release/pathsense_collector ../bin/pathsense_collector