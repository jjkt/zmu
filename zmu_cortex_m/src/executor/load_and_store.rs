use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::{resolve_addressing, ExecuteResult};

use crate::{
    bus::Bus,
    core::{
        instruction::Reg3FullParams,
        operation::{shift, sign_extend},
        register::{Apsr, BaseReg, Reg},
    },
};

/// Branching operations
pub trait IsaLoadAndStore {
    fn exec_ldr_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_ldrb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_ldrh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_ldrsh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
    fn exec_ldrsb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult;
}

impl IsaLoadAndStore for Processor {
    fn exec_ldr_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();
            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let rn = self.get_r(params.rn);
            let (address, offset_address) =
                resolve_addressing(rn, offset, params.add, params.index);

            let data = self.read32(address)?;
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            if params.rt == Reg::PC {
                self.load_write_pc(data)?;
                return Ok(ExecuteSuccess::Branched { cycles: 2 });
            } else {
                self.set_r(params.rt, data);
                return Ok(ExecuteSuccess::Taken { cycles: 2 });
            }
        }

        Ok(ExecuteSuccess::NotTaken)
    }
    
    fn exec_ldrb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();

            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let (address, offset_address) =
                resolve_addressing(self.get_r(params.rn), offset, params.add, params.index);

            let data = u32::from(self.read8(address)?);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, data);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();
            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let (address, offset_address) =
                resolve_addressing(self.get_r(params.rn), offset, params.add, params.index);

            let data = u32::from(self.read16(address)?);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, data);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrsh_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();
            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let (address, offset_address) =
                resolve_addressing(self.get_r(params.rn), offset, params.add, params.index);

            let data = u32::from(self.read16(address)?);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, sign_extend(data, 15, 32) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_ldrsb_reg(&mut self, params: &Reg3FullParams) -> ExecuteResult {
        if self.condition_passed() {
            let rm = self.get_r(params.rm);
            let c = self.psr.get_c();
            let offset = shift(rm, params.shift_t, params.shift_n as usize, c);

            let (address, offset_address) =
                resolve_addressing(self.get_r(params.rn), offset, params.add, params.index);

            let data = u32::from(self.read8(address)?);
            if params.wback {
                self.set_r(params.rn, offset_address);
            }

            self.set_r(params.rt, sign_extend(data, 7, 32) as u32);
            return Ok(ExecuteSuccess::Taken { cycles: 2 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }
}
