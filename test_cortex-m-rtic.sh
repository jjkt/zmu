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


declare -a arr=("big-struct-opt" "binds" "capacity" "cancel-reschedule" "cfg-whole-task"  "common" "complex" "declared_locals" "destructure" "extern_binds" "extern_spawn" "generics" "hardware" "idle-wfi" "idle" "init" "locals" "lock-free" "lock" "message_passing" "message" "multilock" "not-sync" "only-shared-access" "periodic-at" "periodic-at2" "periodic" "pool" "preempt" "resource-user-struct" "schedule" "shared" "smallest" "spawn" "static" "t-binds" "t-cfg-resources" "t-htask-main" "t-idle-main" "t-late-not-send" "t-schedule" "t-spawn" "task" )
cd tests/cortex-m-rtic
cargo build --target thumbv7m-none-eabi
for i in "${arr[@]}"
do
   echo "cargo build --target thumbv7m-none-eabi --example $i"
   cargo build --target thumbv7m-none-eabi --example $i
done
cd ../..

for i in "${arr[@]}"
do
   echo "./target/release/zmu-armv7m run tests/cortex-m-rtic/target/thumbv7m-none-eabi/debug/examples/$i"
   timeout 1s ./target/release/zmu-armv7m run tests/cortex-m-rtic/target/thumbv7m-none-eabi/debug/examples/$i || true
done