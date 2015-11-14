#![allow(dead_code)]
#![allow(unused_variables)]
// http://ecee.colorado.
// edu/ecen3000/labs/lab3/files/DDI0419C_arm_architecture_v6m_reference_manual.
// pdf
use std::io::Cursor;
use std::io::SeekFrom;
use std::io::Seek;

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};


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

pub struct APSR {
    value: u32,
}

impl ApsrBits for APSR {

    fn set_n(&mut self, n: bool) {
        self.value = match n {
            true => self.value | (1 << 31),
            false => self.value & !(1 << 31),
        }
    }

    fn set_z(&mut self, z: bool) {
        self.value = match z {
            true => self.value | (1 << 30),
            false => self.value & !(1 << 30),
        }
    }

    fn set_c(&mut self, c: bool) {
        self.value = match c {
            true => self.value | (1 << 29),
            false => self.value & !(1 << 29),
        }
    }

    fn set_v(&mut self, v: bool) {
        self.value = match v {
            true => self.value | (1 << 28),
            false => self.value & !(1 << 28),
        }
    }

    fn get_n(&self) -> bool {
        (self.value & 31) != 0
    }
    fn get_z(&self) -> bool {
        (self.value & 30) != 0
    }
    fn get_c(&self) -> bool {
        (self.value & 29) != 0
    }
    fn get_v(&self) -> bool {
        (self.value & 28) != 0
    }

}



pub struct Core {
    pc: u32,
    lr: u32,
    sp: StackPointer,
    r: [u32; 15],
    psr: u32,
    apsr: APSR,
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
            lr: 0,
            sp: StackPointer::MSP(0),
            psr: 0,
            apsr: APSR { value: 0 },
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

pub trait ApsrBits {

    fn set_n(&mut self, n: bool);
    fn set_z(&mut self, z: bool);
    fn set_c(&mut self, c: bool);
    fn set_v(&mut self, v: bool);

    fn get_n(&self) -> bool;
    fn get_z(&self) -> bool;
    fn get_c(&self) -> bool;
    fn get_v(&self) -> bool;

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
    R13,
    R14, // LR
    R15,
}

fn from_u8(n: u8) -> Option<Reg> {
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
        13 => Some(Reg::R13),
        14 => Some(Reg::R14),
        15 => Some(Reg::R15),
        _ => None,
    }
}

fn to_u8(reg: Reg) -> u8 {
    match reg {
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
        Reg::R13 => 13,
        Reg::R14 => 14,
        Reg::R15 => 15,
    }
}

pub enum Op {
    MOV {
        rd: Reg,
        rm: Reg,
    },
    MOVS {
        rd: Reg,
        rm: Reg,
    },
    MOVS_imm {
        rd: Reg,
        imm8: u8,
    },
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
    CMP_imm {
        rn: Reg,
        imm8: u8,
    },
    CMN,
    ORR,
    MUL,
    BIC,
    MVN,
    ADD,
    BX {
        rd: Reg,
    },
    BLX,
    SUB,
    BL {
        offset: i32,
    },
}


