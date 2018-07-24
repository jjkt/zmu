use bit_field::BitField;
use bus::Bus;
use core::fault::Fault;
use core::instruction::{SRType, CpsEffect, Instruction};
use core::operation::{add_with_carry, condition_passed, decode_imm_shift, shift_c, sign_extend};
use core::register::{Apsr, Ipsr, Reg, SpecialReg};
use core::Core;
use semihosting::decode_semihostcmd;
use semihosting::semihost_return;
use semihosting::SemihostingCommand;
use semihosting::SemihostingResponse;

#[allow(unused_variables)]
pub fn execute<T: Bus, F>(
    mut core: &mut Core<T>,
    instruction: &Instruction,
    mut semihost_func: F,
) -> Option<Fault>
where
    F: FnMut(&SemihostingCommand) -> SemihostingResponse,
{
    match *instruction {
        Instruction::ADC_reg {
            ref rn,
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let r_n = core.get_r(rn);
            let r_m = core.get_r(rm);
            let (result, carry, overflow) = add_with_carry(r_n, r_m, core.psr.get_c());

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::ASR_imm {
            ref rd,
            ref rm,
            ref imm5,
            ref setflags,
        } => {
            let (_, shift_n) = decode_imm_shift(0b10, *imm5);

            let (result, carry) = shift_c(
                core.get_r(rm),
                SRType::ASR,
                u32::from(shift_n),
                core.psr.get_c(),
            );

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
            }
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::ASR_reg {
            ref rd,
            ref rm,
            ref rn,
            ref setflags,
        } => {
            let shift_n = core.get_r(rm).get_bits(0..8);
            let (result, carry) = shift_c(
                core.get_r(rn),
                SRType::ASR,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::BIC_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let result = core.get_r(rn) & (core.get_r(rm) ^ 0xffff_ffff);
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
            }
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::CPS { ref im } => {
            if im == &CpsEffect::IE {
                core.primask = false;
            } else {
                core.primask = true;
            }
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::DMB => {
            core.add_pc(2);
            core.cycle_count += 4;
            None
        }
        Instruction::DSB => {
            core.add_pc(2);
            core.cycle_count += 4;
            None
        }
        Instruction::ISB => {
            core.add_pc(2);
            core.cycle_count += 4;
            None
        }
        Instruction::MRS {
            ref rd,
            ref spec_reg,
        } => {
            match spec_reg {
                //APSR => {core.set_r(rd, core.psr.value & 0xf000_0000),
                &SpecialReg::IPSR => {
                    let ipsr_val = core.psr.get_exception_number() as u32;
                    core.set_r(rd, ipsr_val);
                }
                //MSP => core.set_r(rd, core.get_r(Reg::MSP)),
                //PSP => core.set_r(rd, core.get_r(Reg::PSP),
                &SpecialReg::PRIMASK => {
                    let primask = core.primask as u32;
                    core.set_r(rd, primask);
                }
                //CONTROL => core.set_r(rd,core.control as u32),
                _ => panic!("unsupported MRS operation"),
            }

            core.add_pc(4);
            core.cycle_count += 4;
            None
        }
        Instruction::MSR_reg {
            ref rn,
            ref spec_reg,
        } => {
            match spec_reg {
                //APSR => {core.set_r(rd, core.psr.value & 0xf000_0000),
                /*&SpecialReg::IPSR => {
                    let ipsr_val = core.psr.get_exception_number() as u32;
                    core.set_r(rd, ipsr_val);
                }*/
                &SpecialReg::MSP => {
                    let msp = core.get_r(rn);
                    core.set_msp(msp);
                }
                &SpecialReg::PSP => {
                    let psp = core.get_r(rn);
                    core.set_psp(psp);
                }
                //PSP => core.set_r(rd, core.get_r(Reg::PSP),
                &SpecialReg::PRIMASK => {
                    let primask = core.get_r(rn) & 1 == 1;
                    core.primask = primask;
                }
                //CONTROL => core.set_r(rd,core.control as u32),
                _ => panic!("unsupported MSR operation"),
            }

            core.add_pc(4);
            core.cycle_count += 4;
            None
        }
        Instruction::MOV_reg {
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let result = core.get_r(rm);
            core.set_r(rd, result);

            if *rd != Reg::PC {
                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                }
                core.add_pc(2);
            }

            core.cycle_count += 1;
            None
        }
        Instruction::LSL_imm {
            ref rd,
            ref rm,
            ref imm5,
            ref setflags,
        } => {
            let (_, shift_n) = decode_imm_shift(0b00, *imm5);
            let (result, carry) = shift_c(
                core.get_r(rm),
                SRType::LSL,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::LSL_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let shift_n = core.get_r(rm).get_bits(0..8);
            let (result, carry) = shift_c(
                core.get_r(rn),
                SRType::LSL,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::LSR_imm {
            ref rd,
            ref rm,
            ref imm5,
            ref setflags,
        } => {
            let (_, shift_n) = decode_imm_shift(0b01, *imm5);
            let (result, carry) = shift_c(
                core.get_r(rm),
                SRType::LSR,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::LSR_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let shift_n = core.get_r(rm).get_bits(0..8);
            let (result, carry) = shift_c(
                core.get_r(rn),
                SRType::LSR,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::BL { imm32 } => {
            let pc = core.get_r(&Reg::PC);
            core.set_r(&Reg::LR, pc | 0x01);
            let target = ((pc as i32) + imm32) as u32;
            core.branch_write_pc(target);
            core.cycle_count += 4;
            None
        }

        Instruction::BKPT { imm32 } => {
            if imm32 == 0xab {
                let r0 = core.get_r(&Reg::R0);
                let r1 = core.get_r(&Reg::R1);
                let semihost_cmd = decode_semihostcmd(r0, r1, &mut core);
                let semihost_response = semihost_func(&semihost_cmd);
                semihost_return(&mut core, &semihost_response);
            }
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::NOP => {
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::MUL {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let operand1 = core.get_r(rn);
            let operand2 = core.get_r(rm);

            let result = operand1.wrapping_mul(operand2);

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            // TODO or 32 if multiplier implementation is the cheaper one
            None
        }

        Instruction::ORR_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let r_n = core.get_r(rn);
            let r_m = core.get_r(rm);

            let result = r_n | r_m;

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::EOR_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let r_n = core.get_r(rn);
            let r_m = core.get_r(rm);

            let result = r_n ^ r_m;

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::AND_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let r_n = core.get_r(rn);
            let r_m = core.get_r(rm);

            let result = r_n & r_m;

            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::BX { ref rm } => {
            let r_m = core.get_r(rm);
            core.bx_write_pc(r_m);
            core.cycle_count += 3;
            None
        }

        Instruction::BLX { ref rm } => {
            let pc = core.get_r(&Reg::PC);
            let target = core.get_r(rm);
            core.set_r(&Reg::LR, (((pc - 2) >> 1) << 1) | 1);
            core.blx_write_pc(target);
            core.cycle_count += 3;
            None
        }

        Instruction::LDM {
            ref registers,
            ref rn,
        } => {
            let regs_size = 4 * (registers.len() as u32);

            let mut address = core.get_r(rn);

            for reg in registers.iter() {
                let value = core.bus.read32(address);
                core.set_r(&reg, value);
                address += 4;
            }

            if !registers.contains(rn) {
                core.add_r(rn, regs_size);
            }

            core.add_pc(2);
            core.cycle_count += 1 + registers.len() as u64;
            None
        }
        Instruction::MOV_imm {
            ref rd,
            imm32,
            setflags,
        } => {
            let result = imm32 as u32;
            core.set_r(rd, result);
            if setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
            }
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::MVN_reg {
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let result = core.get_r(rm) ^ 0xFFFF_FFFF;
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
            }
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::MVN_imm {
            ref rd,
            ref imm32,
            ref setflags,
        } => unimplemented!(),

        Instruction::B { ref cond, imm32 } => if condition_passed(cond, &core.psr) {
            let pc = core.get_r(&Reg::PC);
            let target = ((pc as i32) + imm32) as u32;
            core.branch_write_pc(target);
            core.cycle_count += 3;
            None
        } else {
            core.add_pc(2);
            core.cycle_count += 1;
            None
        },

        Instruction::CMP_imm { ref rn, imm32 } => {
            let (result, carry, overflow) =
                add_with_carry(core.get_r(rn), imm32 ^ 0xFFFF_FFFF, true);
            core.psr.set_n(result);
            core.psr.set_z(result);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::CMP_reg { ref rn, ref rm } => {
            let (result, carry, overflow) =
                add_with_carry(core.get_r(rn), core.get_r(rm) ^ 0xFFFF_FFFF, true);
            core.psr.set_n(result);
            core.psr.set_z(result);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::CMN_reg { ref rn, ref rm } => {
            let (result, carry, overflow) = add_with_carry(core.get_r(rn), core.get_r(rm), false);
            core.psr.set_n(result);
            core.psr.set_z(result);
            core.psr.set_c(carry);
            core.psr.set_v(overflow);
            core.add_pc(2);
            core.cycle_count += 1;
            None
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
            core.cycle_count += 1 + registers.len() as u64;
            None
        }

        Instruction::POP { ref registers } => {
            let regs_size = 4 * (registers.len() as u32);
            let sp = core.get_r(&Reg::SP);
            let mut address = sp;

            for reg in registers.iter() {
                if reg == Reg::PC {
                    let target = core.bus.read32(address);
                    core.bx_write_pc(target);
                } else {
                    let value = core.bus.read32(address);
                    core.set_r(&reg, value);
                }
                address += 4;
            }

            core.set_r(&Reg::SP, sp + regs_size);
            if registers.contains(&Reg::PC) {
                core.cycle_count += 4 + registers.len() as u64;
            } else {
                core.cycle_count += 1 + registers.len() as u64;
                core.add_pc(2);
            }
            None
        }

        Instruction::LDR_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = core.get_r(rn) + imm32;
            let value = core.bus.read32(address);
            core.set_r(rt, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::LDR_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = core.get_r(rn) + core.get_r(rm);
            let value = core.bus.read32(address);
            core.set_r(rt, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::LDRB_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = core.get_r(rn) + imm32;
            let value = u32::from(core.bus.read8(address));
            core.set_r(rt, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::LDRB_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = core.get_r(rn) + core.get_r(rm);
            let value = u32::from(core.bus.read8(address));
            core.set_r(rt, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::LDRH_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = core.get_r(rn) + imm32;
            let value = u32::from(core.bus.read16(address));
            core.set_r(rt, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::LDRH_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = core.get_r(rn) + core.get_r(rm);
            let value = u32::from(core.bus.read16(address));
            core.set_r(rt, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::LDRSH_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = core.get_r(rn) + core.get_r(rm);
            let data = u32::from(core.bus.read16(address));
            core.set_r(rt, sign_extend(data, 15, 32) as u32);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::LDRSB_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = core.get_r(rn) + core.get_r(rm);
            let data = u32::from(core.bus.read8(address));
            core.set_r(rt, sign_extend(data, 7, 32) as u32);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::SBC_reg {
            ref rn,
            ref rd,
            ref rm,
            ref setflags,
        } => {
            let r_n = core.get_r(rn);
            let r_m = core.get_r(rm);
            let (result, carry, overflow) =
                add_with_carry(r_n, r_m ^ 0xffff_ffff, core.psr.get_c());

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::STM {
            ref registers,
            ref rn,
        } => {
            let regs_size = 4 * (registers.len() as u32);

            let mut address = core.get_r(rn);

            for reg in registers.iter() {
                let r = core.get_r(&reg);
                core.bus.write32(address, r);
                address += 4;
            }

            //wback always true
            core.add_r(rn, regs_size);

            core.add_pc(2);
            core.cycle_count += 1 + registers.len() as u64;
            None
        }

        Instruction::STR_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = core.get_r(rn) + imm32;
            let value = core.get_r(rt);
            core.bus.write32(address, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::STR_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = core.get_r(rn) + core.get_r(rm);
            let value = core.get_r(rt);
            core.bus.write32(address, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::STRB_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = core.get_r(rn) + core.get_r(rm);
            let value = core.get_r(rt);
            core.bus.write8(address, value.get_bits(0..8) as u8);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::STRB_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = core.get_r(rn) + imm32;
            let value = core.get_r(rt);
            core.bus.write8(address, value.get_bits(0..8) as u8);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::STRH_imm {
            ref rt,
            ref rn,
            imm32,
        } => {
            let address = core.get_r(rn) + imm32;
            let value = core.get_r(rt);
            core.bus.write16(address, value.get_bits(0..16) as u16);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::STRH_reg {
            ref rt,
            ref rn,
            ref rm,
        } => {
            let address = core.get_r(rn) + core.get_r(rm);
            let value = core.get_r(rt);
            core.bus.write16(address, value.get_bits(0..16) as u16);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::LDR_lit { ref rt, imm32 } => {
            let base = core.get_r(&Reg::PC) & 0xffff_fffc;
            let value = core.bus.read32(base + imm32);
            core.set_r(rt, value);
            core.add_pc(2);
            core.cycle_count += 2;
            None
        }

        Instruction::ADD_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let (result, carry, overflow) = add_with_carry(core.get_r(rn), core.get_r(rm), false);

            if rd == &Reg::PC {
                core.branch_write_pc(result);
                core.cycle_count += 3;
            } else {
                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                }
                core.set_r(rd, result);
                core.add_pc(2);
                core.cycle_count += 1;
            }
            None
        }

        Instruction::ADD_imm {
            ref rn,
            ref rd,
            imm32,
            ref setflags,
        } => {
            let r_n = core.get_r(rn);
            let (result, carry, overflow) = add_with_carry(r_n, imm32, false);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::ADR { ref rd, imm32 } => {
            let result = (core.get_r(&Reg::PC) & 0xffff_fffc) + imm32;
            core.set_r(rd, result);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::RSB_imm {
            ref rd,
            ref rn,
            imm32,
            ref setflags,
        } => {
            let r_n = core.get_r(rn);
            let (result, carry, overflow) = add_with_carry(r_n ^ 0xFFFF_FFFF, imm32, true);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::SUB_imm {
            ref rn,
            ref rd,
            imm32,
            ref setflags,
        } => {
            let r_n = core.get_r(rn);
            let (result, carry, overflow) = add_with_carry(r_n, imm32 ^ 0xFFFF_FFFF, true);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.set_r(rd, result);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::SUB_reg {
            ref rn,
            ref rd,
            ref rm,
            ref setflags,
            ref shift_t,
            ref shift_n
        } => {
            let r_n = core.get_r(rn);
            let r_m = core.get_r(rm);
            let (result, carry, overflow) = add_with_carry(r_n, r_m ^ 0xFFFF_FFFF, true);
            core.set_r(rd, result);

            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
            }

            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::TST_reg { ref rn, ref rm } => {
            let result = core.get_r(rn) & core.get_r(rm);

            core.psr.set_n(result);
            core.psr.set_z(result);
            //core.psr.set_c(carry); carry = shift_c()
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::UXTB { ref rd, ref rm } => {
            let rotated = core.get_r(rm);
            core.set_r(rd, rotated.get_bits(0..8));
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::UXTH { ref rd, ref rm } => {
            let rotated = core.get_r(rm);
            core.set_r(rd, rotated.get_bits(0..16));
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::SXTB { ref rd, ref rm } => {
            let rotated = core.get_r(rm);
            core.set_r(rd, sign_extend(rotated.get_bits(0..8), 7, 32) as u32);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        Instruction::SXTH { ref rd, ref rm } => {
            let rotated = core.get_r(rm);
            core.set_r(rd, sign_extend(rotated.get_bits(0..16), 15, 32) as u32);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::REV { ref rd, ref rm } => {
            let rm_ = core.get_r(rm);
            core.set_r(
                rd,
                ((rm_ & 0xff) << 24)
                    + ((rm_ & 0xff00) << 8)
                    + ((rm_ & 0xff_0000) >> 8)
                    + ((rm_ & 0xff00_0000) >> 24),
            );
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::REV16 { ref rd, ref rm } => {
            let rm_ = core.get_r(rm);
            core.set_r(
                rd,
                ((rm_ & 0xff) << 8)
                    + ((rm_ & 0xff00) >> 8)
                    + ((rm_ & 0xff_0000) << 8)
                    + ((rm_ & 0xff00_0000) >> 8),
            );
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::REVSH { ref rd, ref rm } => {
            let rm_ = core.get_r(rm);
            core.set_r(
                rd,
                ((sign_extend(rm_ & 0xff, 7, 24) as u32) << 8) + ((rm_ & 0xff00) >> 8),
            );
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::ROR_reg {
            ref rd,
            ref rn,
            ref rm,
            ref setflags,
        } => {
            let shift_n = core.get_r(rm) & 0xff;
            let (result, carry) = shift_c(
                core.get_r(rn),
                SRType::ROR,
                u32::from(shift_n),
                core.psr.get_c(),
            );
            core.set_r(rd, result);
            if *setflags {
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
            }
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::SVC { ref imm32 } => {
            println!("SVC {}", imm32);
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::SEV => {
            println!("SEV");
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }
        Instruction::WFE => {
            core.cycle_count += 1;
            core.add_pc(2);
            None
        }
        Instruction::WFI => {
            core.cycle_count += 1;
            core.add_pc(2);
            None
        }
        Instruction::YIELD => {
            println!("YIELD");
            core.add_pc(2);
            core.cycle_count += 1;
            None
        }

        // ARMv7-M
        Instruction::MCR {
            ref rt,
            ref coproc,
            ref opc1,
            ref opc2,
            ref crn,
            ref crm,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::MCR2 {
            ref rt,
            ref coproc,
            ref opc1,
            ref opc2,
            ref crn,
            ref crm,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::LDC_imm {
            ref coproc,
            ref imm32,
            ref crd,
            ref rn,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::LDC2_imm {
            ref coproc,
            ref imm32,
            ref crd,
            ref rn,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::UDIV {
            ref rd,
            ref rn,
            ref rm,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::UMLAL {
            ref rdlo,
            ref rdhi,
            ref rn,
            ref rm,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::SMLAL {
            ref rdlo,
            ref rdhi,
            ref rn,
            ref rm,
        } => unimplemented!(),

        Instruction::UDF {
            ref imm32,
            ref opcode,
        } => {
            println!("UDF {}, {}", imm32, opcode);
            panic!("undefined");
            //Some(Fault::UndefinedInstruction)
        }
    }
}
