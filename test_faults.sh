#!/bin/bash
set -euo pipefail
{ set +x; } 2>/dev/null

if [ -z "${GCC_HOME:-}" ]; then
    echo "GCC_HOME is undefined"
    exit 1
fi

if ! command -v "$GCC_HOME/bin/arm-none-eabi-gcc" >/dev/null 2>&1; then
    echo "GCC for ARM is not installed. Please install from developer.arm.com"
    exit 1
fi

build_bench() {
    pushd tests/fault-test-bench >/dev/null
    make -s clean
    make -s
    popd >/dev/null
}

run_expect_success_contains() {
    local expected="$1"
    shift
    local output

    output=$("$@" 2>&1)
    echo "$output"

    if [[ "$output" != *"$expected"* ]]; then
        echo "Expected output to contain: $expected"
        exit 1
    fi
}

run_expect_failure_contains() {
    local expected="$1"
    shift
    local output

    set +e
    output=$("$@" 2>&1)
    local status=$?
    set -e

    echo "$output"

    if [[ $status -eq 0 ]]; then
        echo "Expected command to fail"
        exit 1
    fi

    if [[ "$output" != *"$expected"* ]]; then
        echo "Expected output to contain: $expected"
        exit 1
    fi
}

combo_flags_armv7m() {
    local -n out_flags="$1"
    local hardfault="$2"
    local memmanage="$3"
    local usagefault="$4"

    out_flags=()

    if [[ "$hardfault" == 0 ]]; then
        out_flags+=(--no-trap hardfault)
    fi
    if [[ "$memmanage" == 1 ]]; then
        out_flags+=(--trap memmanage)
    fi
    if [[ "$usagefault" == 1 ]]; then
        out_flags+=(--trap usagefault)
    fi
}

hardfault_flags_armv6m() {
    local -n out_flags="$1"
    local hardfault="$2"

    out_flags=()

    if [[ "$hardfault" == 0 ]]; then
        out_flags+=(--no-trap hardfault)
    fi
}

run_combo_matrix() {
    local usage_elf="tests/fault-test-bench/fault-usage-cm3.elf"
    local memmanage_elf="tests/fault-test-bench/fault-memmanage-cm3.elf"
    local hardfault_elf="tests/fault-test-bench/fault-hardfault-cm0.elf"
    local -a armv7m_flags
    local -a armv6m_flags

    for hardfault in 0 1; do
        for memmanage in 0 1; do
            for usagefault in 0 1; do
                combo_flags_armv7m armv7m_flags "$hardfault" "$memmanage" "$usagefault"
                hardfault_flags_armv6m armv6m_flags "$hardfault"

                echo "==== fault matrix: hardfault=$hardfault memmanage=$memmanage usagefault=$usagefault ===="

                if [[ "$usagefault" == 1 ]]; then
                    run_expect_failure_contains \
                        "fault=UndefInstr, exception=UsageFault" \
                        ./target/release/zmu-armv7m run "${armv7m_flags[@]}" "$usage_elf"
                else
                    run_expect_success_contains \
                        "UsageFault_Handler marker=0x55534654" \
                        ./target/release/zmu-armv7m run "${armv7m_flags[@]}" "$usage_elf"
                fi

                if [[ "$memmanage" == 1 ]]; then
                    run_expect_failure_contains \
                        "fault=DAccViol, exception=MemoryManagementFault" \
                        ./target/release/zmu-armv7m run "${armv7m_flags[@]}" "$memmanage_elf"
                else
                    run_expect_success_contains \
                        "MemManage_Handler marker=0x4d4d4654" \
                        ./target/release/zmu-armv7m run "${armv7m_flags[@]}" "$memmanage_elf"
                fi

                if [[ "$hardfault" == 1 ]]; then
                    run_expect_failure_contains \
                        "fault=UndefInstr, exception=HardFault" \
                        ./target/release/zmu-armv6m run "${armv6m_flags[@]}" "$hardfault_elf"
                else
                    run_expect_success_contains \
                        "HardFault_Handler marker=0x48444654" \
                        ./target/release/zmu-armv6m run "${armv6m_flags[@]}" "$hardfault_elf"
                fi
            done
        done
    done
}

run_alias_checks() {
    run_expect_failure_contains \
        "fault=UndefInstr, exception=UsageFault" \
        ./target/release/zmu-armv7m run --fault-trap tests/fault-test-bench/fault-usage-cm3.elf

    run_expect_failure_contains \
        "fault=DAccViol, exception=MemoryManagementFault" \
        ./target/release/zmu-armv7m run --fault-trap tests/fault-test-bench/fault-memmanage-cm3.elf

    run_expect_failure_contains \
        "fault=UndefInstr, exception=HardFault" \
        ./target/release/zmu-armv6m run tests/fault-test-bench/fault-hardfault-cm0.elf
}

run_lockup_check() {
    run_expect_failure_contains \
        "lockup trap: fault=UndefInstr, exception=HardFault" \
        ./target/release/zmu-armv6m run --no-trap all tests/fault-test-bench/fault-lockup-cm0.elf
}

build_bench
run_combo_matrix
run_alias_checks
run_lockup_check

echo "fault bench trap matrix passed"