pub fn decode_16(command: u16) -> Option<Op> {
    println!("decoding {:x}", command);
    match command & 0xc000 {
        0b0000_0000_0000_0000_u16 => {
            // Shift, add, substract, move and compare
            match command & 0x3800 {
                0b00000_00_0_00000000 => Some(Op::LSL),
                0b00001_00_0_00000000 => Some(Op::LSR),
                0b00010_00_0_00000000 => Some(Op::ASR),
                0b00011_00_0_00000000 => None, //ADD, SUB, ADD 3-bit, SUB 3-bit immediate
                0b001_00_000_00000000 => Some(Op::MOVS_imm {
                    rd: from_u8((bits16(command, 8, 3) & 7 as u16) as u8).unwrap(),
                    imm8: (command & 0xff) as u8,
                }),
                0b00101_00_0_00000000 => Some(Op::CMP_imm {
                    rn: from_u8((bits16(command, 8, 3) & 7 as u16) as u8).unwrap(),
                    imm8: (command & 0xff) as u8,
                }),
                0b00110_00_0_00000000 => Some(Op::ADD),
                0b00111_00_0_00000000 => Some(Op::SUB),
                _ => None,
            }
        }
        0b0100_0000_0000_0000_u16 => match command & 0xffc0 {
            0b010000_0000_000000 => Some(Op::AND),
            0b010000_0001_000000 => Some(Op::EOR),
            0b010000_0010_000000 => Some(Op::LSL),
            0b010000_0011_000000 => Some(Op::LSR),
            0b010000_0100_000000 => Some(Op::ASR),
            0b010000_0101_000000 => Some(Op::ADC),
            0b010000_0110_000000 => Some(Op::SBC),
            0b010000_0111_000000 => Some(Op::ROR),
            0b010000_1000_000000 => Some(Op::TST),
            0b010000_1001_000000 => Some(Op::RSB),
            0b010000_1010_000000 => Some(Op::CMP),
            0b010000_1011_000000 => Some(Op::CMN),
            0b010000_1100_000000 => Some(Op::ORR),
            0b010000_1101_000000 => Some(Op::MUL),
            0b010000_1110_000000 => Some(Op::BIC),
            0b010000_1111_000000 => Some(Op::MVN),

            0b010001_0000_000000 => Some(Op::ADD),
            0b010001_0100_000000 => None,
            0b010001_0101_000000 |
            0b010001_0110_000000 |
            0b010001_0111_000000 => Some(Op::CMP),
            0b010001_1000_000000 |
            0b010001_1001_000000 |
            0b010001_1010_000000 |
            0b010001_1011_000000 => Some(Op::MOV {
                rd: from_u8(((command & 8) + ((command & 0x80)) >> 4) as u8).unwrap(),
                rm: from_u8(((command >> 3) & 0xf) as u8).unwrap(),
            }),
            0b010001_1100_000000 |
            0b010001_1101_000000 =>
                Some(Op::BX { rd: from_u8(bits16(command, 3, 4) as u8).unwrap() }),
            0b010001_1110_000000 |
            0b010001_1111_000000 => Some(Op::BLX),
            _ => None,

        },
        0b1000_0000_0000_0000_u16 => {
            // generate pc relative addr, sp rela, misc
            None
        }
        0b1100_0000_0000_0000_u16 => {
            // store, load multiple, branch, svc, uncond branch
            None
        }
        _ => None,
    }
}

fn bits16(input: u16, pos: u16, width: u16) -> u16 {

    (input & (((1 << width) - 1) << pos)) >> pos
}

fn bits32(input: u32, pos: u8, width: u8) -> u32 {

    (input & (((1 << width) - 1) << pos)) >> pos
}

#[test]
fn test_bits16() {
    assert!(bits16(0x0001, 0, 1) == 1);
    assert!(bits16(0xffff, 13, 1) == 1);
    assert!(bits16(0xff00, 8, 8) == 0xff);
    assert!(bits16(0x0ab0, 4, 8) == 0xab);
}

fn sign_extend(input: u32, pos: u8) -> i32 {
    match input & (1 << pos) {
        0 => input as i32,
        _ => (input | (bits32(0xffffffff, pos, 32 - pos) << pos)) as i32,
    }

}

