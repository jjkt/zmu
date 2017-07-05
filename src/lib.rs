#![allow(dead_code)]
#![allow(unused_variables)]
/* http://ecee.colorado.edu/ecen3000/labs/lab3/files/DDI0419C_arm_architecture_v6m_reference_manual.pdf */
use std::io::Cursor;
use std::io::SeekFrom;
use std::io::Seek;

extern crate byteorder;
extern crate bit_field;
use byteorder::{LittleEndian, ReadBytesExt};
use bit_field::BitField;

pub fn is_thumb32(word: u16) -> bool {
    match word >> 11 {
        0b11101 | 0b11110 | 0b11111 => true,
        _ => false,
    }
}

enum StackPointer {
    MSP(u32),
    PSP(u32),
}

enum ProcessorMode {
    ThreadMode,
    HandlerMode,
}

trait Apsr {
    fn get_n(&self) -> bool;
    fn set_n(&mut self, n: bool);

    fn get_z(&self) -> bool;
    fn set_z(&mut self, z: bool);

    fn get_c(&self) -> bool;
    fn set_c(&mut self, c: bool);

    fn get_v(&self) -> bool;
    fn set_v(&mut self, v: bool);

    fn get_q(&self) -> bool;
    fn set_q(&mut self, q: bool);
}

trait Ipsr {
    fn get_exception_number(&self) -> u8;
}

trait Primask {
    fn get_primask(&self) -> bool;
}

trait ControlRegister {
    fn get_active_stack_pointer(&self) -> StackPointer;
}


impl Apsr for u32 {
    fn get_n(&self) -> bool {
        (*self).get_bit(31)
    }
    fn set_n(&mut self, n: bool) {
        (*self).set_bit(31, n);
    }

    fn get_z(&self) -> bool {
        (*self).get_bit(30)
    }
    fn set_z(&mut self, z: bool) {
        (*self).set_bit(30, z);
    }

    fn get_c(&self) -> bool {
        (*self).get_bit(29)
    }
    fn set_c(&mut self, c: bool) {
        (*self).set_bit(29, c);
    }
    fn get_v(&self) -> bool {
        (*self).get_bit(28)
    }
    fn set_v(&mut self, v: bool) {
        (*self).set_bit(28, v);
    }

    fn get_q(&self) -> bool {
        (*self).get_bit(27)
    }
    fn set_q(&mut self, q: bool) {
        (*self).set_bit(27, q);
    }
}


pub struct Core {
    pc: u32,
    msp: u32,
    psp: u32,
    r: [u32; 15],

    apsr: u32,
    ipsr: u32,
    epsr: u32,

    primask: u32,
    control: u32,

    mode: ProcessorMode,
}

impl Core {
    pub fn new() -> Core {
        Core {
            mode: ProcessorMode::ThreadMode,
            pc: 0,
            msp: 0,
            psp: 0,
            apsr: 0,
            ipsr: 0,
            epsr: 0,
            primask: 0,
            control: 0,
            r: [0; 15],
        }
    }
}

pub trait Fetch {
    fn fetch32(&mut self, addr: u32) -> u32;
    fn fetch16(&mut self, addr: u32) -> u16;
}

#[derive(PartialEq)]
pub enum Condition {
    EQ, // Equal
    NE, // Not Equal
    CS, // Carry Set
    CC, // Carry clear
    MI, // Minus, negative
    PL, // Plus, positive or zero
    VS, // Overflow
    VC, // No overflow
    HI, // Unsigned higher
    LS, // Unsigned lower or same
    GE, // Signer greater than or equal
    LT, // Signed less than
    GT, // Signed greater than
    LE, // Signed less than or equal
    AL, // None or (AL = optional mnemonic extension for always)
}

impl Condition {
    fn value(&self) -> usize {
        match *self {
            Condition::EQ => 0b0000, 
            Condition::NE => 0b0001, 
            Condition::CS => 0b0010, 
            Condition::CC => 0b0011, 
            Condition::MI => 0b0100, 
            Condition::PL => 0b0101, 
            Condition::VS => 0b0110, 
            Condition::VC => 0b0111, 
            Condition::HI => 0b1000, 
            Condition::LS => 0b1001, 
            Condition::GE => 0b1010, 
            Condition::LT => 0b1011, 
            Condition::GT => 0b1100, 
            Condition::LE => 0b1101, 
            Condition::AL => 0b1110, 
        }
    }

