//!
//! Representation of Cortex M Instruction set
//!

use crate::core::condition::Condition;
use crate::core::register::{ExtensionReg, Reg};
use crate::core::thumb::ThumbCode;
use enum_set::EnumSet;

#[derive(Debug, PartialEq, Copy, Clone)]
///
/// Types of shift operations supported
pub enum SRType {
    /// logical shift left
    LSL,
    /// logical shift right
    LSR,
    /// arithmetic shift right
    ASR,
    /// rotate right
    RRX,
    /// rotate right
    ROR,
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// IT instruction conditions
pub enum ITCondition {
    /// condition normal operation
    Then,
    /// condition inverted operation
    Else,
}

#[derive(PartialEq, Debug, Copy, Clone)]
///
/// Coding of imm32+carry variants for more efficient run time behaviour
///
pub enum Imm32Carry {
    /// Precalculated value carry value was not relevant
    /// for the decoding.
    NoCarry {
        /// imm32 original value
        imm32: u32,
    },
    /// Precalculated values for cases ASPR.C == 0 and ASPR.C ==1
    /// If carry was relevant for the decoding
    /// tuple of (immediate value, carry).
    Carry {
        /// Values of imm32 and carry, when carry was originally 0.
        imm32_c0: (u32, bool),
        /// Values of imm32 and carry, when carry was originally 1.
        imm32_c1: (u32, bool),
    },
}

#[derive(PartialEq, Debug, Copy, Clone)]
/// Instruction flags setting variants
pub enum SetFlags {
    /// Set Always
    True,
    /// Set Never
    False,
    /// Set when not in "IT" block
    NotInITBlock,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct CondBranchParams {
    pub cond: Condition,
    pub imm32: i32,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3ShiftParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub shift_t: SRType,
    pub shift_n: u8,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3Params {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub setflags: SetFlags,
}
#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3NoSetFlagsParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3FullParams {
    pub rt: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub shift_t: SRType,
    pub shift_n: u8,
    pub index: bool,
    pub add: bool,
    pub wback: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2FullParams {
    pub rt: Reg,
    pub rn: Reg,
    pub imm32: u32,
    pub index: bool,
    pub add: bool,
    pub wback: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct MrsParams {
    pub rd: Reg,
    pub sysm: u8,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct MsrParams {
    pub rn: Reg,
    pub sysm: u8,
    pub mask: u8,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2UsizeParams {
    pub rd: Reg,
    pub rm: Reg,
    pub rotation: usize,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3UsizeParams {
    pub rd: Reg,
    pub rm: Reg,
    pub rn: Reg,
    pub rotation: usize,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg4NoSetFlagsParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub ra: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ShiftParams {
    pub rd: Reg,
    pub rm: Reg,
    pub shift_t: SRType,
    pub shift_n: u8,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ShiftNParams {
    pub rd: Reg,
    pub rm: Reg,
    pub shift_n: u8,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2Params {
    pub rd: Reg,
    pub rm: Reg,
    pub setflags: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ImmParams {
    pub rd: Reg,
    pub rn: Reg,
    pub imm32: u32,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ImmCarryParams {
    pub rd: Reg,
    pub rn: Reg,
    pub imm32: Imm32Carry,
    pub setflags: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RegImmCarryParams {
    pub rd: Reg,
    pub imm32: Imm32Carry,
    pub setflags: SetFlags,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RegImmCarryNoSetFlagsParams {
    pub rn: Reg,
    pub imm32: Imm32Carry,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2ShiftNoSetFlagsParams {
    pub rn: Reg,
    pub rm: Reg,
    pub shift_t: SRType,
    pub shift_n: u8,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2RnRmParams {
    pub rn: Reg,
    pub rm: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2RdRmParams {
    pub rd: Reg,
    pub rm: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2RtRnParams {
    pub rt: Reg,
    pub rn: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3RdRtRnParams {
    pub rd: Reg,
    pub rt: Reg,
    pub rn: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3RdRtRnImm32Params {
    pub rd: Reg,
    pub rt: Reg,
    pub rn: Reg,
    pub imm32: u32,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg2RtRnImm32Params {
    pub rt: Reg,
    pub rn: Reg,
    pub imm32: u32,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct UbfxParams {
    pub rd: Reg,
    pub rn: Reg,
    pub lsb: usize,
    pub widthminus1: usize,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct MovtParams {
    pub rd: Reg,
    pub imm16: u16,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RegImmParams {
    pub r: Reg,
    pub imm32: u32,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg643232Params {
    pub rdlo: Reg,
    pub rdhi: Reg,
    pub rm: Reg,
    pub rn: Reg,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg3HighParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub n_high: bool,
    pub m_high: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Reg4HighParams {
    pub rd: Reg,
    pub rn: Reg,
    pub rm: Reg,
    pub ra: Reg,
    pub n_high: bool,
    pub m_high: bool,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ParamsRegImm32 {
    pub rn: Reg,
    pub imm32: u32,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct BfcParams {
    pub rd: Reg,
    pub lsbit: usize,
    pub msbit: usize,
}

#[allow(missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct BfiParams {
    pub rd: Reg,
    pub rn: Reg,
    pub lsbit: usize,
    pub width: usize,
}

#[allow(non_camel_case_types, missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
///
/// Instruction set
/// These "micro instructions" are produced by the decoder
/// and operated on by the executor.
/// Note that the instruction list is not 1:1 to
/// the mnemonics listed in the ref manual, instead
/// the exact variant is decoded to allow faster runtime.
pub enum Instruction {
    // --------------------------------------------
    //
    // Group: Branch instructions
    //
    // --------------------------------------------
    /// Branch to target address (on condition)
    B_t13 {
        params: CondBranchParams,
        thumb32: bool,
    },
    /// Branch to target address
    B_t24 {
        imm32: i32,
        thumb32: bool,
    },
    /// Call a subroutine
    BL {
        imm32: i32,
    },
    /// Call a subroutine, optionally change instruction set
    BLX {
        rm: Reg,
    },
    /// Change to target address, change instruction set
    BX {
        rm: Reg,
    },
    /// Compare and branch on  Zero
    CBZ {
        params: ParamsRegImm32,
    },
    /// Compare and branch on Nonzero
    CBNZ {
        params: ParamsRegImm32,
    },
    /// Table branch, byte offsets
    TBB {
        params: Reg2RnRmParams,
    },
    /// Table branch, halfword offsets
    TBH {
        params: Reg2RnRmParams,
    },

    // --------------------------------------------
    //
    // Group: Standard data-processing instructions
    //
    // --------------------------------------------
    /// Add (immediate)
    ADD_imm {
        params: Reg2ImmParams,
        thumb32: bool,
    },
    /// Add (register)
    ADD_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Add (register, sp)
    ADD_sp_reg {
        params: Reg2ShiftParams,
        thumb32: bool,
    },
    /// Add with Carry
    ADC_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Add with Carry
    ADC_imm {
        params: Reg2ImmParams,
    },

    /// Form PC-relative Address
    ADR {
        params: RegImmParams,
        thumb32: bool,
    },

    /// Bitwise AND
    AND_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Bitwise AND
    AND_imm {
        params: Reg2ImmCarryParams,
    },

    /// Bitwise Bit Clear
    BIC_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Bitwise Bit Clear
    BIC_imm {
        params: Reg2ImmCarryParams,
    },

    /// Compare Negative
    CMN_reg {
        params: Reg2ShiftNoSetFlagsParams,
        thumb32: bool,
    },
    /// Compare Negative
    CMN_imm {
        params: RegImmParams,
    },

    /// Compare
    CMP_imm {
        params: RegImmParams,
        thumb32: bool,
    },
    /// Compare
    CMP_reg {
        params: Reg2ShiftNoSetFlagsParams,
        thumb32: bool,
    },

    /// Bitwise Exclusive OR
    EOR_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Bitwise Exclusive OR
    EOR_imm {
        params: Reg2ImmCarryParams,
    },

    /// Copies operand to destination
    MOV_imm {
        params: RegImmCarryParams,
        thumb32: bool,
    },
    /// Copies operand to destination
    MOV_reg {
        params: Reg2Params,
        thumb32: bool,
    },

    /// Bitwise NOT
    MVN_reg {
        params: Reg2ShiftParams,
        thumb32: bool,
    },
    /// Bitwise NOT
    MVN_imm {
        params: RegImmCarryParams,
    },
    /// Bitwise OR NOT
    ORN_reg {
        params: Reg3ShiftParams,
    },

    /// Bitwise OR
    ORR_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Bitwise OR
    ORR_imm {
        params: Reg2ImmCarryParams,
    },

    /// Reverse subtract
    RSB_imm {
        params: Reg2ImmParams,
        thumb32: bool,
    },
    /// Reverse subtract
    RSB_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },

    /// Subtract with Carry
    SBC_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },
    /// Subtract with Carry
    SBC_imm {
        params: Reg2ImmParams,
    },

    /// Subtract
    SUB_imm {
        params: Reg2ImmParams,
        thumb32: bool,
    },
    /// Subtract
    SUB_reg {
        params: Reg3ShiftParams,
        thumb32: bool,
    },

    /// Test equivalence
    TEQ_reg {
        params: Reg2ShiftNoSetFlagsParams,
    },
    /// Test equivalence
    TEQ_imm {
        params: RegImmCarryNoSetFlagsParams,
    },

    /// Test
    TST_reg {
        params: Reg2ShiftNoSetFlagsParams,
        thumb32: bool,
    },
    /// Test
    TST_imm {
        params: RegImmCarryNoSetFlagsParams,
    },

    // --------------------------------------------
    //
    // Group: Shift instructions
    //
    // --------------------------------------------
    /// Arithmetic shift right
    ASR_imm {
        params: Reg2ShiftNParams,
        thumb32: bool,
    },
    /// Arithmetic shift right
    ASR_reg {
        params: Reg3Params,
        thumb32: bool,
    },
    /// Logical Shift Left (immediate)
    LSL_imm {
        params: Reg2ShiftNParams,
        thumb32: bool,
    },
    /// Logical Shift Left (register)
    LSL_reg {
        params: Reg3Params,
        thumb32: bool,
    },
    /// Logical Shift Right (immediate)
    LSR_imm {
        params: Reg2ShiftNParams,
        thumb32: bool,
    },
    /// Logical Shift Right (register)
    LSR_reg {
        params: Reg3Params,
        thumb32: bool,
    },
    /// Rotate Right (immediate)
    ROR_imm {
        params: Reg2ShiftNParams,
    },
    /// Rotate Right (register)
    ROR_reg {
        params: Reg3Params,
        thumb32: bool,
    },
    /// Rotate Right with Extend
    RRX {
        params: Reg2Params,
    },

    // --------------------------------------------
    //
    // Group: Multiply instructions
    //
    // --------------------------------------------
    /// Multipy and Accumulate
    MLA {
        params: Reg4NoSetFlagsParams,
    },
    /// Multipy and Subtract
    MLS {
        params: Reg4NoSetFlagsParams,
    },
    /// Multipy
    MUL {
        params: Reg3Params,
        thumb32: bool,
    },
    // --------------------------------------------
    //
    // Group: Signed multiply instructions (ArmV7-m)
    //
    // --------------------------------------------
    /// Signed Multiply and Accumulate (Long)
    SMLAL {
        params: Reg643232Params,
    },
    /// Signed Multiply (Long)
    SMULL {
        params: Reg643232Params,
    },

    // --------------------------------------------
    //
    // Group: Unsigned Multiply instructions (ARMv7-M base architecture)
    //
    // --------------------------------------------
    UMLAL {
        params: Reg643232Params,
    },
    UMULL {
        params: Reg643232Params,
    },
    // --------------------------------------------
    //
    // Group: Signed Multiply instructions (ARMv7-M DSP extension)
    //
    // --------------------------------------------
    /// Signed multiply: halfwords
    /// variants: SMULTT, SMULBB, SMULTB, SMULBT
    SMUL {
        params: Reg3HighParams,
    },
    /// Signed multiply and Accumulate, halfwords
    /// variants: SMLATT, SMLABB, SMLATB, SMLABT
    SMLA {
        params: Reg4HighParams,
    },

    //SMLAL second variant?
    //SMLALD
    //SMLAW
    //SMLSD
    //SMLSLD
    //SMMLA
    //SMMLS
    //SMMUL
    //SMUAD

    // --------------------------------------------
    //
    // Group: Saturating instructions (ARMv7-M base arch)
    //
    // --------------------------------------------

    //SSAT
    //USAT

    // --------------------------------------------
    //
    // Group: Unsigned Saturating instructions (ARMv7-M DSP extensions)
    //
    // --------------------------------------------
    //USAT16
    //SSAT16

    // --------------------------------------------
    //
    // Group: Saturating add/sub (ARMv7-M DSP extensions)
    //
    // --------------------------------------------
    //QADD
    //QSUB
    //QDADD
    //QDSUB

    // --------------------------------------------
    //
    // Group: Packing and unpacking instructions
    //
    // --------------------------------------------
    /// Signed Extend Byte
    SXTB {
        params: Reg2UsizeParams,
        thumb32: bool,
    },
    /// Signed Extend Halfword
    SXTH {
        params: Reg2UsizeParams,
        thumb32: bool,
    },
    /// Unsigned Extend Byte
    UXTB {
        params: Reg2UsizeParams,
        thumb32: bool,
    },
    /// Unsigned Extend Halfword
    UXTH {
        params: Reg2UsizeParams,
        thumb32: bool,
    },
    // --------------------------------------------
    //
    // Group: Packing and unpacking instructions (DSP extensions)
    //
    // --------------------------------------------
    //PKHBT, PKHTB
    //SXTAB
    //SXTAB16
    //SXTAH
    //SXTB16
    UXTAB {
        params: Reg3UsizeParams,
    },
    //UXTAB16
    //UXTAH
    //UXTB16

    // --------------------------------------------
    //
    // Group: Divide instructions
    //
    // --------------------------------------------
    /// signed divide
    SDIV {
        params: Reg3NoSetFlagsParams,
    },
    /// Unsigned divide
    UDIV {
        params: Reg3NoSetFlagsParams,
    },

    // --------------------------------------------
    //
    // Group: Parallel add / sub (DSP extension)
    //
    // --------------------------------------------
    //SADD16, QADD16, SHADD16, UADD16, UQADD16, UHADD16
    //SASX, QASX, SHASX, UASX, UQASX, UHSX
    //SSAX, QSAX, SHSAX, USAX, UQSAX, UHSAX
    //SSUB16, QSUB16, SHSUB16, USUB16, UQSUB16, UHSUB16
    //SADD8, QADD8, SHADD8, UADD8, UQADD8, UHADD8
    //SSUB8, QSUB8, SHSUB8, USUB8, UQSUB8, UHSUB8
    UADD8 {
        params: Reg3NoSetFlagsParams,
    },

    // --------------------------------------------
    //
    // Group: Miscellaneous data-processing instructions
    //
    // --------------------------------------------
    /// Bit Field Clear
    BFC {
        params: BfcParams,
    },
    /// Bit Field Insert
    BFI {
        params: BfiParams,
    },
    /// Count Leading Zeros
    CLZ {
        params: Reg2RdRmParams,
    },
    /// Move Top
    MOVT {
        params: MovtParams,
    },
    // RBIT
    /// Byte-reverse word
    REV {
        params: Reg2RdRmParams,
        thumb32: bool,
    },

    /// Byte-reverse packed half-word
    REV16 {
        params: Reg2RdRmParams,
        thumb32: bool,
    },

    /// Byte-reverse signed half-word
    REVSH {
        params: Reg2RdRmParams,
        thumb32: bool,
    },

    //SBFX - signed bit field extract
    /// Unsigned bit field extract
    UBFX {
        params: UbfxParams,
    },

    // --------------------------------------------
    //
    // Group: Miscellaneous data-processing instructions (DSP extensions)
    //
    // --------------------------------------------
    /// Select bytes using GE flags
    SEL {
        params: Reg3NoSetFlagsParams,
    },
    //USAD8
    //USADA8

    // --------------------------------------------
    //
    // Group: Status register access instructions
    //
    // --------------------------------------------
    /// Move to Register from Special Register
    MRS {
        params: MrsParams,
    },
    /// Move to Special Register from ARM Register
    MSR_reg {
        params: MsrParams,
    },
    /// Change Processor State
    CPS {
        im: bool,
        #[cfg(any(armv7m, armv7em))]
        affect_pri: bool,
        #[cfg(any(armv7m, armv7em))]
        affect_fault: bool,
    },

    // --------------------------------------------
    //
    // Group:  Load and Store instructions
    //
    // --------------------------------------------
    LDR_reg {
        params: Reg3FullParams,
        thumb32: bool,
    },

    LDRB_reg {
        params: Reg3FullParams,
        thumb32: bool,
    },

    LDRH_reg {
        params: Reg3FullParams,
        thumb32: bool,
    },

    LDRSB_reg {
        params: Reg3FullParams,
        thumb32: bool,
    },

    LDRSH_reg {
        params: Reg3FullParams,
        thumb32: bool,
    },

    LDR_imm {
        params: Reg2FullParams,
        thumb32: bool,
    },
    LDRB_imm {
        params: Reg2FullParams,
        thumb32: bool,
    },
    LDRH_imm {
        params: Reg2FullParams,
        thumb32: bool,
    },
    LDRSB_imm {
        params: Reg2FullParams,
        thumb32: bool,
    },
    LDRSH_imm {
        params: Reg2FullParams,
        thumb32: bool,
    },

    LDR_lit {
        rt: Reg,
        imm32: u32,
        add: bool,
        thumb32: bool,
    },

    STRD_imm {
        rn: Reg,
        rt: Reg,
        rt2: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
    },

    STR_imm {
        params: Reg2FullParams,
        thumb32: bool,
    },
    STRB_reg {
        params: Reg3FullParams,
        thumb32: bool,
    },

    STRH_reg {
        params: Reg3FullParams,
        thumb32: bool,
    },
    STR_reg {
        params: Reg3FullParams,
        thumb32: bool,
    },

    STRH_imm {
        params: Reg2FullParams,
        thumb32: bool,
    },
    STRB_imm {
        params: Reg2FullParams,
        thumb32: bool,
    },

    LDREX {
        params: Reg2RtRnImm32Params,
    },

    LDREXB {
        params: Reg2RtRnParams,
    },

    LDREXH {
        params: Reg2RtRnParams,
    },

    LDRD_imm {
        rn: Reg,
        rt: Reg,
        rt2: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
    },

    STREX {
        params: Reg3RdRtRnImm32Params,
    },

    STREXB {
        params: Reg3RdRtRnParams,
    },

    STREXH {
        params: Reg3RdRtRnParams,
    },

    // --------------------------------------------
    //
    // Group:  Load and Store Multiple instructions
    //
    // --------------------------------------------
    LDM {
        rn: Reg,
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    POP {
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    PUSH {
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    STM {
        rn: Reg,
        registers: EnumSet<Reg>,
        wback: bool,
        thumb32: bool,
    },
    STMDB {
        rn: Reg,
        registers: EnumSet<Reg>,
        wback: bool,
    },

    // --------------------------------------------
    //
    // Group: Miscellaneous
    //
    // --------------------------------------------
    //CLREX
    //DBG
    /// Data Memory Barrier
    DMB,
    /// Data Synchronization Barrier
    DSB,
    /// Instruction Synchronization Barrier
    ISB,

    /// If-then
    IT {
        x: Option<ITCondition>,
        y: Option<ITCondition>,
        z: Option<ITCondition>,
        firstcond: Condition,
        mask: u8,
    },
    /// No operation
    NOP {
        thumb32: bool,
    },

    /// Preload data (immediate)
    PLD_imm {
        rn: Reg,
        imm32: u32,
        add: bool,
    },
    /// Preload data (literal)
    PLD_lit {
        imm32: u32,
        add: bool,
    },
    /// Preload data (register)
    PLD_reg {
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
    },
    /// Send event
    SEV {
        thumb32: bool,
    },
    /// Wait for Event
    WFE {
        thumb32: bool,
    },
    /// Wait for interrupt
    WFI {
        thumb32: bool,
    },
    /// Yield
    YIELD {
        thumb32: bool,
    },
    // --------------------------------------------
    //
    // Group: Exception generating instructions
    //
    // --------------------------------------------
    /// supervisor call
    SVC {
        imm32: u32,
    },
    /// Breakpoint
    BKPT {
        imm32: u32,
    },
    // --------------------------------------------
    //
    // Group: Coprocessor instructions
    //
    // --------------------------------------------
    //CDP, CDP2
    MCR {
        rt: Reg,
        coproc: u8,
        opc1: u8,
        opc2: u8,
        crn: u8,
        crm: u8,
    },
    MCR2 {
        rt: Reg,
        coproc: u8,
        opc1: u8,
        opc2: u8,
        crn: u8,
        crm: u8,
    },
    //MCRR, MCRR2
    //MRC, MRC2
    //MRRC, MRRC2
    LDC_imm {
        coproc: u8,
        imm32: u32,
        crd: u8,
        rn: Reg,
    },

    LDC2_imm {
        coproc: u8,
        imm32: u32,
        crd: u8,
        rn: Reg,
    },

    //STC, STC2
    UDF {
        imm32: u32,
        opcode: ThumbCode,
        thumb32: bool,
    },
    // --------------------------------------------
    //
    // Group: Floating-point load and store instructions
    //
    // --------------------------------------------
    /// FP Load register
    VLDR {
        dd: ExtensionReg,
        rn: Reg,
        add: bool,
        imm32: u32,
    },
    /// FP Store register
    VSTR {
        dd: ExtensionReg,
        rn: Reg,
        add: bool,
        imm32: u32,
    },
    // VLDM
    // VPOP
    // VPUSH
    // VSTM

    // --------------------------------------------
    //
    // Group: Floating-point register transfer instructions
    //
    // --------------------------------------------

    // VMOV
    //VMRS
    //VMRS

    // --------------------------------------------
    //
    // Group: Floating-point data-processing instructions
    //
    // --------------------------------------------
    // VABS
    //VADD
    //VCMP
    //VCVT
    //VDIV
    //VFMA
    //VFNMA
    //VMAXNM
    //VMLA
    //VMOV
    //VMOV
    //VMUL
    //VNEG
    //VNMLA
    //VRINTA
    //VRINTZ
    //VSEL
    //VSQRT
    //VSUB
}

use std::fmt;

fn format_adressing_mode(
    name: &str,
    f: &mut fmt::Formatter,
    params: Reg2FullParams,
    thumb32: bool,
) -> fmt::Result {
    if params.index {
        if params.wback {
            // Pre-indexed
            write!(
                f,
                "{}{} {}, [{} , #{}{}]!",
                name,
                if thumb32 { ".W" } else { "" },
                params.rt,
                params.rn,
                if params.add { "+" } else { "-" },
                params.imm32
            )
        } else {
            // Offset
            write!(
                f,
                "{}{} {}, [{} {{, #{}{}}}]",
                name,
                if thumb32 { ".W" } else { "" },
                params.rt,
                params.rn,
                if params.add { "+" } else { "-" },
                params.imm32
            )
        }
    } else {
        // Post-indexed
        write!(
            f,
            "{}{} {}, [{}], #{}{}",
            name,
            if thumb32 { ".W" } else { "" },
            params.rt,
            params.rn,
            if params.add { "+" } else { "-" },
            params.imm32
        )
    }
}

#[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
fn format_adressing_mode2(
    name: &str,
    f: &mut fmt::Formatter,
    rn: Reg,
    rt: Reg,
    rt2: Reg,
    imm32: u32,
    index: bool,
    add: bool,
    wback: bool,
    thumb32: bool,
) -> fmt::Result {
    if index {
        if wback {
            // Pre-indexed
            write!(
                f,
                "{}{} {}, {}, [{} , #{}{}]!",
                name,
                if thumb32 { ".W" } else { "" },
                rt,
                rt2,
                rn,
                if add { "+" } else { "-" },
                imm32
            )
        } else {
            // Offset
            write!(
                f,
                "{}{} {}, {}, [{} {{, #{}{}}}]",
                name,
                if thumb32 { ".W" } else { "" },
                rt,
                rt2,
                rn,
                if add { "+" } else { "-" },
                imm32
            )
        }
    } else {
        // Post-indexed
        write!(
            f,
            "{}{} {}, {},  [{}], #{}{}",
            name,
            if thumb32 { ".W" } else { "" },
            rt,
            rt2,
            rn,
            if add { "+" } else { "-" },
            imm32
        )
    }
}

fn setflags_to_str(setflags: SetFlags) -> &'static str {
    match setflags {
        SetFlags::True => "s",
        SetFlags::False | SetFlags::NotInITBlock => "",
    }
}

#[allow(clippy::cognitive_complexity)]
#[allow(unused_variables)]
#[allow(clippy::too_many_lines)]
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: shift_t, shift_n formattings missing.
        // TODO: some of the wide instruction formattings missing.
        match *self {
            Self::ADD_imm { params, thumb32 } => {
                if params.rn == params.rd {
                    write!(
                        f,
                        "add{}{} {}, #{}",
                        if thumb32 { ".W" } else { "" },
                        setflags_to_str(params.setflags),
                        params.rd,
                        params.imm32
                    )
                } else {
                    write!(
                        f,
                        "add{}{} {}, {}, #{}",
                        if thumb32 { ".W" } else { "" },
                        setflags_to_str(params.setflags),
                        params.rd,
                        params.rn,
                        params.imm32
                    )
                }
            }
            Self::ADC_imm { params } => write!(
                f,
                "adc{}.W {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.imm32
            ),
            Self::ADD_reg { params, thumb32 } => write!(
                f,
                "add{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ADD_sp_reg { params, thumb32 } => write!(
                f,
                "add{}{} {}, SP, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ADC_reg { params, thumb32 } => write!(
                f,
                "adc{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ADR { params, thumb32 } => write!(
                f,
                "adr{} {}, pc, 0x#{:x}",
                if thumb32 { ".W" } else { "" },
                params.r,
                params.imm32
            ),
            Self::AND_reg { params, thumb32 } => write!(
                f,
                "and{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::AND_imm { params } => write!(
                f,
                "and{}.W {},{}, #{}",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),

            Self::ASR_imm { params, thumb32 } => write!(
                f,
                "asr{}{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                params.shift_n
            ),
            Self::ROR_imm { params } => write!(
                f,
                "ror{}.w {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rm,
                params.shift_n
            ),
            Self::ASR_reg { params, thumb32 } => write!(
                f,
                "asr{}{} {}, {}, {}",
                if thumb32 { ".W" } else { "" },
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.rm
            ),
            Self::BIC_reg { params, thumb32 } => write!(
                f,
                "bic{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::BIC_imm { params } => write!(
                f,
                "bic{} {}, {}, #{}",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::TEQ_imm { params } => write!(
                f,
                "teq.w {}, #{}",
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::B_t13 { params, thumb32 } => write!(
                f,
                "b{}{} {}",
                params.cond,
                if thumb32 { ".W" } else { "" },
                params.imm32
            ),
            Self::B_t24 { imm32, thumb32 } => {
                write!(f, "b{} {}", if thumb32 { ".W" } else { "" }, imm32)
            }
            Self::BL { imm32 } => write!(f, "bl 0x#{:x}", imm32),
            Self::BX { rm } => write!(f, "bx {}", rm),
            Self::BLX { rm } => write!(f, "blx {}", rm),
            Self::BKPT { imm32 } => write!(f, "bkpt #{}", imm32),

            Self::BFI { params } => write!(
                f,
                "bfi {}, {}, #{}, #{}",
                params.rd, params.rn, params.lsbit, params.width
            ),

            Self::BFC { params } => write!(
                f,
                "bfc {}, #{}, #{}",
                params.rd,
                params.lsbit,
                params.msbit - params.lsbit + 1
            ),

            Self::CMN_reg { params, thumb32 } => write!(
                f,
                "cmn{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::CMN_imm { params } => write!(f, "cmn.W {}, #{}", params.r, params.imm32),
            Self::CBZ { params } => write!(f, "cbz {}, #{}", params.rn, params.imm32,),
            Self::CBNZ { params } => write!(f, "cbnz {}, #{}", params.rn, params.imm32,),
            Self::CLZ { params } => write!(f, "clz {},{}", params.rd, params.rm),
            Self::CMP_imm { params, thumb32 } => write!(
                f,
                "cmp{} {}, #{}",
                if thumb32 { ".W" } else { "" },
                params.r,
                params.imm32
            ),
            Self::CMP_reg { params, thumb32 } => write!(
                f,
                "cmp{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),

            #[cfg(any(armv6m))]
            Self::CPS { im } => write!(f, "cps{} i", if im { "ID" } else { "IE" }),
            #[cfg(any(armv7m, armv7em))]
            Self::CPS {
                im,
                affect_pri,
                affect_fault,
            } => write!(
                f,
                "cps{} {}{}",
                if im { "ID" } else { "IE" },
                if affect_pri { "i" } else { "" },
                if affect_fault { "f" } else { "" }
            ),
            Self::DMB => write!(f, "dmb"),
            Self::DSB => write!(f, "dsb"),
            Self::EOR_reg { params, thumb32 } => write!(
                f,
                "eor{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ISB => write!(f, "isb"),
            Self::IT {
                ref x,
                ref y,
                ref z,
                ref firstcond,
                ref mask,
            } => {
                let x_str = match x {
                    Some(c) => format!("{}", c),
                    None => String::new(),
                };
                let y_str = match y {
                    Some(c) => format!("{}", c),
                    None => String::new(),
                };
                let z_str = match z {
                    Some(c) => format!("{}", c),
                    None => String::new(),
                };
                write!(f, "it{}{}{} {}", x_str, y_str, z_str, firstcond)
            }

            Self::LDM {
                rn,
                registers,
                thumb32,
            } => write!(
                f,
                "ldm{} {}, {{{:?}}}",
                if thumb32 { ".W" } else { "" },
                rn,
                registers
            ),
            Self::LDR_reg { params, thumb32 } => write!(
                f,
                "ldr{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                params.rt,
                params.rn,
                params.rm
            ),
            Self::LDR_imm { params, thumb32 } => format_adressing_mode("ldr", f, params, thumb32),
            Self::LDR_lit {
                rt,
                imm32,
                thumb32,
                add,
            } => {
                if imm32 == 0 {
                    write!(f, "ldr{} {}, [pc]", if thumb32 { ".W" } else { "" }, rt)
                } else {
                    write!(
                        f,
                        "ldr{} {}, [pc, #{}{}]",
                        if thumb32 { ".W" } else { "" },
                        rt,
                        if add { "+" } else { "-" },
                        imm32
                    )
                }
            }
            Self::LDREX { params } => {
                write!(f, "ldrex {}, {}, #{}", params.rt, params.rn, params.imm32)
            }
            Self::LDREXB { params } => write!(f, "ldrexb {}, {}", params.rt, params.rn),
            Self::LDREXH { params } => write!(f, "ldrexh {}, {}", params.rt, params.rn),

            Self::LDRB_imm { params, thumb32 } => format_adressing_mode("ldrb", f, params, thumb32),
            Self::LDRB_reg { params, thumb32 } => write!(
                f,
                "ldrb{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                params.rt,
                params.rn,
                params.rm
            ),
            Self::LDRH_imm { params, thumb32 } => format_adressing_mode("ldrh", f, params, thumb32),
            Self::LDRH_reg { params, thumb32 } => write!(
                f,
                "ldrh{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                params.rt,
                params.rn,
                params.rm
            ),
            Self::LDRSB_reg { params, thumb32 } => {
                write!(f, "ldrsb {}, [{}, {}]", params.rt, params.rn, params.rm)
            }
            Self::LDRSH_reg { params, thumb32 } => write!(
                f,
                "ldrsh{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                params.rt,
                params.rn,
                params.rm
            ),
            Self::LSL_imm { params, thumb32 } => write!(
                f,
                "lsl{}{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                params.shift_n
            ),
            Self::LSL_reg { params, thumb32 } => write!(
                f,
                "lsl{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::LSR_reg { params, thumb32 } => write!(
                f,
                "lsr{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::LSR_imm { params, thumb32 } => write!(
                f,
                "lsr{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rm,
                params.shift_n
            ),
            Self::MSR_reg { params } => write!(f, "msr {}, {}", params.sysm, params.rn),
            Self::MRS { params } => write!(f, "mrs {}, {}", params.rd, params.sysm),
            Self::MUL { params, thumb32 } => write!(
                f,
                "mul{} {}, {}, {}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.rm
            ),
            Self::SMUL { params } => write!(
                f,
                "smul{}{} {}, {}, {}",
                if params.n_high { "T" } else { "B" },
                if params.m_high { "T" } else { "B" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::SMLA { params } => write!(
                f,
                "smla{}{} {}, {}, {}, {}",
                if params.n_high { "T" } else { "B" },
                if params.m_high { "T" } else { "B" },
                params.rd,
                params.rn,
                params.rm,
                params.ra
            ),
            Self::MOV_reg { params, thumb32 } => write!(
                f,
                "mov{}{} {}, {}",
                if params.setflags { "s" } else { "" },
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm
            ),
            Self::MOV_imm { params, thumb32 } => write!(
                f,
                "mov{}{} {}, #{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::MOVT { params } => write!(f, "movt {}, #{}", params.rd, params.imm16),
            Self::LDRSH_imm { params, thumb32 } => {
                format_adressing_mode("ldrsh", f, params, thumb32)
            }

            Self::LDRSB_imm { params, thumb32 } => {
                format_adressing_mode("ldrsb", f, params, thumb32)
            }
            Self::MVN_reg { params, thumb32 } => write!(
                f,
                "mvn{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::MVN_imm { params } => write!(
                f,
                "mvn{} {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::NOP { .. } => write!(f, "nop"),
            Self::ORR_reg { params, thumb32 } => write!(
                f,
                "orr{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::ORR_imm { params } => write!(
                f,
                "orr{} {}, {}, #{}",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::ORN_reg { params } => write!(
                f,
                "orn{}.w {}, {}, {}{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::EOR_imm { params } => write!(
                f,
                "eor{} {}, {}, #{}",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::POP { registers, thumb32 } => {
                write!(f, "pop{} {:?}", if thumb32 { ".W" } else { "" }, registers)
            }
            Self::PUSH { thumb32, registers } => {
                write!(f, "push{} {:?}", if thumb32 { ".W" } else { "" }, registers)
            }
            Self::PLD_imm { rn, imm32, add } => {
                write!(f, "pld [{}, {}{}]", rn, if add { "+" } else { "-" }, imm32)
            }
            Self::PLD_lit { imm32, add } => {
                write!(f, "pld [PC, {}{}]", if add { "+" } else { "-" }, imm32)
            }
            Self::PLD_reg {
                rn,
                rm,
                shift_t,
                shift_n,
            } => write!(
                f,
                "pld [{}, {}, {}]",
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),

            Self::REV { params, .. } => write!(f, "rev {}, {}", params.rd, params.rm),
            Self::REV16 { params, .. } => write!(f, "rev16 {}, {}", params.rd, params.rm),
            Self::REVSH { params, .. } => write!(f, "revsh {}, {}", params.rd, params.rm),
            Self::ROR_reg { params, .. } => write!(
                f,
                "ror{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.rm
            ),
            Self::RSB_imm { params, thumb32 } => write!(
                f,
                "rsb{}{} {}, {}, #{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.imm32
            ),
            Self::RRX { params } => write!(
                f,
                "mov.w{} {}, {}, rrx",
                if params.setflags { "s" } else { "" },
                params.rd,
                params.rm,
            ),

            Self::SBC_imm { params } => write!(
                f,
                "sbc{}.W {}, {}, #{}",
                setflags_to_str(params.setflags),
                params.rd,
                params.rn,
                params.imm32
            ),
            Self::RSB_reg { params, thumb32 } => write!(
                f,
                "rsb{}{} {}, {}, {}{}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),

            Self::SEV { .. } => write!(f, "sev"),
            Self::SBC_reg { params, thumb32 } => write!(
                f,
                "sbc{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::STM {
                rn,
                wback,
                registers,
                thumb32,
            } => write!(
                f,
                "stm{} {}{}, {{{:?}}}",
                if thumb32 { ".W" } else { "" },
                rn,
                if wback { "!" } else { "" },
                registers
            ),
            Self::STMDB {
                rn,
                wback,
                registers,
            } => write!(
                f,
                "stmdb {}{}, {{{:?}}}",
                rn,
                if wback { "!" } else { "" },
                registers
            ),
            Self::STR_imm { params, thumb32 } => format_adressing_mode("str", f, params, thumb32),
            Self::STREX { params } => write!(
                f,
                "strex {}, {}, {}, #{}",
                params.rd, params.rt, params.rn, params.imm32
            ),
            Self::STREXB { params } => {
                write!(f, "strexb {}, {}, {}", params.rd, params.rt, params.rn)
            }
            Self::STREXH { params } => {
                write!(f, "strexh {}, {}, {} ", params.rd, params.rt, params.rn)
            }

            Self::STRD_imm {
                rn,
                rt,
                rt2,
                imm32,
                index,
                add,
                wback,
            } => format_adressing_mode2("strd", f, rn, rt, rt2, imm32, index, add, wback, true),
            Self::LDRD_imm {
                rn,
                rt,
                rt2,
                imm32,
                index,
                add,
                wback,
            } => format_adressing_mode2("ldrd", f, rn, rt, rt2, imm32, index, add, wback, true),
            Self::STR_reg { params, thumb32 } => {
                write!(f, "str {}, [{}, {}]", params.rt, params.rn, params.rm)
            }
            Self::STRB_imm { params, thumb32 } => format_adressing_mode("strb", f, params, thumb32),
            Self::STRB_reg { params, thumb32 } => {
                write!(f, "strb {}, [{}, {}]", params.rt, params.rn, params.rm)
            }
            Self::STRH_imm { params, thumb32 } => format_adressing_mode("strh", f, params, thumb32),
            Self::STRH_reg { params, thumb32 } => {
                write!(f, "strh {}, [{}, {}]", params.rt, params.rn, params.rm)
            }
            Self::SUB_imm { params, thumb32 } => {
                if params.rd == params.rn {
                    write!(
                        f,
                        "sub{}{} {}, #{}",
                        setflags_to_str(params.setflags),
                        if thumb32 { ".W" } else { "" },
                        params.rd,
                        params.imm32
                    )
                } else {
                    write!(
                        f,
                        "sub{}{} {}, {}, #{}",
                        setflags_to_str(params.setflags),
                        if thumb32 { ".W" } else { "" },
                        params.rd,
                        params.rn,
                        params.imm32
                    )
                }
            }
            Self::SUB_reg { params, thumb32 } => write!(
                f,
                "sub{}{} {}, {}, {}",
                setflags_to_str(params.setflags),
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rn,
                params.rm
            ),
            Self::TEQ_reg { params } => write!(
                f,
                "teq.W {}, {}, {}",
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::SVC { imm32 } => write!(f, "svc #{}", imm32),
            Self::SXTH { params, thumb32 } => write!(
                f,
                "sxth{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),

            Self::SXTB { params, thumb32 } => write!(
                f,
                "sxtb{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),
            Self::TBB { params } => write!(f, "tbb [{}, {}]", params.rn, params.rm),
            Self::TBH { params } => write!(f, "tbh [{}, {}, lsl #1]", params.rn, params.rm),
            Self::TST_reg { params, thumb32 } => write!(
                f,
                "tst{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rn,
                params.rm,
                if params.shift_n > 0 {
                    format!(", {:?} {}", params.shift_t, params.shift_n)
                } else {
                    "".to_string()
                }
            ),
            Self::TST_imm { params } => write!(
                f,
                "tst {}, #{}",
                params.rn,
                match params.imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Self::UDF {
                imm32, ref opcode, ..
            } => write!(f, "udf {} (opcode = {})", imm32, opcode),

            Self::UADD8 { params } => {
                write!(f, "uadd8 {}, {}, {}", params.rd, params.rn, params.rm)
            }
            Self::SEL { params } => write!(f, "sel {}, {}, {}", params.rd, params.rn, params.rm),
            // ARMv7-M
            Self::UDIV { params } => write!(f, "udiv {}, {}, {}", params.rd, params.rn, params.rm),
            Self::SDIV { params } => write!(f, "sdiv {}, {}, {}", params.rd, params.rn, params.rm),
            // ARMv7-M
            Self::UMLAL { params } => write!(
                f,
                "umlal {}, {}, {}, {}",
                params.rdlo, params.rdhi, params.rn, params.rm
            ),
            // ARMv7-M
            Self::UMULL { params } => write!(
                f,
                "umull {}, {}, {}, {}",
                params.rdlo, params.rdhi, params.rn, params.rm
            ),
            Self::SMULL { params } => write!(
                f,
                "smull {}, {}, {}, {}",
                params.rdlo, params.rdhi, params.rn, params.rm
            ),
            // ARMv7-M
            Self::MLA { params } => write!(
                f,
                "mla {}, {}, {}, {}",
                params.rd, params.rn, params.rm, params.ra
            ),
            // ARMv7-M
            Self::MLS { params } => write!(
                f,
                "mls {}, {}, {}, {}",
                params.rd, params.rn, params.rm, params.ra
            ),
            // ARMv7-M
            Self::SMLAL { params } => write!(
                f,
                "smlal {}, {}, {}, {}",
                params.rdlo, params.rdhi, params.rn, params.rm
            ),
            Self::UXTB { params, thumb32 } => write!(
                f,
                "uxtb{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),
            Self::UXTAB { params } => write!(
                f,
                "uxtb.w {},{},{} {}",
                params.rd,
                params.rn,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),
            Self::UXTH { params, thumb32 } => write!(
                f,
                "uxth{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                params.rd,
                params.rm,
                if params.rotation > 0 {
                    format!("{}", params.rotation)
                } else {
                    "".to_string()
                }
            ),
            Self::UBFX { params } => write!(
                f,
                "ubfx {}, {}, #{}, #{}",
                params.rd,
                params.rn,
                params.lsb,
                params.widthminus1 + 1
            ),
            Self::VLDR {
                dd,
                rn,
                add,
                imm32,
            } => write!(f, "vldr {}, {}", dd, rn),
            Self::VSTR {
                dd,
                rn,
                add,
                imm32,
            } => write!(f, "vstr {}, {}", dd, rn),

            Self::WFE { .. } => write!(f, "wfe"),
            Self::WFI { .. } => write!(f, "wfi"),
            Self::YIELD { .. } => write!(f, "yield"),
            // ARMv7-M
            Self::MCR {
                ref rt,
                ref coproc,
                ref opc1,
                ref opc2,
                ref crn,
                ref crm,
            } => write!(f, "mcr"),

            // ARMv7-M
            Self::MCR2 {
                ref rt,
                ref coproc,
                ref opc1,
                ref opc2,
                ref crn,
                ref crm,
            } => write!(f, "mcr2"),

            // ARMv7-M
            Self::LDC_imm {
                ref coproc,
                ref imm32,
                ref crd,
                ref rn,
            } => write!(f, "ldc"),

            // ARMv7-M
            Self::LDC2_imm {
                ref coproc,
                ref imm32,
                ref crd,
                ref rn,
            } => write!(f, "ldc2"),
        }
    }
}

#[allow(unused_variables)]
impl fmt::Display for ITCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Then => write!(f, "t"),
            Self::Else => write!(f, "e"),
        }
    }
}

#[allow(clippy::cognitive_complexity)]
#[allow(unused_variables)]
#[allow(clippy::too_many_lines)]
/// Get the size of an instruction in bytes
pub fn instruction_size(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::ADC_imm { .. } => 4,
        Instruction::ADC_reg { params, thumb32 } => isize_t(*thumb32),
        Instruction::ADD_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::ADD_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::ADD_sp_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::ADR { thumb32, .. } => isize_t(*thumb32),
        Instruction::AND_imm { .. } => 4,
        Instruction::AND_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::ASR_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::ASR_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::B_t13 { thumb32, .. } => isize_t(*thumb32),
        Instruction::B_t24 { thumb32, .. } => isize_t(*thumb32),
        Instruction::BFI { .. } => 4,
        Instruction::BFC { .. } => 4,
        Instruction::BIC_imm { .. } => 4,
        Instruction::BIC_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::BKPT { .. } => 2,
        Instruction::BL { .. } => 4,
        Instruction::BLX { .. } => 2,
        Instruction::BX { .. } => 2,

        Instruction::CBZ { .. } => 2,
        Instruction::CBNZ { .. } => 2,
        //CDP
        //CLREX
        Instruction::CLZ { .. } => 4,
        Instruction::CMN_imm { .. } => 4,
        Instruction::CMN_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::CMP_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::CMP_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::CPS { .. } => 2,

        Instruction::DMB { .. } => 4,
        Instruction::DSB { .. } => 4,

        Instruction::EOR_imm { .. } => 4,
        Instruction::EOR_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::ISB { .. } => 4,
        Instruction::IT { .. } => 2,

        Instruction::LDC_imm { .. } => 4,
        Instruction::LDC2_imm { .. } => 4,
        Instruction::LDM { thumb32, .. } => isize_t(*thumb32),
        //LDMDB
        Instruction::LDR_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::LDR_lit { thumb32, .. } => isize_t(*thumb32),
        Instruction::LDR_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::LDRB_imm { thumb32, .. } => isize_t(*thumb32),
        //LDRB_lit
        Instruction::LDRB_reg { thumb32, .. } => isize_t(*thumb32),
        //LDRBT
        Instruction::LDRD_imm { .. } => 4,
        //LDRD_lit
        Instruction::LDREX { .. } => 4,
        Instruction::LDREXB { .. } => 4,
        Instruction::LDREXH { .. } => 4,
        Instruction::LDRH_imm { thumb32, .. } => isize_t(*thumb32),
        //LDRH_lit
        Instruction::LDRH_reg { thumb32, .. } => isize_t(*thumb32),
        //LDRHT
        Instruction::LDRSB_imm { thumb32, .. } => isize_t(*thumb32),
        //LDRSB_lit
        Instruction::LDRSB_reg { thumb32, .. } => isize_t(*thumb32),
        //LDRSBT
        Instruction::LDRSH_imm { thumb32, .. } => isize_t(*thumb32),
        //LDRSH_lit
        Instruction::LDRSH_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::LSL_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::LSL_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::LSR_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::LSR_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::MCR { .. } => 4,
        Instruction::MCR2 { .. } => 4,
        Instruction::MLA { .. } => 4,
        Instruction::MLS { .. } => 4,
        Instruction::MOV_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::MOV_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::MOVT { .. } => 4,
        //MRC, MRC2
        //MRRC, MRRC2
        Instruction::MRS { .. } => 4,
        Instruction::MSR_reg { .. } => 4,
        Instruction::MUL { thumb32, .. } => isize_t(*thumb32),
        Instruction::MVN_imm { .. } => 4,
        Instruction::MVN_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::NOP { thumb32, .. } => isize_t(*thumb32),

        //ORN_imm
        Instruction::ORN_reg { .. } => 4,
        Instruction::ORR_imm { .. } => 4,
        Instruction::ORR_reg { thumb32, .. } => isize_t(*thumb32),

        //PKHBT, PKHTB
        Instruction::PLD_imm { .. } => 4,
        Instruction::PLD_lit { .. } => 4,
        Instruction::PLD_reg { .. } => 4,
        //PLI_imm,
        //PLI_reg
        Instruction::POP { thumb32, .. } => isize_t(*thumb32),
        Instruction::PUSH { thumb32, .. } => isize_t(*thumb32),

        //QADD16
        //QADD8
        //QASX
        //QSAX
        //QADD
        //QSUB
        //QDADD
        //QDSUB
        //QSUB16
        //QSUB8

        //RBIT
        Instruction::REV { thumb32, .. } => isize_t(*thumb32),
        Instruction::REV16 { thumb32, .. } => isize_t(*thumb32),
        Instruction::REVSH { thumb32, .. } => isize_t(*thumb32),
        Instruction::ROR_imm { .. } => 4,
        Instruction::ROR_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::RRX { .. } => 4,
        Instruction::RSB_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::RSB_reg { thumb32, .. } => 4,
        //SADD16
        //SADD8
        //SASX
        Instruction::SBC_imm { .. } => 4,
        Instruction::SBC_reg { thumb32, .. } => isize_t(*thumb32),
        //SBFX
        Instruction::SDIV { .. } => 4,
        Instruction::SEL { .. } => 4,
        Instruction::SEV { thumb32, .. } => isize_t(*thumb32),
        //SHADD16
        //SHADD8
        //SHASX
        //SHSAX
        //SHSUB16
        //SHSUB8
        Instruction::SMLA { .. } => 4,
        //SMLAD
        Instruction::SMLAL { .. } => 4,
        //SMLAL second variant?
        //SMLALD
        //SMLAW
        //SMLSD
        //SMLSLD
        //SMMLA
        //SMMLS
        //SMMUL
        //SMUAD
        Instruction::SMUL { .. } => 4,
        Instruction::SMULL { .. } => 4,
        //SMULW
        //SMUSD
        //SSAT
        //SSAT16
        //SSAX
        //SSUB16
        //SSUB8
        //STC, STC2
        Instruction::STM { thumb32, .. } => isize_t(*thumb32),
        Instruction::STMDB { .. } => 4,
        Instruction::STR_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::STR_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::STRB_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::STRB_reg { thumb32, .. } => isize_t(*thumb32),
        //STRBT
        Instruction::STRD_imm { .. } => 4,
        Instruction::STREX { .. } => 4,
        Instruction::STREXB { .. } => 4,
        Instruction::STREXH { .. } => 4,
        Instruction::STRH_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::STRH_reg { thumb32, .. } => isize_t(*thumb32),
        //STRHT
        //STRT
        Instruction::SUB_imm { thumb32, .. } => isize_t(*thumb32),
        Instruction::SUB_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::SVC { .. } => 2,
        //SXTAB
        //SXTAB16
        //SXTAH
        Instruction::SXTB { thumb32, .. } => isize_t(*thumb32),
        //SXTB16
        Instruction::SXTH { thumb32, .. } => isize_t(*thumb32),

        Instruction::TBB { .. } => 4,
        Instruction::TBH { .. } => 4,
        Instruction::TEQ_imm { .. } => 4,
        Instruction::TEQ_reg { .. } => 4,
        Instruction::TST_imm { .. } => 4,
        Instruction::TST_reg { thumb32, .. } => isize_t(*thumb32),

        Instruction::UADD8 { .. } => 4,
        //UADD16
        //UASX
        Instruction::UBFX { .. } => 4,
        Instruction::UDF { thumb32, .. } => isize_t(*thumb32),
        Instruction::UDIV { .. } => 4,
        //UHADD16
        //UHADD8
        //UHASX
        //UHSAX
        //UHSUB16
        //UHSUB8
        //UMAAL
        Instruction::UMLAL { .. } => 4,
        Instruction::UMULL { .. } => 4,
        //UQADD16
        //UQADD8
        //UQASX
        //UQSAX
        //UQSUB16
        //UQSUB8
        //USAD8
        //USADA8
        //USAT
        //USAT16
        //USAX
        //USUB16
        //USUB8
        Instruction::UXTAB { .. } => 4,
        //UXTAB16
        //UXTAH
        Instruction::UXTB { thumb32, .. } => isize_t(*thumb32),
        Instruction::UXTH { thumb32, .. } => isize_t(*thumb32),

        //VABS
        //VADD
        //VCMP
        //VCVTX
        //VCVT
        //VCVTB
        //VCVTT
        //VDIV
        //VFMA
        //VFMS
        //VFNMA
        //VFNMS
        //VLDM
        //VMAXNM
        //VMINNM
        //VMLA
        //VMLS
        //VMOV_imm
        //VMON_reg
        //VMOVX
        //VMRS
        //VMSR
        //VMUL
        //VNEG
        //VNMLA,VNMLS, VNMUL
        //VPOP
        //VPUSH
        //VRINTA, VRINTN, VRINTP, VRiNTM
        //VRINTX,
        //VRINTZ, VRINTR
        //VSEL
        //VSQRT
        //VSTM
        //VSTR
        //VSUB
        Instruction::WFE { thumb32, .. } => isize_t(*thumb32),
        Instruction::WFI { thumb32, .. } => isize_t(*thumb32),
        Instruction::YIELD { thumb32, .. } => isize_t(*thumb32),
        Instruction::VLDR { .. } => 4,
        Instruction::VSTR { .. } => 4,
    }
}

#[inline(always)]
fn isize_t(thumb32: bool) -> usize {
    if thumb32 {
        4
    } else {
        2
    }
}
