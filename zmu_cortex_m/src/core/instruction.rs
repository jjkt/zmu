//!
//! Representation of Cortex M Instruction set
//!

use crate::core::condition::Condition;
use crate::core::register::Reg;
use crate::core::register::SpecialReg;
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
    /// precalculated value carry value was not relevant
    /// for the decoding
    NoCarry {
        /// imm32 original value
        imm32: u32,
    },
    /// precalculated values for cases ASPR.C == 0 and ASPR.C ==1
    /// if carry was relevant for the decoding
    /// tuple of (immediate value, carry)
    Carry {
        /// values of imm32 and carry, when carry was originally 0
        imm32_c0: (u32, bool),
        /// values of imm32 and carry, when carry was originally 1
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

#[allow(non_camel_case_types, missing_docs)]
#[derive(PartialEq, Debug, Copy, Clone)]
///
/// List of acknowledged instrctions
///
pub enum Instruction {
    ADC_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: SetFlags,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    ADC_imm {
        rd: Reg,
        rn: Reg,
        imm32: u32,
        setflags: SetFlags,
    },
    ADD_imm {
        rn: Reg,
        rd: Reg,
        imm32: u32,
        setflags: SetFlags,
        thumb32: bool,
    },
    ADD_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: SetFlags,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    ADD_sp_reg {
        rd: Reg,
        rm: Reg,
        setflags: bool,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    ADR {
        rd: Reg,
        imm32: u32,
        thumb32: bool,
    },
    AND_reg {
        rd: Reg,
        rm: Reg,
        rn: Reg,
        setflags: SetFlags,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    AND_imm {
        rd: Reg,
        rn: Reg,
        imm32: Imm32Carry,
        setflags: bool,
    },
    ASR_imm {
        rd: Reg,
        rm: Reg,
        shift_n: u8,
        setflags: SetFlags,
        thumb32: bool,
    },
    ASR_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: SetFlags,
        thumb32: bool,
    },

    B_t13 {
        cond: Condition,
        imm32: i32,
        thumb32: bool,
    },
    B_t24 {
        imm32: i32,
        thumb32: bool,
    },
    BIC_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: SetFlags,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    BIC_imm {
        rd: Reg,
        rn: Reg,
        imm32: Imm32Carry,
        setflags: bool,
    },
    BKPT {
        imm32: u32,
    },
    BL {
        imm32: i32,
    },
    BLX {
        rm: Reg,
    },
    BX {
        rm: Reg,
    },
    BFI {
        rd: Reg,
        rn: Reg,
        lsbit: usize,
        msbit: usize,
    },
    CBZ {
        rn: Reg,
        nonzero: bool,
        imm32: u32,
    },
    CLZ {
        rd: Reg,
        rm: Reg,
    },
    CMN_reg {
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    CMN_imm {
        rn: Reg,
        imm32: u32,
    },
    CMP_imm {
        rn: Reg,
        imm32: u32,
        thumb32: bool,
    },
    CMP_reg {
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    CPS {
        im: CpsEffect,
    },
    DMB,
    DSB,
    EOR_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        setflags: SetFlags,
        thumb32: bool,
    },
    EOR_imm {
        rd: Reg,
        rn: Reg,
        imm32: Imm32Carry,
        setflags: bool,
    },
    ROR_imm {
        rd: Reg,
        rm: Reg,
        shift_n: u8,
        setflags: bool,
    },
    ISB,
    IT {
        x: Option<ITCondition>,
        y: Option<ITCondition>,
        z: Option<ITCondition>,
        firstcond: Condition,
        mask: u8,
    },

    // ARMv7-M
    LDC_imm {
        coproc: u8,
        imm32: u32,
        crd: u8,
        rn: Reg,
    },

    // ARMv7-M
    LDC2_imm {
        coproc: u8,
        imm32: u32,
        crd: u8,
        rn: Reg,
    },

    LDM {
        rn: Reg,
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    LDR_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDR_lit {
        rt: Reg,
        imm32: u32,
        add: bool,
        thumb32: bool,
    },
    LDR_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRB_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRB_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRH_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRH_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRSB_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    LDRSB_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },

    LDRSH_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },

    LDRSH_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },

    LSL_imm {
        rd: Reg,
        rm: Reg,
        shift_n: u8,
        setflags: SetFlags,
        thumb32: bool,
    },
    LSL_reg {
        rd: Reg,
        rm: Reg,
        rn: Reg,
        setflags: SetFlags,
        thumb32: bool,
    },
    LSR_imm {
        rd: Reg,
        rm: Reg,
        shift_n: u8,
        setflags: SetFlags,
        thumb32: bool,
    },
    LSR_reg {
        rd: Reg,
        rm: Reg,
        rn: Reg,
        setflags: SetFlags,
        thumb32: bool,
    },

    // ARMv7-M
    MCR {
        rt: Reg,
        coproc: u8,
        opc1: u8,
        opc2: u8,
        crn: u8,
        crm: u8,
    },
    // ARMv7-M
    MCR2 {
        rt: Reg,
        coproc: u8,
        opc1: u8,
        opc2: u8,
        crn: u8,
        crm: u8,
    },

    MOV_imm {
        rd: Reg,
        imm32: Imm32Carry,
        setflags: SetFlags,
        thumb32: bool,
    },
    MOV_reg {
        rd: Reg,
        rm: Reg,
        setflags: bool,
        thumb32: bool,
    },
    MOVT {
        rd: Reg,
        imm16: u16,
    },
    MRS {
        rd: Reg,
        spec_reg: SpecialReg,
    },
    MSR_reg {
        rn: Reg,
        spec_reg: SpecialReg,
    },
    MUL {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: SetFlags,
        thumb32: bool,
    },
    MVN_reg {
        rd: Reg,
        rm: Reg,
        setflags: SetFlags,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    MVN_imm {
        rd: Reg,
        imm32: Imm32Carry,
        setflags: bool,
    },
    NOP {
        thumb32: bool,
    },
    ORR_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        setflags: SetFlags,
        thumb32: bool,
    },
    ORR_imm {
        rd: Reg,
        rn: Reg,
        imm32: Imm32Carry,
        setflags: bool,
    },
    ORN_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        setflags: bool,
    },
    POP {
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    PLD_imm {
        rn: Reg,
        imm32: u32,
        add: bool,
    },
    PLD_lit {
        imm32: u32,
        add: bool,
    },
    PLD_reg {
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
    },
    PUSH {
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    REV {
        rd: Reg,
        rm: Reg,
        thumb32: bool,
    },
    REV16 {
        rd: Reg,
        rm: Reg,
        thumb32: bool,
    },
    REVSH {
        rd: Reg,
        rm: Reg,
        thumb32: bool,
    },
    ROR_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: SetFlags,
        thumb32: bool,
    },
    RSB_imm {
        rd: Reg,
        rn: Reg,
        imm32: u32,
        setflags: SetFlags,
        thumb32: bool,
    },
    RSB_reg {
        rd: Reg,
        rm: Reg,
        rn: Reg,
        setflags: bool,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    RRX {
        rd: Reg,
        rm: Reg,
        setflags: bool,
    },
    SBC_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: SetFlags,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    SBC_imm {
        rd: Reg,
        rn: Reg,
        imm32: u32,
        setflags: bool,
    },
    SEV {
        thumb32: bool,
    },
    SEL {
        rd: Reg,
        rn: Reg,
        rm: Reg,
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
    STR_imm {
        rn: Reg,
        rt: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
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
    LDRD_imm {
        rn: Reg,
        rt: Reg,
        rt2: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
    },
    STR_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STRB_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STRB_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STRH_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    STRH_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
        shift_t: SRType,
        shift_n: u8,
        index: bool,
        add: bool,
        wback: bool,
        thumb32: bool,
    },
    SUB_imm {
        rd: Reg,
        rn: Reg,
        imm32: u32,
        setflags: SetFlags,
        thumb32: bool,
    },
    SUB_reg {
        rm: Reg,
        rn: Reg,
        rd: Reg,
        setflags: SetFlags,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    SVC {
        imm32: u32,
    },
    SXTB {
        rd: Reg,
        rm: Reg,
        rotation: usize,
        thumb32: bool,
    },
    SXTH {
        rd: Reg,
        rm: Reg,
        rotation: usize,
        thumb32: bool,
    },
    TST_reg {
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
        thumb32: bool,
    },
    TST_imm {
        rn: Reg,
        imm32: Imm32Carry,
    },
    TEQ_reg {
        rn: Reg,
        rm: Reg,
        shift_t: SRType,
        shift_n: u8,
    },
    TEQ_imm {
        rn: Reg,
        imm32: Imm32Carry,
    },
    TBB {
        rn: Reg,
        rm: Reg,
    },
    TBH {
        rn: Reg,
        rm: Reg,
    },
    UDF {
        imm32: u32,
        opcode: ThumbCode,
        thumb32: bool,
    },
    UADD8 {
        rd: Reg,
        rn: Reg,
        rm: Reg,
    },
    // ARMv7-M
    UBFX {
        rd: Reg,
        rn: Reg,
        lsb: usize,
        widthminus1: usize,
    },
    // ARMv7-M
    UDIV {
        rd: Reg,
        rn: Reg,
        rm: Reg,
    },
    // ARMv7-M
    SDIV {
        rd: Reg,
        rn: Reg,
        rm: Reg,
    },
    // ARMv7-M
    MLA {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        ra: Reg,
    },
    // ARMv7-M
    MLS {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        ra: Reg,
    },
    // ARMv7-M
    UMLAL {
        rm: Reg,
        rdlo: Reg,
        rdhi: Reg,
        rn: Reg,
    },
    // ARMv7-M
    UMULL {
        rm: Reg,
        rdlo: Reg,
        rdhi: Reg,
        rn: Reg,
    },
    // ARMv7-M
    SMLAL {
        rm: Reg,
        rdlo: Reg,
        rdhi: Reg,
        rn: Reg,
    },
    // ARMv7-Me
    SMUL {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        n_high: bool,
        m_high: bool,
    },
    SMULL {
        rdlo: Reg,
        rdhi: Reg,
        rn: Reg,
        rm: Reg,
    },
    // ARMv7-Me
    SMLA {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        ra: Reg,
        n_high: bool,
        m_high: bool,
    },
    UXTB {
        rd: Reg,
        rm: Reg,
        thumb32: bool,
        rotation: usize,
    },
    UXTH {
        rd: Reg,
        rm: Reg,
        rotation: usize,
        thumb32: bool,
    },
    UXTAB {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        rotation: usize,
    },
    WFE {
        thumb32: bool,
    },
    WFI {
        thumb32: bool,
    },
    YIELD {
        thumb32: bool,
    },
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
/// variant of CPS call
pub enum CpsEffect {
    /// Interrupt enable
    IE,
    /// Interrupt disable
    ID,
}

impl fmt::Display for CpsEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CpsEffect::IE => write!(f, "IE"),
            CpsEffect::ID => write!(f, "ID"),
        }
    }
}

use std::fmt;

fn format_adressing_mode(
    name: &str,
    f: &mut fmt::Formatter,
    rn: Reg,
    rt: Reg,
    imm32: u32,
    index: bool,
    add: bool,
    wback: bool,
    thumb32: bool,
) -> fmt::Result {
    if index {
        if !wback {
            // Offset
            write!(
                f,
                "{}{} {}, [{} {{, #{}{}}}]",
                name,
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                if add { "+" } else { "-" },
                imm32
            )
        } else {
            // Pre-indexed
            write!(
                f,
                "{}{} {}, [{} , #{}{}]!",
                name,
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                if add { "+" } else { "-" },
                imm32
            )
        }
    } else {
        // Post-indexed
        write!(
            f,
            "{}{} {}, [{}], #{}{}",
            name,
            if thumb32 { ".W" } else { "" },
            rt,
            rn,
            if add { "+" } else { "-" },
            imm32
        )
    }
}

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
        if !wback {
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
        } else {
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
        SetFlags::False => "",
        SetFlags::NotInITBlock => "",
    }
}

#[allow(clippy::cyclomatic_complexity)]
#[allow(unused_variables)]
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: shift_t, shift_n formattings missing.
        // TODO: some of the wide instruction formattings missing.
        match *self {
            Instruction::ADD_imm {
                rn,
                rd,
                imm32,
                ref setflags,
                thumb32,
            } => {
                if rn == rd {
                    write!(
                        f,
                        "add{}{} {}, #{}",
                        if thumb32 { ".W" } else { "" },
                        setflags_to_str(*setflags),
                        rd,
                        imm32
                    )
                } else {
                    write!(
                        f,
                        "add{}{} {}, {}, #{}",
                        if thumb32 { ".W" } else { "" },
                        setflags_to_str(*setflags),
                        rd,
                        rn,
                        imm32
                    )
                }
            }
            Instruction::ADC_imm {
                rd,
                rn,
                imm32,
                ref setflags,
            } => write!(
                f,
                "adc{}.W {}, {}, #{}",
                setflags_to_str(*setflags),
                rd,
                rn,
                imm32
            ),
            Instruction::ADD_reg {
                rm,
                rn,
                rd,
                ref setflags,
                ref shift_t,
                shift_n,
                thumb32,
            } => write!(
                f,
                "add{}{} {}, {}, {}{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::ADD_sp_reg {
                rm,
                rd,
                setflags,
                ref shift_t,
                shift_n,
                thumb32,
            } => write!(
                f,
                "add{}{} {}, SP, {}{}",
                if setflags { "s" } else { "" },
                if thumb32 { ".W" } else { "" },
                rd,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::ADC_reg {
                rm,
                rn,
                rd,
                ref setflags,
                ref shift_t,
                shift_n,
                thumb32,
            } => write!(
                f,
                "adc{}{} {}, {}, {}{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::ADR { rd, imm32, thumb32 } => write!(
                f,
                "adr{} {}, pc, 0x#{:x}",
                if thumb32 { ".W" } else { "" },
                rd,
                imm32
            ),
            Instruction::AND_reg {
                rd,
                rn,
                rm,
                ref shift_t,
                shift_n,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "and{}{} {}, {}, {}{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::AND_imm {
                rd,
                rn,
                ref imm32,
                setflags,
            } => write!(
                f,
                "and{}.W {},{}, #{}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),

            Instruction::ASR_imm {
                rd,
                rm,
                shift_n,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "asr{}{} {}, {}, #{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rm,
                shift_n
            ),
            Instruction::ROR_imm {
                rd,
                rm,
                shift_n,
                setflags,
            } => write!(
                f,
                "ror{}.w {}, {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                rm,
                shift_n
            ),
            Instruction::ASR_reg {
                rd,
                rn,
                rm,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "asr{}{} {}, {}, {}",
                if thumb32 { ".W" } else { "" },
                setflags_to_str(*setflags),
                rd,
                rn,
                rm
            ),
            Instruction::BIC_reg {
                rd,
                rn,
                rm,
                ref shift_t,
                shift_n,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "bic{}{} {}, {}, {}{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::BIC_imm {
                rd,
                rn,
                ref imm32,
                setflags,
            } => write!(
                f,
                "bic{} {}, {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Instruction::TEQ_imm { rn, ref imm32 } => write!(
                f,
                "teq.w {}, #{}",
                rn,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Instruction::B_t13 {
                ref cond,
                imm32,
                thumb32,
            } => write!(f, "b{}{} {}", cond, if thumb32 { ".W" } else { "" }, imm32),
            Instruction::B_t24 { imm32, thumb32 } => {
                write!(f, "b{} {}", if thumb32 { ".W" } else { "" }, imm32)
            }
            Instruction::BL { imm32 } => write!(f, "bl 0x#{:x}", imm32),
            Instruction::BX { rm } => write!(f, "bx {}", rm),
            Instruction::BLX { rm } => write!(f, "blx {}", rm),
            Instruction::BKPT { imm32 } => write!(f, "bkpt #{}", imm32),

            Instruction::BFI {
                ref rn,
                ref rd,
                ref lsbit,
                ref msbit,
            } => write!(f, "bfi {}, {}, #{}, #{}", rd, rn, lsbit, msbit - lsbit + 1),

            Instruction::CMN_reg {
                rn,
                rm,
                ref shift_t,
                shift_n,
                thumb32,
            } => write!(
                f,
                "cmn{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::CMN_imm { rn, imm32 } => write!(f, "cmn.W {}, #{}", rn, imm32),
            Instruction::CBZ { rn, nonzero, imm32 } => write!(
                f,
                "cb{}z {}, #{}",
                if nonzero { "n" } else { "" },
                rn,
                imm32,
            ),
            Instruction::CLZ { rd, rm } => write!(f, "clz {},{}", rd, rm),
            Instruction::CMP_imm { rn, imm32, thumb32 } => write!(
                f,
                "cmp{} {}, #{}",
                if thumb32 { ".W" } else { "" },
                rn,
                imm32
            ),
            Instruction::CMP_reg {
                rn,
                rm,
                ref shift_t,
                shift_n,
                thumb32,
            } => write!(
                f,
                "cmp{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::CPS { im } => write!(f, "cps{}", im),
            Instruction::DMB => write!(f, "dmb"),
            Instruction::DSB => write!(f, "dsb"),
            Instruction::EOR_reg {
                rd,
                rn,
                rm,
                ref shift_t,
                shift_n,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "eor{}{} {}, {}, {}{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::ISB => write!(f, "isb"),
            Instruction::IT {
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

            Instruction::LDM {
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
            Instruction::LDR_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(
                f,
                "ldr{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                rm
            ),
            Instruction::LDR_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldr", f, rn, rt, imm32, index, add, wback, thumb32),
            Instruction::LDR_lit {
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
            Instruction::LDRB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldrb", f, rn, rt, imm32, index, add, wback, thumb32),
            Instruction::LDRB_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(
                f,
                "ldrb{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                rm
            ),
            Instruction::LDRH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldrh", f, rn, rt, imm32, index, add, wback, thumb32),
            Instruction::LDRH_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(
                f,
                "ldrh{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                rm
            ),
            Instruction::LDRSB_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                wback,
                add,
                thumb32,
            } => write!(f, "ldrsb {}, [{}, {}]", rt, rn, rm),
            Instruction::LDRSH_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(
                f,
                "ldrsh{} {}, [{}, {}]",
                if thumb32 { ".W" } else { "" },
                rt,
                rn,
                rm
            ),
            Instruction::LSL_imm {
                rd,
                rm,
                shift_n,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "lsl{}{} {}, {}, #{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rm,
                shift_n
            ),
            Instruction::LSL_reg {
                rd,
                rn,
                rm,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "lsl{}{} {}, {}, {}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::LSR_reg {
                rd,
                rn,
                rm,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "lsr{}{} {}, {}, {}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::LSR_imm {
                rd,
                rm,
                shift_n,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "lsr{} {}, {}, #{}",
                setflags_to_str(*setflags),
                rd,
                rm,
                shift_n
            ),
            Instruction::MSR_reg { spec_reg, rn } => write!(f, "msr {}, {}", spec_reg, rn),
            Instruction::MRS { rd, spec_reg } => write!(f, "mrs {}, {}", rd, spec_reg),
            Instruction::MUL {
                rd,
                rn,
                rm,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "mul{} {}, {}, {}",
                setflags_to_str(*setflags),
                rd,
                rn,
                rm
            ),
            Instruction::SMUL {
                rd,
                rn,
                rm,
                n_high,
                m_high,
            } => write!(
                f,
                "smul{}{} {}, {}, {}",
                if n_high { "T" } else { "B" },
                if m_high { "T" } else { "B" },
                rd,
                rn,
                rm
            ),
            Instruction::SMLA {
                rd,
                rn,
                rm,
                ra,
                n_high,
                m_high,
            } => write!(
                f,
                "smla{}{} {}, {}, {}, {}",
                if n_high { "T" } else { "B" },
                if m_high { "T" } else { "B" },
                rd,
                rn,
                rm,
                ra
            ),
            Instruction::MOV_reg {
                rd,
                rm,
                setflags,
                thumb32,
            } => write!(
                f,
                "mov{}{} {}, {}",
                if setflags { "s" } else { "" },
                if thumb32 { ".W" } else { "" },
                rd,
                rm
            ),
            Instruction::MOV_imm {
                rd,
                ref imm32,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "mov{}{} {}, #{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Instruction::MOVT { rd, imm16 } => write!(f, "movt {}, #{}", rd, imm16),
            Instruction::LDRSH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldrsh", f, rn, rt, imm32, index, add, wback, thumb32),

            Instruction::LDRSB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("ldrsb", f, rn, rt, imm32, index, add, wback, thumb32),

            Instruction::MVN_reg {
                rd,
                rm,
                ref setflags,
                ref shift_t,
                shift_n,
                thumb32,
            } => write!(
                f,
                "mvn{}{} {}, {}, {}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::MVN_imm {
                rd,
                ref imm32,
                setflags,
            } => write!(
                f,
                "mvn{} {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Instruction::NOP { .. } => write!(f, "nop"),
            Instruction::ORR_reg {
                rd,
                rn,
                rm,
                ref shift_t,
                shift_n,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "orr{}{} {}, {}, {}{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::ORR_imm {
                rd,
                rn,
                ref imm32,
                setflags,
            } => write!(
                f,
                "orr{} {}, {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Instruction::ORN_reg {
                rd,
                rn,
                rm,
                ref shift_t,
                shift_n,
                setflags,
            } => write!(
                f,
                "orn{}.w {}, {}, {}{}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::EOR_imm {
                rd,
                rn,
                ref imm32,
                setflags,
            } => write!(
                f,
                "eor{} {}, {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Instruction::POP { registers, thumb32 } => {
                write!(f, "pop{} {:?}", if thumb32 { ".W" } else { "" }, registers)
            }
            Instruction::PUSH { thumb32, registers } => {
                write!(f, "push{} {:?}", if thumb32 { ".W" } else { "" }, registers)
            }
            Instruction::PLD_imm { rn, imm32, add } => {
                write!(f, "pld [{}, {}{}]", rn, if add { "+" } else { "-" }, imm32)
            }
            Instruction::PLD_lit { imm32, add } => {
                write!(f, "pld [PC, {}{}]", if add { "+" } else { "-" }, imm32)
            }
            Instruction::PLD_reg {
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

            Instruction::REV { rd, rm, .. } => write!(f, "rev {}, {}", rd, rm),
            Instruction::REV16 { rd, rm, .. } => write!(f, "rev16 {}, {}", rd, rm),
            Instruction::REVSH { rd, rm, .. } => write!(f, "revsh {}, {}", rd, rm),
            Instruction::ROR_reg {
                rd,
                rn,
                rm,
                ref setflags,
                ..
            } => write!(
                f,
                "ror{} {}, {}, #{}",
                setflags_to_str(*setflags),
                rd,
                rn,
                rm
            ),
            Instruction::RSB_imm {
                rd,
                rn,
                imm32,
                ref setflags,
                thumb32,
            } => write!(
                f,
                "rsb{}{} {}, {}, #{}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                imm32
            ),
            Instruction::RRX { rd, rm, setflags } => write!(
                f,
                "mov.w{} {}, {}, rrx",
                if setflags { "s" } else { "" },
                rd,
                rm,
            ),

            Instruction::SBC_imm {
                rd,
                rn,
                imm32,
                setflags,
            } => write!(
                f,
                "sbc{}.W {}, {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                imm32
            ),
            Instruction::RSB_reg {
                rd,
                rn,
                rm,
                ref shift_t,
                shift_n,
                setflags,
                thumb32,
            } => write!(
                f,
                "rsb{}{} {}, {}, {}{}",
                if setflags { "s" } else { "" },
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),

            Instruction::SEV { .. } => write!(f, "sev"),
            Instruction::SBC_reg {
                rd,
                rn,
                rm,
                ref setflags,
                thumb32,
                ref shift_t,
                shift_n,
            } => write!(
                f,
                "sbc{}{} {}, {}, {}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::STM {
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
            Instruction::STMDB {
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
            Instruction::STR_imm {
                rn,
                rt,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("str", f, rn, rt, imm32, index, add, wback, thumb32),
            Instruction::STRD_imm {
                rn,
                rt,
                rt2,
                imm32,
                index,
                add,
                wback,
            } => format_adressing_mode2("strd", f, rn, rt, rt2, imm32, index, add, wback, true),
            Instruction::LDRD_imm {
                rn,
                rt,
                rt2,
                imm32,
                index,
                add,
                wback,
            } => format_adressing_mode2("ldrd", f, rn, rt, rt2, imm32, index, add, wback, true),
            Instruction::STR_reg {
                rn,
                rm,
                rt,
                index,
                add,
                wback,
                thumb32,
                ref shift_t,
                shift_n,
            } => write!(f, "str {}, [{}, {}]", rt, rn, rm),
            Instruction::STRB_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("strb", f, rn, rt, imm32, index, add, wback, thumb32),
            Instruction::STRB_reg {
                rt,
                rn,
                rm,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(f, "strb {}, [{}, {}]", rt, rn, rm),
            Instruction::STRH_imm {
                rt,
                rn,
                imm32,
                index,
                add,
                wback,
                thumb32,
            } => format_adressing_mode("strh", f, rn, rt, imm32, index, add, wback, thumb32),
            Instruction::STRH_reg {
                rn,
                rm,
                rt,
                ref shift_t,
                shift_n,
                index,
                add,
                wback,
                thumb32,
            } => write!(f, "strh {}, [{}, {}]", rt, rn, rm),
            Instruction::SUB_imm {
                rd,
                rn,
                imm32,
                ref setflags,
                thumb32,
            } => {
                if rd == rn {
                    write!(
                        f,
                        "sub{}{} {}, #{}",
                        setflags_to_str(*setflags),
                        if thumb32 { ".W" } else { "" },
                        rd,
                        imm32
                    )
                } else {
                    write!(
                        f,
                        "sub{}{} {}, {}, #{}",
                        setflags_to_str(*setflags),
                        if thumb32 { ".W" } else { "" },
                        rd,
                        rn,
                        imm32
                    )
                }
            }
            Instruction::SUB_reg {
                rm,
                rn,
                rd,
                ref setflags,
                ref shift_t,
                shift_n,
                thumb32,
            } => write!(
                f,
                "sub{}{} {}, {}, {}",
                setflags_to_str(*setflags),
                if thumb32 { ".W" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::TEQ_reg {
                rm,
                rn,
                ref shift_t,
                shift_n,
            } => write!(
                f,
                "teq.W {}, {}, {}",
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::SVC { imm32 } => write!(f, "svc #{}", imm32),
            Instruction::SXTH {
                rd,
                rm,
                thumb32,
                rotation,
            } => write!(
                f,
                "sxth{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                rd,
                rm,
                if rotation > 0 {
                    format!("{}", rotation)
                } else {
                    "".to_string()
                }
            ),

            Instruction::SXTB {
                rd,
                rm,
                thumb32,
                rotation,
            } => write!(
                f,
                "sxtb{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                rd,
                rm,
                if rotation > 0 {
                    format!("{}", rotation)
                } else {
                    "".to_string()
                }
            ),
            Instruction::TBB { rn, rm } => write!(f, "tbb [{}, {}]", rn, rm),
            Instruction::TBH { rn, rm } => write!(f, "tbh [{}, {}, lsl #1]", rn, rm),
            Instruction::TST_reg {
                rn,
                rm,
                ref shift_t,
                shift_n,
                thumb32,
            } => write!(
                f,
                "tst{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                rn,
                rm,
                if shift_n > 0 {
                    format!(", {:?} {}", shift_t, shift_n)
                } else {
                    "".to_string()
                }
            ),
            Instruction::TST_imm { rn, ref imm32 } => write!(
                f,
                "tst {}, #{}",
                rn,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Instruction::UDF {
                imm32, ref opcode, ..
            } => write!(f, "udf {} (opcode = {})", imm32, opcode),

            Instruction::UADD8 { rd, rn, rm } => write!(f, "uadd8 {}, {}, {}", rd, rn, rm),
            Instruction::SEL { rd, rn, rm } => write!(f, "sel {}, {}, {}", rd, rn, rm),
            // ARMv7-M
            Instruction::UDIV { rd, rn, rm } => write!(f, "udiv {}, {}, {}", rd, rn, rm),
            Instruction::SDIV { rd, rn, rm } => write!(f, "sdiv {}, {}, {}", rd, rn, rm),
            // ARMv7-M
            Instruction::UMLAL { rdlo, rdhi, rn, rm } => {
                write!(f, "umlal {}, {}, {}, {}", rdlo, rdhi, rn, rm)
            }
            // ARMv7-M
            Instruction::UMULL { rdlo, rdhi, rn, rm } => {
                write!(f, "umull {}, {}, {}, {}", rdlo, rdhi, rn, rm)
            }
            Instruction::SMULL { rdlo, rdhi, rn, rm } => {
                write!(f, "smull {}, {}, {}, {}", rdlo, rdhi, rn, rm)
            }
            // ARMv7-M
            Instruction::MLA { rd, rn, rm, ra } => write!(f, "mla {}, {}, {}, {}", rd, rn, rm, ra),
            // ARMv7-M
            Instruction::MLS { rd, rn, rm, ra } => write!(f, "mls {}, {}, {}, {}", rd, rn, rm, ra),
            // ARMv7-M
            Instruction::SMLAL { rdlo, rdhi, rn, rm } => {
                write!(f, "smlal {}, {}, {}, {}", rdlo, rdhi, rn, rm)
            }
            Instruction::UXTB {
                rd,
                rm,
                thumb32,
                rotation,
            } => write!(
                f,
                "uxtb{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                rd,
                rm,
                if rotation > 0 {
                    format!("{}", rotation)
                } else {
                    "".to_string()
                }
            ),
            Instruction::UXTAB {
                rd,
                rn,
                rm,
                rotation,
            } => write!(
                f,
                "uxtb.w {},{},{} {}",
                rd,
                rn,
                rm,
                if rotation > 0 {
                    format!("{}", rotation)
                } else {
                    "".to_string()
                }
            ),
            Instruction::UXTH {
                rd,
                rm,
                rotation,
                thumb32,
            } => write!(
                f,
                "uxth{} {}, {}{}",
                if thumb32 { ".W" } else { "" },
                rd,
                rm,
                if rotation > 0 {
                    format!("{}", rotation)
                } else {
                    "".to_string()
                }
            ),
            Instruction::UBFX {
                rd,
                rn,
                lsb,
                widthminus1,
            } => write!(f, "ubfx {}, {}, #{}, #{}", rd, rn, lsb, widthminus1 + 1),

            Instruction::WFE { .. } => write!(f, "wfe"),
            Instruction::WFI { .. } => write!(f, "wfi"),
            Instruction::YIELD { .. } => write!(f, "yield"),
            // ARMv7-M
            Instruction::MCR {
                ref rt,
                ref coproc,
                ref opc1,
                ref opc2,
                ref crn,
                ref crm,
            } => write!(f, "mcr"),

            // ARMv7-M
            Instruction::MCR2 {
                ref rt,
                ref coproc,
                ref opc1,
                ref opc2,
                ref crn,
                ref crm,
            } => write!(f, "mcr2"),

            // ARMv7-M
            Instruction::LDC_imm {
                ref coproc,
                ref imm32,
                ref crd,
                ref rn,
            } => write!(f, "ldc"),

            // ARMv7-M
            Instruction::LDC2_imm {
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
            ITCondition::Then => write!(f, "t"),
            ITCondition::Else => write!(f, "e"),
        }
    }
}

#[allow(clippy::cyclomatic_complexity)]
#[allow(unused_variables)]
/// Get the size of an instruction in bytes
pub fn instruction_size(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::ADC_imm { .. } => 4,
        Instruction::ADC_reg { thumb32, .. } => isize_t(*thumb32),
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
        //BFC
        Instruction::BIC_imm { .. } => 4,
        Instruction::BIC_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::BKPT { .. } => 2,
        Instruction::BL { .. } => 4,
        Instruction::BLX { .. } => 2,
        Instruction::BX { .. } => 2,

        Instruction::CBZ { .. } => 2,
        //CDP
        //CLREX
        Instruction::CLZ { .. } => 4,
        Instruction::CMN_imm { rn, imm32 } => 4,
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
        //LDREX
        //LDREXB
        //LDREXH
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

        //QADD
        //QADD16
        //QADD8
        //QASX
        //QDADD
        //QDSUB
        //QSAX
        //QSUB
        //QSUB16
        //QSUB8

        //RBIT
        Instruction::REV { thumb32, .. } => isize_t(*thumb32),
        Instruction::REV16 { thumb32, .. } => isize_t(*thumb32),
        Instruction::REVSH { thumb32, .. } => isize_t(*thumb32),
        Instruction::ROR_imm { .. } => 4,
        Instruction::ROR_reg { thumb32, .. } => isize_t(*thumb32),
        Instruction::RRX { rd, rm, setflags } => 4,
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
        //STREX
        //STREXB
        //STREXH
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
        Instruction::TEQ_imm { rn, .. } => 4,
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
        //VLDR
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
