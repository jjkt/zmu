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
                    //PSR
                    if params.sysm.get_bit(0) {
                        value.set_bits(0..9, self.psr.value.get_bits(0..9));
                    }
                    if params.sysm.get_bit(1) {
                        value.set_bits(24..27, 0);
                        value.set_bits(10..16, 0);
                    }
                    if !params.sysm.get_bit(2) {
                        value.set_bits(27..32, self.psr.value.get_bits(27..32));
                        //TODO have_dsp
                    }
                }
                0b00001 => {
                    if self.current_mode_is_privileged() {
                        match params.sysm.get_bits(0..3) {
                            // PSP, MSP
                            0 => {
                                value = self.msp;
                            }
                            1 => {
                                value = self.psp;
                            }
                            _ => (),
                        }
                    }
                }
                0b00010 => match params.sysm.get_bits(0..3) {
                    0b000 => {
                        //PRIMASK
                        if self.current_mode_is_privileged() {
                            value.set_bit(0, self.primask);
                        }
                    }
                    #[cfg(not(feature = "armv6m"))]
                    0b001 => {
                        //BASEPRI
                        if self.current_mode_is_privileged() {
                            value.set_bits(0..8, u32::from(self.basepri));
                        }
                    }
                    #[cfg(not(feature = "armv6m"))]
                    0b010 => {
                        //BASEPRI_MAX
                        if self.current_mode_is_privileged() {
                            value.set_bits(0..8, u32::from(self.basepri));
                        }
                    }
                    #[cfg(not(feature = "armv6m"))]
                    0b011 => {
                        //FAULTMASK
                        if self.current_mode_is_privileged() {
                            value.set_bit(0, self.faultmask);
                        }
                    }
                    0b100 => {
                        //CONTROL
                        value.set_bit(0, self.control.n_priv);
                        value.set_bit(1, self.control.sp_sel);
                        #[cfg(feature = "has-fp")]
                        value.set_bit(2, self.control.fpca);
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
                    //PSR
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
                0b00001 => {
                    if self.current_mode_is_privileged() {
                        match params.sysm.get_bits(0..3) {
                            //PSP, MSP
                            0 => self.msp = r_n,
                            1 => self.psp = r_n,
                            _ => (),
                        }
                    }
                }
                0b00010 => match params.sysm.get_bits(0..3) {
                    0b000 => {
                        if self.current_mode_is_privileged() {
                            //PRIMASK
                            self.primask = r_n.get_bit(0);
                            self.execution_priority = self.get_execution_priority();
                        }
                    }
                    #[cfg(not(feature = "armv6m"))]
                    0b001 => {
                        if self.current_mode_is_privileged() {
                            //BASEPRI
                            self.basepri = r_n.get_bits(0..8) as u8;
                            self.execution_priority = self.get_execution_priority();
                        }
                    }
                    #[cfg(not(feature = "armv6m"))]
                    0b010 => {
                        //BASEPRI_MAX
                        if self.current_mode_is_privileged() {
                            let low_rn = r_n.get_bits(0..8) as u8;
                            if low_rn != 0 && low_rn < self.basepri || self.basepri == 0 {
                                self.basepri = low_rn;
                                self.execution_priority = self.get_execution_priority();
                            }
                        }
                    }
                    #[cfg(not(feature = "armv6m"))]
                    0b011 => {
                        //FAULTMASK
                        if self.current_mode_is_privileged() && self.execution_priority > -1 {
                            self.faultmask = r_n.get_bit(0);
                            self.execution_priority = self.get_execution_priority();
                        }
                    }
                    0b100 => {
                        if self.current_mode_is_privileged() {
                            //CONTROL
                            self.control.n_priv = r_n.get_bit(0);
                            if self.mode == ProcessorMode::ThreadMode {
                                self.control.sp_sel = r_n.get_bit(1);
                            }
                            // if have_fp, set control.fpca to r[n]<2>
                            #[cfg(feature = "has-fp")]
                            {
                                self.control.fpca = r_n.get_bit(2);
                            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::register::Apsr;
    use crate::core::register::Ipsr;
    use crate::core::register::Reg;
    use crate::core::register::SpecialReg;

    #[test]
    fn test_exec_mrs_apsr() {
        // Arrange
        let mut processor = Processor::new();

        processor.psr.set_q(true);
        processor.psr.set_v(true);
        processor.psr.set_c(true);
        processor.psr.set_z_bit(true);
        processor.psr.set_n_bit(true);
        processor.psr.set_isr_number(4);

        //TODO: has_dsp_ext -> ge bits

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R1,
                sysm: u8::from(SpecialReg::APSR),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R1), 0xF800_0000);
    }

    #[test]
    fn test_exec_mrs_iapsr() {
        // Arrange
        let mut processor = Processor::new();

        processor.psr.set_q(true);
        processor.psr.set_v(true);
        processor.psr.set_c(true);
        processor.psr.set_z_bit(true);
        processor.psr.set_n_bit(true);
        processor.psr.set_isr_number(4);

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R1,
                sysm: u8::from(SpecialReg::IAPSR),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R1), 0xF800_0004);
    }

    #[test]
    fn test_exec_mrs_eapsr() {
        // Arrange
        let mut processor = Processor::new();

        processor.psr.set_q(true);
        processor.psr.set_v(true);
        processor.psr.set_c(true);
        processor.psr.set_z_bit(true);
        processor.psr.set_n_bit(true);
        processor.psr.set_isr_number(4);
        processor.set_r(Reg::R1, 0xffff_ffff);

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R1,
                sysm: u8::from(SpecialReg::EAPSR),
            })
            .unwrap();

        // Assert (EPSR reads as zero, APSR read, exception ignored)
        assert_eq!(processor.get_r(Reg::R1), 0xF800_0000);
    }

    #[test]
    fn test_exec_mrs_xpsr() {
        // Arrange
        let mut processor = Processor::new();

        processor.psr.set_q(true);
        processor.psr.set_v(true);
        processor.psr.set_c(true);
        processor.psr.set_z_bit(true);
        processor.psr.set_n_bit(true);
        processor.psr.set_isr_number(4);
        processor.set_r(Reg::R2, 0xffff_ffff);

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::XPSR),
            })
            .unwrap();

        // Assert (EPSR reads as zero, APSR read, exception read)
        assert_eq!(processor.get_r(Reg::R2), 0xF800_0004);
    }

    #[test]
    fn test_exec_mrs_ipsr() {
        // Arrange
        let mut processor = Processor::new();

        processor.psr.set_q(true);
        processor.psr.set_v(true);
        processor.psr.set_c(true);
        processor.psr.set_z_bit(true);
        processor.psr.set_n_bit(true);
        processor.psr.set_isr_number(4);
        processor.set_r(Reg::R2, 0xffff_ffff);

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::IPSR),
            })
            .unwrap();

        // Assert (EPSR reads as zero, APSR ignored, exception read)
        assert_eq!(processor.get_r(Reg::R2), 0x0000_0004);
    }

    #[test]
    fn test_exec_mrs_epsr() {
        // Arrange
        let mut processor = Processor::new();

        processor.psr.set_q(true);
        processor.psr.set_v(true);
        processor.psr.set_c(true);
        processor.psr.set_z_bit(true);
        processor.psr.set_n_bit(true);
        processor.psr.set_isr_number(4);
        processor.set_r(Reg::R2, 0xffff_ffff);

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::EPSR),
            })
            .unwrap();

        // Assert (EPSR reads as zero, APSR ignored, exception ignored)
        assert_eq!(processor.get_r(Reg::R2), 0x0000_0000);
    }

    #[test]
    fn test_exec_mrs_iepsr() {
        // Arrange
        let mut processor = Processor::new();

        processor.psr.set_q(true);
        processor.psr.set_v(true);
        processor.psr.set_c(true);
        processor.psr.set_z_bit(true);
        processor.psr.set_n_bit(true);
        processor.psr.set_isr_number(9);
        processor.set_r(Reg::R2, 0xffff_ffff);

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::IEPSR),
            })
            .unwrap();

        // Assert (EPSR reads as zero, APSR ignored, exception read)
        assert_eq!(processor.get_r(Reg::R2), 0x0000_0009);
    }

    #[test]
    fn test_exec_mrs_msp_privileged_thread_mode() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_msp(0xcafe_babe);
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = false; // privileged

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::MSP),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0xcafe_babe);
    }

    #[test]
    fn test_exec_mrs_msp_unprivileged_thread_mode() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_msp(0xcafe_babe);
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = true; // unprivileged

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::MSP),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0);
    }

    #[test]
    fn test_exec_mrs_msp_privileged_handler_mode() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_msp(0xcafe_babe);
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::HandlerMode;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::MSP),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0xcafe_babe);
    }

    #[test]
    fn test_exec_mrs_msp_unprivileged_handler_mode() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_msp(0xcafe_babe);
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::HandlerMode;
        processor.control.n_priv = true;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::MSP),
            })
            .unwrap();

        // Assert (handler mode remains privileged)
        assert_eq!(processor.get_r(Reg::R2), 0xcafe_babe);
    }

    #[test]
    fn test_exec_mrs_psp_privileged_thread_mode() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_msp(0xcafe_babe);
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = false; // privileged

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::PSP),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0xdead_beef);
    }

    #[test]
    fn test_exec_mrs_psp_unprivileged_thread_mode() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_msp(0xcafe_babe);
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = true; // unprivileged

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::PSP),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0);
    }

    #[test]
    fn test_exec_mrs_psp_privileged_handler_mode() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_msp(0xcafe_babe);
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::HandlerMode;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::PSP),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0xdead_beef);
    }

    #[test]
    fn test_exec_mrs_psp_unprivileged_handler_mode() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_msp(0xcafe_babe);
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::HandlerMode;
        processor.control.n_priv = true;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::PSP),
            })
            .unwrap();

        // Assert (handler mode remains privileged)
        assert_eq!(processor.get_r(Reg::R2), 0xdead_beef);
    }

    #[test]
    fn test_exec_mrs_primask_privileged_true() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.primask = true;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::PRIMASK),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 1);
    }

    #[test]
    fn test_exec_mrs_primask_unprivileged_true() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.primask = true;
        processor.control.n_priv = true; // unprivileged
        processor.mode = ProcessorMode::ThreadMode;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::PRIMASK),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0);
    }

    #[test]
    fn test_exec_mrs_primask_unprivileged_handler_mode_reads_value() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.primask = true;
        processor.control.n_priv = true;
        processor.mode = ProcessorMode::HandlerMode;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::PRIMASK),
            })
            .unwrap();

        // Assert (handler mode remains privileged)
        assert_eq!(processor.get_r(Reg::R2), 1);
    }

    #[test]
    fn test_exec_mrs_primask_false() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.primask = false;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::PRIMASK),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_mrs_basepri() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.basepri = 0x80;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::BASEPRI),
            })
            .unwrap();

        // Assert (value got)
        assert_eq!(processor.get_r(Reg::R2), 0x80);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_mrs_basepri_unprivileged_thread_mode_reads_zero() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.basepri = 0x80;
        processor.control.n_priv = true;
        processor.mode = ProcessorMode::ThreadMode;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::BASEPRI),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 0);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_mrs_basepri_unprivileged_handler_mode_reads_value() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.basepri = 0x80;
        processor.control.n_priv = true;
        processor.mode = ProcessorMode::HandlerMode;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::BASEPRI),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 0x80);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_mrs_basepri_max_reads_basepri() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.basepri = 0x80;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::BASEPRI_MAX),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 0x80);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_mrs_faultmask_privileged_true() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.faultmask = true;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::FAULTMASK),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 1);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_mrs_faultmask_unprivileged_thread_mode_reads_zero() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.faultmask = true;
        processor.control.n_priv = true;
        processor.mode = ProcessorMode::ThreadMode;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::FAULTMASK),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 0);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_mrs_faultmask_unprivileged_handler_mode_reads_value() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.faultmask = true;
        processor.control.n_priv = true;
        processor.mode = ProcessorMode::HandlerMode;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::FAULTMASK),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 1);
    }

    #[test]
    fn test_exec_mrs_control_privileged_thread_mode_reads_zero() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = false;
        processor.control.sp_sel = false;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::CONTROL),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 0);
    }

    #[test]
    fn test_exec_mrs_control_unprivileged_thread_mode_reads_npriv_and_spsel() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = true;
        processor.control.sp_sel = true;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::CONTROL),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 0b11);
    }

    #[test]
    fn test_exec_mrs_control_handler_mode_reads_thread_privilege_and_msp_selection() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R2, 0xffff_ffff);
        processor.mode = ProcessorMode::HandlerMode;
        processor.control.n_priv = true;
        processor.control.sp_sel = false;

        // Act
        processor
            .exec_mrs(MrsParams {
                rd: Reg::R2,
                sysm: u8::from(SpecialReg::CONTROL),
            })
            .unwrap();

        // Assert
        assert_eq!(processor.get_r(Reg::R2), 0b01);
    }

    #[test]
    fn test_exec_msr_control_privileged_thread_mode_updates_npriv_and_spsel() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = false;
        processor.control.sp_sel = false;
        processor.set_r(Reg::R3, 0b11);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R3,
                sysm: u8::from(SpecialReg::CONTROL),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert!(processor.control.n_priv);
        assert!(processor.control.sp_sel);
    }

    #[test]
    fn test_exec_msr_control_unprivileged_thread_mode_has_no_effect() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = true;
        processor.control.sp_sel = false;
        processor.set_r(Reg::R3, 0b10);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R3,
                sysm: u8::from(SpecialReg::CONTROL),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert!(processor.control.n_priv);
        assert!(!processor.control.sp_sel);
    }

    #[test]
    fn test_exec_msr_control_handler_mode_updates_npriv_but_preserves_spsel() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::HandlerMode;
        processor.control.n_priv = false;
        processor.control.sp_sel = true;
        processor.set_r(Reg::R3, 0b00);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R3,
                sysm: u8::from(SpecialReg::CONTROL),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert!(!processor.control.n_priv);
        assert!(processor.control.sp_sel);
    }

    #[test]
    fn test_exec_msr_control_handler_mode_can_set_thread_npriv() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::HandlerMode;
        processor.control.n_priv = false;
        processor.control.sp_sel = false;
        processor.set_r(Reg::R3, 0b11);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R3,
                sysm: u8::from(SpecialReg::CONTROL),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert!(processor.control.n_priv);
        assert!(!processor.control.sp_sel);
    }

    #[test]
    fn test_exec_msr_apsr_nzcvq_updates_flag_bits() {
        // Arrange
        let mut processor = Processor::new();

        processor.set_r(Reg::R4, 0xF800_0000);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::APSR),
                mask: 0b10,
            })
            .unwrap();

        // Assert
        assert!(processor.psr.get_n());
        assert!(processor.psr.get_z());
        assert!(processor.psr.get_c());
        assert!(processor.psr.get_v());
        assert!(processor.psr.get_q());
    }

    #[test]
    fn test_exec_msr_msp_privileged_thread_mode_updates_value() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = false;
        processor.set_msp(0xcafe_babe);
        processor.set_r(Reg::R4, 0x1234_5678);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::MSP),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.msp, 0x1234_5678);
    }

    #[test]
    fn test_exec_msr_msp_unprivileged_thread_mode_has_no_effect() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = true;
        processor.set_msp(0xcafe_babe);
        processor.set_r(Reg::R4, 0x1234_5678);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::MSP),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.msp, 0xcafe_babe);
    }

    #[test]
    fn test_exec_msr_psp_handler_mode_updates_value() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::HandlerMode;
        processor.control.n_priv = true;
        processor.set_psp(0xdead_beef);
        processor.set_r(Reg::R4, 0x1234_5678);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::PSP),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.psp, 0x1234_5678);
    }

    #[test]
    fn test_exec_msr_primask_unprivileged_thread_mode_has_no_effect() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = true;
        processor.primask = false;
        processor.set_r(Reg::R4, 1);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::PRIMASK),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert!(!processor.primask);
    }

    #[test]
    fn test_exec_msr_primask_handler_mode_updates_value() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::HandlerMode;
        processor.control.n_priv = true;
        processor.primask = false;
        processor.set_r(Reg::R4, 1);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::PRIMASK),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert!(processor.primask);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_msr_basepri_updates_low_byte() {
        // Arrange
        let mut processor = Processor::new();

        processor.basepri = 0;
        processor.set_r(Reg::R4, 0x1234_5678);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::BASEPRI),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.basepri, 0x78);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_msr_basepri_unprivileged_thread_mode_has_no_effect() {
        // Arrange
        let mut processor = Processor::new();

        processor.mode = ProcessorMode::ThreadMode;
        processor.control.n_priv = true;
        processor.basepri = 0x40;
        processor.set_r(Reg::R4, 0x80);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::BASEPRI),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.basepri, 0x40);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_msr_basepri_max_updates_when_current_is_zero() {
        // Arrange
        let mut processor = Processor::new();

        processor.basepri = 0;
        processor.set_r(Reg::R4, 0x20);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::BASEPRI_MAX),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.basepri, 0x20);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_msr_basepri_max_updates_when_new_value_is_lower() {
        // Arrange
        let mut processor = Processor::new();

        processor.basepri = 0x40;
        processor.set_r(Reg::R4, 0x20);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::BASEPRI_MAX),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.basepri, 0x20);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_msr_basepri_max_ignores_zero_when_basepri_is_set() {
        // Arrange
        let mut processor = Processor::new();

        processor.basepri = 0x40;
        processor.set_r(Reg::R4, 0);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::BASEPRI_MAX),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.basepri, 0x40);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_msr_basepri_max_ignores_higher_value() {
        // Arrange
        let mut processor = Processor::new();

        processor.basepri = 0x20;
        processor.set_r(Reg::R4, 0x40);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::BASEPRI_MAX),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert_eq!(processor.basepri, 0x20);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_msr_faultmask_updates_when_allowed() {
        // Arrange
        let mut processor = Processor::new();

        processor.execution_priority = 0;
        processor.faultmask = false;
        processor.set_r(Reg::R4, 1);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::FAULTMASK),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert!(processor.faultmask);
    }

    #[test]
    #[cfg(not(feature = "armv6m"))]
    fn test_exec_msr_faultmask_does_not_update_when_execution_priority_is_negative_one() {
        // Arrange
        let mut processor = Processor::new();

        processor.execution_priority = -1;
        processor.faultmask = false;
        processor.set_r(Reg::R4, 1);

        // Act
        processor
            .exec_msr(MsrParams {
                rn: Reg::R4,
                sysm: u8::from(SpecialReg::FAULTMASK),
                mask: 0,
            })
            .unwrap();

        // Assert
        assert!(!processor.faultmask);
    }
}
