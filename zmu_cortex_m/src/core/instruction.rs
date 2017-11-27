use core::register::Reg;
use core::register::SpecialReg;
use core::condition::Condition;
use core::ThumbCode;
use enum_set::EnumSet;


#[allow(non_camel_case_types)]
pub enum Instruction {
    ADC_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    ADD_imm {
        rn: Reg,
        rd: Reg,
        imm32: u32,
        setflags: bool,
    },
    ADD_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
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
    CMN_reg { rn: Reg, rm: Reg },
    CMP_imm { rn: Reg, imm32: u32 },
    CMP_reg { rm: Reg, rn: Reg },
    CPS,
    DMB,
    DSB,
    EOR_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    ISB,
    LDM { rn: Reg, registers: EnumSet<Reg> },
    LDR_imm { rt: Reg, rn: Reg, imm32: u32 },
    LDR_lit { rt: Reg, imm32: u32 },
    LDR_reg { rt: Reg, rn: Reg, rm: Reg },
    LDRB_imm { rt: Reg, rn: Reg, imm32: u32 },
    LDRB_reg { rt: Reg, rn: Reg, rm: Reg },
    LDRH_imm { rt: Reg, rn: Reg, imm32: u32 },
    LDRH_reg { rt: Reg, rn: Reg, rm: Reg },
    LDRSB_reg { rt: Reg, rn: Reg, rm: Reg },
    LDRSH_reg { rt: Reg, rn: Reg, rm: Reg },
    LSL_imm {
        rd: Reg,
        rm: Reg,
        imm5: u8,
        setflags: bool,
    },
    LSL_reg {
        rd: Reg,
        rm: Reg,
        rn: Reg,
        setflags: bool,
    },
    LSR_imm {
        rd: Reg,
        rm: Reg,
        imm5: u8,
        setflags: bool,
    },
    LSR_reg {
        rd: Reg,
        rm: Reg,
        rn: Reg,
        setflags: bool,
    },
    MOV_imm { rd: Reg, imm32: u32, setflags: bool },
    MOV_reg { rd: Reg, rm: Reg, setflags: bool },
    MRS { rd: Reg, spec_reg: SpecialReg },
    MSR_reg { rn: Reg, spec_reg: SpecialReg },
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
    REV { rd: Reg, rm: Reg },
    REV16 { rd: Reg, rm: Reg },
    REVSH { rd: Reg, rm: Reg },
    ROR_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    RSB_imm {
        rd: Reg,
        rn: Reg,
        imm32: u32,
        setflags: bool,
    },
    SBC_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    SEV,
    STM { rn: Reg, registers: EnumSet<Reg> },
    STR_imm { rn: Reg, rt: Reg, imm32: u32 },
    STR_reg { rm: Reg, rn: Reg, rt: Reg },
    STRB_imm { rn: Reg, rt: Reg, imm32: u32 },
    STRB_reg { rm: Reg, rn: Reg, rt: Reg },
    STRH_imm { rt: Reg, rn: Reg, imm32: u32 },
    STRH_reg { rm: Reg, rn: Reg, rt: Reg },
    SUB_imm {
        rd: Reg,
        rn: Reg,
        imm32: u32,
        setflags: bool,
    },
    SUB_reg {
        rm: Reg,
        rn: Reg,
        rd: Reg,
        setflags: bool,
    },
    SVC { imm32: u32 },
    SXTB { rd: Reg, rm: Reg },
    SXTH { rd: Reg, rm: Reg },
    TST_reg { rn: Reg, rm: Reg },
    UDF { imm32: u32, opcode: ThumbCode },
    UXTB { rd: Reg, rm: Reg },
    UXTH { rd: Reg, rm: Reg },
    WFE,
    WFI,
    YIELD,
}

use std::fmt;


impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::ADD_imm {
                rn,
                rd,
                imm32,
                setflags,
            } => write!(
                f,
                "ADD{} {},{},#0x{:x}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                imm32
            ),
            Instruction::ADD_reg {
                rm,
                rn,
                rd,
                setflags,
            } => write!(
                f,
                "ADD{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
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
            Instruction::ADR { rd, imm32 } => write!(f, "ADR {}, PC, 0x#{:x}", rd, imm32),
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
                "ASR{} {}, {}, 0x#{:x}",
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
            Instruction::B { ref cond, imm32 } => write!(f, "B{}.N {}", cond, imm32),
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
            Instruction::BL { imm32 } => write!(f, "BL 0x#{:x}", imm32),
            Instruction::BX { rm } => write!(f, "BX {}", rm),
            Instruction::BLX { rm } => write!(f, "BLX {}", rm),
            Instruction::BKPT { imm32 } => write!(f, "BKPT 0x#{:x}", imm32),
            Instruction::CMN_reg { rn, rm } => write!(f, "CMN {}, {}", rn, rm),
            Instruction::CMP_imm { rn, imm32 } => write!(f, "CMP {}, #{}", rn, imm32),
            Instruction::CMP_reg { rn, rm } => write!(f, "CMP {}, {}", rn, rm),
            Instruction::CPS => write!(f, "CPS"),
            Instruction::DMB => write!(f, "DMB"),
            Instruction::DSB => write!(f, "DSB"),
            Instruction::EOR_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "EOR{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::ISB => write!(f, "ISB"),
            Instruction::LDM { rn, registers } => write!(f, "LDM {}, {{{:?}}}", rn, registers),
            Instruction::LDR_reg { rt, rn, rm } => write!(f, "LDR {}, [{}, {}]", rt, rn, rm),
            Instruction::LDR_imm { rt, rn, imm32 } => write!(f, "LDR {}, [{},#0x{:x}]", rt, rn, imm32),
            Instruction::LDR_lit { rt, imm32 } => write!(f, "LDR {}, [PC, #0x{:x}]", rt, imm32),
            Instruction::LDRB_imm { rt, rn, imm32 } => {
                write!(f, "LDRB {}, [{},#0x{:x}]", rt, rn, imm32)
            }
            Instruction::LDRB_reg { rt, rn, rm } => write!(f, "LDRB {}, [{}, {}]", rt, rn, rm),
            Instruction::LDRH_imm { rt, rn, imm32 } => {
                write!(f, "LDRH {}, [{},#0x{:x}]", rt, rn, imm32)
            }
            Instruction::LDRH_reg { rt, rn, rm } => write!(f, "LDRH {}, [{}, {}]", rt, rn, rm),
            Instruction::LDRSB_reg { rt, rn, rm } => write!(f, "LDRSB {}, [{},{}]", rt, rn, rm),
            Instruction::LDRSH_reg { rt, rn, rm } => write!(f, "LDRSH {}, [{},{}]", rt, rn, rm),
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
            Instruction::LSL_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "LSL{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::LSR_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "LSR{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::LSR_imm {
                rd,
                rm,
                imm5,
                setflags,
            } => write!(
                f,
                "LSR{} {}, {}, #{}",
                if setflags { "S" } else { "" },
                rd,
                rm,
                imm5
            ),
            Instruction::MSR_reg { spec_reg, rn } => write!(f, "MSR {},{}", spec_reg, rn),
            Instruction::MRS { rd, spec_reg } => write!(f, "MRS {},{}", rd, spec_reg),
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
                "MOV{} {},#0x{:x}",
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
            Instruction::REV { rd, rm } => write!(f, "REV {},{}", rd, rm),
            Instruction::REV16 { rd, rm } => write!(f, "REV16 {},{}", rd, rm),
            Instruction::REVSH { rd, rm } => write!(f, "REVSH {},{}", rd, rm),
            Instruction::ROR_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "ROR{} {},{},#{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::RSB_imm {
                rd,
                rn,
                imm32,
                setflags,
            } => write!(
                f,
                "RSB{} {},{},#{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                imm32
            ),
            Instruction::SEV => write!(f, "SEV"),
            Instruction::SBC_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "SBC{} {},{},{}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::STM { rn, registers } => write!(f, "STM {}, {{{:?}}}", rn, registers),
            Instruction::STR_imm { rn, rt, imm32 } => {
                write!(f, "STR {}, [{},#0x{:x}]", rt, rn, imm32)
            }
            Instruction::STR_reg { rn, rm, rt } => write!(f, "STR {}, [{}, {}]", rt, rn, rm),
            Instruction::STRB_imm { rn, rt, imm32 } => {
                write!(f, "STRB {}, [{},#0x{:x}]", rt, rn, imm32)
            }
            Instruction::STRB_reg { rn, rm, rt } => write!(f, "STRB {}, [{}, {}]", rt, rn, rm),
            Instruction::STRH_imm { rt, rn, imm32 } => {
                write!(f, "STRH {}, [{},#0x{:x}]", rt, rn, imm32)
            }
            Instruction::STRH_reg { rn, rm, rt } => write!(f, "STRH {}, [{}, {}]", rt, rn, rm),
            Instruction::SUB_imm {
                rd,
                rn,
                imm32,
                setflags,
            } => write!(
                f,
                "SUB{} {},{},#0x{:x}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                imm32
            ),
            Instruction::SUB_reg {
                rm,
                rn,
                rd,
                setflags,
            } => write!(
                f,
                "SUB{} {}, {}, {}",
                if setflags { "S" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::SVC { imm32 } => write!(f, "SVC #0x{:x}", imm32),
            Instruction::SXTB { rd, rm } => write!(f, "SXTB {},{}", rd, rm),
            Instruction::SXTH { rd, rm } => write!(f, "SXTH {},{}", rd, rm),
            Instruction::TST_reg { rn, rm } => write!(f, "TST {},{}", rn, rm),
            Instruction::UDF { imm32, ref opcode } => {
                write!(f, "UDF {} (opcode = {})", imm32, opcode)
            }
            Instruction::UXTB { rd, rm } => write!(f, "UXTB {}, {}", rd, rm),
            Instruction::UXTH { rd, rm } => write!(f, "UXTH {},{}", rd, rm),
            Instruction::WFE => write!(f, "WFE"),
            Instruction::WFI => write!(f, "WFI"),
            Instruction::YIELD => write!(f, "YIELD"),
        }
    }
}
