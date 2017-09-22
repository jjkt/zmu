use bit_field::BitField;

use instruction::Op;
use register::Reg;
use register::Apsr;
use core::Core;
use memory::Fetch;

use operation::sign_extend;
use operation::add_with_carry;
use operation::condition_passed;

pub fn execute<T: Fetch>(core: &mut Core<T>, op: Option<Op>) {
    match op {
        None => panic!("undefined code"),
        Some(oper) => {
            match oper {
                Op::MOV { rd, rm } => {
                    core.r[rd.value() as usize] = core.r[rm.value()];
                    core.r[Reg::PC.value()] += 2;
                }
                Op::BL { imm32 } => {
                    let pc = core.r[Reg::PC.value()] + 4;
                    core.r[Reg::LR.value()] = pc | 0x01;
                    core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                }
                Op::BX { rm } => {
                    core.r[Reg::PC.value()] = core.r[rm.value() as usize] & 0xfffffffe;
                }
                Op::MOV_imm8 { rd, imm8 } => {
                    core.r[rd.value()] = imm8 as u32;
                    core.r[Reg::PC.value()] += 2;
                }
                Op::B_imm8 { cond, imm8 } => {
                    let imm32 = sign_extend((imm8 as u32) << 1, 8, 32);
                    if condition_passed(cond, &core.apsr) {
                        let pc = core.r[Reg::PC.value()] + 4;
                        core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                    } else {
                        core.r[Reg::PC.value()] += 2;
                    }
                }
                Op::B_imm11 { imm11 } => {
                    let pc = core.r[Reg::PC.value()] + 4;
                    let imm32 = sign_extend((imm11  as u32) << 1, 11, 32);
                    core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
                }

                Op::CMP_imm8 { rn, imm8 } => {
                    let imm32 = imm8 as u32;
                    let (result, carry, overflow) =
                        add_with_carry(core.r[rn.value()], imm32 ^ 0xFFFFFFFF, true);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::CMP { rn, rm } => {
                    let (result, carry, overflow) =
                        add_with_carry(core.r[rn.value()], core.r[rm.value()] ^ 0xFFFFFFFF, true);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);
                    core.r[Reg::PC.value()] += 2;
                }
                
                Op::PUSH { registers } => {
                    let address = core.msp - 4 * (registers.len() as u32);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::LDR { rt, imm8 } => {
                    let imm32 = (imm8 as u32) << 2;
                    let address = (core.r[Reg::PC.value()] & 0xfffffffc) + imm32;
                    core.r[rt.value()] = core.memory.fetch32(address);
                    core.r[Reg::PC.value()] += 2;
                }
                Op::ADD { rdn, rm } => {
                    //TODO: reading R[15] should yield PC+4 here
                    let (result, carry, overflow) =
                        add_with_carry(core.r[rdn.value()], core.r[rm.value()], false);
                    core.r[rdn.value()] = result;
                    core.r[Reg::PC.value()] += 2;
                }
                Op::ADD_imm8 { rdn, imm8 } => {
                    let (result, carry, overflow) =
                        add_with_carry(core.r[rdn.value()], imm8 as u32, false);
                    core.apsr.set_n(result.get_bit(31));
                    core.apsr.set_z(result == 0);
                    core.apsr.set_c(carry);
                    core.apsr.set_v(overflow);

                    core.r[rdn.value()] = result;
                    core.r[Reg::PC.value()] += 2;

                }
                Op::ADDS { rm, rn, rd } => {
                    let (result, carry, overflow) =
                        add_with_carry(core.r[rn.value()], core.r[rm.value()], false);
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
