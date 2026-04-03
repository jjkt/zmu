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
cargo test -q --no-default-features --features "armv6m"
echo "TESTING armv7m"
cargo test -q --no-default-features --features "armv7m"
echo "TESTING armv7em"
cargo test -q --no-default-features --features "armv7em"
cd ..

build_product() {
    local feature_set="$1"
    local bin_name="$2"
    echo "building $bin_name with features: $feature_set"
    cargo build -q --release --no-default-features --features "$feature_set" --bin "$bin_name"
}

build_product "cortex-m0" "zmu-cortex-m0"
build_product "cortex-m0plus" "zmu-cortex-m0plus"
build_product "cortex-m3" "zmu-cortex-m3"
build_product "cortex-m4" "zmu-cortex-m4"
build_product "cortex-m4f" "zmu-cortex-m4f"
build_product "cortex-m7-d16" "zmu-cortex-m7-d16"
build_product "cortex-m7-sp-d16" "zmu-cortex-m7-sp-d16"
build_product "stm32f103" "zmu-stm32f103"

