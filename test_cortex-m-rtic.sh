#!/bin/bash
set -e
{ set +x; } 2>/dev/null

echo ""
echo -e "\e[1m========================================"
echo -e "\e[1mTEST: cortex-m-rtic crate examples"
echo -e "\e[1m========================================\e[0m"

if [ ! -d "tests/cortex-m-rtic" ] ; then
   git clone https://github.com/rtic-rs/cortex-m-rtic.git tests/cortex-m-rtic
else
    cd "tests/cortex-m-rtic"
    git pull https://github.com/rtic-rs/cortex-m-rtic.git
    cd ..
    cd ..
fi


# 
#  
declare -a arr=("baseline" "binds" "capacity" "cfg"  "destructure" "generics" "hardware" "idle" "init" "late" "lock" "message" "not-sync" "only-shared-access" "periodic" "pool" "preempt" "resource" "schedule"  "smallest" "shared-with-init" "spawn" "spawn2" "static" "task" "task-local" "task-local-minimal" "types")
cd tests/cortex-m-rtic
cargo build --target thumbv7m-none-eabi
for i in "${arr[@]}"
do
   echo "cargo build --target thumbv7m-none-eabi --features="__v7" --example $i"
   cargo build --target thumbv7m-none-eabi --features="__v7" --example $i
done
cd ../..

for i in "${arr[@]}"
do
   echo "./target/release/zmu-armv7m run tests/cortex-m-rtic/target/thumbv7m-none-eabi/debug/examples/$i"
   timeout 1s ./target/release/zmu-armv7m run tests/cortex-m-rtic/target/thumbv7m-none-eabi/debug/examples/$i || true
done