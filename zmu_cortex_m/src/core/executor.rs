use crate::bus::Bus;
use crate::core::fault::Fault;
use crate::core::instruction::{CpsEffect, Imm32Carry, Instruction, SRType};
use crate::core::operation::{add_with_carry, ror, shift, shift_c, sign_extend};
use crate::core::register::{Apsr, Ipsr, Reg, SpecialReg};
use crate::core::Core;
use crate::semihosting::decode_semihostcmd;
use crate::semihosting::semihost_return;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;
use bit_field::BitField;

#[derive(PartialEq, Debug)]
pub enum ExecuteResult {
    // Instruction execution resulted in a fault.
    Fault { fault: Fault },
    // The instruction was taken normally
    Taken { cycles: u64 },
    // The instruction was not taken as the condition did not pass
    NotTaken,
    // The execution branched to a new address, pc was set accordingly
    Branched { cycles: u64 },
}

fn resolve_addressing(rn: u32, imm32: u32, add: bool, index: bool) -> (u32, u32) {
    let offset_address = if add { rn + imm32 } else { rn - imm32 };

    let address = if index { offset_address } else { rn };
    (address, offset_address)
}

fn expand_conditional_carry(imm32: &Imm32Carry, carry: bool) -> (u32, bool) {
    match imm32 {
        Imm32Carry::NoCarry { imm32 } => (*imm32, carry),
        Imm32Carry::Carry { imm32_c0, imm32_c1 } => {
            if carry {
                *imm32_c1
            } else {
                *imm32_c0
            }
        }
    }
}

