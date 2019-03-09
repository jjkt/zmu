#!/bin/sh

echo "running library tests..."
cd zmu_cortex_m
echo "TESTING armv6m"
cargo test --features armv6m
echo "TESTING armv7m"
cargo test --features armv7m
echo "TESTING armv7em"
cargo test --features armv7em
cd ..

cargo build --release --no-default-features --features "armv6m"
cp ./target/release/zmu ./target/release/zmu-armv6m

cargo build --release --no-default-features --features "armv7m"
cp ./target/release/zmu ./target/release/zmu-armv7m

cargo build --release --no-default-features --features "armv7em"
cp ./target/release/zmu ./target/release/zmu-armv7em
