use core::register::Reg;
use core::condition::Condition;
use enum_set::EnumSet;
use std::fmt;


#[allow(non_camel_case_types)]
pub enum Instruction {
    ADC_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    ADDS { rm: Reg, rn: Reg, rd: Reg },
    ADD { rm: Reg, rdn: Reg },
    ADD_imm {
        rn: Reg,
        rd: Reg,
        imm32: u32,
        setflags: bool,
    },
    ADR { rd: Reg, imm32: u32 },
    AND_reg {
        rd: Reg,
        rm: Reg,
        rn: Reg,
        setflags: bool,
    },
    ASR_imm {
        rd: Reg,
        rm: Reg,
        imm5: u8,
        setflags: bool,
    },
    ASR_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },

    B { cond: Condition, imm32: i32 },
    BIC_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    BKPT { imm32: u32 },
    BL { imm32: i32 },
    BLX { rm: Reg },
    BX { rm: Reg },
    CMN_reg {rn: Reg, rm: Reg},
    CMP_imm { rn: Reg, imm32: u32 },
    CMP { rm: Reg, rn: Reg },
    CPS,
    CPY,
    DMB,
    DSB,
    EOR,
    ISB,
    LDM { rn: Reg, registers: EnumSet<Reg> },
    LDR_imm { rt: Reg, rn: Reg, imm32: u32 },
    LDR_lit { rt: Reg, imm32: u32 },
    LDR_reg { rt: Reg, rn: Reg, rm: Reg },
    LDRB_imm { rt: Reg, rn: Reg, imm32: u32 },
    LDRB_reg { rt: Reg, rn: Reg, rm: Reg },
    LDRH_imm { rt: Reg, rn: Reg, imm32: u32 },
    LDRH_reg { rt: Reg, rn: Reg, rm: Reg },
    LDRSB_reg,
    LDRSH_reg,
    LSL_imm {
        rd: Reg,
        rm: Reg,
        imm5: u8,
        setflags: bool,
    },
    LSL_reg,
    LSR_imm {
        rd: Reg,
        rm: Reg,
        imm5: u8,
        setflags: bool,
    },
    LSR_reg,
    MOV_reg { rd: Reg, rm: Reg, setflags: bool },
    MOV_imm { rd: Reg, imm32: u32, setflags: bool },
    MRS,
    MRS_reg,
    MUL {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    MVN_reg { rd: Reg, rm: Reg, setflags: bool },
    NOP,
    ORR {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    POP { registers: EnumSet<Reg> },
    PUSH { registers: EnumSet<Reg> },
    REV,
    REV16,
    REVSH,
    ROR,
    RSB,
    SBC,
    SEV,
    STM { rn: Reg, registers: EnumSet<Reg> },
    STR_imm { rn: Reg, rt: Reg, imm32: u32 },
    STR_reg { rm: Reg, rn: Reg, rt: Reg },
    STRB_imm { rn: Reg, rt: Reg, imm32: u32 },
    STRB_reg,
    STRH_imm { rt: Reg, rn: Reg, imm32: u32 },
    STRH_reg,
    SUB_imm {
        rd: Reg,
        rn: Reg,
        imm32: u32,
        setflags: bool,
    },
    SUBS_reg { rm: Reg, rn: Reg, rd: Reg },
    SVC,
    SXTB,
    SXTH,
    TST_reg { rn: Reg, rm: Reg },
    UDF,
    UXTB { rd: Reg, rm: Reg },
    UXTH { rd: Reg, rm: Reg },
    WFE,
    WFI,
    YIELD,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::ADD { rdn, rm } => write!(f, "ADD {},{},{}", rdn, rdn, rm),
            Instruction::ADD_imm {
                rn,
                rd,
                imm32,
                setflags,
            } => write!(
                f,
                "ADD{} {},{},#{:x}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                imm32
            ),
            Instruction::ADDS { rm, rn, rd } => write!(f, "ADDS {},{},{}", rd, rn, rm),
            Instruction::ADC_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "ADC{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::ADR { rd, imm32 } => write!(f, "ADR {}, PC, #{:x}", rd, imm32),
            Instruction::AND_reg {
                rn,
                rd,
                rm,
                setflags,
            } => write!(
                f,
                "AND{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::ASR_imm {
                rd,
                rm,
                imm5,
                setflags,
            } => write!(
                f,
                "ASR{} {}, {}, #{:x}",
                if setflags { "S" } else { "" },
                rd,
                rm,
                imm5
            ),
            Instruction::ASR_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "ASR{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::B { ref cond, imm32 } => write!(f, "B{} {}", cond, imm32),
            Instruction::BIC_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "BIC{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::BL { imm32 } => write!(f, "BL #{:x}", imm32),
            Instruction::BX { rm } => write!(f, "BX {}", rm),
            Instruction::BLX { rm } => write!(f, "BLX {}", rm),
            Instruction::BKPT { imm32 } => write!(f, "BKPT #{:x}", imm32),
            Instruction::CMN_reg { rn, rm } => write!(f, "CMN {}, {}", rn, rm),
            Instruction::CMP_imm { rn, imm32 } => write!(f, "CMP {}, #{}", rn, imm32),
            Instruction::CMP { rn, rm } => write!(f, "CMP {}, {}", rn, rm),
            Instruction::CPS => write!(f, "CPS"),
            Instruction::CPY => write!(f, "CPY"),
            Instruction::DMB => write!(f, "DMB"),
            Instruction::DSB => write!(f, "DSB"),
            Instruction::EOR => write!(f, "EOR"),
            Instruction::ISB => write!(f, "ISB"),
            Instruction::LDM { rn, registers } => write!(f, "LDM {}, {{{:?}}}", rn, registers),
            Instruction::LDR_reg { rt, rn, rm } => write!(f, "LDR {}, [{}, {}]", rt, rn, rm),
            Instruction::LDR_imm { rt, rn, imm32 } => write!(f, "LDR {},[{},#{}]", rt, rn, imm32),
            Instruction::LDR_lit { rt, imm32 } => write!(f, "LDR {},[PC, #{:x}]", rt, imm32),
            Instruction::LDRB_imm { rt, rn, imm32 } => {
                write!(f, "LDRB {},[{},#{:x}]", rt, rn, imm32)
            }
            Instruction::LDRB_reg { rt, rn, rm } => write!(f, "LDRB {}, [{}, {}]", rt, rn, rm),
            Instruction::LDRH_imm { rt, rn, imm32 } => {
                write!(f, "LDRH {},[{},#{:x}]", rt, rn, imm32)
            }
            Instruction::LDRH_reg { rt, rn, rm } => write!(f, "LDRH {}, [{}, {}]", rt, rn, rm),
            Instruction::LDRSB_reg => write!(f, "LDRSB reg"),
            Instruction::LDRSH_reg => write!(f, "LDRSH reg"),
            Instruction::LSL_imm {
                rd,
                rm,
                imm5,
                setflags,
            } => write!(
                f,
                "LSL{} {}, {}, #{}",
                if setflags { "S" } else { "" },
                rd,
                rm,
                imm5
            ),
            Instruction::LSL_reg => write!(f, "LSL reg"),
            Instruction::LSR_imm {
                rd,
                rm,
                imm5,
                setflags,
            } => write!(
                f,
                "LSR{} {}, {}, #{:x}",
                if setflags { "S" } else { "" },
                rd,
                rm,
                imm5
            ),
            Instruction::LSR_reg => write!(f, "LSR reg"),
            Instruction::MRS_reg => write!(f, "MSR reg"),
            Instruction::MRS => write!(f, "MSR"),
            Instruction::MUL {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "MUL{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::MOV_reg { rd, rm, setflags } => {
                write!(f, "MOV{} {},{}", if setflags { "S" } else { "" }, rd, rm)
            }
            Instruction::MOV_imm {
                rd,
                imm32,
                setflags,
            } => write!(
                f,
                "MOV{} {},#{:x}",
                if setflags { "S" } else { "" },
                rd,
                imm32
            ),
            Instruction::MVN_reg { rd, rm, setflags } => {
                write!(f, "MVN{} {},{}", if setflags { "S" } else { "" }, rd, rm)
            }
            Instruction::NOP => write!(f, "NOP"),
            Instruction::ORR {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "ORR{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::POP { registers } => write!(f, "POP {:?}", registers),
            Instruction::PUSH { registers } => write!(f, "PUSH {:?}", registers),
            Instruction::REV => write!(f, "REV"),
            Instruction::REV16 => write!(f, "REV16"),
            Instruction::REVSH => write!(f, "REVSH"),
            Instruction::ROR => write!(f, "ROR"),
            Instruction::RSB => write!(f, "RSB"),
            Instruction::SBC => write!(f, "SBC"),
            Instruction::SEV => write!(f, "SEV"),
            Instruction::STM { rn, registers } => write!(f, "STM {}, {{{:?}}}", rn, registers),
            Instruction::STR_imm { rn, rt, imm32 } => {
                write!(f, "STR {}, [{},#{:x}]", rt, rn, imm32)
            }
            Instruction::STR_reg { rn, rm, rt } => write!(f, "STR {}, [{}, {}]", rt, rn, rm),
            Instruction::STRB_imm { rn, rt, imm32 } => {
                write!(f, "STRB {}, [{},#{:x}]", rt, rn, imm32)
            }
            Instruction::STRB_reg => write!(f, "STRB_reg"),
            Instruction::STRH_imm { rt, rn, imm32 } => {
                write!(f, "STRH {}, [{},#{:x}]", rt, rn, imm32)
            }
            Instruction::STRH_reg => write!(f, "STRH_reg"),
            Instruction::SUB_imm {
                rd,
                rn,
                imm32,
                setflags,
            } => write!(
                f,
                "SUB{} {},{},#{:x}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                imm32
            ),
            Instruction::SUBS_reg { rm, rn, rd } => write!(f, "SUBS {}, {}, {}", rd, rn, rm),
            Instruction::SVC => write!(f, "SVC"),
            Instruction::SXTB => write!(f, "SXTB"),
            Instruction::SXTH => write!(f, "SXTH"),
            Instruction::TST_reg { rn, rm } => write!(f, "TST {},{}", rn, rm),
            Instruction::UDF => write!(f, "UDF"),
            Instruction::UXTB { rd, rm } => write!(f, "UXTB {}, {}", rd, rm),
            Instruction::UXTH { rd, rm } => write!(f, "UXTH {},{}", rd, rm),
            Instruction::WFE => write!(f, "WFE"),
            Instruction::WFI => write!(f, "WFI"),
            Instruction::YIELD => write!(f, "YIELD"),
        }
    }
}
