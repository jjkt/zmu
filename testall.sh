#!/bin/bash
set -e
{ set +x; } 2>/dev/null

echo "building..."
./buildall.sh > test.log


function arch_supports_cores()
{
   case "$1" in
   "armv6m") 
      cores=("cm0" "cm0p")
      ;;
   "armv7m") 
      cores=("cm0" "cm0p" "cm3")
      ;;
   "armv7em") 
      cores=("cm0" "cm0p" "cm3" "cm4" "cm4f")
      ;;
   esac
}

declare -a archs=("armv6m" "armv7m" "armv7em")
declare -a gcc_tests=("hello_world" "pi" "instruction-test-bench")

for i in "${gcc_tests[@]}"
do
   cd tests/$i
   make -s clean 1> test.log
   make -s 1> test.log
   cd ../..
   echo "========================================"
   echo "TEST: $i"
   echo "========================================"
   for a in "${archs[@]}"
   do
      arch_supports_cores $a
      for c in "${cores[@]}"
      do
         echo "./target/release/zmu-$a run tests/$i/$i-$c.elf"
         ./target/release/zmu-$a run tests/$i/$i-$c.elf
         echo ""
      done
   done
done


git clone https://github.com/rust-embedded/cortex-m-quickstart tests/rustbook

sed -i 's/{{authors}}/jjkt/g' tests/rustbook/Cargo.toml
sed -i 's/{{project-name}}/zmu-tests/g' tests/rustbook/Cargo.toml
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
#
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
#
echo ""
echo "========================================"
echo "TEST: cortex-m-rtfm crate examples"
echo "========================================"

git clone https://github.com/rtfm-rs/cortex-m-rtfm.git tests/cortex-m-rtfm
#
##
## TODO: not yet working examples:
## - "ramfunc" => ramfuncs currently not supported by the emulator because of the caching
#
declare -a arr=("baseline" "binds" "capacity" "cfg" "destructure" "generics" "hardware" "idle" "init" "late" "lock" "message" "not-send" "not-sync" "only-shared-access" "periodic" "pool" "preempt" "resource" "smallest" "schedule" "shared-with-init" "task" "types")
cd tests/cortex-m-rtfm
cargo build --target thumbv7m-none-eabi
for i in "${arr[@]}"
do
   cargo build --target thumbv7m-none-eabi --features="__v7" --example $i
done
cd ../..

for i in "${arr[@]}"
do
   echo "armv7m->cortex-m-rtfm examples/$i"
   timeout 2s ./target/release/zmu-armv7m run tests/cortex-m-rtfm/target/thumbv7m-none-eabi/debug/examples/$i
done



##
## coremark
##
#echo ""
#echo "========================================"
#echo "TEST: Coremark with IAR toolchain"
#echo "========================================"
#echo "armv6m->cm0"
#echo "----------------------------------------"
#./target/release/zmu-armv6m run tests/coremark/coremark-iar-cm0.elf
#echo "armv7m->cm3"
#echo "----------------------------------------"
#./target/release/zmu-armv7m run tests/coremark/coremark-iar-cm3.elf
#echo "armv7m->cm4"
#echo "----------------------------------------"
#./target/release/zmu-armv7m run tests/coremark/coremark-iar-cm4.elf
#
#echo ""
#echo "all done"
#set -x;