    fn from_u16(n: u16) -> Option<Condition> {
        match n {
            0 => Some(Condition::EQ),
            1 => Some(Condition::NE),
            2 => Some(Condition::CS),
            3 => Some(Condition::CC),
            4 => Some(Condition::MI),
            5 => Some(Condition::PL),
            6 => Some(Condition::VS),
            7 => Some(Condition::VC),
            8 => Some(Condition::HI),
            9 => Some(Condition::LS),
            10 => Some(Condition::GE),
            11 => Some(Condition::LT),
            12 => Some(Condition::GT),
            13 => Some(Condition::LE),
            14 => Some(Condition::AL),
            _ => None,
        }
    }
}

#[derive(PartialEq)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    SP,
    LR,
    PC,
}

impl Reg {
    fn value(&self) -> usize {
        match *self {
            Reg::R0 => 0,
            Reg::R1 => 1,
            Reg::R2 => 2,
            Reg::R3 => 3,
            Reg::R4 => 4,
            Reg::R5 => 5,
            Reg::R6 => 6,
            Reg::R7 => 7,
            Reg::R8 => 8,
            Reg::R9 => 9,
            Reg::R10 => 10,
            Reg::R11 => 11,
            Reg::R12 => 12,
            Reg::SP => 13,
            Reg::LR => 14,
            Reg::PC => 15,
        }
    }

    fn from_u16(n: u16) -> Option<Reg> {
        match n {
            0 => Some(Reg::R0),
            1 => Some(Reg::R1),
            2 => Some(Reg::R2),
            3 => Some(Reg::R3),
            4 => Some(Reg::R4),
            5 => Some(Reg::R5),
            6 => Some(Reg::R6),
            7 => Some(Reg::R7),
            8 => Some(Reg::R8),
            9 => Some(Reg::R9),
            10 => Some(Reg::R10),
            11 => Some(Reg::R11),
            12 => Some(Reg::R12),
            13 => Some(Reg::SP),
            14 => Some(Reg::LR),
            15 => Some(Reg::PC),
            _ => None,
        }
    }
}




#[allow(non_camel_case_types)]
pub enum Op {
    MOV { rd: Reg, rm: Reg },
    MOV_imm8 { rd: Reg, imm8: u8 },
    LSL,
    LSR,
    ASR,
    AND,
    EOR,
    ADC,
    SBC,
    ROR,
    TST,
    RSB,
    CMP,
    CMP_imm8 { rn: Reg, imm8: u8 },
    CMN,
    ORR,
    MUL,
    BIC,
    MVN,
    ADD,
    ADD_imm3,
    ADD_imm8,
    SUB,
    SUB_imm3,
    SUB_imm8,
    BX { rm: Reg },
    BLX,
    BL { imm32: i32 },
    B_imm8 { cond: Condition, imm8: u8 },
    B_imm11 { imm11: u16 },
    SVC,
    UDF,
}