pub fn decode_32(t1: u16, t2: u16) -> Option<Op> {
    println!("decoding {:x} {:x}", t1, t2);
    match t1 & 0b1111000000000000 {
        0b1111000000000000 => {
            match t2 & 0b1111000000000000 {
                0b1111000000000000 | 0b1101000000000000 => {
                    Some(Op::BL {
                        offset: {
                            let s = bits16(t1, 10, 1) as u32;
                            let imm10 = bits16(t1, 0, 10) as u32;
                            let j1 = bits16(t2, 13, 1) as u32;
                            let j2 = bits16(t2, 11, 1) as u32;
                            let imm11 = bits16(t2, 0, 11) as u32;
                            let i1 = (!((j1 ^ s) != 0)) as u32;
                            let i2 = (!((j2 ^ s) != 0)) as u32;
                            let imm32 = (s << 24) + (i1 << 23) + (i2 << 22) + (imm10 << 12) +
                                        (imm11 << 1);
                            sign_extend(imm32, 24)
                        },
                    })
                }
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn execute(core: &mut Core, op: Option<Op>) {
    match op {
        None => {
            panic!("unknown operation");
        }
        Some(oper) => {
            match oper {
                Op::MOV{rd,rm} => {
                    core.r[to_u8(rd) as usize] = core.r[to_u8(rm) as usize];
                    core.pc = core.pc + 2;
                }
                Op::MOVS_imm{rd,imm8} => {
                    let imm32 = imm8 as u32;
                    core.pc = core.pc + 2;
                    core.r[to_u8(rd) as usize] = imm32;
                    core.apsr.set_n((imm32 & (1 << 31)) != 0);
                    core.apsr.set_z(imm32 == 0);
                }
                Op::BL{offset} => {
                    core.r[to_u8(Reg::R14) as usize] = core.pc + 4;
                    core.pc = 4 + ((core.pc as i32) + offset) as u32;

                }
                Op::BX{rd} => {
                    core.pc = core.r[to_u8(rd) as usize];
                }
                _ => {
                    panic!("unimplemented op");
                }
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
        false => {
            decode_16(hw)
        }

    }
}

pub fn run_bin<T: Fetch>(memory: &mut T) {
    let mut core = Core::new();
    let reset_vector = memory.fetch32(4);

    core.pc = reset_vector & 0xfffffffe;
    core.epsr = (reset_vector & 1) << 24;
    let sp = memory.fetch32(0);
    core.sp = StackPointer::MSP(sp);

    loop {
        println!("pc = {}, apsr = {}", core.pc, core.apsr.value);
        let pc = core.pc;
        let op = fetch_and_decode(memory, pc);
        execute(&mut core, op);
    }
}

struct ConstMemory<'a> {
    reader: Cursor<&'a [u8]>,
}

impl<'a> ConstMemory<'a>{
    pub fn new(bin: &[u8]) -> ConstMemory {
        ConstMemory { reader: Cursor::new(bin) }
    }
}

impl<'a> Fetch for ConstMemory<'a>
{
    fn fetch16(&mut self, addr: u32) -> u16 {
        self.reader.seek(SeekFrom::Start(addr as u64));
        self.reader.read_u16::<LittleEndian>().unwrap()
    }

    fn fetch32(&mut self, addr: u32) -> u32 {
        self.reader.seek(SeekFrom::Start(addr as u64));
        self.reader.read_u32::<LittleEndian>().unwrap()
    }
}


// ---------------------------------------------------------
// TESTS BELOW
// ---------------------------------------------------------

#[test]
fn test_is_thumb32() {
    assert!(is_thumb32(0b1110100000000000));
    assert!(is_thumb32(0b1111000000000000));
    assert!(is_thumb32(0b1111100000000000));
    assert!(is_thumb32(0b1110000000000000) == false);
    assert!(is_thumb32(0b1111111111111111));
    assert!(is_thumb32(0b0000000000000001) == false);
    assert!(is_thumb32(0xf7ff));
}

#[test]
fn test_decode_thumb16() {
    // mov
    match decode_16(0x4600).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R0);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4608).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R1);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4610).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R2);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4618).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R3);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4620).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R4);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4628).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R5);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4630).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R6);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4638).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R7);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4640).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R8);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4648).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R9);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4650).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R10);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4658).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R11);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4660).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R12);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4668).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R13);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4670).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R14);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x4678).unwrap() {
        Op::MOV {rd,rm} => {
            assert!(rd == Reg::R0);
            assert!(rm == Reg::R15);
        }
        _ => {
            assert!(false);
        }
    }

    match decode_16(0x2001).unwrap() {
        Op::MOVS_imm {rd,imm8} => {
            assert!(rd == Reg::R0);
            assert!(imm8 == 1);
        }
        _ => {
            assert!(false);
        }
    }

    match decode_16(0x4770).unwrap() {
        Op::BX {rd} => {
            assert!(rd == Reg::R14);
        }
        _ => {
            assert!(false);
        }
    }
    match decode_16(0x2800).unwrap() {
        Op::CMP_imm {rn, imm8} => {
            assert!(rn == Reg::R0);
            assert!(imm8 == 0);
        }
        _ => {
            assert!(false);
        }
    }

}

