use crate::core::condition::Condition;
use crate::core::register::Reg;
use crate::core::register::SpecialReg;
use crate::core::ThumbCode;
use enum_set::EnumSet;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SRType {
    LSL,
    LSR,
    ASR,
    RRX,
    ROR,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ITCondition {
    Then,
    Else,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Imm32Carry {
    /// precalculated value carry value was not relevant
    /// for the decoding
    NoCarry { imm32: u32 },
    /// precalculated values for cases ASPR.C == 0 and ASPR.C ==1
    /// if carry was relevant for the decoding
    /// tuple of (immediate value, carry)
    Carry {
        imm32_c0: (u32, bool),
        imm32_c1: (u32, bool),
    },
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SetFlags {
    True,
    False,
    NotInITBlock,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Copy, Clone)]
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
    },
    MVN_imm {
        rd: Reg,
        imm32: Imm32Carry,
        setflags: bool,
    },
    NOP,
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
    POP {
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    PUSH {
        registers: EnumSet<Reg>,
        thumb32: bool,
    },
    REV {
        rd: Reg,
        rm: Reg,
    },
    REV16 {
        rd: Reg,
        rm: Reg,
    },
    REVSH {
        rd: Reg,
        rm: Reg,
    },
    ROR_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: SetFlags,
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
    SEV,
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
    TBB {
        rn: Reg,
        rm: Reg,
    },
    UDF {
        imm32: u32,
        opcode: ThumbCode,
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
    WFE,
    WFI,
    YIELD,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum CpsEffect {
    IE,
    // Interrupt enable
    ID, // Interrupt disable
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
            } => write!(
                f,
                "asr{} {}, {}, {}",
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
            } => write!(
                f,
                "lsl{} {}, {}, {}",
                setflags_to_str(*setflags),
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
            } => write!(f, "mvn{} {}, {}", setflags_to_str(*setflags), rd, rm),
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
            Instruction::NOP => write!(f, "nop"),
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
            Instruction::REV { rd, rm } => write!(f, "rev {}, {}", rd, rm),
            Instruction::REV16 { rd, rm } => write!(f, "rev16 {}, {}", rd, rm),
            Instruction::REVSH { rd, rm } => write!(f, "revsh {}, {}", rd, rm),
            Instruction::ROR_reg {
                rd,
                rn,
                rm,
                ref setflags,
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

            Instruction::SEV => write!(f, "sev"),
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
            Instruction::TST_reg { rn, rm } => write!(f, "tst {}, {}", rn, rm),
            Instruction::TST_imm { rn, ref imm32 } => write!(
                f,
                "tst {}, #{}",
                rn,
                match *imm32 {
                    Imm32Carry::NoCarry { imm32 } => imm32,
                    Imm32Carry::Carry { imm32_c0, imm32_c1 } => imm32_c0.0,
                }
            ),
            Instruction::UDF { imm32, ref opcode } => {
                write!(f, "udf {} (opcode = {})", imm32, opcode)
            }

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

            Instruction::WFE => write!(f, "wfe"),
            Instruction::WFI => write!(f, "wfi"),
            Instruction::YIELD => write!(f, "yield"),
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
pub fn instruction_size(instruction: &Instruction) -> usize {
    match instruction {
        Instruction::ADD_reg {
            rm,
            rn,
            rd,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::ADD_sp_reg {
            rm,
            rd,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }

        Instruction::ADC_reg {
            rm,
            rn,
            rd,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::ADD_imm {
            rn,
            rd,
            imm32,
            setflags,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDR_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDR_lit {
            rt,
            imm32,
            thumb32,
            add,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::PUSH { thumb32, registers } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::CMP_imm { rn, imm32, thumb32 } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::STR_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::STRD_imm {
            rt,
            rt2,
            rn,
            imm32,
            index,
            add,
            wback,
        } => 4,
        Instruction::LDRD_imm {
            rt,
            rt2,
            rn,
            imm32,
            index,
            add,
            wback,
        } => 4,
        Instruction::LDRSH_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDRH_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDRB_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDRSB_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDR_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::STRB_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::STRH_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::STR_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDRSH_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDRSB_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::MOV_imm {
            rd,
            imm32,
            setflags,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::SUB_imm {
            rd,
            rn,
            imm32,
            setflags,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::SUB_reg {
            rm,
            rn,
            rd,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::STRH_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::UXTH {
            rd,
            rm,
            rotation,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::UXTB {
            rd,
            rm,
            rotation,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::UXTAB {
            rd,
            rn,
            rm,
            rotation,
        } => 4,
        Instruction::SXTH {
            rd,
            rm,
            rotation,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::SXTB {
            rd,
            rm,
            rotation,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
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
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::STRB_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDRH_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::MSR_reg { rn, spec_reg } => 4,
        Instruction::MRS { rd, spec_reg } => 4,
        Instruction::MLA { rd, rn, rm, ra } => 4,
        Instruction::MLS { rd, rn, rm, ra } => 4,
        Instruction::BL { imm32 } => 4,
        Instruction::B_t13 {
            cond,
            imm32,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::B_t24 { imm32, thumb32 } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::BFI {
            rn,
            rd,
            lsbit,
            msbit,
        } => 4,

        Instruction::TBB { rn, rm } => 4,
        Instruction::UDIV { rd, rn, rm } => 4,
        Instruction::SDIV { rd, rn, rm } => 4,
        Instruction::AND_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => 4,
        Instruction::TST_imm { rn, imm32 } => 4,
        Instruction::UBFX {
            rd,
            rn,
            lsb,
            widthminus1,
        } => 4,
        Instruction::EOR_reg {
            rd,
            rn,
            rm,
            setflags,
            thumb32,
            shift_t,
            shift_n,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::ORR_reg {
            rd,
            rn,
            rm,
            setflags,
            thumb32,
            shift_t,
            shift_n,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::BIC_reg {
            rd,
            rn,
            rm,
            setflags,
            thumb32,
            shift_t,
            shift_n,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::AND_reg {
            rd,
            rn,
            rm,
            setflags,
            thumb32,
            shift_t,
            shift_n,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LSL_imm {
            rd,
            rm,
            shift_n,
            thumb32,
            setflags,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LSR_imm {
            rd,
            rm,
            shift_n,
            thumb32,
            setflags,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LSR_reg {
            rd,
            rm,
            rn,
            thumb32,
            setflags,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::ASR_imm {
            rd,
            rm,
            shift_n,
            thumb32,
            setflags,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::ROR_imm {
            rd,
            rm,
            shift_n,
            setflags,
        } => 4,
        Instruction::MUL {
            rd,
            rm,
            rn,
            thumb32,
            setflags,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::MOV_reg {
            rd,
            rm,
            setflags,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::CMN_reg {
            rn,
            rm,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::CMP_reg {
            rn,
            rm,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::TEQ_reg {
            rn,
            rm,
            shift_t,
            shift_n,
        } => 4,

        Instruction::CMN_imm { rn, imm32 } => 4,
        Instruction::MVN_imm {
            rd,
            imm32,
            setflags,
        } => 4,
        Instruction::ORR_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => 4,
        Instruction::EOR_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => 4,
        Instruction::ADR { rd, imm32, thumb32 } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::UMLAL { rm, rdlo, rdhi, rn } => 4,
        Instruction::UMULL { rm, rdlo, rdhi, rn } => 4,
        Instruction::RSB_imm {
            rd,
            rn,
            imm32,
            setflags,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::SBC_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => 4,
        Instruction::ADC_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => 4,
        Instruction::SBC_reg {
            rm,
            rn,
            rd,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::STMDB {
            rn,
            wback,
            registers,
        } => 4,
        Instruction::STM {
            rn,
            wback,
            registers,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::LDM {
            rn,
            registers,
            thumb32,
        } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }
        Instruction::BIC_imm {
            rd,
            rn,
            ref imm32,
            setflags,
        } => 4,

        Instruction::RSB_reg {
            rd,
            rn,
            rm,
            ref shift_t,
            shift_n,
            setflags,
            thumb32,
        } => 4,
        Instruction::CLZ { rd, rm } => 4,

        Instruction::SMUL {
            rd,
            rn,
            rm,
            m_high,
            n_high,
        } => 4,
        Instruction::SMLA {
            rd,
            rn,
            rm,
            ra,
            m_high,
            n_high,
        } => 4,
        Instruction::POP { registers, thumb32 } => {
            if *thumb32 {
                4
            } else {
                2
            }
        }

        _ => 2,
    }
}
