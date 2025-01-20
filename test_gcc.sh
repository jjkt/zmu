#!/bin/bash
set -e
{ set +x; } 2>/dev/null

if [ -z "$GCC_HOME" ]; then
    echo "GCC_HOME is undefined"
    exit
fi

CC=$GCC_HOME/bin/arm-none-eabi-gcc

if ! command -v $CC &> /dev/null
then
    echo "GCC for ARM is not installed. Please install from developer.arm.com"
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
      cores=("cm0" "cm0p" "cm3" "cm4" "cm4f" "cm7-d16" "cm7-sp-d16")
      ;;
   esac
}

declare -a archs=("armv6m" "armv7m" "armv7em")
declare -a gcc_tests=("hello_world" "instruction-test-bench" "pi")

for i in "${gcc_tests[@]}"
do
   cd tests/$i
   make -s clean
   make
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
         # read return code and abort on failure:
         ./target/release/zmu-$a run tests/$i/$i-$c.elf
         if [[ $? -ne 0 ]]; then
            echo "Test failed"
            exit $?
         fi
         echo ""
      done
   done
done
