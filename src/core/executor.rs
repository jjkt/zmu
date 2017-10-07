use bit_field::BitField;
use core::instruction::Instruction;
use core::register::Reg;
use core::register::Apsr;
use core::Core;
use bus::Bus;

use core::operation::add_with_carry;
use core::operation::condition_passed;

fn read_reg<T: Bus>(core: &mut Core<T>, r: Reg) -> u32 {
    match r {
        Reg::PC => core.r[r.value()] + 4,
        _ => core.r[r.value()],
    }
}

pub fn execute<T: Bus>(core: &mut Core<T>, op: Option<Instruction>) {
    match op {
        None => panic!("undefined code"),
        Some(oper) => {
            println!("{}", oper);
            match oper {
                Instruction::MOV_reg { rd, rm, setflags } => {
                    let result = read_reg(core, rm);
                    core.r[rd.value() as usize] = result;

                    if rd != Reg::PC {
                        if setflags {
                            core.psr.set_n(result.get_bit(31));
                            core.psr.set_z(result == 0);
                        }
                        core.r[Reg::PC.value()] += 2;
                    }
                }
                Instruction::BL { imm32 } => {
                    let pc = read_reg(core, Reg::PC);
                    core.r[Reg::LR.value()] = pc | 0x01;
                    core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                }
                Instruction::BX { rm } => {
                    core.r[Reg::PC.value()] = read_reg(core, rm) & 0xfffffffe;
                }
                Instruction::BLX { rm } => {
                    let pc = read_reg(core, Reg::PC);
                    core.r[Reg::LR.value()] = (((pc - 2) >> 1) << 1) | 1;
                    core.r[Reg::PC.value()] = read_reg(core, rm) & 0xfffffffe;
                }
                Instruction::MOV_imm { rd, imm32 } => {
                    core.r[rd.value()] = imm32 as u32;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::B { cond, imm32 } => {
                    if condition_passed(cond, &core.psr) {
                        let pc = read_reg(core, Reg::PC);
                        core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                    } else {
                        core.r[Reg::PC.value()] += 2;
                    }
                }

                Instruction::CMP_imm { rn, imm32 } => {
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32 ^ 0xFFFFFFFF, true);
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::CMP { rn, rm } => {
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), read_reg(core, rm) ^ 0xFFFFFFFF, true);
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }

                Instruction::PUSH { registers } => {

                    let regs_size = 4 * (registers.len() as u32);
                    let sp = core.get_sp();
                    let mut address = sp - regs_size;

                    for reg in registers.iter() {
                        println!("pushing reg {} (address 0x{:x}) ", reg, address);
                        core.bus.write32(address, core.r[reg.value()]);
                        address += 4;
                    }

                    core.set_sp(sp - regs_size);
                    core.r[Reg::PC.value()] += 2;
                }
                
                Instruction::POP { registers } => {
                    let regs_size = 4 * (registers.len() as u32);
                    let sp = core.get_sp();
                    let mut address = sp;

                    for reg in registers.iter() {
                        println!("popping reg {} (address 0x{:x}) ", reg, address);
                        core.r[reg.value()] = core.bus.read32(address);
                        address += 4;
                    }

                    core.set_sp(sp + regs_size);
                    if registers.contains(&Reg::PC) == false {
                        core.r[Reg::PC.value()] += 2;
                    }

                }

                Instruction::LDR_imm { rt, rn, imm32 } => {
                    let address = read_reg(core, rn) + imm32;
                    core.r[rt.value()] = core.bus.read32(address & 0xfffffffc);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::STR_imm { rt, rn, imm32 } => {
                    let address = (read_reg(core, rn) + imm32) & 0xfffffffc;
                    let value = read_reg(core, rt);
                    core.bus.write32(address, value);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::LDR_lit { rt, imm32 } => {
                    let base = read_reg(core, Reg::PC) & 0xfffffffc;
                    core.r[rt.value()] = core.bus.read32(base + imm32);
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::ADD { rdn, rm } => {
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rdn), read_reg(core, rm), false);
                    core.r[rdn.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::ADDS_imm { rn, rd, imm32 } => {
                    let r_n = read_reg(core, rn);
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32, false);

                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::SUBS_imm { rn, rd, imm32 } => {
                    let r_n = read_reg(core, rn);
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), imm32 ^ 0xFFFFFFFF, true);

                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::ADDS { rm, rn, rd } => {
                    let (result, carry, overflow) =
                        add_with_carry(read_reg(core, rn), read_reg(core, rm), false);
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);

                    core.r[rd.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Instruction::TST_reg { rn, rm } => {
                    let result = read_reg(core, rn) & read_reg(core, rm);

                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    //core.psr.set_c(carry); carry = shift_c()
                    core.r[Reg::PC.value()] += 2;
                }

                _ => panic!("unimplemented instruction"),
            }
        }
    }
}