pub fn decode_16(command: u16) -> Option<Op> {
    println!("decoding thumb16: 0x{:x}", command);
    match command & 0xc000 {
        0b0000_0000_0000_0000_u16 => {
            // Shift (immediate), add, substract, move and compare
            match (command & 0b00_11111_0_0000_0000) >> 9 {
                0b000_00 | 0b000_01 | 0b000_10 | 0b000_11 => Some(Op::LSL),
                0b001_00 | 0b001_01 | 0b001_10 | 0b001_11 => Some(Op::LSR),
                0b010_00 | 0b010_01 | 0b010_10 | 0b010_11 => Some(Op::ASR),
                0b011_00 => Some(Op::ADD),
                0b011_01 => Some(Op::SUB),
                0b011_10 => Some(Op::ADD_imm3),
                0b011_11 => Some(Op::SUB_imm3),
                0b100_00 | 0b100_01 | 0b100_10 | 0b100_11 => {
                    Some(Op::MOV_imm8 {
                        //rd: reg_from_u8((((command & 0b111_0000_0000)) >> 8) as u8).unwrap(),
                        rd: Reg::from_u16(command.get_bits(7..10)).unwrap(),
                        imm8: command.get_bits(0..8) as u8,
                    })
                }
                0b101_00 | 0b101_01 | 0b101_10 | 0b101_11 => {
                    Some(Op::CMP_imm8 {
                        rn: Reg::from_u16(command.get_bits(7..10)).unwrap(),
                        imm8: command.get_bits(0..8) as u8,
                    })
                }
                0b110_00 | 0b110_01 | 0b110_10 | 0b110_11 => Some(Op::ADD_imm8),
                0b111_00 | 0b111_01 | 0b111_10 | 0b111_11 => Some(Op::ADD_imm8),
                0b100_00_0_00000000 => None,
                _ => None,
            }
        }
        0b0100_0000_0000_0000_u16 => {
            // data process, special data, load from lp...
            match command & 0xffc0 {
                0b010000_0000_000000_u16 => Some(Op::AND),
                0b010000_0001_000000_u16 => Some(Op::EOR),
                0b010000_0010_000000_u16 => Some(Op::LSL),
                0b010000_0011_000000_u16 => Some(Op::LSR),
                0b010000_0100_000000_u16 => Some(Op::ASR),
                0b010000_0101_000000_u16 => Some(Op::ADC),
                0b010000_0110_000000_u16 => Some(Op::SBC),
                0b010000_0111_000000_u16 => Some(Op::ROR),
                0b010000_1000_000000_u16 => Some(Op::TST),
                0b010000_1001_000000_u16 => Some(Op::RSB),
                0b010000_1010_000000_u16 => Some(Op::CMP),
                0b010000_1011_000000_u16 => Some(Op::CMN),
                0b010000_1100_000000_u16 => Some(Op::ORR),
                0b010000_1101_000000_u16 => Some(Op::MUL),
                0b010000_1110_000000_u16 => Some(Op::BIC),
                0b010000_1111_000000_u16 => Some(Op::MVN),

                0b010001_0000_000000_u16 => Some(Op::ADD),
                0b010001_0100_000000_u16 => None,
                0b010001_0101_000000_u16 |
                0b010001_0110_000000_u16 |
                0b010001_0111_000000_u16 => Some(Op::CMP),
                0b010001_1000_000000_u16 |
                0b010001_1001_000000_u16 |
                0b010001_1010_000000_u16 |
                0b0100_0110_1100_0000_u16 => {
                    Some(Op::MOV {
                        rd: Reg::from_u16((command & 8) + ((command & 0x80)) >> 4).unwrap(),
                        rm: Reg::from_u16((command >> 3) & 0xf).unwrap(),
                    })
                }
                0b0100_0111_0100_0000_u16 => {
                    Some(Op::BX { rm: Reg::from_u16((command >> 3) & 0xf).unwrap() })
                }
                0b010001_1110_000000_u16 |
                0b010001_1111_000000_u16 => Some(Op::BLX),
                _ => None,

            }
        }
        0b1000_0000_0000_0000_u16 => {
            // generate pc relative addr, sp rela, misc
            None
        }
        0b1100_0000_0000_0000_u16 => {
            // store, load multiple, branch, svc, uncond branch
            println!("maybe branch 0x{:x} {:b}",
                     command,
                     command.get_bits(12..16));
            match command.get_bits(12..16) {
                0b1101 => {
                    println!("maybe branch {:b}", command.get_bits(12..16));
                    let cond = command.get_bits(8..12);
                    if cond == 0b1111 {
                        return Some(Op::SVC);
                    }
                    if cond == 0b1110 {
                        return Some(Op::UDF);
                    }

                    Some(Op::B_imm8 {
                        cond: Condition::from_u16(cond).unwrap(),
                        imm8: command.get_bits(0..8) as u8,
                    })

                }
                0b1110 => {
                    // TODOcould be B_imm11
                    None
                }
                _ => None,

            }
        }
        _ => None,
    }
}

pub fn sign_extend(word: u32, topbit: u8, size: u8) -> i32 {
    if word & (1 << topbit) == (1 << topbit) {
        return (word | (((1 << (size - topbit)) - 1) << topbit)) as i32;
    }
    word as i32
}

//A 5.3.1 Branch and misc (thumb32)
pub fn decode_branch_and_misc(t1: u16, t2: u16) -> Option<Op> {
    let op1 = (t1 >> 4) & 0x7f;
    let op2 = (t2 >> 12) & 0x07;

    match op2 {
        0x7 | 0x5 => {
            let s = ((t1 >> 10) & 1) as u32;
            let imm10 = (t1 & 0x3ff) as u32;

            let j1 = ((t2 >> 13) & 1) as u32;
            let j2 = ((t2 >> 11) & 1) as u32;
            let imm11 = (t2 & 0x7ff) as u32;


            let i1 = ((j1 ^ s) ^ 1) as u32;
            let i2 = ((j2 ^ s) ^ 1) as u32;

            let imm = sign_extend((imm11 << 1) + (imm10 << 12) + (i2 << 22) + (i1 << 23) +
                                  (s << 24),
                                  24,
                                  32);

            Some(Op::BL { imm32: imm as i32 })
        }
        _ => {
            Some(Op::MOV {
                rd: Reg::R0,
                rm: Reg::R1,
            })
        }//others
    }


}

