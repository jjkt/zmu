use crate::core::instruction::VPushPopParams;
use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::ExecuteResult;
use crate::{
    bus::Bus,
    core::{
        instruction::VLoadAndStoreParams,
        register::{BaseReg, ExtensionReg, ExtensionRegOperations, Reg},
    },
};

/// Multiply operations
pub trait IsaFloatingPointLoadAndStore {
    fn exec_vldr(&mut self, params: &VLoadAndStoreParams) -> ExecuteResult;
    fn exec_vstr(&mut self, params: &VLoadAndStoreParams) -> ExecuteResult;
    fn exec_vpush(&mut self, params: &VPushPopParams) -> ExecuteResult;
    fn exec_vpop(&mut self, params: &VPushPopParams) -> ExecuteResult;
}

impl IsaFloatingPointLoadAndStore for Processor {
    fn exec_vldr(&mut self, params: &VLoadAndStoreParams) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();

            let base = match params.rn {
                Reg::PC => self.get_r(Reg::PC) & 0xffff_fffc,
                _ => self.get_r(params.rn),
            };

            let address = if params.add {
                base + params.imm32
            } else {
                base - params.imm32
            };
            match params.dd {
                ExtensionReg::Single { reg } => {
                    let data = self.read32(address)?;
                    self.set_sr(reg, data);
                }
                ExtensionReg::Double { reg } => {
                    let word1 = self.read32(address)?;
                    let word2 = self.read32(address + 4)?;
                    self.set_dr(reg, word1, word2);
                }
            }

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vstr(&mut self, params: &VLoadAndStoreParams) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();

            let base = self.get_r(params.rn);

            let address = if params.add {
                base + params.imm32
            } else {
                base - params.imm32
            };
            match params.dd {
                ExtensionReg::Single { reg } => {
                    let value = self.get_sr(reg);
                    self.write32(address, value)?;
                }
                ExtensionReg::Double { reg } => {
                    let (low_word, high_word) = self.get_dr(reg);
                    self.write32(address, low_word)?;
                    self.write32(address + 4, high_word)?;
                }
            }

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vpush(&mut self, params: &VPushPopParams) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();

            let sp = self.get_r(Reg::SP);
            let mut address = sp - params.imm32;
            self.set_r(Reg::SP, address);

            if params.single_regs {
                for reg in &params.single_precision_registers {
                    let value = self.get_sr(reg);
                    self.write32(address, value)?;
                    address += 4;
                }
            } else {
                for reg in &params.double_precision_registers {
                    let (low_word, high_word) = self.get_dr(reg);
                    if self.big_endian() {
                        self.write32(address, high_word)?;
                        self.write32(address + 4, low_word)?;
                    } else {
                        self.write32(address, low_word)?;
                        self.write32(address + 4, high_word)?;
                    }
                    address += 8;
                }
            }

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_vpop(&mut self, params: &VPushPopParams) -> ExecuteResult {
        if self.condition_passed() {
            //self.execute_fp_check();

            let sp = self.get_r(Reg::SP);
            let mut address = sp;
            self.set_r(Reg::SP, sp + params.imm32);
            if params.single_regs {
                for reg in &params.single_precision_registers {
                    address += 4;
                    let value = self.read32(address)?;
                    self.set_sr(reg, value);
                }
            } else {
                for reg in &params.double_precision_registers {
                    address += 8;
                    let low_word = self.read32(address)?;
                    let high_word = self.read32(address + 4)?;
                    self.set_dr(reg, low_word, high_word);
                }
            }

            return Ok(ExecuteSuccess::Taken { cycles: 1 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
