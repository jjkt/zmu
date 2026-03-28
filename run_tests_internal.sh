#!/bin/bash
set -e
{ set +x; } 2>/dev/null

echo "building..."
./buildall.sh

##
## GCC none abi / cortex-m examples
##
./test_gcc.sh

##
## Fault trap matrix
##
./test_faults.sh

##
## Coremark
##
./test_coremark.sh

##
## Lilos
##
./test_lilos.sh

##
## ARM CMSIS DSP
./test_arm-cmsis-dsp.sh

##
## TODO: Rusty Clock
##
# https://github.com/TeXitoi/rusty-clock.git
