#!/bin/bash
set -euo pipefail
{ set +x; } 2>/dev/null

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
source "$SCRIPT_DIR/scripts/test_output.sh"

if [ -z "${GCC_HOME:-}" ]; then
    echo "GCC_HOME is undefined"
    exit
fi

CC=$GCC_HOME/bin/arm-none-eabi-gcc

if ! command -v "$CC" &> /dev/null
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


function core_runner()
{
   case "$1" in
   "cm0")
      printf './target/release/zmu-cortex-m0'
      ;;
   "cm0p")
      printf './target/release/zmu-cortex-m0plus'
      ;;
   "cm3")
      printf './target/release/zmu-cortex-m3'
      ;;
   "cm4")
      printf './target/release/zmu-cortex-m4'
      ;;
   "cm4f")
      printf './target/release/zmu-cortex-m4f'
      ;;
   "cm7-d16")
      printf './target/release/zmu-cortex-m7-d16'
      ;;
   "cm7-sp-d16")
      printf './target/release/zmu-cortex-m7-sp-d16'
      ;;
   *)
      echo "Unknown core: $1" >&2
      exit 1
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
declare -a gcc_tests=("hello_world" "instruction-test-bench" "pi" "minimal")

for i in "${gcc_tests[@]}"
do
   cd tests/$i
   make -s clean
   make -s
   cd ../..
   for a in "${archs[@]}"
   do
      arch_supports_cores $a
      for c in "${cores[@]}"
      do
         runner=$(core_runner "$c")
         label="$i/$a/$c"
         if expected=$(expected_stdout "$i"); then
            test_expect_success_exact "$label" "$expected" "$runner" run "tests/$i/$i-$c.elf"
         else
            test_expect_success_contains "$label" "" "$runner" run "tests/$i/$i-$c.elf"
         fi
      done
   done
done

ensure_itmdump

cd tests/hello_world_itm
make -s clean
make -s
cd ../..

for a in "${archs[@]}"
do
   arch_supports_cores $a
   for c in "${cores[@]}"
   do
      runner=$(core_runner "$c")
      label="hello_world_itm/$a/$c"
      expected=$(expected_stdout "hello_world_itm")
      test_expect_success_exact \
         "$label" \
         "$expected" \
         bash -o pipefail -c '"$1" run --itm /dev/stdout "$2" | "$3"' _ "$runner" "tests/hello_world_itm/hello_world_itm-$c.elf" "$ITMDUMP_BIN"
   done
done

test_print_summary "gcc test bench passed"
