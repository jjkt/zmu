#!/bin/bash

: "${PASS_COUNT:=0}"

test_progress_dot() {
    printf '.'
    PASS_COUNT=$((PASS_COUNT + 1))
}

test_print_failure() {
    local label="$1"
    local summary="$2"
    local expected="$3"
    local output="$4"

    printf '\n[%s] %s\n' "$label" "$summary"
    if [[ -n "$expected" ]]; then
        printf 'expected: %s\n' "$expected"
    fi
    printf '%s\n' "$output"
}

test_expect_success_exact() {
    local label="$1"
    local expected="$2"
    shift 2

    local output
    local status

    set +e
    output=$("$@" 2>&1)
    status=$?
    set -e

    if [[ $status -ne 0 ]]; then
        test_print_failure "$label" "failed" "$expected" "$output"
        return $status
    fi

    if [[ "$output" != "$expected" ]]; then
        test_print_failure "$label" "unexpected output" "$expected" "$output"
        return 1
    fi

    test_progress_dot
}

test_expect_success_contains() {
    local label="$1"
    local expected="$2"
    shift 2

    local output
    local status

    set +e
    output=$("$@" 2>&1)
    status=$?
    set -e

    if [[ $status -ne 0 ]]; then
        test_print_failure "$label" "failed" "$expected" "$output"
        return $status
    fi

    if [[ -z "$expected" ]]; then
        test_progress_dot
        return 0
    fi

    if [[ "$output" != *"$expected"* ]]; then
        test_print_failure "$label" "unexpected output" "$expected" "$output"
        return 1
    fi

    test_progress_dot
}

test_expect_failure_contains() {
    local label="$1"
    local expected="$2"
    shift 2

    local output
    local status

    set +e
    output=$("$@" 2>&1)
    status=$?
    set -e

    if [[ $status -eq 0 ]]; then
        test_print_failure "$label" "unexpected success" "non-zero exit status" "$output"
        return 1
    fi

    if [[ "$output" != *"$expected"* ]]; then
        test_print_failure "$label" "unexpected trap output" "$expected" "$output"
        return 1
    fi

    test_progress_dot
}

test_print_summary() {
    local label="$1"

    printf '\n%s (%s checks)\n' "$label" "$PASS_COUNT"
}