// A5.3 check thumb32 encodings
pub fn decode_32(t1: u16, t2: u16) -> Option<Op> {
    println!("decoding thumb32: 0x{:X} 0x{:X}", t1, t2);

    //let op1 = (t1 >> 11) & 0x03;
    // 1010 1010 1010 1010
    // 1010 1

    let op1 = (t1 >> 11) & 0x03;
    let op = (t2 >> 15) & 0x01;

    if op1 != 0x2 {
        return None;
    }
    if op != 0x01 {
        return None;
    }


    decode_branch_and_misc(t1, t2)
}

//
// Add two numbers and carry
//
// x + y + carry
//
// return tuple of (result, carry, overflow)
//
fn add_with_carry(x: u32, y: u32, carry_in: bool) -> (u32, bool, bool) {
    let unsigned_sum = x as u64 + y as u64 + (carry_in as u64);
    let signed_sum = (x as i32) + (y as i32) + (carry_in as i32);
    let result = (unsigned_sum & 0xffffffff) as u32; // same value as signed_sum<N-1:0>
    let carry_out = (result as u64) != unsigned_sum;
    let overflow = (result as i32) != signed_sum;

    (result, carry_out, overflow)
}

//
// This function performs the condition test for an instruction, based on:
// • the two Thumb conditional branch encodings, encodings T1 andT3 of the B instruction
// • the current values of the xPSR.IT[7:0] bits for other Thumb instructions.
//
fn condition_passed(condition: Condition, aspr: &u32) -> bool {

    match condition {
        Condition::EQ => aspr.get_z(),
        Condition::NE => !aspr.get_z(),
        Condition::CS => aspr.get_c(),
        Condition::CC => !aspr.get_c(),
        Condition::MI => aspr.get_n(),
        Condition::PL => !aspr.get_n(),

        Condition::VS => aspr.get_v(),
        Condition::VC => !aspr.get_v(),

        Condition::HI => aspr.get_c() && aspr.get_z(),
        Condition::LS => !(aspr.get_c() && aspr.get_z()),

        Condition::GE => aspr.get_n() == aspr.get_v(),
        Condition::LT => !(aspr.get_n() == aspr.get_v()),

        Condition::GT => (aspr.get_n() == aspr.get_v()) && !aspr.get_z(),
        Condition::LE => !((aspr.get_n() == aspr.get_v()) && !aspr.get_z()),

        Condition::AL => true,

    }



}

pub fn execute(core: &mut Core, op: Option<Op>) {
    match op {
        None => panic!("undefined code"),
        Some(oper) => {
            match oper {
                Op::MOV { rd, rm } => {
                    core.pc = core.pc + 2;
                    core.r[rd.value() as usize] = core.r[rm.value()];
                }
                Op::BL { imm32 } => {
                    core.pc = core.pc + 4; // thumb32 instruction
                    core.r[Reg::LR.value()] = core.pc | 0x01;
                    core.pc = ((core.pc as i32) + imm32) as u32;
                }
                Op::BX { rm } => {
                    core.pc = core.pc + 2;
                    core.pc = core.r[rm.value() as usize] & 0xfffffffe;
                }
                Op::MOV_imm8 { rd, imm8 } => {
                    core.pc = core.pc + 2;
                    core.r[rd.value()] = imm8 as u32;
                }
                Op::B_imm8 { cond, imm8 } => {
                    core.pc = core.pc + 2;
                    let imm32 = sign_extend(imm8 as u32, 8, 32);
                    if condition_passed(cond, &core.apsr) {
                        core.pc = ((core.pc as i32) + imm32) as u32;
                    }

                }
                Op::CMP_imm8 { rn, imm8 } => {
                    core.pc = core.pc + 2;
                    let imm32 = imm8 as u32;
                    let (result, carry, overflow) =
                        add_with_carry(core.r[rn.value()], imm32 ^ 0xFFFFFFFF, true);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);

                    println!(" apsr is 0x{:x}", core.apsr);
                    core.r[rn.value()] = imm8 as u32;
                }
                _ => {}
            }
        }
    }
}

pub fn fetch_and_decode<T: Fetch>(memory: &mut T, pc: u32) -> Option<Op> {
    let hw = memory.fetch16(pc);
    match is_thumb32(hw) {
        true => {
            let hw2 = memory.fetch16(pc + 2);
            decode_32(hw, hw2)
        }
        false => decode_16(hw),

    }
}

