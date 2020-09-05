#!/bin/bash
set -e
{ set +x; } 2>/dev/null

# cortex-m0 cortex-m0.small-multiply cortex-m0plus cortex-m0plus.small-multiply cortex-m1 cortex-m1.small-multiply 
# cortex-m23 cortex-m3 cortex-m33 cortex-m33+nodsp cortex-m4 cortex-m7

if ! command -v arm-none-eabi-gcc &> /dev/null
then
    echo "GCC for ARM is not installed. Please install with: 'sudo apt install gcc-arm-none-eabi'"
    exit
fi


declare -a targets=("cortex-m0" "cortex-m3" "cortex-m4")

if [ ! -d "tests/coremark/coremark" ] ; then
   git clone https://github.com/eembc/coremark.git  tests/coremark/coremark
fi

if [ ! -d "tests/coremark/coremark/zmu" ] ; then
    mkdir tests/coremark/coremark/zmu
    cp -f tests/coremark/core_portme.c tests/coremark/coremark/zmu/
    cp -f tests/coremark/core_portme.h tests/coremark/coremark/zmu/
    cp -f tests/coremark/core_portme.mak tests/coremark/coremark/zmu/
    cp -f tests/coremark/link.ld tests/coremark/coremark/zmu/
fi

cd tests/coremark/coremark

for i in "${targets[@]}"
do
   echo -e "\e[1m========================================"
   echo -e "\e[1mCOREMARK GCC: $i"
   echo -e "\e[1m========================================\e[0m"
   make -s PORT_DIR=zmu clean
   make -s PORT_DIR=zmu XCFLAGS="-mcpu=$i"
   cp run1.log "run1_$i.log"
   cp run2.log "run2_$i.log"
   cat run2.log
   rm -f run1.log
   rm -f run2.log
done
