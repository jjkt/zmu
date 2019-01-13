#!/bin/sh

./buildall.sh

#
# hello world
#
cd tests/hello_world
make clean
make
cd ../..
./target/release/zmu-armv6m run tests/hello_world/hello_world-cm0.elf
./target/release/zmu-armv6m run tests/hello_world/hello_world-cm0p.elf
./target/release/zmu-armv7m run tests/hello_world/hello_world-cm3.elf
./target/release/zmu-armv7m run tests/hello_world/hello_world-cm4.elf
./target/release/zmu-armv7m run tests/hello_world/hello_world-cm4f.elf

#
# pi
#
cd tests/pi
make clean
make
cd ../..
./target/release/zmu-armv6m run tests/pi/pi-cm0.elf
./target/release/zmu-armv6m run tests/pi/pi-cm0p.elf
./target/release/zmu-armv7m run tests/pi/pi-cm3.elf
./target/release/zmu-armv7m run tests/pi/pi-cm4.elf
#./target/release/zmu run tests/pi/pi-cm4f.elf

#
# coremark
# 
./target/release/zmu-armv7m run tests/coremark/coremark-iar-cm0.elf
./target/release/zmu-armv7m run tests/coremark/coremark-iar-cm3.elf
./target/release/zmu-armv7m run tests/coremark/coremark-iar-cm4.elf


#
# rust hello world
#
cd tests/hello_world_rust
cargo build --example hello
cd ../..
./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/hello
./target/release/zmu-armv7m run tests/rustbook/target/thumbv7m-none-eabi/debug/examples/exception
