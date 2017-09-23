use bit_field::BitField;

use instruction::Op;
use register::Reg;
use register::Apsr;
use core::Core;
use memory::Fetch;

use operation::sign_extend;
use operation::add_with_carry;
use operation::condition_passed;

fn read_reg<T: Fetch>(core: &mut Core<T>, r: Reg) -> u32 {
    match r {
        Reg::PC => core.r[r.value()] + 4,
        _ => core.r[r.value()],
    }
}

pub fn execute<T: Fetch>(core: &mut Core<T>, op: Option<Op>) {
    match op {
        None => panic!("undefined code"),
        Some(oper) => {
            match oper {
                Op::MOV { rd, rm } => {
                    println!("MOV");
                    core.r[rd.value() as usize] = read_reg(core, rm);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::BL { imm32 } => {
                    println!("BL");
                    let pc = read_reg(core, Reg::PC);
                    core.r[Reg::LR.value()] = pc | 0x01;
                    core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                }
                Op::BX { rm } => {
                    println!("BX");
                    core.r[Reg::PC.value()] = read_reg(core, rm) & 0xfffffffe;
                }
                Op::BLX { rm } => {
                    println!("BLX");
                    let pc = read_reg(core, Reg::PC);
                    core.r[Reg::LR.value()] = (pc - 2) | 0x01;
                    core.r[Reg::PC.value()] = ((pc as i32) + (rm.value() as i32)) as u32;
                }
                Op::MOV_imm8 { rd, imm8 } => {
                    println!("MOV_imm8");
                    core.r[rd.value()] = imm8 as u32;
                    core.r[Reg::PC.value()] += 2;
                }
                Op::B_imm8 { cond, imm8 } => {
                    println!("B_imm8");
                    let imm32 = sign_extend((imm8 as u32) << 1, 8, 32);
                    if condition_passed(cond, &core.apsr) {
                        let pc = read_reg(core, Reg::PC);
                        core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                    } else {
                        core.r[Reg::PC.value()] += 2;
                    }
                }
                Op::B_imm11 { imm11 } => {
                    println!("B_imm11");
                    let pc = read_reg(core, Reg::PC);
                    let imm32 = sign_extend((imm11 as u32) << 1, 11, 32);
                    core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                }

                Op::CMP_imm8 { rn, imm8 } => {
                    println!("CMP_imm8");
                    let imm32 = imm8 as u32;
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32 ^ 0xFFFFFFFF, true);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::CMP { rn, rm } => {
                    println!("CMP");
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), read_reg(core, rm) ^ 0xFFFFFFFF, true);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }

                Op::PUSH { registers } => {
                    println!("PUSH");
                    let address = core.msp - 4 * (registers.len() as u32);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::LDR { rt, imm8 } => {
                    println!("LDR");
                    let imm32 = (imm8 as u32) << 2;
                    let pc = read_reg(core, Reg::PC);
                    let address = (pc & 0xfffffffc) + imm32;
                    core.r[rt.value()] = core.memory.fetch32(address);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::ADD { rdn, rm } => {

                    println!("ADD");
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rdn), read_reg(core, rm), false);
                    core.r[rdn.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Op::ADDS_imm { rn, rd, imm32 } => {
                    println!("ADDS_imm R{:x}, R{:x}, #{:x}", rn.value(), rd.value(), imm32);
                    let r_n = read_reg(core, rn);
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32 as u32, false);

                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;

                }
                Op::ADDS { rm, rn, rd } => {
                    println!("ADDS");
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), read_reg(core, rm), false);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;

                }

                _ => panic!("unimplemented instruction") ,
            }
        }
    }
}
