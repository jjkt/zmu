#!/bin/bash
set -e
{ set +x; } 2>/dev/null

echo "building..."
./buildall.sh

##
## GCC none abi / cortex-m examples
##
./test_gcc.sh
#

##
## RustBook examples
##
./test_rustbook.sh

##
## Cortex-M RTIC tests
##
# TODO .. RTIC has changed the examples completely, need actual device simulation support..
#./test_cortex-m-rtic.sh

##
## Coremark
##
./test_coremark.sh

##
## ARM CMSIS DSP
##
# TODO: enable once basic level VFP support is done
# ./test_arm-cmsis-dsp.sh

##
## TODO: Rusty Clock
##
# https://github.com/TeXitoi/rusty-clock.git
