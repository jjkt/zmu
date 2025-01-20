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


make -s RUNNER="../../target/release/zmu-armv6m run" TARGET=cm0 XCFLAGS="-mcpu=cortex-m0" run
make -s RUNNER="../../target/release/zmu-armv6m run" TARGET=cm0p XCFLAGS="-mcpu=cortex-m0plus" run
make -s RUNNER="../../target/release/zmu-armv7m run" TARGET=cm3 XCFLAGS="-mcpu=cortex-m3"  run
make -s RUNNER="../../target/release/zmu-armv7m run" TARGET=cm4 XCFLAGS="-mcpu=cortex-m4" run
make -s RUNNER="../../target/release/zmu-armv7em run" TARGET=cm4f XCFLAGS="-mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16" run
make -s RUNNER="../../target/release/zmu-armv7em run" TARGET=cm7-d16 XCFLAGS="-mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-d16" run
make -s RUNNER="../../target/release/zmu-armv7em run" TARGET=cm7-sp-d16 XCFLAGS="-mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-sp-d16" run
