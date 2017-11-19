use core::operation::{add_with_carry, condition_passed, decode_imm_shift, shift_c, sign_extend};
use core::operation::SRType;
use bit_field::BitField;
use core::instruction::Instruction;
use core::register::{Apsr, Reg};
use core::Core;
use bus::Bus;

fn read_reg<T: Bus>(core: &mut Core<T>, r: &Reg) -> u32 {
    match *r {
        Reg::PC => core.r[r.value()] + 4,
        _ => core.r[r.value()],
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

            core.r[rd.value()] = result;
            core.r[Reg::PC.value()] += 2;
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
            core.r[rd.value() as usize] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.r[Reg::PC.value()] += 2;
        }
        Instruction::BIC_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let result = read_reg(core, rn) & (read_reg(core, rm) ^ 0xffff_ffff);
            core.r[rd.value()] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::MOV_reg {
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let result = read_reg(core, rm);
            core.r[rd.value() as usize] = result;

            if *rd != Reg::PC {
                if *setflags {
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                }
                core.r[Reg::PC.value()] += 2;
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
            core.r[rd.value() as usize] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.r[Reg::PC.value()] += 2;
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
            core.r[rd.value() as usize] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.r[Reg::PC.value()] += 2;
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
            core.r[rd.value() as usize] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
            }

            core.r[Reg::PC.value()] += 2;
        }
        Instruction::BL { imm32 } => {
            let pc = read_reg(core, &Reg::PC);
            core.r[Reg::LR.value()] = pc | 0x01;
            core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
        }
        Instruction::BKPT { imm32 } => {
            bkpt_func(imm32, core.r[Reg::R0.value()], core.r[Reg::R1.value()]);

            core.r[Reg::PC.value()] += 2;
        }
        Instruction::NOP => {
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::MUL {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let operand1 = read_reg(core, rn);
            let operand2 = read_reg(core, rm);


            let result = operand1 * operand2;

            core.r[rd.value() as usize] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }

            core.r[Reg::PC.value()] += 2;
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

            core.r[rd.value() as usize] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }

            core.r[Reg::PC.value()] += 2;
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

            core.r[rd.value() as usize] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }

            core.r[Reg::PC.value()] += 2;
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

            core.r[rd.value() as usize] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }

            core.r[Reg::PC.value()] += 2;
        }
        Instruction::BX { ref rm } => {
            core.r[Reg::PC.value()] = read_reg(core, rm) & 0xffff_fffe;
        }
        Instruction::BLX { ref rm } => {
            let pc = read_reg(core, &Reg::PC);
            core.r[Reg::LR.value()] = (((pc - 2) >> 1) << 1) | 1;
            core.r[Reg::PC.value()] = read_reg(core, rm) & 0xffff_fffe;
        }
        Instruction::LDM {
            ref registers,
            ref rn,
        } => {
            let regs_size = 4 * (registers.len() as u32);

            let mut address = read_reg(core, rn);

            for reg in registers.iter() {
                core.r[reg.value()] = core.bus.read32(address);
                address += 4;
            }

            if !registers.contains(rn) {
                core.r[rn.value()] += regs_size;
            }

            core.r[Reg::PC.value()] += 2;
        }
        Instruction::MOV_imm {
            rd,
            imm32,
            setflags,
        } => {
            let result = imm32 as u32;
            core.r[rd.value()] = result;
            if setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::MVN_reg {
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let result = read_reg(core, rm) ^ 0xFFFF_FFFF;
            core.r[rd.value()] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
            }
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::B { ref cond, imm32 } => if condition_passed(cond, &core.psr) {
            let pc = read_reg(core, &Reg::PC);
            core.r[Reg::PC.value()] = ((pc as i32) + imm32) as u32;
        } else {
            core.r[Reg::PC.value()] += 2;
        },

        Instruction::CMP_imm { ref rn, imm32 } => {
            let (result, carry, overflow) =
                add_with_carry(read_reg(core, rn), imm32 ^ 0xFFFF_FFFF, true);
            core.psr.set_n(result.get_bit(31));
            core.psr.set_z(result == 0);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::CMP_reg { ref rn, ref rm } => {
            let (result, carry, overflow) =
                add_with_carry(read_reg(core, rn), read_reg(core, rm) ^ 0xFFFF_FFFF, true);
            core.psr.set_n(result.get_bit(31));
            core.psr.set_z(result == 0);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::CMN_reg { ref rn, ref rm } => {
            let (result, carry, overflow) =
                add_with_carry(read_reg(core, rn), read_reg(core, rm), false);
            core.psr.set_n(result.get_bit(31));
            core.psr.set_z(result == 0);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.r[Reg::PC.value()] += 2;
        }

        Instruction::PUSH { ref registers } => {
            let regs_size = 4 * (registers.len() as u32);
            let sp = core.get_sp();
            let mut address = sp - regs_size;

            for reg in registers.iter() {
                core.bus.write32(address, core.r[reg.value()]);
                address += 4;
            }

            core.set_sp(sp - regs_size);
            core.r[Reg::PC.value()] += 2;
        }

        Instruction::POP { ref registers } => {
            let regs_size = 4 * (registers.len() as u32);
            let sp = core.get_sp();
            let mut address = sp;

            for reg in registers.iter() {
                if reg == Reg::PC {
                    core.r[reg.value()] = core.bus.read32(address) & 0xffff_fffe;
                } else {
                    core.r[reg.value()] = core.bus.read32(address);
                }
                address += 4;
            }

            core.set_sp(sp + regs_size);
            if !registers.contains(&Reg::PC) {
                core.r[Reg::PC.value()] += 2;
            }
        }

        Instruction::LDR_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            core.r[rt.value()] = core.bus.read32(address);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::LDR_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            core.r[rt.value()] = core.bus.read32(address);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::LDRB_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            core.r[rt.value()] = u32::from(core.bus.read8(address));
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::LDRH_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            core.r[rt.value()] = u32::from(core.bus.read16(address));
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::LDRSH_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let data = u32::from(core.bus.read16(address));
            core.r[rt.value()] = sign_extend(data, 15, 32) as u32;
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::LDRSB_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let data = u32::from(core.bus.read8(address));
            core.r[rt.value()] = sign_extend(data, 7, 32) as u32;
            core.r[Reg::PC.value()] += 2;
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

            core.r[rd.value()] = result;
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::STM {
            ref registers,
            ref rn,
        } => {
            let regs_size = 4 * (registers.len() as u32);

            let mut address = read_reg(core, rn);

            for reg in registers.iter() {
                core.bus.write32(address, core.r[reg.value()]);
                address += 4;
            }

            //wback always true
            core.r[rn.value()] += regs_size;

            core.r[Reg::PC.value()] += 2;
        }
        Instruction::STR_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            let value = read_reg(core, rt);
            core.bus.write32(address, value);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::STR_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = read_reg(core, rt);
            core.bus.write32(address, value);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::STRB_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = read_reg(core, rt);
            core.bus.write8(address, value.get_bits(0..8) as u8);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::STRB_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = (read_reg(core, rn) + imm32) & 0xffff_fffc;
            let value = read_reg(core, rt);
            core.bus.write8(address, value.get_bits(0..8) as u8);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::STRH_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = read_reg(core, rn) + imm32;
            let value = read_reg(core, rt);
            core.bus.write16(address, value.get_bits(0..16) as u16);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::STRH_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = read_reg(core, rn) + read_reg(core, rm);
            let value = read_reg(core, rt);
            core.bus.write16(address, value.get_bits(0..16) as u16);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::LDR_lit { ref rt, imm32 } => {
            let base = read_reg(core, &Reg::PC) & 0xffff_fffc;
            core.r[rt.value()] = core.bus.read32(base + imm32);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::ADD_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let (result, carry, overflow) =
                add_with_carry(read_reg(core, rn), read_reg(core, rm), false);

            if rd == &Reg::PC
            {
                core.r[rd.value()] = result & 0xffff_fffe;
            }
            else {
                if *setflags {
                    core.psr.set_n(result.get_bit(31));
                    core.psr.set_z(result == 0);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                }
                core.r[rd.value()] = result;
                core.r[Reg::PC.value()] += 2;
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

            core.r[rd.value()] = result;
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::ADR { ref rd, imm32 } => {
            let result = (read_reg(core, &Reg::PC) & 0xffff_fffc) + imm32;
            core.r[rd.value()] = result;
            core.r[Reg::PC.value()] += 2;
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

            core.r[rd.value()] = result;
            core.r[Reg::PC.value()] += 2;
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
            core.r[rd.value()] = result;

            if *setflags {
                core.psr.set_n(result.get_bit(31));
                core.psr.set_z(result == 0);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.r[Reg::PC.value()] += 2;
        }
        Instruction::TST_reg { ref rn, ref rm } => {
            let result = read_reg(core, rn) & read_reg(core, rm);

            core.psr.set_n(result.get_bit(31));
            core.psr.set_z(result == 0);
            //core.psr.set_c(carry); carry = shift_c()
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::UXTB { ref rd, ref rm } => {
            let rotated = read_reg(core, rm);
            core.r[rd.value()] = rotated.get_bits(0..8);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::UXTH { ref rd, ref rm } => {
            let rotated = read_reg(core, rm);
            core.r[rd.value()] = rotated.get_bits(0..16);
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::SXTB { ref rd, ref rm } => {
            let rotated = read_reg(core, rm);
            core.r[rd.value()] = sign_extend(rotated.get_bits(0..8), 7, 32) as u32;
            core.r[Reg::PC.value()] += 2;
        }
        Instruction::SXTH { ref rd, ref rm } => {
            let rotated = read_reg(core, rm);
            core.r[rd.value()] = sign_extend(rotated.get_bits(0..16), 15, 32) as u32;
            core.r[Reg::PC.value()] += 2;
        }

        _ => panic!(
            "unimplemented instruction {} at {:#x}",
            instruction,
            core.r[Reg::PC.value()]
        ),
    }
}
