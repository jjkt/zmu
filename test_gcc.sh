#!/bin/bash
set -e
{ set +x; } 2>/dev/null

if ! command -v arm-none-eabi-gcc &> /dev/null
then
    echo "GCC for ARM is not installed. Please install with: 'sudo apt install gcc-arm-none-eabi'"
    exit
fi


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
   make -s clean
   make -s
   cd ../..
   for a in "${archs[@]}"
   do
      echo -e "\e[1m========================================"
      echo -e "\e[1mGCC TEST: $i / $a"
      echo -e "\e[1m========================================\e[0m"
      arch_supports_cores $a
      for c in "${cores[@]}"
      do
         echo "./target/release/zmu-$a run tests/$i/$i-$c.elf"
         ./target/release/zmu-$a run tests/$i/$i-$c.elf
         echo ""
      done
   done
done
