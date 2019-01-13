#!/bin/sh

cargo build --release --no-default-features --features "armv6m"
cp ./target/release/zmu ./target/release/zmu-armv6m

cargo build --release --no-default-features --features "armv7m"
cp ./target/release/zmu ./target/release/zmu-armv7m
