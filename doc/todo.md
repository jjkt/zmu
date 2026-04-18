# TODO

## ARM Cortex Core behavior
- [ ] Account for exception entry and return cycles instead of using TODOs and the fixed 12-cycle fault path
- [ ] implement have_dsp_ext feature flag, use it to gate SSAT, USAT, pop_stack, MRS, MSR functionalities
- [ ] Model full `CONTROL` on FP cores, including `FPCA`
- [ ] Implement `AIRCR` write semantics: `VECTKEY`, `PRIGROUP`, and reset bits
- [ ] Wire endianness reporting to `AIRCR.ENDIANNESS` and core support rules
- [ ] Audit and model remaining visible SCB reset defaults per core variant, including `CPUID`, `ICSR`, `AIRCR`, `SCR`, `CCR`, `ICTR`, and `ACTLR`
- [ ] Define invalid-width behavior for `UBFX` and `SBFX`
- [ ] Implement more semihosting commands beyond the current console and `:semihosting-features` subset
- [ ] Handle unknown semihosting commands

## Platform and memory
- [ ] Add `Processor::ram_memory()` to match `flash_memory()`
- [ ] Remove hardcoded flash and RAM sizes from `Processor::new()` (`64 KiB` flash, `128 KiB` SRAM)
- [ ] Expose flash and RAM size plus base address in `zmu run`
- [ ] Let ELF loading accept `PT_LOAD` images smaller than the allocated flash backing
- [ ] Support multiple RAM, ROM, and peripheral regions instead of one remap window plus one flash and one SRAM object
- [ ] Add `MPU` modeling; `zmu_cortex_m` has none yet
- [ ] Make the generic device return bus faults or unmapped responses for peripheral space instead of silent zero reads and writes
- [ ] Split core platform config from concrete device models so `NVIC`, `SysTick`, and SCB setup is not tied to one built-in `Device`
- [ ] Add external peripheral models beyond STM32F1xx, with pluggable address maps and `NVIC` IRQ wiring

## Scripting
- [ ] Keep core architectural behavior in Rust: `CPU`, faults, exceptions, `NVIC`, `SysTick`, memory model
- [ ] Allow scripts or declarative device definitions to describe flash and RAM sizes, memory regions, remaps, and peripheral address maps
- [ ] Keep the peripheral API small: `read`, `write`, `tick`, `reset`, IRQ reporting
- [ ] Separate register-map data from optional scripted side effects
- [ ] Evaluate a scriptable device and peripheral layer for board-specific MMIO
- [ ] Compare `Lua`, `Rhai`, Python, JS, and declarative `TOML` or `YAML` plus hooks for determinism, speed, and ease of contribution
- [ ] Define how board and device profiles load scripts or data at runtime
- [ ] Decide how scripted peripherals report unmapped access, bus faults, and IRQ assertions
- [ ] Define whether coprocessor or other implementation-defined instructions may dispatch into scripts, and keep that boundary explicit
- [ ] Assess debugging and testability of scripted peripherals: trace visibility, deterministic replay, unit-test harnesses
- [ ] Assess the security model for loading external scripts

## ARM FP support
- [ ] Split ARM FP support by profile: FPv4-SP-D16, FPv5-SP-D16, FPv5-D16, and Armv8.1-M FP/FP16
- [ ] Make PPB writes update `CPACR`, `FPCCR`, `FPCAR`, and `FPDSCR`, not just reads
- [ ] Make `execute_fp_check()` enforce FP enablement via `CPACR` and `NOCP`
- [ ] Run the same FP enable checks for `VMRS` and `VMOV` transfer paths
- [ ] Replace the enabled-exception `todo!()` in `fp_process_exception()` with real FP trap and fault handling
- [ ] Implement `decode_VMOV_cr_scalar()` and execute `Instruction::VMOV_cr_scalar`
- [ ] Implement `decode_VMOV_scalar_cr()` and execute `Instruction::VMOV_scalar_cr`
- [ ] Implement `decode_VMOV_cr2_sp2()` and execute `Instruction::VMOV_cr2_sp2`
- [ ] Add decode, execute, and tests for `VMLA`
- [ ] Add decode, execute, and tests for `VFNMA`
- [ ] Add decode, execute, and tests for `VNMLA`
- [ ] Add decode, execute, and tests for `VMAXNM`
- [ ] Add decode, execute, and tests for `VMINNM`
- [ ] Add decode, execute, and tests for `VRINTA`
- [ ] Add decode, execute, and tests for `VRINTM`
- [ ] Add decode, execute, and tests for `VRINTN`
- [ ] Add decode, execute, and tests for `VRINTP`
- [ ] Add decode, execute, and tests for `VRINTR`
- [ ] Add decode, execute, and tests for `VRINTX`
- [ ] Add decode, execute, and tests for `VRINTZ`
- [ ] Add decode, execute, and tests for `VCVTA`
- [ ] Add decode, execute, and tests for `VCVTM`
- [ ] Add decode, execute, and tests for `VCVTN`
- [ ] Add decode, execute, and tests for `VCVTP`
- [ ] Add half-precision FP support with profile gating
- [ ] Add focused tests for missing `VMOV` transfer forms and FP-disabled or FP-trap behavior