pub fn run_bin<T: Fetch>(memory: &mut T) {
    let mut core = Core::new();
    let reset_vector = memory.fetch32(4);

    core.pc = reset_vector & 0xfffffffe;
    core.epsr = (reset_vector & 1) << 24;
    core.msp = memory.fetch32(0);

    loop {
        println!("pc = 0x{:X}", core.pc);
        let pc = core.pc;
        let op = fetch_and_decode(memory, pc);
        execute(&mut core, op);
    }
}

#[test]
fn test_is_thumb32() {
    assert!(is_thumb32(0b1110100000000000));
    assert!(is_thumb32(0b1111000000000000));
    assert!(is_thumb32(0b1111100000000000));
    assert!(is_thumb32(0b1110000000000000) == false);
    assert!(is_thumb32(0b1111111111111111));
    assert!(is_thumb32(0b0000000000000001) == false);
}

#[test]
fn test_decode_thumb16() {
    // mov
    match decode_16(0x4600).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R0);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4608).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R1);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4610).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R2);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4618).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R3);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4620).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R4);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4628).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R5);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4630).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R6);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4638).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R7);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4640).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R8);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4648).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R9);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4650).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R10);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4658).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R11);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4660).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R12);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4668).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::SP);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4670).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::LR);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4678).unwrap() {
        Op::MOV { rd, rm } => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::PC);
        }
        _ => {
            assert!(false);
        }
    }

    //MOVS (mov immediate)
    match decode_16(0x2001).unwrap() {
        Op::MOV_imm8 { rd, imm8 } => {
            assert!(rd == Reg::R0);
            assert!(imm8 == 1);
        }
        _ => {
            assert!(false);
        }
    }

    //BX LR
    match decode_16(0x4770).unwrap() {
        Op::BX { rm } => {
            assert!(rm == Reg::LR);
        }
        _ => {
            assert!(false);
        }
    }
    //CMP R0, R0
    match decode_16(0x2800).unwrap() {
        Op::CMP_imm8 { rn, imm8 } => {
            assert!(rn == Reg::R0);
            assert!(imm8 == 0);
        }
        _ => {
            assert!(false);
        }
    }
    // BEQ.N
    match decode_16(0xd001).unwrap() {
        Op::B_imm8 { cond, imm8 } => {
            assert!(cond == Condition::EQ);
            assert!(imm8 == 2);
        }
        _ => {
            assert!(false);
        }
    }

    // PUSH  {R4, LR}
    match decode_16(0xb510).unwrap() {
        Op::PUSH { cond, imm8 } => {
            assert!(cond == Condition::EQ);
            assert!(imm8 == 2);
        }
        _ => {
            assert!(false);
        }
    }

}

struct ConstMemory<'a> {
    reader: Cursor<&'a [u8]>,
}

impl<'a> ConstMemory<'a> {
    pub fn new(bin: &[u8]) -> ConstMemory {
        ConstMemory { reader: Cursor::new(bin) }
    }
}

impl<'a> Fetch for ConstMemory<'a> {
    fn fetch16(&mut self, addr: u32) -> u16 {
        self.reader.seek(SeekFrom::Start(addr as u64)).unwrap();
        self.reader.read_u16::<LittleEndian>().unwrap()
    }

    fn fetch32(&mut self, addr: u32) -> u32 {
        self.reader.seek(SeekFrom::Start(addr as u64)).unwrap();
        self.reader.read_u32::<LittleEndian>().unwrap()
    }
}


