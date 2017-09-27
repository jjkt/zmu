use bit_field::BitField;

use instruction::Op;
use register::Reg;
use register::Apsr;
use core::Core;
use memory::Fetch;

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
                Op::MOV_reg { rd, rm, setflags } => {
                    println!("MOV {:?},{:?}", rd, rm);
                    let result = read_reg(core, rm);
                    core.r[rd.value() as usize] = result;

                    if rd != Reg::PC {
                        if setflags {
                            core.apsr.set_n(result.get_bit(31));
                            core.apsr.set_z(result == 0);
                        }
                        core.r[Reg::PC.value()] += 2;
                    }
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
                    println!("BLX R{}", rm.value());
                    let pc = read_reg(core, Reg::PC);
                    core.r[Reg::LR.value()] = (((pc - 2) >> 1) << 1) | 1;
                    core.r[Reg::PC.value()] = read_reg(core, rm) & 0xfffffffe;
                }
                Op::MOV_imm { rd, imm32 } => {
                    println!("MOV_imm {:?}, {}", rd, imm32);
                    core.r[rd.value()] = imm32 as u32;
                    core.r[Reg::PC.value()] += 2;
                }
                Op::B { cond, imm32 } => {
                    println!("B{:?} {}", cond, imm32);
                    if condition_passed(cond, &core.apsr) {
                        let pc = read_reg(core, Reg::PC);
                        core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                    } else {
                        core.r[Reg::PC.value()] += 2;
                    }
                }

                Op::CMP_imm { rn, imm32 } => {
                    println!("CMP_imm {:?}, {}", rn, imm32);
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32 ^ 0xFFFFFFFF, true);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::CMP { rn, rm } => {
                    println!("CMP {:?}, {:?}", rn, rm);
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
                Op::LDR_imm { rt, rn, imm32 } => {
                    println!("LDR R{}, R{}, {}", rt.value(), rn.value(), imm32);
                    let address = read_reg(core, rn) + imm32;
                    core.r[rt.value()] = core.memory.fetch32(address & 0xfffffffc);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::LDR_lit { rt, imm32 } => {
                    println!("LDR R{},[PC, #{}]", rt.value(), imm32);
                    let base = read_reg(core, Reg::PC) & 0xfffffffc;
                    core.r[rt.value()] = core.memory.fetch32(base + imm32);
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
                    println!("ADDS_imm R{:x}, R{:x}, #{}", rn.value(), rd.value(), imm32);
                    let r_n = read_reg(core, rn);
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32, false);

                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;

                }
                Op::ADDS { rm, rn, rd } => {
                    println!("ADDS R{}, R{}, R{}", rn.value(), rd.value(), rm.value());
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), read_reg(core, rm), false);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;

                }

                _ => panic!("unimplemented instruction"),
            }
        }
    }
}
