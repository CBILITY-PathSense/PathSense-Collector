rm -d -r ../bin
mkdir -p ../bin
cargo build --release
cp target/release/pathsense_collector ../bin/pathsense_collector