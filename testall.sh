#!/bin/sh
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


#
# rust hello world
#
echo ""
echo "========================================"
echo "TEST: Rustbook examples, compiled with Rust"
echo "========================================"
cd tests/rustbook
cargo build --example hello
cargo build --example exception
cargo build --example itm
cd ../..
echo "armv7m->hello"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/hello
echo "armv7m->itm"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/itm
echo "armv7m->exception"
echo "----------------------------------------"
./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/exception

set -x;