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


function ensure_itmdump()
{
   if command -v itmdump &> /dev/null; then
      ITMDUMP_BIN=$(command -v itmdump)
      return
   fi

   if [ -x "$HOME/.cargo/bin/itmdump" ]; then
      ITMDUMP_BIN="$HOME/.cargo/bin/itmdump"
      return
   fi

   if ! command -v cargo &> /dev/null; then
      echo "itmdump is required for ITM tests and cargo is not available to install it"
      exit 1
   fi

   cargo install itm

   if command -v itmdump &> /dev/null; then
      ITMDUMP_BIN=$(command -v itmdump)
      return
   fi

   if [ -x "$HOME/.cargo/bin/itmdump" ]; then
      ITMDUMP_BIN="$HOME/.cargo/bin/itmdump"
      return
   fi

   echo "Failed to locate itmdump after installing itm"
   exit 1
}


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


function expected_stdout()
{
   case "$1" in
   "hello_world")
      printf 'hello, world'
      ;;
   "hello_world_itm")
      printf 'Hello, world!'
      ;;
   *)
      return 1
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
         if expected=$(expected_stdout "$i"); then
            output=$(./target/release/zmu-$a run tests/$i/$i-$c.elf)
            status=$?
            echo "$output"
            if [[ $status -ne 0 ]]; then
               echo "Test failed"
               exit $status
            fi
            if [[ "$output" != "$expected" ]]; then
               echo "Unexpected stdout"
               exit 1
            fi
         else
            ./target/release/zmu-$a run tests/$i/$i-$c.elf
            if [[ $? -ne 0 ]]; then
               echo "Test failed"
               exit $?
            fi
         fi
         echo ""
      done
   done
done

ensure_itmdump

cd tests/hello_world_itm
make -s clean
make
cd ../..

for a in "${archs[@]}"
do
   echo -e "\e[1m========================================"
   echo -e "\e[1mGCC TEST: hello_world_itm / $a"
   echo -e "\e[1m========================================\e[0m"
   arch_supports_cores $a
   for c in "${cores[@]}"
   do
      echo "./target/release/zmu-$a run --itm /dev/stdout tests/hello_world_itm/hello_world_itm-$c.elf | $ITMDUMP_BIN"
      expected=$(expected_stdout "hello_world_itm")
      output=$(./target/release/zmu-$a run --itm /dev/stdout tests/hello_world_itm/hello_world_itm-$c.elf | "$ITMDUMP_BIN")
      status=$?
      echo "$output"
      if [[ $status -ne 0 ]]; then
         echo "ITM test failed"
         exit $status
      fi
      if [[ "$output" != "$expected" ]]; then
         echo "Unexpected ITM output"
         exit 1
      fi
      echo ""
   done
done
