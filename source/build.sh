sudo apt-get -y update
sudo apt-get -y install \
  pkg-config \
  build-essential \
  v4l-utils \
  libopencv-dev \
  libclang-dev \
  clang

sudo chmod 666 /dev/video*

sudo rm -d -r ../bin
sudo rm pathsense-collector

mkdir -p ../bin
cd pathsense_collector
cargo build --release

cp target/release/pathsense_collector ../../bin/pathsense_collector
cd ..

rustc pathsense-collector.rs
cp pathsense-collector ../pathsense-collector
cd ..

sudo chmod +x pathsense-collector
