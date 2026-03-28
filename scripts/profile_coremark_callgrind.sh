#!/bin/bash
set -euo pipefail

usage() {
    cat <<'EOF'
Usage: scripts/profile_coremark_callgrind.sh [cortex-m0|cortex-m3|cortex-m4] [report-lines]

Builds the matching CoreMark ELF and zmu host binary, runs valgrind callgrind,
then generates a callgrind_annotate report.

Environment:
  GCC_HOME   Path to the Arm GNU toolchain root.
    COREMARK_ITERATIONS  Fixed CoreMark iteration count for stable profiles. Default: 60
    CALLGRIND_CACHE_SIM  Set to 1 to enable cache simulation counters.
    CALLGRIND_BRANCH_SIM Set to 1 to enable branch prediction counters.
    CALLGRIND_EXTRA_ARGS Extra valgrind callgrind arguments.

Examples:
  GCC_HOME=$HOME/.local/arm-gnu-toolchain/15.2.rel1 scripts/profile_coremark_callgrind.sh
  GCC_HOME=$HOME/.local/arm-gnu-toolchain/15.2.rel1 scripts/profile_coremark_callgrind.sh cortex-m4 200
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
fi

TARGET="${1:-cortex-m3}"
REPORT_LINES="${2:-120}"
COREMARK_ITERATIONS="${COREMARK_ITERATIONS:-60}"
CALLGRIND_CACHE_SIM="${CALLGRIND_CACHE_SIM:-0}"
CALLGRIND_BRANCH_SIM="${CALLGRIND_BRANCH_SIM:-0}"
CALLGRIND_EXTRA_ARGS="${CALLGRIND_EXTRA_ARGS:-}"

if [[ -z "${GCC_HOME:-}" ]]; then
    echo "GCC_HOME is undefined"
    exit 1
fi

if ! command -v "$GCC_HOME/bin/arm-none-eabi-gcc" >/dev/null 2>&1; then
    echo "arm-none-eabi-gcc not found under GCC_HOME=$GCC_HOME"
    exit 1
fi

if ! command -v valgrind >/dev/null 2>&1; then
    echo "valgrind is not installed"
    exit 1
fi

if ! command -v callgrind_annotate >/dev/null 2>&1; then
    echo "callgrind_annotate is not available"
    exit 1
fi

case "$TARGET" in
    cortex-m0)
        FEATURES="cortex-m0"
        ZMU_BIN="zmu-cortex-m0"
        ;;
    cortex-m3)
        FEATURES="cortex-m3"
        ZMU_BIN="zmu-cortex-m3"
        ;;
    cortex-m4)
        FEATURES="cortex-m4"
        ZMU_BIN="zmu-cortex-m4"
        ;;
    *)
        echo "unsupported target: $TARGET"
        usage
        exit 1
        ;;
esac

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
COREMARK_DIR="$REPO_ROOT/tests/coremark/coremark"
OUTPUT_DIR="$REPO_ROOT/target/callgrind"
CALLGRIND_OUT="$OUTPUT_DIR/callgrind.coremark.${TARGET}.out"
REPORT_OUT="$OUTPUT_DIR/callgrind.coremark.${TARGET}.annotate.txt"

CALLGRIND_ARGS=(
    --tool=callgrind
    --callgrind-out-file="$CALLGRIND_OUT"
)

ANNOTATE_ARGS=(--auto=yes)

if [[ "$CALLGRIND_CACHE_SIM" == "1" ]]; then
    CALLGRIND_ARGS+=(--cache-sim=yes)
    ANNOTATE_ARGS+=(--show=Ir,I1mr,ILmr,Dr,D1mr,DLmr,Dw,D1mw,DLmw)
fi

if [[ "$CALLGRIND_BRANCH_SIM" == "1" ]]; then
    CALLGRIND_ARGS+=(--branch-sim=yes)
    if [[ "$CALLGRIND_CACHE_SIM" == "1" ]]; then
        ANNOTATE_ARGS+=(--show-percs=yes)
    else
        ANNOTATE_ARGS+=(--show=Ir,Bc,Bcm,Bi,Bim)
    fi
fi

if [[ -n "$CALLGRIND_EXTRA_ARGS" ]]; then
    # shellcheck disable=SC2206
    EXTRA_ARGS=( $CALLGRIND_EXTRA_ARGS )
    CALLGRIND_ARGS+=("${EXTRA_ARGS[@]}")
fi

mkdir -p "$OUTPUT_DIR"

cd "$REPO_ROOT"

if [[ ! -d "$COREMARK_DIR" ]]; then
    git clone https://github.com/eembc/coremark.git "$COREMARK_DIR"
fi

mkdir -p "$COREMARK_DIR/zmu"
cp -f tests/coremark/core_portme.c "$COREMARK_DIR/zmu/"
cp -f tests/coremark/core_portme.h "$COREMARK_DIR/zmu/"
cp -f tests/coremark/core_portme.mak "$COREMARK_DIR/zmu/"
cp -f tests/coremark/link.ld "$COREMARK_DIR/zmu/"
cp -f tests/common.ld tests/coremark/common.ld

echo "Building host emulator for $TARGET"
cargo build -q --release --no-default-features --features "$FEATURES" --bin "$ZMU_BIN"

echo "Building CoreMark ELF for $TARGET"
(
    cd "$COREMARK_DIR"
    make -s PORT_DIR=zmu clean
    make -s PORT_DIR=zmu XCFLAGS="-mcpu=$TARGET" compile
)

echo "Running callgrind"
cd "$REPO_ROOT"
valgrind "${CALLGRIND_ARGS[@]}" \
    "./target/release/$ZMU_BIN" run ./tests/coremark/coremark/coremark.elf 0x0 0x0 0x66 "$COREMARK_ITERATIONS" 7 1 2000

echo "Generating annotated report"
callgrind_annotate "${ANNOTATE_ARGS[@]}" "$CALLGRIND_OUT" > "$REPORT_OUT"

echo
echo "Callgrind output: $CALLGRIND_OUT"
echo "Annotated report: $REPORT_OUT"
echo
head -n "$REPORT_LINES" "$REPORT_OUT"