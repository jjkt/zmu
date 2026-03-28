# SCB Fault Status Register Plan

## Scope

This plan covers the TODO item in `doc/todo.md`:

- Update SCB fault status bits and registers: `CFSR`, `HFSR`, `SHCSR`, `MMFAR`, `BFAR`

It does not implement the follow-on TODO for full fault escalation rules, but it identifies the places where that next step depends on this work.

## Current State

The emulator already has raw storage for the relevant SCB registers in `Processor` and exposes them through PPB reads:

- `SHCSR`
- `CFSR`
- `HFSR`
- `MMFAR`
- `BFAR`

The current gaps are architectural rather than structural:

1. Fault events do not latch status bits into `CFSR` or `HFSR`.
2. `SHCSR` enable/active/pended semantics are not modeled beyond raw storage.
3. `MMFAR` and `BFAR` are not populated from real fault context.
4. PPB writes for these registers are largely missing, including write-one-to-clear behavior.
5. The current fault path usually does not preserve the faulting address, which blocks correct `MMFAR` and `BFAR` support.

## Design Goals

1. Keep the implementation minimal but architecturally coherent.
2. Centralize SCB fault status updates so individual execution paths do not duplicate bit logic.
3. Preserve enough metadata on faults to support valid-address latching.
4. Avoid baking escalation policy into the first pass beyond what is required for register correctness.
5. Drive the work through small red-green cycles instead of a large multi-file implementation pass.
6. Add regression coverage for both register values and clear behavior.

## TDD Strategy

Each increment should follow the same rule:

1. Write one narrowly-scoped test that expresses the next architectural behavior.
2. Run it and confirm it fails for the expected reason.
3. Implement the smallest change that makes that test pass.
4. Re-run the focused test.
5. Re-run the surrounding regression set to catch collateral breakage.
6. Refactor only after the test is green and only if the refactor preserves green status.

The plan should avoid adding multiple new behaviors behind one initial failing test. If a test would require touching several unrelated fault paths at once, split it.

### Preferred Red-Green Granularity

Use this order of preference for test scope:

1. Unit test for raw register semantics.
2. Narrow integration test for fault-to-register translation.
3. End-to-end `fault-test-bench` coverage only after the lower-level behavior exists.

This keeps failures diagnostic. A red test should identify one missing semantic, not a bundle of missing infrastructure.

## Proposed Implementation Shape

### 1. Introduce explicit fault status update helpers

Add SCB helper methods that express architectural side effects instead of open-coding raw bit operations.

Likely home:

- `zmu_cortex_m/src/peripheral/scb.rs`

Suggested responsibilities:

1. Set enable bits in `SHCSR` through explicit accessors.
2. Set active or pended fault indicators in `SHCSR` where applicable.
3. Latch individual `CFSR` sub-bits for MemManage, BusFault, and UsageFault causes.
4. Latch `HFSR` bits for vector-table and forced-fault cases when that logic is introduced.
5. Update `MMFAR` or `BFAR` and their corresponding valid bits.
6. Apply write-one-to-clear behavior for `CFSR` and `HFSR`.

This keeps register semantics in one layer and leaves fault production sites to report facts rather than manipulate registers directly.

### 2. Enrich the fault model with optional metadata

The current `Fault` enum is good for architectural classification, but it does not consistently carry contextual data.

At minimum, the implementation likely needs optional metadata for:

1. Faulting address for data access faults.
2. Faulting address for instruction fetch faults, if the implementation wants to distinguish address-valid cases cleanly.
3. Whether the fault arose during vector fetch or exception stacking.

Likely homes:

- `zmu_cortex_m/src/core/fault.rs`
- `zmu_cortex_m/src/core/fetch.rs`
- `zmu_cortex_m/src/core/exception.rs`
- `zmu_cortex_m/src/executor/load_and_store.rs`

One practical approach is to keep `Fault` as the fault kind and pair it with a lightweight context struct passed into the status-latching layer.

## Register Semantics To Implement

### `SHCSR`

Implement at least the bits needed for current fault handling:

1. MemManage enable.
2. BusFault enable.
3. UsageFault enable.
4. MemManage active.
5. BusFault active.
6. UsageFault active.

Notes:

1. Active-bit handling should reflect exception entry and return, not just raw fault occurrence.
2. If a smaller first pass is needed, enable-bit writes and active-bit reads can be split, but the plan should prefer doing them together because test expectations become clearer.

### `CFSR`

Implement bit latching for fault causes already modeled in the codebase.

Initial target set:

1. `IACCVIOL`
2. `DACCVIOL`
3. `MUNSTKERR`
4. `MSTKERR`
5. `IBUSERR`
6. `PRECISERR`
7. `IMPRECISERR` if the emulator currently distinguishes it
8. `UNSTKERR`
9. `STKERR`
10. `UNDEFINSTR`
11. `INVSTATE`
12. `INVPC`
13. `NOCP`
14. `UNALIGNED`
15. `DIVBYZERO`

Notes:

1. `MMARVALID` and `BFARVALID` live inside `CFSR` and must track whether `MMFAR` or `BFAR` currently contains a valid latched address.
2. Writes to `CFSR` should clear only the bits written as `1`.

### `HFSR`

Initial target set:

1. `VECTTBL` for vector table read faults.
2. Reserve `FORCED` hookup for the next TODO, but shape helpers so it can be added without restructuring.

Notes:

1. `HFSR` should also use write-one-to-clear behavior for implemented status bits.

### `MMFAR` and `BFAR`

Implement latched address behavior for faults that have a meaningful fault address.

Initial target set:

1. `MMFAR` for MemManage address faults such as `IACCVIOL` and `DACCVIOL` when an address is known.
2. `BFAR` for BusFault address faults such as `PRECISERR` and `IBUSERR` when an address is known.

Notes:

1. If a fault does not provide a reliable address, do not overwrite the register or set the valid bit.
2. Clear semantics should match the associated valid bits in `CFSR`.

## Likely File Touch Points

### Core register and helper layer

- `zmu_cortex_m/src/peripheral/scb.rs`
- `zmu_cortex_m/src/lib.rs`

### PPB access semantics

- `zmu_cortex_m/src/bus/mod.rs`

### Fault production and dispatch

- `zmu_cortex_m/src/core/fault.rs`
- `zmu_cortex_m/src/core/fetch.rs`
- `zmu_cortex_m/src/core/exception.rs`
- `zmu_cortex_m/src/executor/mod.rs`
- `zmu_cortex_m/src/executor/load_and_store.rs`
- possibly `zmu_cortex_m/src/core/reset.rs`

### Tests

- existing Rust unit tests near `peripheral/scb.rs`
- new architectural tests under `tests/fault-test-bench`
- possibly targeted host-side tests if direct register inspection is easier there

## TDD Execution Order

The work should proceed as a sequence of explicit red-green steps.

### Step 1: register clear semantics first

Status: complete

RED test:

1. Add unit tests for `CFSR` write-one-to-clear behavior.
2. Add unit tests for `HFSR` write-one-to-clear behavior.
3. Add a unit test for writable `SHCSR` enable bits.

GREEN implementation:

1. Define SCB bit constants.
2. Add helper methods and PPB write semantics only for these raw register behaviors.

Reason for starting here:

1. This is the smallest isolated slice.
2. It builds the register interface before fault paths depend on it.

### Step 2: fault-to-status mapping without address latching

Status: complete

RED test:

1. Add a focused test that an undefined instruction sets the expected UsageFault bit.
2. Add a focused test that invalid exception return sets `INVPC`.
3. Add a focused test that vector-table read failure sets `HFSR.VECTTBL`.

GREEN implementation:

1. Add central helpers for latching `CFSR` and `HFSR` bits from a fault kind.
2. Update the dispatch path to invoke those helpers.
3. Do not introduce `MMFAR` and `BFAR` yet unless a test already needs them.

Reason for this split:

1. It proves the status-latching path before metadata plumbing is added.

### Step 3: address-carrying faults and valid bits

Status: complete

RED test:

1. Add a focused test that a MemManage data access violation sets `DACCVIOL` plus `MMARVALID` and latches `MMFAR`.
2. Add a focused test that a BusFault with a precise address sets `BFARVALID` and latches `BFAR`.
3. Add a focused test that faults without a reliable address do not set valid bits or overwrite address registers.

GREEN implementation:

1. Introduce a lightweight fault-status context with optional address metadata.
2. Feed that metadata from the relevant load/store and fetch paths.
3. Extend the SCB latching helpers to update valid bits and address registers.

Implemented:

1. Added `FaultStatusContext` with optional fault-address metadata.
2. Wired address capture into load/store data accesses and instruction fetch faults.
3. Latched `MMFAR` and `BFAR` with `MMARVALID` and `BFARVALID` only when an address is available.
4. Added focused coverage for real load/store and fetch paths plus precise-BusFault and no-address cases.

Reason for this split:

1. Address validity is the first place where fault metadata matters.
2. Keeping it separate avoids over-designing the fault model too early.

### Step 4: stacking and unstacking status bits

Status: complete

RED test:

1. Add a focused test for exception-entry stack failure setting the appropriate stacking fault bit.
2. Add a focused test for unstack failure if there is already a path that can trigger it deterministically.