## Architecture coverage
- [ ] ARMv6-M: improve fault fidelity and cycle accounting

### ARMv7-M
- [ ] `CLREX`
- [ ] `DBG`
- [ ] `CDP`
- [ ] `CDP2`
- [ ] `MCRR`
- [ ] `MCRR2`
- [ ] `MRC`
- [ ] `MRC2`
- [ ] `STC`
- [ ] `STC2`
- [ ] `LDC` literal form
- [ ] `LDC2` literal form

### ARMv7E-M sat/pack
- [ ] `SSAT`
- [ ] `USAT`
- [ ] `SSAT16`
- [ ] `USAT16`
- [ ] `QADD`
- [ ] `QSUB`
- [ ] `QDADD`
- [ ] `QDSUB`
- [ ] `PKHBT`
- [ ] `PKHTB`
- [ ] `SXTAB`
- [ ] `SXTAB16`
- [ ] `SXTAH`
- [ ] `SXTB16`
- [ ] `UXTAB16`
- [ ] `UXTAH`
- [ ] `UXTB16`

### ARMv7E-M parallel
- [ ] `SADD16`
- [ ] `QADD16`
- [ ] `SHADD16`
- [ ] `UADD16`
- [ ] `UQADD16`
- [ ] `UHADD16`
- [ ] `SASX`
- [ ] `QASX`
- [ ] `SHASX`
- [ ] `UASX`
- [ ] `UQASX`
- [ ] `UHSX`
- [ ] `SSAX`
- [ ] `QSAX`
- [ ] `SHSAX`
- [ ] `USAX`
- [ ] `UQSAX`
- [ ] `UHSAX`
- [ ] `SSUB16`
- [ ] `QSUB16`
- [ ] `SHSUB16`
- [ ] `USUB16`
- [ ] `UQSUB16`
- [ ] `UHSUB16`
- [ ] `SADD8`
- [ ] `QADD8`
- [ ] `SHADD8`
- [ ] `UQADD8`
- [ ] `UHADD8`
- [ ] `SSUB8`
- [ ] `QSUB8`
- [ ] `SHSUB8`
- [ ] `USUB8`
- [ ] `UQSUB8`
- [ ] `UHSUB8`

### ARMv7E-M DSP
- [ ] `SMLAD`
- [ ] `SMLADX`
- [ ] `SMLALBB`
- [ ] `SMLALBT`
- [ ] `SMLALTB`
- [ ] `SMLALTT`
- [ ] `SMLALD`
- [ ] `SMLALDX`
- [ ] `SMLAWB`
- [ ] `SMLAWT`
- [ ] `SMLSD`
- [ ] `SMLSDX`
- [ ] `SMLSLD`
- [ ] `SMLSLDX`
- [ ] `SMMLA`
- [ ] `SMMLAR`
- [ ] `SMMLS`
- [ ] `SMMLSR`
- [ ] `SMMUL`
- [ ] `SMMULR`
- [ ] `SMUAD`
- [ ] `SMUADX`
- [ ] `SMULWB`
- [ ] `SMULWT`
- [ ] `SMUSD`
- [ ] `SMUSDX`
- [ ] `UMAAL`

