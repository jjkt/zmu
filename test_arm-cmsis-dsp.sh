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


if [ ! -d "tests/cmsis/CMSIS-DSP" ] ; then
   git clone https://github.com/ARM-software/CMSIS-DSP.git  tests/cmsis/CMSIS-DSP
fi

if [ ! -d "tests/cmsis/CMSIS_5" ] ; then
   git clone https://github.com/ARM-software/CMSIS_5  tests/cmsis/CMSIS_5
fi

cd tests/cmsis

run_target() {
    local runner="$1"
    local target="$2"
    local xcflags="$3"

    make -s TARGET="$target" clean
    make -s RUNNER="$runner" TARGET="$target" XCFLAGS="$xcflags" run
}

run_target "../../target/release/zmu-armv6m run" cm0 "-mcpu=cortex-m0"
run_target "../../target/release/zmu-armv6m run" cm0p "-mcpu=cortex-m0plus"
run_target "../../target/release/zmu-armv7m run" cm3 "-mcpu=cortex-m3"
run_target "../../target/release/zmu-armv7m run" cm4 "-mcpu=cortex-m4"
run_target "../../target/release/zmu-armv7em run" cm4f "-mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16"
run_target "../../target/release/zmu-armv7em run" cm7-d16 "-mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-d16"
run_target "../../target/release/zmu-armv7em run" cm7-sp-d16 "-mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-sp-d16"