GREEN implementation:

1. Feed stacking-origin metadata into the status-latching helper.
2. Set the correct MemManage or BusFault stacking bits.

Implemented:

1. Latched `MSTKERR`, `STKERR`, and `UNSTKERR` for the currently modeled stacking and unstacking fault kinds.
2. Recorded entry-fault status bits both when `handle_fault()` fails during exception entry and when `check_exceptions()` fails while taking a pending exception.
3. Added focused coverage for deterministic exception-entry stack failure through both live paths.

Reason for this split:

1. These faults are semantically distinct and easy to regress if combined with the earlier basic mapping work.

### Step 5: `SHCSR` active-state modeling

Status: complete

RED test:

1. Add a focused test that active MemManage, BusFault, or UsageFault exceptions are reflected in `SHCSR`.
2. Add a focused test that active bits clear on exception return.

GREEN implementation:

1. Tie `SHCSR` active bits to exception state bookkeeping.
2. Avoid simulating unrelated pended-state behavior unless required by the failing test.

Implemented:

1. Added focused SHCSR tests for active MemManage, BusFault, and UsageFault reflection plus active-bit clearing on exception return.
2. Switched SHCSR PPB reads to a computed helper that overlays live exception activity onto the stored register value.
3. Kept writable enable-bit semantics unchanged by preserving raw SHCSR storage for non-active bits and only synthesizing the active fault bits at read time.

Reason for this split:

1. Active-state semantics depend on exception flow, not just register write semantics.

### Step 6: end-to-end confirmation

Status: in progress

RED test:

1. Add `fault-test-bench` scenarios that read back SCB state from handler code or test-side probes.
2. Confirm the new end-to-end tests fail before any final cleanup.

GREEN implementation:

1. Close any gaps between unit-level semantics and architectural execution paths.
2. Keep fixes minimal and driven only by the failing end-to-end assertions.

## Testing Plan

### Unit-level tests

Add or extend Rust tests to verify, one behavior at a time:

1. `SHCSR` enable-bit writes.
2. `CFSR` write-one-to-clear behavior.
3. `HFSR` write-one-to-clear behavior.
4. `MMFAR` and `BFAR` latching only when an address is valid.
5. valid bits in `CFSR` track address-register validity.
6. active-bit reflection in `SHCSR` once exception-state wiring is added.

Rule for unit tests:

1. Introduce the next unit test in a failing state.
2. Land only enough implementation to turn it green.
3. If multiple assertions fail, split the test unless they describe one indivisible behavior.

### Fault-path regression tests

Extend `tests/fault-test-bench` so handlers or test code can read back SCB state after a fault.

Suggested scenarios:

1. Undefined instruction sets the expected UsageFault bit.
2. Invalid exception return sets `INVPC`.
3. MemManage data access violation sets `DACCVIOL` and `MMFAR` when an address is known.
4. BusFault on vector fetch sets `HFSR.VECTTBL`.
5. Stack push failure on exception entry sets the appropriate stacking fault bit.
6. Clearing `CFSR` or `HFSR` through PPB writes removes only the written bits.

Rule for fault-path tests:

1. Add them only after the corresponding lower-level unit behavior exists.
2. Prefer one failing architectural scenario per commit-sized increment.
3. Use them to validate wiring, not as the first place to discover register bit semantics.

### Suggested per-step verification loop

For each red-green increment:

1. Run the new focused test and confirm red.
2. Implement the minimum code to get green.
3. Re-run the focused test.
4. Re-run nearby unit tests in the same module.
5. Re-run the relevant integration script only when the increment touches execution flow.

## Risks And Decisions To Settle During Implementation

1. Whether to model `SHCSR` active bits entirely from exception state or partially from latched software state.
2. Whether `Fault` itself should grow metadata or whether a separate context object is cleaner.
3. Which existing internal errors should be treated as having a reliable fault address.
4. How much of `HFSR.FORCED` plumbing to prepare now without prematurely implementing full escalation rules.

## Suggested Definition Of Done

This TODO can be considered complete when all of the following are true:

1. SCB fault-status registers are both readable and writable with the required semantics.
2. Real fault paths latch the expected `CFSR` or `HFSR` bits.
3. `MMFAR` and `BFAR` capture valid addresses where the emulator can determine them.
4. `SHCSR` reflects fault enables and active state for the implemented fault classes.
5. Each implemented behavior was introduced through an intentional failing red test and then made green with the minimum viable code.
6. There are regression tests covering the implemented status and clear behavior.
7. The next TODO for fault escalation can build on these helpers without redesigning the interface.