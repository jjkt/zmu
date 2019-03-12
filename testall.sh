#!/bin/bash
{ set +x; } 2>/dev/null

echo "building..."
./buildall.sh



#
# hello world
#
echo ""
echo "========================================"
echo "TEST: Hello world with GCC ARM toolchain"
echo "========================================"
cd tests/hello_world
make -s clean
make -s
cd ../..
echo "armv6m->cm0"
echo "----------------------------------------"
./target/release/zmu-armv6m run tests/hello_world/hello_world-cm0.elf
echo "armv6m->cm0p"
echo "----------------------------------------"
./target/release/zmu-armv6m run tests/hello_world/hello_world-cm0p.elf
echo "armv7m->cm3"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/hello_world/hello_world-cm3.elf
echo "armv7m->cm4"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/hello_world/hello_world-cm4.elf
echo "armv7m->cm4"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/hello_world/hello_world-cm4f.elf

#
# pi
#
echo ""
echo "========================================"
echo "TEST: PI calculation with GCC ARM toolchain"
echo "========================================"
cd tests/pi
make -s clean
make -s
cd ../..
echo "armv6m->cm0"
echo "----------------------------------------"
./target/release/zmu-armv6m run tests/pi/pi-cm0.elf
echo "armv6m->cm0"
echo "----------------------------------------"
./target/release/zmu-armv6m run tests/pi/pi-cm0p.elf
echo "armv7m->cm3"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/pi/pi-cm3.elf
echo "armv7m->cm4"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/pi/pi-cm4.elf
#./target/release/zmu run tests/pi/pi-cm4f.elf



#
# RustBook examples
#
echo ""
echo "========================================"
echo "TEST: Rustbook examples, compiled with Rust"
echo "========================================"
cd tests/rustbook
cargo build --example hello
cargo build --example exception
cargo build --example itm
cargo build --example crash

cd ../..
echo "armv7m->hello"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/hello
echo "armv7m->itm"
echo "----------------------------------------"
timeout 1s ./target/release/zmu-armv7m run --itm /dev/stdout tests/rustbook/target/thumbv7m-none-eabi/debug/examples/itm | itmdump
echo "armv7m->exception"
echo "----------------------------------------"
timeout 1s ./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/exception
echo "armv7m->crash"
echo "----------------------------------------"
timeout 1s ./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/crash

echo ""
echo "========================================"
echo "TEST: cortex-m-rtfm crate examples"
echo "========================================"

#
# TODO: not yet working examples: 
# - "capacity" => sometimes randomly panics
# - "late" => "0xf3bf8f5f" instruction not decoded correctly
# - "ramfunc" => ramfuncs currently not supported by the emulator because of the caching
# - "singleton" => sometimes randomly panics
# - "static" => randomly triggers one or two prints (priority issue?)

declare -a arr=("baseline" "binds" "generics" "idle" "init" "interrupt" "lock" "message" "not-send" "not-sync" "resource" "smallest" "task" "schedule" "periodic")
cd tests/cortex-m-rtfm
cargo build
for i in "${arr[@]}"
do
   cargo build --features="timer-queue" --example $i
done
cd ../..

for i in "${arr[@]}"
do
   echo "armv7m->cortex-m-rtfm examples/$i"
   timeout 2s ./target/release/zmu-armv7m run tests/cortex-m-rtfm/target/thumbv7m-none-eabi/debug/examples/$i
done



#
# coremark
# 
echo ""
echo "========================================"
echo "TEST: Coremark with IAR toolchain"
echo "========================================"
echo "armv6m->cm0"
echo "----------------------------------------"
./target/release/zmu-armv6m run tests/coremark/coremark-iar-cm0.elf
echo "armv7m->cm3"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/coremark/coremark-iar-cm3.elf
echo "armv7m->cm4"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/coremark/coremark-iar-cm4.elf

echo ""
echo "all done"
set -x;