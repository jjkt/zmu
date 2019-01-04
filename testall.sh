#!/bin/sh


cd tests/hello_world
make clean
make
cd ../..
./target/release/zmu run tests/hello_world/hello_world-cm0.elf
./target/release/zmu run tests/hello_world/hello_world-cm0p.elf
./target/release/zmu run tests/hello_world/hello_world-cm3.elf
./target/release/zmu run tests/hello_world/hello_world-cm4.elf
./target/release/zmu run tests/hello_world/hello_world-cm4f.elf