### ARMv8-M Baseline
- [ ] Add profile support and feature gating vs `ARMv6-M`
- [ ] Security model: Secure and Non-secure state, secure exception entry and return, `EXC_RETURN` rules, banked stack and system registers
- [ ] Attribution and limits: `SAU`, `IDAU`, Secure and Non-secure `NVIC`, `SysTick`, `VTOR`, `MSPLIM`, `PSPLIM`
- [ ] `BLXNS`
- [ ] `BXNS`
- [ ] `SG`
- [ ] `TT`
- [ ] `TTA`
- [ ] `TTT`
- [ ] `TTAT`

### ARMv8-M Mainline
- [ ] Add profile support and feature gating vs `ARMv7-M` and `ARMv7E-M`
- [ ] Add explicit FPv5 variants for ARMv8-M Mainline: `FPv5-SP-D16` and `FPv5-D16`
- [ ] Gate FP register count, DP support, and `MVFR0`/`MVFR1`/`MVFR2` by selected FPv5 variant
- [ ] Security integration: SecureFault, secure and non-secure `MPU` state, debug and trace visibility, secure peripheral side effects
- [ ] ARMv8-M FP security details: Secure vs Non-secure FP access, banked FP state where needed, lazy stacking, and `CONTROL.FPCA` / `FPCCR` behavior
- [ ] `LDA`
- [ ] `LDAB`
- [ ] `LDAH`
- [ ] `LDAEX`
- [ ] `LDAEXB`
- [ ] `LDAEXH`
- [ ] `STL`
- [ ] `STLB`
- [ ] `STLH`
- [ ] `STLEX`
- [ ] `STLEXB`
- [ ] `STLEXH`
- [ ] `CDP`
- [ ] `CDP2`
- [ ] `MCR`
- [ ] `MCR2`
- [ ] `MCRR`
- [ ] `MCRR2`
- [ ] `MRC`
- [ ] `MRC2`
- [ ] `MRRC`
- [ ] `MRRC2`

### ARMv8.1-M
- [ ] Add profile support and feature gating vs `ARMv8-M Mainline`
- [ ] TrustZone and FPU interaction
- [ ] New `MPU` memory attributes
- [ ] PMU
- [ ] Unprivileged debug
- [ ] RAS
- [ ] `DLS`
- [ ] `WLS`
- [ ] `LE`
- [ ] MVE predication instructions (~7-8 families, ~15-30 encodings)
- [ ] MVE integer load and store instructions (~20-25 families, ~100-170 encodings)
- [ ] MVE FP load and store instructions (~8-10 families, ~20-40 encodings)
- [ ] MVE lane insert and extract instructions (~4-6 families, ~10-20 encodings)
- [ ] MVE widening integer instructions (~14-18 families, ~60-140 encodings)
- [ ] MVE narrowing integer instructions (~8-10 families, ~30-70 encodings)
- [ ] MVE integer MAC instructions (~18-22 families, ~110-220 encodings)
- [ ] MVE FP MAC instructions (~5-7 families, ~15-35 encodings)
- [ ] MVE reduction instructions (~8-10 families, ~35-70 encodings)
- [ ] `CX1*`
- [ ] `CX2*`
- [ ] `CX3*`
- [ ] `VCX1*`
- [ ] `VCX2*`
- [ ] `VCX3*`
- [ ] `AUT`
- [ ] `AUTG`
- [ ] `BTI`
- [ ] `BXAUT`
- [ ] `PAC`
- [ ] `PACBTI`
- [ ] `PACG`

## Tests and tooling
- [ ] Add more unit tests
- [ ] Extend `fault-test-bench` SCB coverage to end-to-end readback and clear semantics such as `INVPC`, `CFSR` W1C, and `HFSR` W1C
- [ ] Make integration scripts rebuild or recopy feature-specific release binaries before invoking `target/release/zmu-armv*` so plain `cargo build --release` cannot leave stale test executables behind
- [ ] Fill in missing instruction formatting
- [ ] Use Rusty Clock or another real project to drive crude hardware simulation through peripherals

## Trace
- [ ] Model ITM enable and control flow (`DEMCR.TRCENA`, `ITM.TCR`, `ITM.TER`, `ITM.LAR`) instead of only raw stimulus-port writes
- [ ] Option to Show "register deltas only"
  - [ ] Print only changed registers
- [ ] Option to show faults in trace
- [ ] Memory / Bus access trace
- [ ] VFP tracing
- [ ] Trustzone trace

## Codebase
- [ ] Try to shorten the clippy #allow list or make the rules more local to impls

## Architecture
- [ ] Support RISC-V