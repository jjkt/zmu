use core::operation::{add_with_carry, condition_passed, decode_imm_shift, shift_c, sign_extend};
use core::operation::SRType;
use bit_field::BitField;
use core::instruction::{CpsEffect, Instruction};
use core::register::{Apsr, Ipsr, Reg};
use core::Core;
use bus::Bus;

fn read_reg<T: Bus>(core: &mut Core<T>, r: &Reg) -> u32 {
    match *r {
        Reg::PC => core.get_r(r) + 4,
        _ => core.get_r(r),
    }
}

pub fn execute<T: Bus, F>(core: &mut Core<T>, instruction: &Instruction, mut bkpt_func: F)
where
    F: FnMut(u32, u32, u32),
{
    match *instruction {
        Instruction::ADC_reg {
            ref rn,
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let r_m = read_reg(core, rm);
            let (result, carry, overflow) = add_with_carry(r_n, r_m, core.psr.get_c());

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
        }
        Instruction::ASR_imm {
            ref rd,
            ref rm,
            ref imm5,
            ref setflags,
        } => {
            let (_, shift_n) = decode_imm_shift(0b10, *imm5);
            let (result, carry) = shift_c(
                read_reg(core, rm),
                SRType::ASR,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
        }
        Instruction::ASR_reg {
            ref rd,
            ref rm,
            ref rn,
            ref setflags,
        } => {
            let shift_n = read_reg(core, rm).get_bits(0..8);
            let (result, carry) = shift_c(
                read_reg(core, rn),
                SRType::ASR,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
        }
        Instruction::BIC_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let result = read_reg(core, rn) & (read_reg(core, rm) ^ 0xffff_ffff);
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }
            core.add_pc(2);
        }
        Instruction::CPS { ref im } => {
            if im == &CpsEffect::IE {
                core.primask = false;
            } else {
                core.primask = true;
            }
            core.add_pc(2);
        }
        Instruction::DMB => {
            core.add_pc(2);
        }
        Instruction::DSB => {
            core.add_pc(2);
        }
        Instruction::ISB => {
            core.add_pc(2);
        }
        Instruction::MRS {
            ref rd,
            ref spec_reg,
        } => {
            match spec_reg {
                //APSR => {core.set_r(rd, core.psr.value & 0xf000_0000),
                IPSR => {
                    let ipsr = core.psr.get_exception_number() as u32;
                    core.set_r(rd, ipsr);
                }
                //MSP => core.set_r(rd, core.get_r(Reg::MSP)),
                //PSP => core.set_r(rd, core.get_r(Reg::PSP),
                //PRIMASK => core.set_r(rd,core.primask as u32),
                //CONTROL => core.set_r(rd,core.control as u32),
                _ => panic!("unsupported MRS operation"),
            }

            core.add_pc(4);
        }
        Instruction::MOV_reg {
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let result = read_reg(core, rm);
            core.set_r(rd, result);

            if *rd != Reg::PC {
                if *setflags {
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                }
                core.add_pc(2);
            }
        }
        Instruction::LSL_imm {
            ref rd,
            ref rm,
            ref imm5,
            ref setflags,
        } => {
            let (_, shift_n) = decode_imm_shift(0b00, *imm5);
            let (result, carry) = shift_c(
                read_reg(core, rm),
                SRType::LSL,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
        }
        Instruction::LSL_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let shift_n = read_reg(core, rm).get_bits(0..8);
            let (result, carry) = shift_c(
                read_reg(core, rn),
                SRType::LSL,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
        }

        Instruction::LSR_imm {
            ref rd,
            ref rm,
            ref imm5,
            ref setflags,
        } => {
            let (_, shift_n) = decode_imm_shift(0b01, *imm5);
            let (result, carry) = shift_c(
                read_reg(core, rm),
                SRType::LSR,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
        }
        Instruction::LSR_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let shift_n = read_reg(core, rm).get_bits(0..8);
            let (result, carry) = shift_c(
                read_reg(core, rn),
                SRType::LSR,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
        }
        Instruction::BL { imm32 } => {
            let pc = read_reg(core, &Reg::PC);
            core.set_r(&Reg::LR, pc | 0x01);
            core.set_r(&Reg::PC, ((pc as i32) + imm32) as u32);
        }
        Instruction::BKPT { imm32 } => {
            bkpt_func(imm32, core.get_r(&Reg::R0), core.get_r(&Reg::R1));

            core.add_pc(2);
        }
        Instruction::NOP => {
            core.add_pc(2);
        }
        Instruction::MUL {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let operand1 = read_reg(core, rn);
            let operand2 = read_reg(core, rm);


            let result = operand1.wrapping_mul(operand2);

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }

            core.add_pc(2);
        }
        Instruction::ORR {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let r_m = read_reg(core, rm);

            let result = r_n | r_m;

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }

            core.add_pc(2);
        }
        Instruction::EOR_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let r_m = read_reg(core, rm);

            let result = r_n ^ r_m;

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }

            core.add_pc(2);
        }
        Instruction::AND_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let r_m = read_reg(core, rm);

            let result = r_n & r_m;

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }

            core.add_pc(2);
        }
        Instruction::BX { ref rm } => {
            let r_m = read_reg(core, rm) & 0xffff_fffe;
            core.set_r(&Reg::PC, r_m);
        }
        Instruction::BLX { ref rm } => {
            let pc = read_reg(core, &Reg::PC);
            let value = read_reg(core, rm) & 0xffff_fffe;
            core.set_r(&Reg::LR, (((pc - 2) >> 1) << 1) | 1);
            core.set_r(&Reg::PC, value);
        }
        Instruction::LDM {
            ref registers,
            ref rn,
        } => {
            let regs_size = 4 * (registers.len() as u32);

            let mut address = read_reg(core, rn);

            for reg in registers.iter() {
                let value = core.bus.read32(address);
                core.set_r(&reg, value);
                address += 4;
            }

            if !registers.contains(rn) {
                core.add_r(rn, regs_size);
            }

            core.add_pc(2);
        }
        Instruction::MOV_imm {
            ref rd,
            imm32,
            setflags,
        } => {
            let result = imm32 as u32;
            core.set_r(rd, result);
            if setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }
            core.add_pc(2);
        }
        Instruction::MVN_reg {
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let result = read_reg(core, rm) ^ 0xFFFF_FFFF;
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }
            core.add_pc(2);
        }
        Instruction::B { ref cond, imm32 } => if condition_passed(cond, &core.psr) {
            let pc = read_reg(core, &Reg::PC);
            core.set_r(&Reg::PC, ((pc as i32) + imm32) as u32);
        } else {
            core.add_pc(2);
        },

        Instruction::CMP_imm { ref rn, imm32 } => {
            let (result, carry, overflow) =
                add_with_carry(read_reg(core, rn), imm32 ^ 0xFFFF_FFFF, true);
            core.psr.set_n(result.get_bit(31));
            core.psr.set_z(result == 0);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.add_pc(2);
        }
        Instruction::CMP_reg { ref rn, ref rm } => {
            let (result, carry, overflow) =
                add_with_carry(read_reg(core, rn), read_reg(core, rm) ^ 0xFFFF_FFFF, true);
            core.psr.set_n(result.get_bit(31));
            core.psr.set_z(result == 0);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.add_pc(2);
        }
        Instruction::CMN_reg { ref rn, ref rm } => {
            let (result, carry, overflow) =
                add_with_carry(read_reg(core, rn), read_reg(core, rm), false);
            core.psr.set_n(result.get_bit(31));
            core.psr.set_z(result == 0);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.add_pc(2);
        }

        Instruction::PUSH { ref registers } => {
            let regs_size = 4 * (registers.len() as u32);
            let sp = core.get_r(&Reg::SP);
            let mut address = sp - regs_size;

            for reg in registers.iter() {
                let value = core.get_r(&reg);
                core.bus.write32(address, value);
                address += 4;
            }

            core.set_r(&Reg::SP, sp - regs_size);
            core.add_pc(2);
        }

        Instruction::POP { ref registers } => {
            let regs_size = 4 * (registers.len() as u32);
            let sp = core.get_r(&Reg::SP);
            let mut address = sp;

            for reg in registers.iter() {
                if reg == Reg::PC {
                    let value = core.bus.read32(address) & 0xffff_fffe;
                    core.set_r(&reg, value);
                } else {
                    let value = core.bus.read32(address);
                    core.set_r(&reg, value);
                }
                address += 4;
            }

            core.set_r(&Reg::SP, sp + regs_size);
            if !registers.contains(&Reg::PC) {
                core.add_pc(2);
            }
        }

        Instruction::LDR_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            let value = core.bus.read32(address);
            core.set_r(rt, value);
            core.add_pc(2);
        }
        Instruction::LDR_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = core.bus.read32(address);
            core.set_r(rt, value);
            core.add_pc(2);
        }
        Instruction::LDRB_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            let value = u32::from(core.bus.read8(address));
            core.set_r(rt, value);
            core.add_pc(2);
        }
        Instruction::LDRB_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = u32::from(core.bus.read8(address));
            core.set_r(rt, value);
            core.add_pc(2);
        }
        Instruction::LDRH_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            let value = u32::from(core.bus.read16(address));
            core.set_r(rt, value);
            core.add_pc(2);
        }
        Instruction::LDRH_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = u32::from(core.bus.read16(address));
            core.set_r(rt, value);
            core.add_pc(2);
        }
        Instruction::LDRSH_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let data = u32::from(core.bus.read16(address));
            core.set_r(rt, sign_extend(data, 15, 32) as u32);
            core.add_pc(2);
        }
        Instruction::LDRSB_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let data = u32::from(core.bus.read8(address));
            core.set_r(rt, sign_extend(data, 7, 32) as u32);
            core.add_pc(2);
        }
        Instruction::SBC_reg {
            ref rn,
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let r_m = read_reg(core, rm);
            let (result, carry, overflow) =
                add_with_carry(r_n, r_m ^ 0xffff_ffff, core.psr.get_c());

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
        }
        Instruction::STM {
            ref registers,
            ref rn,
        } => {
            let regs_size = 4 * (registers.len() as u32);

            let mut address = read_reg(core, rn);

            for reg in registers.iter() {
                let r = core.get_r(&reg);
                core.bus.write32(address, r);
                address += 4;
            }

            //wback always true
            core.add_r(rn, regs_size);

            core.add_pc(2);
        }
        Instruction::STR_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            let value = read_reg(core, rt);
            core.bus.write32(address, value);
            core.add_pc(2);
        }
        Instruction::STR_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = read_reg(core, rt);
            core.bus.write32(address, value);
            core.add_pc(2);
        }
        Instruction::STRB_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = read_reg(core, rt);
            core.bus.write8(address, value.get_bits(0..8) as u8);
            core.add_pc(2);
        }
        Instruction::STRB_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            let value = read_reg(core, rt);
            core.bus.write8(address, value.get_bits(0..8) as u8);
            core.add_pc(2);
        }
        Instruction::STRH_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            let value = read_reg(core, rt);
            core.bus.write16(address, value.get_bits(0..16) as u16);
            core.add_pc(2);
        }
        Instruction::STRH_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = read_reg(core, rt);
            core.bus.write16(address, value.get_bits(0..16) as u16);
            core.add_pc(2);
        }
        Instruction::LDR_lit { ref rt, imm32 } => {
            let base = read_reg(core, &Reg::PC) & 0xffff_fffc;
            let value = core.bus.read32(base + imm32);
            core.set_r(rt, value);
            core.add_pc(2);
        }
        Instruction::ADD_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let (result, carry, overflow) =
                add_with_carry(read_reg(core, rn), read_reg(core, rm), false);

            if rd == &Reg::PC {
                core.set_r(rd, result & 0xffff_fffe);
            } else {
                if *setflags {
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                }
                core.set_r(rd, result);
                core.add_pc(2);
            }
        }
        Instruction::ADD_imm {
            ref rn,
            ref rd,
            imm32,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let (result, carry, overflow) = add_with_carry(r_n, imm32, false);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
        }
        Instruction::ADR { ref rd, imm32 } => {
            let result = (read_reg(core, &Reg::PC) & 0xffff_fffc) + imm32;
            core.set_r(rd, result);
            core.add_pc(2);
        }
        Instruction::RSB_imm {
            ref rd,
            ref rn,
            imm32,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let (result, carry, overflow) = add_with_carry(r_n ^ 0xFFFF_FFFF, imm32, true);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
        }
        Instruction::SUB_imm {
            ref rn,
            ref rd,
            imm32,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let (result, carry, overflow) = add_with_carry(r_n, imm32 ^ 0xFFFF_FFFF, true);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
        }
        Instruction::SUB_reg {
            ref rn,
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let r_n = read_reg(core, rn);
            let r_m = read_reg(core, rm);
            let (result, carry, overflow) = add_with_carry(r_n, r_m ^ 0xFFFF_FFFF, true);
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.add_pc(2);
        }
        Instruction::TST_reg { ref rn, ref rm } => {
            let result = read_reg(core, rn) & read_reg(core, rm);

            core.psr.set_n(result.get_bit(31));
            core.psr.set_z(result == 0);
            //core.psr.set_c(carry); carry = shift_c()
            core.add_pc(2);
        }
        Instruction::UXTB { ref rd, ref rm } => {
            let rotated = read_reg(core, rm);
            core.set_r(rd, rotated.get_bits(0..8));
            core.add_pc(2);
        }
        Instruction::UXTH { ref rd, ref rm } => {
            let rotated = read_reg(core, rm);
            core.set_r(rd, rotated.get_bits(0..16));
            core.add_pc(2);
        }
        Instruction::SXTB { ref rd, ref rm } => {
            let rotated = read_reg(core, rm);
            core.set_r(rd, sign_extend(rotated.get_bits(0..8), 7, 32) as u32);
            core.add_pc(2);
        }
        Instruction::SXTH { ref rd, ref rm } => {
            let rotated = read_reg(core, rm);
            core.set_r(rd, sign_extend(rotated.get_bits(0..16), 15, 32) as u32);
            core.add_pc(2);
        }
        _ => panic!(
            "unimplemented instruction {} at {:#x}",
            instruction,
            core.get_r(&Reg::PC)
        ),
    }
}
