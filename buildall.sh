#!/bin/bash
set -e
{ set +x; } 2>/dev/null

if ! command -v cargo &> /dev/null
then
    echo "Cargo/rust not installed. Please visit https://rustup.rs/"
    exit
fi

if ! command -v cc &> /dev/null
then
    echo "A Gcc compatible 'cc' linker is not installed. If running ubuntu, please run 'sudo apt install build-essential'."
    exit
fi


echo "running library tests..."
cd zmu_cortex_m
echo "TESTING armv6m"
cargo test -q --features "armv6m generic-device" 
echo "TESTING armv7m"
cargo test -q --features "armv7m generic-device"
echo "TESTING armv7em"
cargo test -q --features "armv7em generic-device"
cd ..

cargo build -q --release --no-default-features --features "armv6m generic-device"
cp ./target/release/zmu ./target/release/zmu-armv6m

cargo build -q --release --no-default-features --features "armv7m generic-device"
cp ./target/release/zmu ./target/release/zmu-armv7m

cargo build -q --release --no-default-features --features "armv7em generic-device"
cp ./target/release/zmu ./target/release/zmu-armv7em

cargo build -q --release --no-default-features --features "armv7em stm32f103" 
cp ./target/release/zmu ./target/release/zmu-stm32f103
