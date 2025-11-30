use super::ExecuteResult;
use crate::Processor;
use crate::executor::{ExecuteSuccess, ExecutorHelper};

use crate::{
    bus::Bus,
    core::{
        instruction::LoadAndStoreMultipleParams,
        register::{BaseReg, Reg},
    },
};
use enum_set::EnumSet;

/// Load and Store operations
pub trait IsaLoadAndStoreMultiple {
    fn exec_stm(&mut self, params: &LoadAndStoreMultipleParams) -> ExecuteResult;
    fn exec_stmdb(&mut self, params: &LoadAndStoreMultipleParams) -> ExecuteResult;
    fn exec_ldm(&mut self, params: &LoadAndStoreMultipleParams) -> ExecuteResult;
    fn exec_push(&mut self, registers: EnumSet<Reg>) -> ExecuteResult;
    fn exec_pop(&mut self, registers: EnumSet<Reg>) -> ExecuteResult;
}

impl IsaLoadAndStoreMultiple for Processor {
    fn exec_stm(&mut self, params: &LoadAndStoreMultipleParams) -> ExecuteResult {
        if self.condition_passed() {
            let regs_size = 4 * (params.registers.len() as u32);

            let mut address = self.get_r(params.rn);

            for reg in &params.registers {
                let r = self.get_r(reg);
                self.write32(address, r)?;
                address += 4;
            }

            if params.wback {
                self.add_r(params.rn, regs_size);
            }
            return Ok(ExecuteSuccess::Taken {
                cycles: 1 + params.registers.len() as u32,
            });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_stmdb(&mut self, params: &LoadAndStoreMultipleParams) -> ExecuteResult {
        if self.condition_passed() {
            let regs_size = 4 * (params.registers.len() as u32);

            let mut address = self.get_r(params.rn) - regs_size;

            for reg in &params.registers {
                let r = self.get_r(reg);
                self.write32(address, r)?;
                address += 4;
            }

            if params.wback {
                self.sub_r(params.rn, regs_size);
            }
            return Ok(ExecuteSuccess::Taken {
                cycles: 1 + params.registers.len() as u32,
            });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldm(&mut self, params: &LoadAndStoreMultipleParams) -> ExecuteResult {
        if self.condition_passed() {
            let regs_size = 4 * (params.registers.len() as u32);

            let mut address = self.get_r(params.rn);

            let mut branched = false;
            for reg in &params.registers {
                let value = self.read32(address)?;
                if reg == Reg::PC {
                    self.load_write_pc(value)?;
                    branched = true;
                } else {
                    self.set_r(reg, value);
                }
                address += 4;
            }

            if !params.registers.contains(&params.rn) {
                self.add_r(params.rn, regs_size);
            }
            let cc = 1 + params.registers.len() as u32;
            if branched {
                return Ok(ExecuteSuccess::Branched { cycles: cc });
            }
            return Ok(ExecuteSuccess::Taken { cycles: cc });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_push(&mut self, registers: EnumSet<Reg>) -> ExecuteResult {
        if self.condition_passed() {
            let regs_size = 4 * (registers.len() as u32);
            let sp = self.get_r(Reg::SP);
            let mut address = sp - regs_size;

            for reg in &registers {
                let value = self.get_r(reg);
                self.write32(address, value)?;
                address += 4;
            }

            self.set_r(Reg::SP, sp - regs_size);
            return Ok(ExecuteSuccess::Taken {
                cycles: 1 + registers.len() as u32,
            });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_pop(&mut self, registers: EnumSet<Reg>) -> ExecuteResult {
        if self.condition_passed() {
            let regs_size = 4 * (registers.len() as u32);
            let sp = self.get_r(Reg::SP);
            let mut address = sp;

            self.set_r(Reg::SP, sp + regs_size);

            for reg in &registers {
                let val = self.read32(address)?;
                if reg == Reg::PC {
                    self.bx_write_pc(val)?;
                } else {
                    self.set_r(reg, val);
                }
                address += 4;
            }

            if registers.contains(&Reg::PC) {
                return Ok(ExecuteSuccess::Branched {
                    cycles: 4 + registers.len() as u32,
                });
            } else {
                return Ok(ExecuteSuccess::Taken {
                    cycles: 1 + registers.len() as u32,
                });
            }
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