#[test]
fn test_hello_world() {
    let hellow_bin: [u8; 1204] =
        [0x08, 0x04, 0x00, 0x20, 0xa1, 0x04, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x83, 0x02, 0x00,
         0x00, 0x83, 0x02, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83,
         0x02, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00,
         0x83, 0x02, 0x00, 0x00, 0x70, 0xb5, 0x06, 0x00, 0x08, 0x00, 0x14, 0x00, 0x00, 0x2b, 0x05,
         0xd0, 0x1d, 0x00, 0x21, 0x78, 0xb0, 0x47, 0x64, 0x1c, 0x6d, 0x1e, 0xfa, 0xd1, 0x70, 0xbd,
         0xfd, 0xb5, 0x86, 0xb0, 0x0c, 0x00, 0x09, 0xe0, 0x01, 0x23, 0x07, 0x9a, 0x21, 0x00, 0x06,
         0x98, 0xff, 0xf7, 0xe9, 0xff, 0x04, 0x00, 0x07, 0x98, 0x40, 0x1c, 0x07, 0x90, 0x07, 0x98,
         0x00, 0x78, 0x00, 0x28, 0x01, 0xd1, 0x09, 0xb0, 0xf0, 0xbd, 0x25, 0x28, 0xed, 0xd1, 0x07,
         0x98, 0x46, 0x78, 0x80, 0x1c, 0x07, 0x90, 0x25, 0x2e, 0x00, 0xd1, 0x95, 0xe0, 0x58, 0x2e,
         0x69, 0xd0, 0x63, 0x2e, 0x0e, 0xd0, 0x64, 0x2e, 0x1a, 0xd0, 0x69, 0x2e, 0x18, 0xd0, 0x6f,
         0x2e, 0x61, 0xd0, 0x70, 0x2e, 0x68, 0xd0, 0x73, 0x2e, 0x70, 0xd0, 0x75, 0x2e, 0x5b, 0xd0,
         0x78, 0x2e, 0x59, 0xd0, 0xdd, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60, 0x01,
         0x68, 0x00, 0x1d, 0x08, 0x9a, 0x10, 0x60, 0xc9, 0xb2, 0x20, 0x00, 0x06, 0x9a, 0x90, 0x47,
         0x04, 0x00, 0xcf, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60, 0x05, 0x68, 0x00,
         0x1d, 0x08, 0x99, 0x08, 0x60, 0x00, 0x2d, 0x04, 0xd5, 0x2d, 0x21, 0x20, 0x00, 0x06, 0x9a,
         0x90, 0x47, 0x04, 0x00, 0x64, 0x26, 0x6f, 0x2e, 0x01, 0xd1, 0x08, 0x21, 0x06, 0xe0, 0x20,
         0x21, 0x31, 0x43, 0x78, 0x29, 0x01, 0xd0, 0x0a, 0x21, 0x00, 0xe0, 0x10, 0x21, 0x01, 0x91,
         0x0b, 0x21, 0x00, 0x91, 0x2f, 0x00, 0x64, 0x2e, 0x03, 0xd1, 0x00, 0x2d, 0x01, 0xd5, 0x78,
         0x42, 0x07, 0x00, 0x00, 0x9d, 0x6d, 0x1e, 0x00, 0x95, 0x38, 0x00, 0x01, 0x99, 0x00, 0xf0,
         0x4b, 0xf8, 0x30, 0x31, 0xc9, 0xb2, 0x3a, 0x29, 0x02, 0xdb, 0x30, 0x00, 0x51, 0x38, 0x09,
         0x18, 0x02, 0xa8, 0x45, 0x19, 0x29, 0x70, 0x38, 0x00, 0x01, 0x99, 0x00, 0xf0, 0x3d, 0xf8,
         0x07, 0x00, 0x02, 0xd0, 0x00, 0x98, 0x01, 0x28, 0xe5, 0xda, 0x20, 0x00, 0x0b, 0x21, 0x00,
         0x9a, 0x8c, 0x1a, 0xb9, 0xd0, 0x29, 0x78, 0x06, 0x9a, 0x90, 0x47, 0x6d, 0x1c, 0x64, 0x1e,
         0xf9, 0xd1, 0xb2, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60, 0x05, 0x68, 0x00,
         0x1d, 0x08, 0x99, 0x08, 0x60, 0xbb, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60,
         0x05, 0x68, 0x00, 0x1d, 0x08, 0x99, 0x08, 0x60, 0x78, 0x26, 0xb1, 0xe7, 0x08, 0x98, 0x00,
         0x68, 0x08, 0x99, 0x08, 0x60, 0x05, 0x68, 0x00, 0x1d, 0x08, 0x99, 0x08, 0x60, 0x28, 0x00,
         0x00, 0xf0, 0x65, 0xf8, 0x01, 0x00, 0x20, 0x00, 0x00, 0x29, 0x90, 0xd0, 0x0c, 0x00, 0x29,
         0x78, 0x06, 0x9a, 0x90, 0x47, 0x6d, 0x1c, 0x64, 0x1e, 0xf9, 0xd1, 0x88, 0xe7, 0x25, 0x21,
         0x83, 0xe7, 0x00, 0x22, 0x03, 0x0a, 0x8b, 0x42, 0x0b, 0xd2, 0x03, 0x09, 0x8b, 0x42, 0x19,
         0xd2, 0x43, 0x08, 0x8b, 0x42, 0x2e, 0xd2, 0x41, 0x1a, 0x00, 0xd2, 0x01, 0x46, 0x52, 0x41,
         0x10, 0x46, 0x70, 0x47, 0xff, 0x22, 0x09, 0x02, 0x3f, 0xd0, 0x12, 0x06, 0x8b, 0x42, 0x05,
         0xd3, 0x12, 0x12, 0x09, 0x02, 0x8b, 0x42, 0x01, 0xd3, 0x12, 0x12, 0x09, 0x02, 0x03, 0x09,
         0x8b, 0x42, 0x19, 0xd3, 0x00, 0xe0, 0x09, 0x0a, 0xc3, 0x09, 0x8b, 0x42, 0x01, 0xd3, 0xcb,
         0x01, 0xc0, 0x1a, 0x52, 0x41, 0x83, 0x09, 0x8b, 0x42, 0x01, 0xd3, 0x8b, 0x01, 0xc0, 0x1a,
         0x52, 0x41, 0x43, 0x09, 0x8b, 0x42, 0x01, 0xd3, 0x4b, 0x01, 0xc0, 0x1a, 0x52, 0x41, 0x03,
         0x09, 0x8b, 0x42, 0x01, 0xd3, 0x0b, 0x01, 0xc0, 0x1a, 0x52, 0x41, 0xc3, 0x08, 0x8b, 0x42,
         0x01, 0xd3, 0xcb, 0x00, 0xc0, 0x1a, 0x52, 0x41, 0x83, 0x08, 0x8b, 0x42, 0x01, 0xd3, 0x8b,
         0x00, 0xc0, 0x1a, 0x52, 0x41, 0x43, 0x08, 0x8b, 0x42, 0x01, 0xd3, 0x4b, 0x00, 0xc0, 0x1a,
         0x52, 0x41, 0x88, 0x42, 0x00, 0xd3, 0x40, 0x1a, 0x52, 0x41, 0xcf, 0xd2, 0x01, 0x46, 0x10,
         0x46, 0x70, 0x47, 0x08, 0xb5, 0x00, 0xf0, 0x0a, 0xf8, 0x08, 0xbd, 0x01, 0x00, 0x00, 0xe0,
         0x49, 0x1c, 0x0a, 0x78, 0x00, 0x2a, 0xfb, 0xd1, 0x08, 0x1a, 0x70, 0x47, 0xfe, 0xe7, 0x70,
         0x47, 0x00, 0x00, 0x80, 0xb5, 0x00, 0xf0, 0x33, 0xf8, 0x02, 0x00, 0x00, 0x23, 0xdb, 0x43,
         0x10, 0x68, 0x98, 0x42, 0x04, 0xd0, 0x11, 0x00, 0x02, 0x20, 0xab, 0xbe, 0x00, 0x20, 0x10,
         0x60, 0x50, 0x68, 0x98, 0x42, 0x04, 0xd0, 0x11, 0x1d, 0x02, 0x20, 0xab, 0xbe, 0x00, 0x20,
         0x50, 0x60, 0x01, 0xbd, 0x00, 0x00, 0x10, 0xb5, 0x84, 0xb0, 0x04, 0x00, 0x00, 0xf0, 0x19,
         0xf8, 0xa1, 0x00, 0x42, 0x18, 0x10, 0x68, 0x00, 0x21, 0xc9, 0x43, 0x88, 0x42, 0x0d, 0xd1,
         0x07, 0xa0, 0x00, 0x90, 0x00, 0x2c, 0x01, 0xd1, 0x00, 0x20, 0x00, 0xe0, 0x04, 0x20, 0x01,
         0x90, 0x03, 0x20, 0x02, 0x90, 0x69, 0x46, 0x01, 0x20, 0xab, 0xbe, 0x10, 0x60, 0x04, 0xb0,
         0x10, 0xbd, 0x3a, 0x74, 0x74, 0x00, 0x00, 0x48, 0x70, 0x47, 0x00, 0x00, 0x00, 0x20, 0x30,
         0xb4, 0x01, 0x21, 0x02, 0x68, 0x00, 0x1d, 0x00, 0x2a, 0x0f, 0xd0, 0x03, 0x68, 0xc3, 0x18,
         0x44, 0x68, 0x08, 0x30, 0x0c, 0x42, 0x02, 0xd0, 0x4d, 0x46, 0x6d, 0x1e, 0x64, 0x19, 0x1d,
         0x68, 0x25, 0x60, 0x1b, 0x1d, 0x24, 0x1d, 0x12, 0x1f, 0xec, 0xd0, 0xf8, 0xe7, 0x30, 0xbc,
         0x70, 0x47, 0x10, 0xb5, 0x07, 0x49, 0x79, 0x44, 0x18, 0x31, 0x06, 0x4c, 0x7c, 0x44, 0x16,
         0x34, 0x04, 0xe0, 0x08, 0x1d, 0x0a, 0x68, 0x89, 0x18, 0x88, 0x47, 0x01, 0x00, 0xa1, 0x42,
         0xf8, 0xd1, 0x10, 0xbd, 0xc0, 0x00, 0x00, 0x00, 0xd0, 0x00, 0x00, 0x00, 0x0e, 0xb4, 0x00,
         0xb5, 0x82, 0xb0, 0x03, 0xa9, 0x00, 0x91, 0x6b, 0x46, 0x02, 0x00, 0x01, 0x21, 0x03, 0x48,
         0x78, 0x44, 0x0a, 0x30, 0xff, 0xf7, 0x76, 0xfe, 0x02, 0x99, 0x06, 0xb0, 0x08, 0x47, 0x05,
         0x01, 0x00, 0x00, 0x38, 0xb5, 0x04, 0x00, 0x00, 0x25, 0xed, 0x43, 0xac, 0x42, 0x09, 0xd0,
         0x69, 0x46, 0x08, 0x70, 0x01, 0x22, 0x01, 0x20, 0x00, 0xf0, 0x06, 0xf8, 0x01, 0x28, 0x01,
         0xd1, 0x20, 0x00, 0x32, 0xbd, 0x28, 0x00, 0x32, 0xbd, 0x80, 0xb5, 0x00, 0x28, 0x02, 0xd4,
         0x00, 0xf0, 0x03, 0xf8, 0x02, 0xbd, 0x00, 0x20, 0x02, 0xbd, 0x80, 0xb5, 0x00, 0x29, 0x01,
         0xd1, 0x00, 0x20, 0x02, 0xbd, 0x01, 0x28, 0x01, 0xd0, 0x02, 0x28, 0x02, 0xd1, 0x00, 0xf0,
         0x04, 0xf8, 0x02, 0xbd, 0x00, 0xf0, 0x11, 0xf8, 0x02, 0xbd, 0x30, 0xb5, 0x83, 0xb0, 0x0c,
         0x00, 0x15, 0x00, 0x01, 0x20, 0xff, 0xf7, 0x70, 0xff, 0x00, 0x90, 0x01, 0x94, 0x02, 0x95,
         0x69, 0x46, 0x05, 0x20, 0xab, 0xbe, 0x28, 0x1a, 0x03, 0xb0, 0x30, 0xbd, 0x10, 0xb5, 0x84,
         0xb0, 0x03, 0x00, 0x14, 0x00, 0x00, 0x93, 0x01, 0x91, 0x02, 0x94, 0x69, 0x46, 0x05, 0x20,
         0xab, 0xbe, 0x01, 0x22, 0x19, 0x00, 0x20, 0x1a, 0xc0, 0x46, 0xc0, 0x46, 0x04, 0xb0, 0x10,
         0xbd, 0xf1, 0xfe, 0xff, 0xff, 0x08, 0x00, 0x00, 0x00, 0x98, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x0b, 0xf8, 0x00, 0x28, 0x01, 0xd0, 0xff,
         0xf7, 0x80, 0xff, 0x00, 0x20, 0xc0, 0x46, 0xc0, 0x46, 0x00, 0xf0, 0x05, 0xf8, 0x00, 0xf0,
         0x0b, 0xf8, 0x01, 0x20, 0x70, 0x47, 0x00, 0x00, 0x80, 0xb5, 0x02, 0x48, 0xff, 0xf7, 0x86,
         0xff, 0x00, 0x20, 0x02, 0xbd, 0x90, 0x04, 0x00, 0x00, 0x80, 0xb5, 0x00, 0xf0, 0x01, 0xf8,
         0x01, 0xbd, 0x07, 0x46, 0x38, 0x46, 0x00, 0xf0, 0x02, 0xf8, 0xfb, 0xe7, 0x00, 0x00, 0x80,
         0xb5, 0xff, 0xf7, 0x0f, 0xff, 0x02, 0x4a, 0x11, 0x00, 0x18, 0x20, 0xab, 0xbe, 0xfb, 0xe7,
         0x26, 0x00, 0x02, 0x00, 0x38, 0xb5, 0x05, 0x00, 0x0c, 0x00, 0x20, 0x00, 0xff, 0xf7, 0x7a,
         0xff, 0xa0, 0x42, 0x00, 0xd0, 0x00, 0x25, 0x28, 0x00, 0x32, 0xbd, 0x00, 0x00, 0x68, 0x65,
         0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x0a, 0x00, 0x00, 0x00, 0xc0,
         0x46, 0xc0, 0x46, 0xc0, 0x46, 0xc0, 0x46, 0xff, 0xf7, 0xba, 0xff, 0xff, 0xff, 0xff, 0xff,
         0xff, 0xff, 0xff, 0xff];

    let mut hellow = ConstMemory::new(&hellow_bin);
    run_bin(&mut hellow);

}
