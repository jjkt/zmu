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
./test_cortex-m-rtic.sh

##
## Coremark
##
./test_coremark.sh
