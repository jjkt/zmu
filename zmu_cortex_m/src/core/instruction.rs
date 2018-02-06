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
    ADR {
        rd: Reg,
        imm32: u32,
    },
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

    B {
        cond: Condition,
        imm32: i32,
    },
    BIC_reg {
        rd: Reg,
        rn: Reg,
        rm: Reg,
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
    CMN_reg {
        rn: Reg,
        rm: Reg,
    },
    CMP_imm {
        rn: Reg,
        imm32: u32,
    },
    CMP_reg {
        rm: Reg,
        rn: Reg,
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
        setflags: bool,
    },
    ISB,
    LDM {
        rn: Reg,
        registers: EnumSet<Reg>,
    },
    LDR_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
    },
    LDR_lit {
        rt: Reg,
        imm32: u32,
    },
    LDR_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
    },
    LDRB_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
    },
    LDRB_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
    },
    LDRH_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
    },
    LDRH_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
    },
    LDRSB_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
    },
    LDRSH_reg {
        rt: Reg,
        rn: Reg,
        rm: Reg,
    },
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
    MOV_imm {
        rd: Reg,
        imm32: u32,
        setflags: bool,
    },
    MOV_reg {
        rd: Reg,
        rm: Reg,
        setflags: bool,
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
        setflags: bool,
    },
    MVN_reg {
        rd: Reg,
        rm: Reg,
        setflags: bool,
    },
    NOP,
    ORR {
        rd: Reg,
        rn: Reg,
        rm: Reg,
        setflags: bool,
    },
    POP {
        registers: EnumSet<Reg>,
    },
    PUSH {
        registers: EnumSet<Reg>,
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
    STM {
        rn: Reg,
        registers: EnumSet<Reg>,
    },
    STR_imm {
        rn: Reg,
        rt: Reg,
        imm32: u32,
    },
    STR_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
    },
    STRB_imm {
        rn: Reg,
        rt: Reg,
        imm32: u32,
    },
    STRB_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
    },
    STRH_imm {
        rt: Reg,
        rn: Reg,
        imm32: u32,
    },
    STRH_reg {
        rm: Reg,
        rn: Reg,
        rt: Reg,
    },
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
    SVC {
        imm32: u32,
    },
    SXTB {
        rd: Reg,
        rm: Reg,
    },
    SXTH {
        rd: Reg,
        rm: Reg,
    },
    TST_reg {
        rn: Reg,
        rm: Reg,
    },
    UDF {
        imm32: u32,
        opcode: ThumbCode,
    },
    UXTB {
        rd: Reg,
        rm: Reg,
    },
    UXTH {
        rd: Reg,
        rm: Reg,
    },
    WFE,
    WFI,
    YIELD,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum CpsEffect {
    IE, // Interrupt enable
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

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::ADD_imm {
                rn,
                rd,
                imm32,
                setflags,
            } => {
                if rn == rd {
                    write!(
                        f,
                        "add{} {}, #{}",
                        if setflags { "s" } else { "" },
                        rd,
                        imm32
                    )
                } else {
                    write!(
                        f,
                        "add{} {}, {}, #{}",
                        if setflags { "s" } else { "" },
                        rd,
                        rn,
                        imm32
                    )
                }
            }
            Instruction::ADD_reg {
                rm,
                rn,
                rd,
                setflags,
            } => write!(
                f,
                "add{} {}, {}, {}",
                if setflags { "s" } else { "" },
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
                "adc{} {}, {}, {}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::ADR { rd, imm32 } => write!(f, "adr {}, pc, 0x#{:x}", rd, imm32),
            Instruction::AND_reg {
                rn,
                rd,
                rm,
                setflags,
            } => write!(
                f,
                "and{} {}, {}, {}",
                if setflags { "s" } else { "" },
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
                "asr{} {}, {}, #{}",
                if setflags { "s" } else { "" },
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
                "asr{} {}, {}, {}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::BIC_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "bic{} {}, {}, {}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::B { ref cond, imm32 } => write!(f, "b{} {}", cond, imm32),
            Instruction::BL { imm32 } => write!(f, "bl 0x#{:x}", imm32),
            Instruction::BX { rm } => write!(f, "bx {}", rm),
            Instruction::BLX { rm } => write!(f, "blx {}", rm),
            Instruction::BKPT { imm32 } => write!(f, "bkpt #{}", imm32),
            Instruction::CMN_reg { rn, rm } => write!(f, "cmn {}, {}", rn, rm),
            Instruction::CMP_imm { rn, imm32 } => write!(f, "cmp {}, #{}", rn, imm32),
            Instruction::CMP_reg { rn, rm } => write!(f, "cmp {}, {}", rn, rm),
            Instruction::CPS { im } => write!(f, "cps{}", im),
            Instruction::DMB => write!(f, "dmb"),
            Instruction::DSB => write!(f, "dsb"),
            Instruction::EOR_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "eor{} {}, {}, {}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::ISB => write!(f, "isb"),
            Instruction::LDM { rn, registers } => write!(f, "ldm {}, {{{:?}}}", rn, registers),
            Instruction::LDR_reg { rt, rn, rm } => write!(f, "ldr {}, [{}, {}]", rt, rn, rm),
            Instruction::LDR_imm { rt, rn, imm32 } => write!(f, "ldr {}, [{}, #{}]", rt, rn, imm32),
            Instruction::LDR_lit { rt, imm32 } => {
                if imm32 == 0 {
                    write!(f, "ldr {}, [pc]", rt)
                } else {
                    write!(f, "ldr {}, [pc, #{}]", rt, imm32)
                }
            }
            Instruction::LDRB_imm { rt, rn, imm32 } => {
                if imm32 == 0 {
                    write!(f, "ldrb {}, [{}]", rt, rn)
                } else {
                    write!(f, "ldrb {}, [{}, #{}]", rt, rn, imm32)
                }
            }
            Instruction::LDRB_reg { rt, rn, rm } => write!(f, "ldrb {}, [{}, {}]", rt, rn, rm),
            Instruction::LDRH_imm { rt, rn, imm32 } => {
                if imm32 == 0 {
                    write!(f, "ldrh {}, [{}]", rt, rn)
                } else {
                    write!(f, "ldrh {}, [{}, #{}]", rt, rn, imm32)
                }
            }
            Instruction::LDRH_reg { rt, rn, rm } => write!(f, "ldrh {}, [{}, {}]", rt, rn, rm),
            Instruction::LDRSB_reg { rt, rn, rm } => write!(f, "ldrsb {}, [{}, {}]", rt, rn, rm),
            Instruction::LDRSH_reg { rt, rn, rm } => write!(f, "ldrsh {}, [{}, {}]", rt, rn, rm),
            Instruction::LSL_imm {
                rd,
                rm,
                imm5,
                setflags,
            } => write!(
                f,
                "lsl{} {}, {}, #{}",
                if setflags { "s" } else { "" },
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
                "lsl{} {}, {}, {}",
                if setflags { "s" } else { "" },
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
                "lsr{} {}, {}, {}",
                if setflags { "s" } else { "" },
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
                "lsr{} {}, {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                rm,
                imm5
            ),
            Instruction::MSR_reg { spec_reg, rn } => write!(f, "msr {}, {}", spec_reg, rn),
            Instruction::MRS { rd, spec_reg } => write!(f, "mrs {}, {}", rd, spec_reg),
            Instruction::MUL {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "mul{} {}, {}, {}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::MOV_reg { rd, rm, setflags } => {
                write!(f, "mov{} {}, {}", if setflags { "s" } else { "" }, rd, rm)
            }
            Instruction::MOV_imm {
                rd,
                imm32,
                setflags,
            } => write!(
                f,
                "mov{} {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                imm32
            ),
            Instruction::MVN_reg { rd, rm, setflags } => {
                write!(f, "mvn{} {}, {}", if setflags { "s" } else { "" }, rd, rm)
            }
            Instruction::NOP => write!(f, "nop"),
            Instruction::ORR {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "orr{} {}, {}, {}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::POP { registers } => write!(f, "pop {:?}", registers),
            Instruction::PUSH { registers } => write!(f, "push {:?}", registers),
            Instruction::REV { rd, rm } => write!(f, "rev {}, {}", rd, rm),
            Instruction::REV16 { rd, rm } => write!(f, "rev16 {}, {}", rd, rm),
            Instruction::REVSH { rd, rm } => write!(f, "revsh {}, {}", rd, rm),
            Instruction::ROR_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "ror{} {}, {}, #{}",
                if setflags { "s" } else { "" },
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
                "rsb{} {}, {}, #{}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                imm32
            ),
            Instruction::SEV => write!(f, "sev"),
            Instruction::SBC_reg {
                rd,
                rn,
                rm,
                setflags,
            } => write!(
                f,
                "sbc{} {}, {}, {}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::STM { rn, registers } => write!(f, "stm {}, {{{:?}}}", rn, registers),
            Instruction::STR_imm { rn, rt, imm32 } => {
                if imm32 == 0 {
                    write!(f, "str {}, [{}]", rt, rn)
                } else {
                    write!(f, "str {}, [{}, #{}]", rt, rn, imm32)
                }
            }
            Instruction::STR_reg { rn, rm, rt } => write!(f, "str {}, [{}, {}]", rt, rn, rm),
            Instruction::STRB_imm { rn, rt, imm32 } => {
                if imm32 == 0 {
                    write!(f, "strb {}, [{}]", rt, rn)
                } else {
                    write!(f, "strb {}, [{}, #{}]", rt, rn, imm32)
                }
            }
            Instruction::STRB_reg { rn, rm, rt } => write!(f, "strb {}, [{}, {}]", rt, rn, rm),
            Instruction::STRH_imm { rt, rn, imm32 } => {
                if imm32 == 0 {
                    write!(f, "strh {}, [{}]", rt, rn)
                } else {
                    write!(f, "strh {}, [{}, #{}]", rt, rn, imm32)
                }
            }
            Instruction::STRH_reg { rn, rm, rt } => write!(f, "strh {}, [{}, {}]", rt, rn, rm),
            Instruction::SUB_imm {
                rd,
                rn,
                imm32,
                setflags,
            } => {
                if rd == rn {
                    write!(
                        f,
                        "sub{} {}, #{}",
                        if setflags { "s" } else { "" },
                        rd,
                        imm32
                    )
                } else {
                    write!(
                        f,
                        "sub{} {}, {}, #{}",
                        if setflags { "s" } else { "" },
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
                setflags,
            } => write!(
                f,
                "sub{} {}, {}, {}",
                if setflags { "s" } else { "" },
                rd,
                rn,
                rm
            ),
            Instruction::SVC { imm32 } => write!(f, "svc #{}", imm32),
            Instruction::SXTB { rd, rm } => write!(f, "sxtb {}, {}", rd, rm),
            Instruction::SXTH { rd, rm } => write!(f, "sxth {}, {}", rd, rm),
            Instruction::TST_reg { rn, rm } => write!(f, "tst {}, {}", rn, rm),
            Instruction::UDF { imm32, ref opcode } => {
                write!(f, "udf {} (opcode = {})", imm32, opcode)
            }
            Instruction::UXTB { rd, rm } => write!(f, "uxtb {}, {}", rd, rm),
            Instruction::UXTH { rd, rm } => write!(f, "uxth {}, {}", rd, rm),
            Instruction::WFE => write!(f, "wfe"),
            Instruction::WFI => write!(f, "wfi"),
            Instruction::YIELD => write!(f, "yield"),
        }
    }
}