#[allow(unused_variables)]
pub fn execute<T: Bus, F>(
    mut core: &mut Core<T>,
    instruction: &Instruction,
    mut semihost_func: F,
) -> ExecuteResult
where
    F: FnMut(&SemihostingCommand) -> SemihostingResponse,
{
    match instruction {
        Instruction::ADC_reg {
            rn,
            rd,
            rm,
            setflags,
        } => {
            if core.condition_passed() {
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
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::ASR_imm {
            rd,
            rm,
            shift_n,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let (result, carry) = shift_c(
                    core.get_r(rm),
                    &SRType::ASR,
                    usize::from(*shift_n),
                    core.psr.get_c(),
                );

                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::ASR_reg {
            rd,
            rm,
            rn,
            setflags,
        } => {
            if core.condition_passed() {
                let shift_n = core.get_r(rm).get_bits(0..8);
                let (result, carry) = shift_c(
                    core.get_r(rn),
                    &SRType::ASR,
                    shift_n as usize,
                    core.psr.get_c(),
                );
                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::BIC_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            if core.condition_passed() {
                let result = core.get_r(rn) & (core.get_r(rm) ^ 0xffff_ffff);
                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::BFI {
            rn,
            rd,
            lsbit,
            msbit,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let r_d = core.get_r(rd);

                let width = (msbit - lsbit) + 1;

                let mut result = r_d;
                result.set_bits(0..width, r_n.get_bits(0..width));

                core.set_r(rd, result);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::CPS { im } => {
            if im == &CpsEffect::IE {
                core.primask = false;
            } else {
                core.primask = true;
            }
            return ExecuteResult::Taken { cycles: 1 };
        }
        Instruction::CBZ { rn, nonzero, imm32 } => {
            if nonzero ^ (core.get_r(rn) == 0) {
                let pc = core.get_r(&Reg::PC);
                core.branch_write_pc(pc + imm32);
                return ExecuteResult::Branched { cycles: 1 };
            } else {
                return ExecuteResult::Taken { cycles: 1 };
            }
        }
        Instruction::DMB => {
            if core.condition_passed() {
                return ExecuteResult::Taken { cycles: 4 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::DSB => {
            if core.condition_passed() {
                return ExecuteResult::Taken { cycles: 4 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::ISB => {
            if core.condition_passed() {
                return ExecuteResult::Taken { cycles: 4 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::IT {
            x,
            y,
            z,
            firstcond,
            mask,
        } => {
            core.set_itstate((((firstcond.value() as u32) << 4) + *mask as u32) as u8);
            return ExecuteResult::Taken { cycles: 4 };
        }

        Instruction::MRS { rd, spec_reg } => {
            if core.condition_passed() {
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
                return ExecuteResult::Taken { cycles: 4 };
            }

            ExecuteResult::NotTaken
        }
        Instruction::MSR_reg { rn, spec_reg } => {
            if core.condition_passed() {
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
                return ExecuteResult::Taken { cycles: 4 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::MOV_reg {
            rd,
            rm,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let result = core.get_r(rm);
                core.set_r(rd, result);

                if *rd != Reg::PC {
                    if *setflags {
                        core.psr.set_n(result);
                        core.psr.set_z(result);
                    }
                    return ExecuteResult::Taken { cycles: 1 };
                } else {
                    unimplemented!()
                }
            }

            ExecuteResult::NotTaken
        }
        Instruction::LSL_imm {
            rd,
            rm,
            shift_n,
            thumb32,
            setflags,
        } => {
            if core.condition_passed() {
                let (result, carry) = shift_c(
                    core.get_r(rm),
                    &SRType::LSL,
                    *shift_n as usize,
                    core.psr.get_c(),
                );
                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LSL_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            if core.condition_passed() {
                let shift_n = core.get_r(rm).get_bits(0..8);
                let (result, carry) = shift_c(
                    core.get_r(rn),
                    &SRType::LSL,
                    shift_n as usize,
                    core.psr.get_c(),
                );
                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LSR_imm {
            rd,
            rm,
            shift_n,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let (result, carry) = shift_c(
                    core.get_r(rm),
                    &SRType::LSR,
                    usize::from(*shift_n),
                    core.psr.get_c(),
                );
                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LSR_reg {
            rd,
            rn,
            rm,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let shift_n = core.get_r(rm).get_bits(0..8);
                let (result, carry) = shift_c(
                    core.get_r(rn),
                    &SRType::LSR,
                    shift_n as usize,
                    core.psr.get_c(),
                );

                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }

            ExecuteResult::NotTaken
        }

        Instruction::BL { imm32 } => {
            if core.condition_passed() {
                let pc = core.get_r(&Reg::PC);
                core.set_r(&Reg::LR, pc | 0x01);
                let target = ((pc as i32) + imm32) as u32;
                core.branch_write_pc(target);
                return ExecuteResult::Branched { cycles: 4 };
            }

            ExecuteResult::NotTaken
        }

        Instruction::BKPT { imm32 } => {
            if *imm32 == 0xab {
                let r0 = core.get_r(&Reg::R0);
                let r1 = core.get_r(&Reg::R1);
                let semihost_cmd = decode_semihostcmd(r0, r1, &mut core);
                let semihost_response = semihost_func(&semihost_cmd);
                semihost_return(&mut core, &semihost_response);
            }
            return ExecuteResult::Taken { cycles: 1 };
        }

        Instruction::NOP => {
            return ExecuteResult::Taken { cycles: 1 };
        }

        Instruction::MUL {
            rd,
            rn,
            rm,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let operand1 = core.get_r(rn);
                let operand2 = core.get_r(rm);

                let result = operand1.wrapping_mul(operand2);

                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::ORR_reg {
            rd,
            rn,
            rm,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let r_m = core.get_r(rm);

                let (shifted, carry) = shift_c(r_m, shift_t, *shift_n as usize, core.psr.get_c());
                let result = r_n | shifted;

                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::ORR_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let (im, carry) = expand_conditional_carry(imm32, core.psr.get_c());

                let result = r_n | im;

                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::EOR_reg {
            rd,
            rn,
            rm,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let r_m = core.get_r(rm);

                let (shifted, carry) = shift_c(r_m, shift_t, *shift_n as usize, core.psr.get_c());

                let result = r_n ^ shifted;

                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }

            ExecuteResult::NotTaken
        }

        Instruction::AND_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let r_m = core.get_r(rm);

                let result = r_n & r_m;

                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::AND_imm {
            rd,
            rn,
            imm32,
            setflags,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let (im, carry) = expand_conditional_carry(imm32, core.psr.get_c());

                let result = r_n & im;

                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::BX { rm } => {
            if core.condition_passed() {
                let r_m = core.get_r(rm);
                core.bx_write_pc(r_m);
                return ExecuteResult::Branched { cycles: 3 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::BLX { rm } => {
            if core.condition_passed() {
                let pc = core.get_r(&Reg::PC);
                let target = core.get_r(rm);
                core.set_r(&Reg::LR, (((pc - 2) >> 1) << 1) | 1);
                core.blx_write_pc(target);
                return ExecuteResult::Branched { cycles: 3 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDM { registers, rn } => {
            if core.condition_passed() {
                let regs_size = 4 * (registers.len() as u32);

                let mut address = core.get_r(rn);

                let mut branched = false;
                for reg in registers.iter() {
                    let value = core.bus.read32(address);
                    if &reg == &Reg::PC {
                        core.load_write_pc(value);
                        branched = true;
                    } else {
                        core.set_r(&reg, value);
                    }
                    address += 4;
                }

                if !registers.contains(rn) {
                    core.add_r(rn, regs_size);
                }
                let cc = 1 + registers.len() as u64;
                if branched {
                    return ExecuteResult::Branched { cycles: cc };
                }
                return ExecuteResult::Taken { cycles: cc };
            }
            ExecuteResult::NotTaken
        }
        Instruction::MOV_imm {
            rd,
            imm32,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let (result, carry) = expand_conditional_carry(&imm32, core.psr.get_c());
                core.set_r(&rd, result);
                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::MVN_reg { rd, rm, setflags } => {
            if core.condition_passed() {
                let result = core.get_r(rm) ^ 0xFFFF_FFFF;
                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::MVN_imm {
            rd,
            imm32,
            setflags,
        } => {
            if core.condition_passed() {
                let (im, carry) = expand_conditional_carry(imm32, core.psr.get_c());
                let result = im ^ 0xFFFF_FFFF;
                core.set_r(rd, result);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::B_t13 {
            cond,
            imm32,
            thumb32,
        } => {
            if core.condition_passed_b(cond) {
                let pc = core.get_r(&Reg::PC);
                let target = ((pc as i32) + imm32) as u32;
                core.branch_write_pc(target);
                return ExecuteResult::Branched { cycles: 3 };
            } else {
                ExecuteResult::NotTaken
            }
        }
        Instruction::B_t24 { imm32, thumb32 } => {
            if core.condition_passed() {
                let pc = core.get_r(&Reg::PC);
                let target = ((pc as i32) + imm32) as u32;
                core.branch_write_pc(target);
                return ExecuteResult::Branched { cycles: 3 };
            } else {
                ExecuteResult::NotTaken
            }
        }

        Instruction::CMP_imm { rn, imm32, thumb32 } => {
            if core.condition_passed() {
                let (result, carry, overflow) =
                    add_with_carry(core.get_r(rn), imm32 ^ 0xFFFF_FFFF, true);
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::CMP_reg { rn, rm } => {
            if core.condition_passed() {
                let (result, carry, overflow) =
                    add_with_carry(core.get_r(rn), core.get_r(rm) ^ 0xFFFF_FFFF, true);
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::CMN_reg {
            rn,
            rm,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if core.condition_passed() {
                let shifted = shift(core.get_r(rm), shift_t, *shift_n as usize, core.psr.get_c());
                let (result, carry, overflow) = add_with_carry(core.get_r(rn), shifted, false);
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::CMN_imm { rn, imm32 } => {
            if core.condition_passed() {
                let (result, carry, overflow) = add_with_carry(core.get_r(rn), *imm32, false);
                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);
                core.psr.set_v(overflow);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::PUSH { registers, thumb32 } => {
            if core.condition_passed() {
                let regs_size = 4 * (registers.len() as u32);
                let sp = core.get_r(&Reg::SP);
                let mut address = sp - regs_size;

                for reg in registers.iter() {
                    let value = core.get_r(&reg);
                    core.bus.write32(address, value);
                    address += 4;
                }

                core.set_r(&Reg::SP, sp - regs_size);
                return ExecuteResult::Taken {
                    cycles: 1 + registers.len() as u64,
                };
            }
            ExecuteResult::NotTaken
        }

        Instruction::POP { registers, thumb32 } => {
            if core.condition_passed() {
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
                    return ExecuteResult::Branched {
                        cycles: 4 + registers.len() as u64,
                    };
                } else {
                    return ExecuteResult::Taken {
                        cycles: 1 + registers.len() as u64,
                    };
                }
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDR_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let data = core.bus.read32(address);
                if *wback {
                    core.set_r(rn, offset_address);
                }

                if rt == &Reg::PC {
                    core.load_write_pc(data);
                    return ExecuteResult::Branched { cycles: 1 };
                } else {
                    core.set_r(rt, data);
                    return ExecuteResult::Taken { cycles: 1 };
                }
            }
            ExecuteResult::NotTaken
        }
        Instruction::LDRSH_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let data = core.bus.read16(address);
                if *wback {
                    core.set_r(rn, offset_address);
                }

                core.set_r(rt, sign_extend(data as u32, 15, 32) as u32);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDR_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let rm_ = core.get_r(rm);
                let offset = shift(rm_, shift_t, *shift_n as usize, core.psr.get_c());

                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), offset, *add, *index);

                let data = core.bus.read32(address);
                if *wback {
                    core.set_r(rn, offset_address);
                }

                if rt == &Reg::PC {
                    core.load_write_pc(data);
                } else {
                    core.set_r(rt, data);
                }
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDRB_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let data = core.bus.read8(address);
                core.set_r(rt, data as u32);

                if *wback {
                    core.set_r(rn, offset_address);
                }

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDRB_reg { rt, rn, rm } => {
            if core.condition_passed() {
                let address = core.get_r(rn) + core.get_r(rm);
                let value = u32::from(core.bus.read8(address));
                core.set_r(rt, value);
                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDRH_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let data = core.bus.read16(address);
                if *wback {
                    core.set_r(rn, offset_address);
                }
                core.set_r(rt, data as u32);

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDRH_reg { rt, rn, rm } => {
            if core.condition_passed() {
                let address = core.get_r(rn) + core.get_r(rm);
                let value = u32::from(core.bus.read16(address));
                core.set_r(rt, value);
                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDRSH_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let rm_ = core.get_r(rm);
                let offset = shift(rm_, shift_t, *shift_n as usize, core.psr.get_c());

                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), offset, *add, *index);

                let data = u32::from(core.bus.read16(address));
                if *wback {
                    core.set_r(rn, offset_address);
                }

                core.set_r(rt, sign_extend(data, 15, 32) as u32);
                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDRSB_reg { rt, rn, rm } => {
            if core.condition_passed() {
                let address = core.get_r(rn) + core.get_r(rm);
                let data = u32::from(core.bus.read8(address));
                core.set_r(rt, sign_extend(data, 7, 32) as u32);
                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::SBC_reg {
            rn,
            rd,
            rm,
            setflags,
        } => {
            if core.condition_passed() {
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
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::STM {
            registers,
            rn,
            wback,
        } => {
            if core.condition_passed() {
                let regs_size = 4 * (registers.len() as u32);

                let mut address = core.get_r(rn);

                for reg in registers.iter() {
                    let r = core.get_r(&reg);
                    core.bus.write32(address, r);
                    address += 4;
                }

                if *wback {
                    core.add_r(rn, regs_size);
                }
                return ExecuteResult::Taken {
                    cycles: 1 + registers.len() as u64,
                };
            }
            ExecuteResult::NotTaken
        }

        Instruction::STR_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let value = core.get_r(rt);
                if *wback {
                    core.set_r(rn, offset_address);
                }

                core.bus.write32(address, value);

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::STRD_imm {
            rt,
            rt2,
            rn,
            imm32,
            index,
            add,
            wback,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let value1 = core.get_r(rt);
                core.bus.write32(address, value1);
                let value2 = core.get_r(rt2);
                core.bus.write32(address + 4, value2);

                if *wback {
                    core.set_r(rn, offset_address);
                }

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::LDRD_imm {
            rt,
            rt2,
            rn,
            imm32,
            index,
            add,
            wback,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let data = core.bus.read32(address);
                core.set_r(rt, data);
                let data2 = core.bus.read32(address + 4);
                core.set_r(rt2, data2);

                if *wback {
                    core.set_r(rn, offset_address);
                }

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::STR_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            thumb32,
            index,
            add,
            wback,
        } => {
            if core.condition_passed() {
                let c = core.psr.get_c();
                let offset = shift(core.get_r(rm), shift_t, *shift_n as usize, c);
                let address = core.get_r(rn) + offset;
                let value = core.get_r(rt);
                core.bus.write32(address, value);

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::STRB_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let c = core.psr.get_c();
                let offset = shift(core.get_r(rm), shift_t, *shift_n as usize, c);
                let address = core.get_r(rn) + offset;
                let value = core.get_r(rt).get_bits(0..8) as u8;
                core.bus.write8(address, value);
                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::STRB_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let value = core.get_r(rt);
                if *wback {
                    core.set_r(rn, offset_address);
                }

                core.bus.write8(address, value.get_bits(0..8) as u8);

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::STRH_imm {
            rt,
            rn,
            imm32,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let (address, offset_address) =
                    resolve_addressing(core.get_r(rn), *imm32, *add, *index);

                let value = core.get_r(rt);
                core.bus.write16(address, value.get_bits(0..16) as u16);

                if *wback {
                    core.set_r(rn, offset_address);
                }

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::STRH_reg {
            rt,
            rn,
            rm,
            shift_t,
            shift_n,
            index,
            add,
            wback,
            thumb32,
        } => {
            if core.condition_passed() {
                let c = core.psr.get_c();
                let offset = shift(core.get_r(rm), shift_t, *shift_n as usize, c);
                let address = core.get_r(rn) + offset;
                let value = core.get_r(rt).get_bits(0..16) as u16;
                core.bus.write16(address, value);
                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::LDR_lit {
            rt,
            imm32,
            add,
            thumb32,
        } => {
            if core.condition_passed() {
                let base = core.get_r(&Reg::PC) & 0xffff_fffc;
                let address = if *add { base + imm32 } else { base - imm32 };
                let data = core.bus.read32(address);

                if rt == &Reg::PC {
                    core.load_write_pc(data);
                } else {
                    core.set_r(rt, data);
                }

                return ExecuteResult::Taken { cycles: 2 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::ADD_reg {
            rd,
            rn,
            rm,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if core.condition_passed() {
                let c = core.psr.get_c();
                let shifted = shift(core.get_r(rm), shift_t, *shift_n as usize, c);
                let (result, carry, overflow) = add_with_carry(core.get_r(rn), shifted, false);

                if rd == &Reg::PC {
                    core.branch_write_pc(result);
                    return ExecuteResult::Branched { cycles: 3 };
                } else {
                    if *setflags {
                        core.psr.set_n(result);
                        core.psr.set_z(result);
                        core.psr.set_c(carry);
                        core.psr.set_v(overflow);
                    }
                    core.set_r(rd, result);
                    return ExecuteResult::Taken { cycles: 1 };
                }
            } else {
                ExecuteResult::NotTaken
            }
        }

        Instruction::ADD_imm {
            rn,
            rd,
            imm32,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let (result, carry, overflow) = add_with_carry(r_n, *imm32, false);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                }

                core.set_r(rd, result);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::ADR { rd, imm32, thumb32 } => {
            if core.condition_passed() {
                let result = (core.get_r(&Reg::PC) & 0xffff_fffc) + imm32;
                core.set_r(rd, result);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::RSB_imm {
            rd,
            rn,
            imm32,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let (result, carry, overflow) = add_with_carry(r_n ^ 0xFFFF_FFFF, *imm32, true);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                }

                core.set_r(rd, result);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::SUB_imm {
            rn,
            rd,
            imm32,
            setflags,
            thumb32,
        } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let (result, carry, overflow) = add_with_carry(r_n, imm32 ^ 0xFFFF_FFFF, true);

                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                    core.psr.set_v(overflow);
                }

                core.set_r(rd, result);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::SUB_reg {
            rn,
            rd,
            rm,
            setflags,
            shift_t,
            shift_n,
            thumb32,
        } => {
            if core.condition_passed() {
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
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::TBB { rn, rm } => {
            if core.condition_passed() {
                let r_n = core.get_r(rn);
                let r_m = core.get_r(rm);
                let pc = core.get_r(&Reg::PC);
                let halfwords = u32::from(core.bus.read8(r_n + r_m));

                core.branch_write_pc(pc + 2 * halfwords);

                return ExecuteResult::Branched { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::TST_reg { rn, rm } => {
            if core.condition_passed() {
                let result = core.get_r(rn) & core.get_r(rm);

                core.psr.set_n(result);
                core.psr.set_z(result);
                //core.psr.set_c(carry); carry = shift_c()
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::TST_imm { rn, imm32 } => {
            if core.condition_passed() {
                let (im, carry) = expand_conditional_carry(imm32, core.psr.get_c());

                let result = core.get_r(rn) & im;

                core.psr.set_n(result);
                core.psr.set_z(result);
                core.psr.set_c(carry);

                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        // ARMv7-M
        Instruction::UBFX {
            rd,
            rn,
            lsb,
            widthminus1,
        } => {
            if core.condition_passed() {
                let msbit = lsb + widthminus1;
                if msbit <= 31 {
                    let data = core.get_r(rn).get_bits(*lsb..(msbit + 1));
                    core.set_r(rd, data);
                } else {
                    panic!();
                }

                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::UXTB {
            rd,
            rm,
            thumb32,
            rotation,
        } => {
            if core.condition_passed() {
                let rotated = ror(core.get_r(rm), *rotation);
                core.set_r(rd, rotated.get_bits(0..8));
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::UXTH {
            rd,
            rm,
            rotation,
            thumb32,
        } => {
            if core.condition_passed() {
                let rotated = ror(core.get_r(rm), *rotation);
                core.set_r(rd, rotated.get_bits(0..16));
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::SXTB {
            rd,
            rm,
            rotation,
            thumb32,
        } => {
            if core.condition_passed() {
                let rotated = ror(core.get_r(rm), *rotation);
                core.set_r(rd, sign_extend(rotated.get_bits(0..8), 7, 32) as u32);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        Instruction::SXTH {
            rd,
            rm,
            rotation,
            thumb32,
        } => {
            if core.condition_passed() {
                let rotated = ror(core.get_r(rm), *rotation);
                core.set_r(rd, sign_extend(rotated.get_bits(0..16), 15, 32) as u32);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::REV { rd, rm } => {
            if core.condition_passed() {
                let rm_ = core.get_r(rm);
                core.set_r(
                    rd,
                    ((rm_ & 0xff) << 24)
                        + ((rm_ & 0xff00) << 8)
                        + ((rm_ & 0xff_0000) >> 8)
                        + ((rm_ & 0xff00_0000) >> 24),
                );
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::REV16 { rd, rm } => {
            if core.condition_passed() {
                let rm_ = core.get_r(rm);
                core.set_r(
                    rd,
                    ((rm_ & 0xff) << 8)
                        + ((rm_ & 0xff00) >> 8)
                        + ((rm_ & 0xff_0000) << 8)
                        + ((rm_ & 0xff00_0000) >> 8),
                );
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::REVSH { rd, rm } => {
            if core.condition_passed() {
                let rm_ = core.get_r(rm);
                core.set_r(
                    rd,
                    ((sign_extend(rm_ & 0xff, 7, 24) as u32) << 8) + ((rm_ & 0xff00) >> 8),
                );
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::ROR_reg {
            rd,
            rn,
            rm,
            setflags,
        } => {
            if core.condition_passed() {
                let shift_n = core.get_r(rm) & 0xff;
                let (result, carry) = shift_c(
                    core.get_r(rn),
                    &SRType::ROR,
                    shift_n as usize,
                    core.psr.get_c(),
                );
                core.set_r(rd, result);
                if *setflags {
                    core.psr.set_n(result);
                    core.psr.set_z(result);
                    core.psr.set_c(carry);
                }
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::SVC { imm32 } => {
            if core.condition_passed() {
                println!("SVC {}", imm32);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::SEV => {
            if core.condition_passed() {
                println!("SEV");
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::WFE => {
            if core.condition_passed() {
                //TODO
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::WFI => {
            if core.condition_passed() {
                //TODO
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        Instruction::YIELD => {
            if core.condition_passed() {
                println!("YIELD");
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        // ARMv7-M
        Instruction::MCR {
            rt,
            coproc,
            opc1,
            opc2,
            crn,
            crm,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::MCR2 {
            rt,
            coproc,
            opc1,
            opc2,
            crn,
            crm,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::LDC_imm {
            coproc,
            imm32,
            crd,
            rn,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::LDC2_imm {
            coproc,
            imm32,
            crd,
            rn,
        } => unimplemented!(),

        // ARMv7-M
        Instruction::UDIV { rd, rn, rm } => {
            if core.condition_passed() {
                let rm_ = core.get_r(rm);
                let result = if rm_ == 0 {
                    if core.integer_zero_divide_trapping_enabled() {
                        return ExecuteResult::Fault {
                            fault: Fault::DivideByZero,
                        };
                    }
                    0
                } else {
                    let rn_ = core.get_r(rn);
                    rn_ / rm_
                };
                core.set_r(rd, result);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        // ARMv7-M
        Instruction::SDIV { rd, rn, rm } => {
            if core.condition_passed() {
                let rm_ = core.get_r(rm);
                let result = if rm_ == 0 {
                    if core.integer_zero_divide_trapping_enabled() {
                        return ExecuteResult::Fault {
                            fault: Fault::DivideByZero,
                        };
                    }
                    0
                } else {
                    let rn_ = core.get_r(rn);
                    (rn_ as i32) / (rm_ as i32)
                };
                core.set_r(rd, result as u32);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        // ARMv7-M
        Instruction::MLA { rd, rn, rm, ra } => {
            if core.condition_passed() {
                let rn_ = core.get_r(rn);
                let rm_ = core.get_r(rm);
                let ra_ = core.get_r(ra);
                let result = rn_.wrapping_mul(rm_).wrapping_add(ra_);

                core.set_r(rd, result);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        // ARMv7-M
        Instruction::MLS { rd, rn, rm, ra } => {
            if core.condition_passed() {
                let rn_ = core.get_r(rn);
                let rm_ = core.get_r(rm);
                let ra_ = core.get_r(ra);
                let result = ra_.wrapping_sub(rn_.wrapping_mul(rm_));

                core.set_r(rd, result);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }
        // ARMv7-M
        Instruction::UMLAL { rdlo, rdhi, rn, rm } => unimplemented!(),
        // ARMv7-M
        Instruction::UMULL { rdlo, rdhi, rn, rm } => {
            if core.condition_passed() {
                let rn_ = core.get_r(rn) as u64;
                let rm_ = core.get_r(rm) as u64;
                let result = rn_.wrapping_mul(rm_);

                core.set_r(rdlo, result.get_bits(0..32) as u32);
                core.set_r(rdhi, result.get_bits(32..64) as u32);
                return ExecuteResult::Taken { cycles: 1 };
            }
            ExecuteResult::NotTaken
        }

        // ARMv7-M
        Instruction::SMLAL { rdlo, rdhi, rn, rm } => unimplemented!(),

        Instruction::UDF { imm32, opcode } => {
            println!("UDF {}, {}", imm32, opcode);
            panic!("undefined");
            //Some(Fault::UndefinedInstruction)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::condition::Condition;
    use crate::core::instruction::ITCondition;
    use crate::memory::ram::*;

    #[test]
    fn test_udiv() {
        // arrange
        let mut bus = RAM::new(0, 1000);
        let mut core = Core::new(&mut bus);
        core.set_r(&Reg::R0, 0x7d0);
        core.set_r(&Reg::R1, 0x3);
        core.psr.value = 0;

        let instruction = Instruction::UDIV {
            rd: Reg::R0,
            rn: Reg::R0,
            rm: Reg::R1,
        };

        // act
        let result = execute(
            &mut core,
            &instruction,
            |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                panic!("should not happen.")
            },
        );

        assert_eq!(result, ExecuteResult::Taken { cycles: 1 });

        assert_eq!(core.get_r(&Reg::R0), 0x29a);
        assert_eq!(core.get_r(&Reg::R1), 0x3);
    }

    #[test]
    fn test_mla() {
        // arrange
        let mut bus = RAM::new(0, 1000);
        let mut core = Core::new(&mut bus);
        core.set_r(&Reg::R7, 0x2);
        core.set_r(&Reg::R2, 0x29a);
        core.set_r(&Reg::R1, 0x2000089C);
        core.psr.value = 0;

        let instruction = Instruction::MLA {
            rd: Reg::R1,
            rn: Reg::R7,
            rm: Reg::R2,
            ra: Reg::R1,
        };

        // act
        let result = execute(
            &mut core,
            &instruction,
            |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                panic!("should not happen.")
            },
        );

        assert_eq!(result, ExecuteResult::Taken { cycles: 1 });

        assert_eq!(core.get_r(&Reg::R1), 0x20000DD0);
    }

    /*"it ne" blokki (alla) ei suoritu oikein, iarissa mov r4, #0 ei ajeta, mutta emussa ajetaan.
    APSR =
        Q = 0
        V = 0
        C = 1
        Z = 1
        N = 0

       BD16        pop {R1, R2, R4, PC}             0x00003484    putchar                             15077 r0:00000049 r1:00000049 r2:00000000 r3:200007e0 r4:00000001 r5:00000049 r6:00000014 r7:0000377d r8:0000373d r9:7fffffff
       42A8        cmp r0, r5                       0x00003748    _Prout                              15078 r0:00000049 r1:00000049 r2:00000000 r3:200007e0 r4:00000001 r5:00000049 r6:00000014 r7:0000377d r8:0000373d r9:7fffffff
       BF18        it ne                            0x0000374A    _Prout                              15082 r0:00000049 r1:00000049 r2:00000000 r3:200007e0 r4:00000001 r5:00000049 r6:00000014 r7:0000377d r8:0000373d r9:7fffffff
       2400        mov r4, #0                       0x0000374C    _Prout                              15083 r0:00000049 r1:00000049 r2:00000000 r3:200007e0 r4:00000000 r5:00000049 r6:00000014 r7:0000377d r8:0000373d r9:7fffffff
    */
    #[test]
    fn test_it_block() {
        // arrange
        let mut bus = RAM::new(0, 1000);
        let mut core = Core::new(&mut bus);
        core.set_r(&Reg::R5, 0x49);
        core.set_r(&Reg::R4, 0x01);
        core.set_r(&Reg::R0, 0x49);
        core.psr.value = 0;

        let i1 = Instruction::CMP_reg {
            rn: Reg::R0,
            rm: Reg::R5,
        };

        let i2 = Instruction::IT {
            x: Some(ITCondition::Then),
            y: None,
            z: None,
            firstcond: Condition::NE,
            mask: 0b1000,
        };
        let i3 = Instruction::MOV_imm {
            rd: Reg::R4,
            imm32: Imm32Carry::NoCarry { imm32: 0 },
            setflags: false,
            thumb32: false,
        };

        core.step(
            &i1,
            |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                panic!("should not happen.")
            },
        );
        core.step(
            &i2,
            |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                panic!("should not happen.")
            },
        );
        core.step(
            &i3,
            |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                panic!("should not happen.")
            },
        );

        assert_eq!(core.get_r(&Reg::R4), 0x01);
        assert!(!core.in_it_block());
    }

    #[test]
    fn test_b_cond() {
        // arrange
        let mut bus = RAM::new(0, 1000);
        let mut core = Core::new(&mut bus);
        core.psr.value = 0;

        let instruction = Instruction::B {
            cond: Condition::EQ,
            imm32: 0,
            thumb32: true,
        };

        // act
        let result = execute(
            &mut core,
            &instruction,
            |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                panic!("should not happen.")
            },
        );

        assert_eq!(result, ExecuteResult::NotTaken);
    }

    #[test]
    fn test_bfi() {
        // arrange
        let mut bus = RAM::new(0, 1000);
        let mut core = Core::new(&mut bus);
        core.psr.value = 0;

        core.set_r(&Reg::R2, 0x11223344);
        core.set_r(&Reg::R3, 0xaabbccdd);
        core.psr.value = 0;

        let instruction = Instruction::BFI {
            rd: Reg::R2,
            rn: Reg::R3,
            lsbit: 0,
            msbit: 7,
        };

        core.step(
            &instruction,
            |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                panic!("should not happen.")
            },
        );

        assert_eq!(core.get_r(&Reg::R3), 0xaabbccdd);
        assert_eq!(core.get_r(&Reg::R2), 0x112233dd);
    }

}
