#!/bin/bash
set -e
{ set +x; } 2>/dev/null

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)

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


declare -a targets=("cm0" "cm3" "cm4f")

core_xcflags() {
    case "$1" in
    "cm0")
        printf '%s' '-mcpu=cortex-m0'
        ;;
    "cm3")
        printf '%s' '-mcpu=cortex-m3'
        ;;
    "cm4f")
        printf '%s' '-mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16'
        ;;
    *)
        echo "Unknown core: $1" >&2
        exit 1
        ;;
    esac
}

core_runner() {
    case "$1" in
    "cm0")
        printf '%s' '../../../target/release/zmu-cortex-m0'
        ;;
    "cm3")
        printf '%s' '../../../target/release/zmu-cortex-m3'
        ;;
    "cm4f")
        printf '%s' '../../../target/release/zmu-cortex-m4f'
        ;;
    *)
        echo "Unknown core: $1" >&2
        exit 1
        ;;
    esac
}

if [ ! -d "$SCRIPT_DIR/tests/coremark/coremark" ] ; then
   git clone https://github.com/eembc/coremark.git "$SCRIPT_DIR/tests/coremark/coremark"
fi

mkdir -p "$SCRIPT_DIR/tests/coremark/coremark/zmu"
cp -f "$SCRIPT_DIR/tests/coremark/core_portme.c" "$SCRIPT_DIR/tests/coremark/coremark/zmu/"
cp -f "$SCRIPT_DIR/tests/coremark/core_portme.h" "$SCRIPT_DIR/tests/coremark/coremark/zmu/"
cp -f "$SCRIPT_DIR/tests/coremark/core_portme.mak" "$SCRIPT_DIR/tests/coremark/coremark/zmu/"
cp -f "$SCRIPT_DIR/tests/coremark/link.ld" "$SCRIPT_DIR/tests/coremark/coremark/zmu/"
cp -f "$SCRIPT_DIR/tests/common.ld" "$SCRIPT_DIR/tests/coremark/common.ld"

cd "$SCRIPT_DIR/tests/coremark/coremark"

for i in "${targets[@]}"
do
   xcflags=$(core_xcflags "$i")
   runner=$(core_runner "$i")

   echo -e "\e[1m========================================"
   echo -e "\e[1mCOREMARK GCC: $i"
   echo -e "\e[1m========================================\e[0m"
   make -s PORT_DIR=zmu clean
   make -s PORT_DIR=zmu XCFLAGS="$xcflags" ZMU="$runner"
   cp run1.log "run1_$i.log"
   cp run2.log "run2_$i.log"
   cat run2.log
   rm -f run1.log
   rm -f run2.log
done
