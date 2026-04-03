#!/bin/bash
set -euo pipefail
{ set +x; } 2>/dev/null

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
source "$SCRIPT_DIR/scripts/test_output.sh"

if [ -z "${GCC_HOME:-}" ]; then
    echo "GCC_HOME is undefined"
    exit 1
fi

if ! command -v "$GCC_HOME/bin/arm-none-eabi-gcc" >/dev/null 2>&1; then
    echo "GCC for ARM is not installed. Please install from developer.arm.com"
    exit 1
fi

declare -A RUNNER_BY_CORE=()
declare -a ARMV6M_FAULT_CORES=()
declare -a ARMV7_FAULT_CORES=()

load_fault_core_metadata() {
    local core
    local runner
    local family

    while read -r core runner family; do
        [[ -n "$core" ]] || continue

        RUNNER_BY_CORE["$core"]="./target/release/${runner}"

        case "$family" in
            armv6m)
                ARMV6M_FAULT_CORES+=("$core")
                ;;
            armv7)
                ARMV7_FAULT_CORES+=("$core")
                ;;
            *)
                echo "Unknown fault core family: $family" >&2
                exit 1
                ;;
        esac
    done < <(make -s -C tests/fault-test-bench print-fault-core-metadata)

    if [[ ${#ARMV6M_FAULT_CORES[@]} -eq 0 || ${#ARMV7_FAULT_CORES[@]} -eq 0 ]]; then
        echo "Failed to load fault core metadata" >&2
        exit 1
    fi
}

build_bench() {
    pushd tests/fault-test-bench >/dev/null
    make -s clean
    make -s
    popd >/dev/null
}

runner_for_core() {
    local runner="${RUNNER_BY_CORE[$1]:-}"

    if [[ -z "$runner" ]]; then
        echo "Unknown core: $1" >&2
        exit 1
    fi

    printf '%s' "$runner"
}

run_expect_success_contains() {
    local case_name="$1"
    shift
    local expected="$1"
    shift

    test_expect_success_contains "$case_name" "$expected" "$@"
}

run_expect_failure_contains() {
    local case_name="$1"
    shift
    local expected="$1"
    shift

    test_expect_failure_contains "$case_name" "$expected" "$@"
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

run_armv7_combo_matrix() {
    local core="$1"
    local usage_elf="tests/fault-test-bench/fault-usage-${core}.elf"
    local memmanage_elf="tests/fault-test-bench/fault-memmanage-${core}.elf"
    local armv7_runner
    local armv6m_core
    local hardfault_elf
    local hardfault_runner
    local -a armv7m_flags
    local -a armv6m_flags

    armv7_runner=$(runner_for_core "$core")

    for hardfault in 0 1; do
        for memmanage in 0 1; do
            for usagefault in 0 1; do
                combo_flags_armv7m armv7m_flags "$hardfault" "$memmanage" "$usagefault"
                hardfault_flags_armv6m armv6m_flags "$hardfault"

                if [[ "$usagefault" == 1 ]]; then
                    run_expect_failure_contains \
                        "usagefault/${core}" \
                        "fault=UndefInstr, exception=UsageFault" \
                        "$armv7_runner" run "${armv7m_flags[@]}" "$usage_elf"
                else
                    run_expect_success_contains \
                        "usagefault/${core}" \
                        "UsageFault_Handler marker=0x55534654" \
                        "$armv7_runner" run "${armv7m_flags[@]}" "$usage_elf"
                fi

                if [[ "$memmanage" == 1 ]]; then
                    run_expect_failure_contains \
                        "memmanage/${core}" \
                        "fault=DAccViol, exception=MemoryManagementFault" \
                        "$armv7_runner" run "${armv7m_flags[@]}" "$memmanage_elf"
                else
                    run_expect_success_contains \
                        "memmanage/${core}" \
                        "MemManage_Handler marker=0x4d4d4654" \
                        "$armv7_runner" run "${armv7m_flags[@]}" "$memmanage_elf"
                fi

                for armv6m_core in "${ARMV6M_FAULT_CORES[@]}"; do
                    hardfault_runner=$(runner_for_core "$armv6m_core")
                    hardfault_elf="tests/fault-test-bench/fault-hardfault-${armv6m_core}.elf"

                    if [[ "$hardfault" == 1 ]]; then
                        run_expect_failure_contains \
                            "hardfault/${armv6m_core}" \
                            "fault=UndefInstr, exception=HardFault" \
                            "$hardfault_runner" run "${armv6m_flags[@]}" "$hardfault_elf"
                    else
                        run_expect_success_contains \
                            "hardfault/${armv6m_core}" \
                            "HardFault_Handler marker=0x48444654" \
                            "$hardfault_runner" run "${armv6m_flags[@]}" "$hardfault_elf"
                    fi
                done
            done
        done
    done
}

run_armv7_status_case() {
    local core="$1"
    local case_name="$2"
    local expected="$3"
    local elf_name="$4"
    local armv7_runner

    armv7_runner=$(runner_for_core "$core")

    run_expect_success_contains \
        "${case_name}/${core} status" \
        "$expected" \
        "$armv7_runner" run --no-trap hardfault "tests/fault-test-bench/${elf_name}-${core}.elf"
}

run_armv7_alias_checks() {
    local core="$1"
    local armv7_runner

    armv7_runner=$(runner_for_core "$core")

    run_expect_failure_contains \
        "usagefault/${core} alias --fault-trap" \
        "fault=UndefInstr, exception=UsageFault" \
        "$armv7_runner" run --fault-trap "tests/fault-test-bench/fault-usage-${core}.elf"

    run_expect_failure_contains \
        "memmanage/${core} alias --fault-trap" \
        "fault=DAccViol, exception=MemoryManagementFault" \
        "$armv7_runner" run --fault-trap "tests/fault-test-bench/fault-memmanage-${core}.elf"

    run_expect_failure_contains \
        "invstate/${core} alias --trap usagefault" \
        "fault=Invstate, exception=UsageFault" \
        "$armv7_runner" run --trap usagefault "tests/fault-test-bench/fault-exception-return-invstate-${core}.elf"

    run_expect_failure_contains \
        "invstate forced/${core} alias --fault-trap" \
        "fault=Invstate, exception=HardFault" \
        "$armv7_runner" run --fault-trap "tests/fault-test-bench/fault-exception-return-invstate-forced-hardfault-${core}.elf"
}

run_lockup_check() {
    local core
    local hardfault_runner

    for core in "${ARMV6M_FAULT_CORES[@]}"; do
        hardfault_runner=$(runner_for_core "$core")

        run_expect_failure_contains \
            "lockup/${core}" \
            "lockup trap: fault=UndefInstr, exception=HardFault" \
            "$hardfault_runner" run --no-trap all "tests/fault-test-bench/fault-lockup-${core}.elf"
    done
}

run_armv7_status_readback_checks() {
    local core="$1"
    run_armv7_status_case \
        "$core" \
        "usagefault" \
        "UsageFault_Handler marker=0x55534654 cfsr=0x00010000 shcsr=0x00040008" \
        "fault-usage"

    run_armv7_status_case \
        "$core" \
        "memmanage" \
        "MemManage_Handler marker=0x4d4d4654 cfsr=0x00000082 mmfar=0x00000000 shcsr=0x00010001" \
        "fault-memmanage"

    run_armv7_status_case \
        "$core" \
        "invstate" \
        "UsageFault_Handler marker=0x55534654 cfsr=0x00020000 shcsr=0x00040008" \
        "fault-exception-return-invstate"

    run_armv7_status_case \
        "$core" \
        "forced-hardfault" \
        "HardFault_Handler marker=0x48444654 case=forced-hardfault cfsr=0x00010000 hfsr=0x40000000" \
        "fault-forced-hardfault"

    run_armv7_status_case \
        "$core" \
        "invstate forced" \
        "HardFault_Handler marker=0x48444654 case=exception-return-invstate-forced-hardfault cfsr=0x00020000 hfsr=0x40000000" \
        "fault-exception-return-invstate-forced-hardfault"

    run_armv7_status_case \
        "$core" \
        "software pend svcall" \
        "SVC_Handler marker=0x53564654 shcsr=0x00000080" \
        "fault-software-pend-svcall"

    run_armv7_status_case \
        "$core" \
        "software pend usagefault" \
        "UsageFault_Handler marker=0x55534654 cfsr=0x00000000 shcsr=0x00040008" \
        "fault-software-pend-usagefault"

    run_armv7_status_case \
        "$core" \
        "software pend memmanage" \
        "MemManage_Handler marker=0x4d4d4654 cfsr=0x00000000 mmfar=0x00000000 shcsr=0x00010001" \
        "fault-software-pend-memmanage"

    run_armv7_status_case \
        "$core" \
        "software pend busfault" \
        "BusFault_Handler marker=0x42534654 cfsr=0x00000000 shcsr=0x00020002" \
        "fault-software-pend-busfault"
}

run_armv6_status_readback_checks() {
    local core="$1"
    local armv6_runner

    armv6_runner=$(runner_for_core "$core")

    run_expect_success_contains \
        "software pend svcall/${core} status" \
        "SVC_Handler marker=0x53564654 shcsr=0x00000000" \
        "$armv6_runner" run --no-trap hardfault "tests/fault-test-bench/fault-software-pend-svcall-${core}.elf"
}

run_armv6_default_hardfault_check() {
    local core
    local hardfault_runner

    for core in "${ARMV6M_FAULT_CORES[@]}"; do
        hardfault_runner=$(runner_for_core "$core")

        run_expect_failure_contains \
            "hardfault/${core} default" \
            "fault=UndefInstr, exception=HardFault" \
            "$hardfault_runner" run "tests/fault-test-bench/fault-hardfault-${core}.elf"
    done
}

load_fault_core_metadata
build_bench
for core in "${ARMV7_FAULT_CORES[@]}"; do
    run_armv7_combo_matrix "$core"
done

for core in "${ARMV7_FAULT_CORES[@]}"; do
    run_armv7_status_readback_checks "$core"
done

for core in "${ARMV6M_FAULT_CORES[@]}"; do
    run_armv6_status_readback_checks "$core"
done

for core in "${ARMV7_FAULT_CORES[@]}"; do
    run_armv7_alias_checks "$core"
done

run_armv6_default_hardfault_check
run_lockup_check

test_print_summary "fault bench trap matrix passed"