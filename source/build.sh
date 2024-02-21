rm -d -r ../bin
rm pathsense-collector
mkdir -p ../bin
sudo apt-get -y update
sudo apt-get -y install \
    build-essential \
    libopencv-dev \
    libclang-dev \
    clang
cd pathsense_collector
cargo build --release
cp target/release/pathsense_collector ../../bin/pathsense_collector
cd ..
rustc pathsense-collector.rs
cp pathsense-collector ../pathsense-collector
cd ..
chmod +x pathsense-collector
