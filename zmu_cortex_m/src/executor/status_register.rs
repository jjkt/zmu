use crate::Processor;

use crate::executor::{ExecuteSuccess, ExecutorHelper};

use super::ExecuteResult;

use crate::{
    ProcessorMode,
    core::{
        bits::Bits,
        exception::ExceptionHandling,
        instruction::{MrsParams, MsrParams},
        register::BaseReg,
    },
};

/// Branching operations
pub trait IsaStatusRegister {
    fn exec_mrs(&mut self, params: MrsParams) -> ExecuteResult;
    fn exec_msr(&mut self, params: MsrParams) -> ExecuteResult;

    #[cfg(feature = "armv6m")]
    fn exec_cps(&mut self, im: bool) -> ExecuteResult;

    #[cfg(not(feature = "armv6m"))]
    fn exec_cps(&mut self, im: bool, affect_pri: bool, affect_fault: bool) -> ExecuteResult;
}

impl IsaStatusRegister for Processor {
    fn exec_mrs(&mut self, params: MrsParams) -> ExecuteResult {
        if self.condition_passed() {
            let mut value: u32 = 0;
            match params.sysm.get_bits(3..8) {
                0b00000 => {
                    if params.sysm.get_bit(0) {
                        value.set_bits(0..9, self.psr.value.get_bits(0..9));
                    }
                    if params.sysm.get_bit(1) {
                        value.set_bits(24..27, 0);
                        value.set_bits(10..16, 0);
                    }
                    if !params.sysm.get_bit(2) {
                        value.set_bits(27..32, self.psr.value.get_bits(27..32));
                    }
                }
                0b00001 => match params.sysm.get_bits(0..3) {
                    0 => {
                        value = self.msp;
                    }
                    1 => {
                        value = self.psp;
                    }
                    _ => (),
                },
                0b00010 => match params.sysm.get_bits(0..3) {
                    0b000 => {
                        value.set_bit(0, self.primask);
                    }
                    0b001 => {
                        value.set_bits(0..8, u32::from(self.basepri));
                    }
                    0b010 => {
                        value.set_bits(0..8, u32::from(self.basepri));
                    }
                    #[cfg(not(feature = "armv6m"))]
                    0b011 => {
                        value.set_bit(0, self.faultmask);
                    }
                    0b100 => {
                        //let ctrl = u8::from(self.control) as u32;
                        //value.set_bits(0..2, ctrl);
                        todo!("unimplemented CONTROL");
                    }
                    _ => (),
                },
                _ => (),
            }
            self.set_r(params.rd, value);
            return Ok(ExecuteSuccess::Taken { cycles: 4 });
        }

        Ok(ExecuteSuccess::NotTaken)
    }

    fn exec_msr(&mut self, params: MsrParams) -> ExecuteResult {
        if self.condition_passed() {
            let r_n = self.get_r(params.rn);
            match params.sysm.get_bits(3..8) {
                0b00000 => {
                    if !params.sysm.get_bit(2) {
                        if params.mask.get_bit(0) {
                            //GE extensions
                            self.psr.value.set_bits(16..20, r_n.get_bits(16..20));
                        }
                        if params.mask.get_bit(1) {
                            // N, Z, C, V, Q
                            self.psr.value.set_bits(27..32, r_n.get_bits(27..32));
                        }
                    }
                }
                0b00001 => match params.sysm.get_bits(0..3) {
                    0 => self.msp = r_n,
                    1 => self.psp = r_n,
                    _ => (),
                },
                0b00010 => match params.sysm.get_bits(0..3) {
                    0b000 => {
                        self.primask = r_n.get_bit(0);
                        self.execution_priority = self.get_execution_priority();
                    }
                    0b001 => {
                        self.basepri = r_n.get_bits(0..8) as u8;
                        self.execution_priority = self.get_execution_priority();
                    }
                    0b010 => {
                        let low_rn = r_n.get_bits(0..8) as u8;
                        if low_rn != 0 && low_rn < self.basepri || self.basepri == 0 {
                            self.basepri = low_rn;
                            self.execution_priority = self.get_execution_priority();
                        }
                    }
                    #[cfg(not(feature = "armv6m"))]
                    0b011 => {
                        if self.execution_priority > -1 {
                            self.faultmask = r_n.get_bit(0);
                            self.execution_priority = self.get_execution_priority();
                        }
                    }
                    0b100 => {
                        self.control.n_priv = r_n.get_bit(0);
                        if self.mode == ProcessorMode::ThreadMode {
                            self.control.sp_sel = r_n.get_bit(1);
                        }
                    }
                    _ => (),
                },
                _ => (),
            }

            return Ok(ExecuteSuccess::Taken { cycles: 4 });
        }
        Ok(ExecuteSuccess::NotTaken)
    }

    #[cfg(feature = "armv6m")]
    fn exec_cps(&mut self, im: bool) -> ExecuteResult {
        if im {
            self.primask = true;
        } else {
            self.primask = false;
        }
        self.execution_priority = self.get_execution_priority();
        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }

    #[cfg(not(feature = "armv6m"))]
    fn exec_cps(&mut self, im: bool, affect_pri: bool, affect_fault: bool) -> ExecuteResult {
        if im {
            if affect_pri {
                self.primask = true;
            }
            if affect_fault && self.execution_priority > -1 {
                self.faultmask = true;
            }
        } else {
            if affect_pri {
                self.primask = false;
            }
            if affect_fault {
                self.faultmask = false;
            }
        }
        self.execution_priority = self.get_execution_priority();
        Ok(ExecuteSuccess::Taken { cycles: 1 })
    }
}