#[test]
fn test_decode_thumb32() {
    match decode_32(0xf7ff, 0xffba).unwrap() {
        Op::BL{offset} => {
            assert!(offset == -140);
        }
        _ => {
            assert!(false);
        }
    }

    match decode_32(0xf000, 0xf80b).unwrap() {
        Op::BL{offset} => {
            println!("offset = {}\n", offset);
            assert!(offset == 22);
        }
        _ => {
            assert!(false);
        }

    }

}
#[test]
fn test_hello_world() {
    let hellow_bin: [u8; 1204] = [0x08, 0x04, 0x00, 0x20, 0xa1, 0x04, 0x00, 0x00, 0x83, 0x02,
                                  0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00,
                                  0x83, 0x02, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x00, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00, 0x83, 0x02,
                                  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x83, 0x02, 0x00, 0x00,
                                  0x83, 0x02, 0x00, 0x00, 0x70, 0xb5, 0x06, 0x00, 0x08, 0x00,
                                  0x14, 0x00, 0x00, 0x2b, 0x05, 0xd0, 0x1d, 0x00, 0x21, 0x78,
                                  0xb0, 0x47, 0x64, 0x1c, 0x6d, 0x1e, 0xfa, 0xd1, 0x70, 0xbd,
                                  0xfd, 0xb5, 0x86, 0xb0, 0x0c, 0x00, 0x09, 0xe0, 0x01, 0x23,
                                  0x07, 0x9a, 0x21, 0x00, 0x06, 0x98, 0xff, 0xf7, 0xe9, 0xff,
                                  0x04, 0x00, 0x07, 0x98, 0x40, 0x1c, 0x07, 0x90, 0x07, 0x98,
                                  0x00, 0x78, 0x00, 0x28, 0x01, 0xd1, 0x09, 0xb0, 0xf0, 0xbd,
                                  0x25, 0x28, 0xed, 0xd1, 0x07, 0x98, 0x46, 0x78, 0x80, 0x1c,
                                  0x07, 0x90, 0x25, 0x2e, 0x00, 0xd1, 0x95, 0xe0, 0x58, 0x2e,
                                  0x69, 0xd0, 0x63, 0x2e, 0x0e, 0xd0, 0x64, 0x2e, 0x1a, 0xd0,
                                  0x69, 0x2e, 0x18, 0xd0, 0x6f, 0x2e, 0x61, 0xd0, 0x70, 0x2e,
                                  0x68, 0xd0, 0x73, 0x2e, 0x70, 0xd0, 0x75, 0x2e, 0x5b, 0xd0,
                                  0x78, 0x2e, 0x59, 0xd0, 0xdd, 0xe7, 0x08, 0x98, 0x00, 0x68,
                                  0x08, 0x99, 0x08, 0x60, 0x01, 0x68, 0x00, 0x1d, 0x08, 0x9a,
                                  0x10, 0x60, 0xc9, 0xb2, 0x20, 0x00, 0x06, 0x9a, 0x90, 0x47,
                                  0x04, 0x00, 0xcf, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99,
                                  0x08, 0x60, 0x05, 0x68, 0x00, 0x1d, 0x08, 0x99, 0x08, 0x60,
                                  0x00, 0x2d, 0x04, 0xd5, 0x2d, 0x21, 0x20, 0x00, 0x06, 0x9a,
                                  0x90, 0x47, 0x04, 0x00, 0x64, 0x26, 0x6f, 0x2e, 0x01, 0xd1,
                                  0x08, 0x21, 0x06, 0xe0, 0x20, 0x21, 0x31, 0x43, 0x78, 0x29,
                                  0x01, 0xd0, 0x0a, 0x21, 0x00, 0xe0, 0x10, 0x21, 0x01, 0x91,
                                  0x0b, 0x21, 0x00, 0x91, 0x2f, 0x00, 0x64, 0x2e, 0x03, 0xd1,
                                  0x00, 0x2d, 0x01, 0xd5, 0x78, 0x42, 0x07, 0x00, 0x00, 0x9d,
                                  0x6d, 0x1e, 0x00, 0x95, 0x38, 0x00, 0x01, 0x99, 0x00, 0xf0,
                                  0x4b, 0xf8, 0x30, 0x31, 0xc9, 0xb2, 0x3a, 0x29, 0x02, 0xdb,
                                  0x30, 0x00, 0x51, 0x38, 0x09, 0x18, 0x02, 0xa8, 0x45, 0x19,
                                  0x29, 0x70, 0x38, 0x00, 0x01, 0x99, 0x00, 0xf0, 0x3d, 0xf8,
                                  0x07, 0x00, 0x02, 0xd0, 0x00, 0x98, 0x01, 0x28, 0xe5, 0xda,
                                  0x20, 0x00, 0x0b, 0x21, 0x00, 0x9a, 0x8c, 0x1a, 0xb9, 0xd0,
                                  0x29, 0x78, 0x06, 0x9a, 0x90, 0x47, 0x6d, 0x1c, 0x64, 0x1e,
                                  0xf9, 0xd1, 0xb2, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99,
                                  0x08, 0x60, 0x05, 0x68, 0x00, 0x1d, 0x08, 0x99, 0x08, 0x60,
                                  0xbb, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60,
                                  0x05, 0x68, 0x00, 0x1d, 0x08, 0x99, 0x08, 0x60, 0x78, 0x26,
                                  0xb1, 0xe7, 0x08, 0x98, 0x00, 0x68, 0x08, 0x99, 0x08, 0x60,
                                  0x05, 0x68, 0x00, 0x1d, 0x08, 0x99, 0x08, 0x60, 0x28, 0x00,
                                  0x00, 0xf0, 0x65, 0xf8, 0x01, 0x00, 0x20, 0x00, 0x00, 0x29,
                                  0x90, 0xd0, 0x0c, 0x00, 0x29, 0x78, 0x06, 0x9a, 0x90, 0x47,
                                  0x6d, 0x1c, 0x64, 0x1e, 0xf9, 0xd1, 0x88, 0xe7, 0x25, 0x21,
                                  0x83, 0xe7, 0x00, 0x22, 0x03, 0x0a, 0x8b, 0x42, 0x0b, 0xd2,
                                  0x03, 0x09, 0x8b, 0x42, 0x19, 0xd2, 0x43, 0x08, 0x8b, 0x42,
                                  0x2e, 0xd2, 0x41, 0x1a, 0x00, 0xd2, 0x01, 0x46, 0x52, 0x41,
                                  0x10, 0x46, 0x70, 0x47, 0xff, 0x22, 0x09, 0x02, 0x3f, 0xd0,
                                  0x12, 0x06, 0x8b, 0x42, 0x05, 0xd3, 0x12, 0x12, 0x09, 0x02,
                                  0x8b, 0x42, 0x01, 0xd3, 0x12, 0x12, 0x09, 0x02, 0x03, 0x09,
                                  0x8b, 0x42, 0x19, 0xd3, 0x00, 0xe0, 0x09, 0x0a, 0xc3, 0x09,
                                  0x8b, 0x42, 0x01, 0xd3, 0xcb, 0x01, 0xc0, 0x1a, 0x52, 0x41,
                                  0x83, 0x09, 0x8b, 0x42, 0x01, 0xd3, 0x8b, 0x01, 0xc0, 0x1a,
                                  0x52, 0x41, 0x43, 0x09, 0x8b, 0x42, 0x01, 0xd3, 0x4b, 0x01,
                                  0xc0, 0x1a, 0x52, 0x41, 0x03, 0x09, 0x8b, 0x42, 0x01, 0xd3,
                                  0x0b, 0x01, 0xc0, 0x1a, 0x52, 0x41, 0xc3, 0x08, 0x8b, 0x42,
                                  0x01, 0xd3, 0xcb, 0x00, 0xc0, 0x1a, 0x52, 0x41, 0x83, 0x08,
                                  0x8b, 0x42, 0x01, 0xd3, 0x8b, 0x00, 0xc0, 0x1a, 0x52, 0x41,
                                  0x43, 0x08, 0x8b, 0x42, 0x01, 0xd3, 0x4b, 0x00, 0xc0, 0x1a,
                                  0x52, 0x41, 0x88, 0x42, 0x00, 0xd3, 0x40, 0x1a, 0x52, 0x41,
                                  0xcf, 0xd2, 0x01, 0x46, 0x10, 0x46, 0x70, 0x47, 0x08, 0xb5,
                                  0x00, 0xf0, 0x0a, 0xf8, 0x08, 0xbd, 0x01, 0x00, 0x00, 0xe0,
                                  0x49, 0x1c, 0x0a, 0x78, 0x00, 0x2a, 0xfb, 0xd1, 0x08, 0x1a,
                                  0x70, 0x47, 0xfe, 0xe7, 0x70, 0x47, 0x00, 0x00, 0x80, 0xb5,
                                  0x00, 0xf0, 0x33, 0xf8, 0x02, 0x00, 0x00, 0x23, 0xdb, 0x43,
                                  0x10, 0x68, 0x98, 0x42, 0x04, 0xd0, 0x11, 0x00, 0x02, 0x20,
                                  0xab, 0xbe, 0x00, 0x20, 0x10, 0x60, 0x50, 0x68, 0x98, 0x42,
                                  0x04, 0xd0, 0x11, 0x1d, 0x02, 0x20, 0xab, 0xbe, 0x00, 0x20,
                                  0x50, 0x60, 0x01, 0xbd, 0x00, 0x00, 0x10, 0xb5, 0x84, 0xb0,
                                  0x04, 0x00, 0x00, 0xf0, 0x19, 0xf8, 0xa1, 0x00, 0x42, 0x18,
                                  0x10, 0x68, 0x00, 0x21, 0xc9, 0x43, 0x88, 0x42, 0x0d, 0xd1,
                                  0x07, 0xa0, 0x00, 0x90, 0x00, 0x2c, 0x01, 0xd1, 0x00, 0x20,
                                  0x00, 0xe0, 0x04, 0x20, 0x01, 0x90, 0x03, 0x20, 0x02, 0x90,
                                  0x69, 0x46, 0x01, 0x20, 0xab, 0xbe, 0x10, 0x60, 0x04, 0xb0,
                                  0x10, 0xbd, 0x3a, 0x74, 0x74, 0x00, 0x00, 0x48, 0x70, 0x47,
                                  0x00, 0x00, 0x00, 0x20, 0x30, 0xb4, 0x01, 0x21, 0x02, 0x68,
                                  0x00, 0x1d, 0x00, 0x2a, 0x0f, 0xd0, 0x03, 0x68, 0xc3, 0x18,
                                  0x44, 0x68, 0x08, 0x30, 0x0c, 0x42, 0x02, 0xd0, 0x4d, 0x46,
                                  0x6d, 0x1e, 0x64, 0x19, 0x1d, 0x68, 0x25, 0x60, 0x1b, 0x1d,
                                  0x24, 0x1d, 0x12, 0x1f, 0xec, 0xd0, 0xf8, 0xe7, 0x30, 0xbc,
                                  0x70, 0x47, 0x10, 0xb5, 0x07, 0x49, 0x79, 0x44, 0x18, 0x31,
                                  0x06, 0x4c, 0x7c, 0x44, 0x16, 0x34, 0x04, 0xe0, 0x08, 0x1d,
                                  0x0a, 0x68, 0x89, 0x18, 0x88, 0x47, 0x01, 0x00, 0xa1, 0x42,
                                  0xf8, 0xd1, 0x10, 0xbd, 0xc0, 0x00, 0x00, 0x00, 0xd0, 0x00,
                                  0x00, 0x00, 0x0e, 0xb4, 0x00, 0xb5, 0x82, 0xb0, 0x03, 0xa9,
                                  0x00, 0x91, 0x6b, 0x46, 0x02, 0x00, 0x01, 0x21, 0x03, 0x48,
                                  0x78, 0x44, 0x0a, 0x30, 0xff, 0xf7, 0x76, 0xfe, 0x02, 0x99,
                                  0x06, 0xb0, 0x08, 0x47, 0x05, 0x01, 0x00, 0x00, 0x38, 0xb5,
                                  0x04, 0x00, 0x00, 0x25, 0xed, 0x43, 0xac, 0x42, 0x09, 0xd0,
                                  0x69, 0x46, 0x08, 0x70, 0x01, 0x22, 0x01, 0x20, 0x00, 0xf0,
                                  0x06, 0xf8, 0x01, 0x28, 0x01, 0xd1, 0x20, 0x00, 0x32, 0xbd,
                                  0x28, 0x00, 0x32, 0xbd, 0x80, 0xb5, 0x00, 0x28, 0x02, 0xd4,
                                  0x00, 0xf0, 0x03, 0xf8, 0x02, 0xbd, 0x00, 0x20, 0x02, 0xbd,
                                  0x80, 0xb5, 0x00, 0x29, 0x01, 0xd1, 0x00, 0x20, 0x02, 0xbd,
                                  0x01, 0x28, 0x01, 0xd0, 0x02, 0x28, 0x02, 0xd1, 0x00, 0xf0,
                                  0x04, 0xf8, 0x02, 0xbd, 0x00, 0xf0, 0x11, 0xf8, 0x02, 0xbd,
                                  0x30, 0xb5, 0x83, 0xb0, 0x0c, 0x00, 0x15, 0x00, 0x01, 0x20,
                                  0xff, 0xf7, 0x70, 0xff, 0x00, 0x90, 0x01, 0x94, 0x02, 0x95,
                                  0x69, 0x46, 0x05, 0x20, 0xab, 0xbe, 0x28, 0x1a, 0x03, 0xb0,
                                  0x30, 0xbd, 0x10, 0xb5, 0x84, 0xb0, 0x03, 0x00, 0x14, 0x00,
                                  0x00, 0x93, 0x01, 0x91, 0x02, 0x94, 0x69, 0x46, 0x05, 0x20,
                                  0xab, 0xbe, 0x01, 0x22, 0x19, 0x00, 0x20, 0x1a, 0xc0, 0x46,
                                  0xc0, 0x46, 0x04, 0xb0, 0x10, 0xbd, 0xf1, 0xfe, 0xff, 0xff,
                                  0x08, 0x00, 0x00, 0x00, 0x98, 0x00, 0x00, 0x00, 0x00, 0x00,
                                  0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x0b, 0xf8,
                                  0x00, 0x28, 0x01, 0xd0, 0xff, 0xf7, 0x80, 0xff, 0x00, 0x20,
                                  0xc0, 0x46, 0xc0, 0x46, 0x00, 0xf0, 0x05, 0xf8, 0x00, 0xf0,
                                  0x0b, 0xf8, 0x01, 0x20, 0x70, 0x47, 0x00, 0x00, 0x80, 0xb5,
                                  0x02, 0x48, 0xff, 0xf7, 0x86, 0xff, 0x00, 0x20, 0x02, 0xbd,
                                  0x90, 0x04, 0x00, 0x00, 0x80, 0xb5, 0x00, 0xf0, 0x01, 0xf8,
                                  0x01, 0xbd, 0x07, 0x46, 0x38, 0x46, 0x00, 0xf0, 0x02, 0xf8,
                                  0xfb, 0xe7, 0x00, 0x00, 0x80, 0xb5, 0xff, 0xf7, 0x0f, 0xff,
                                  0x02, 0x4a, 0x11, 0x00, 0x18, 0x20, 0xab, 0xbe, 0xfb, 0xe7,
                                  0x26, 0x00, 0x02, 0x00, 0x38, 0xb5, 0x05, 0x00, 0x0c, 0x00,
                                  0x20, 0x00, 0xff, 0xf7, 0x7a, 0xff, 0xa0, 0x42, 0x00, 0xd0,
                                  0x00, 0x25, 0x28, 0x00, 0x32, 0xbd, 0x00, 0x00, 0x68, 0x65,
                                  0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64,
                                  0x0a, 0x00, 0x00, 0x00, 0xc0, 0x46, 0xc0, 0x46, 0xc0, 0x46,
                                  0xc0, 0x46, 0xff, 0xf7, 0xba, 0xff, 0xff, 0xff, 0xff, 0xff,
                                  0xff, 0xff, 0xff, 0xff];

    let mut hellow = ConstMemory::new(&hellow_bin);
    run_bin(&mut hellow);

}
