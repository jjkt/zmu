use crate::core::exception::Exception;
use crate::core::Processor;

pub trait SysTick {
    fn write_syst_rvr(&mut self, value: u32);
    fn write_syst_cvr(&mut self, _value: u32);
    fn write_syst_csr(&mut self, value: u32);
    fn read_syst_csr(&self) -> u32;
    fn read_syst_rvr(&self) -> u32;
    fn read_syst_cvr(&self) -> u32;
    fn read_syst_calib(&self) -> u32;
    fn syst_step(&mut self) -> Option<Exception>;
}

const SYST_ENABLE: u32 = 1;
const SYST_TICKINT: u32 = 1 << 1;
const SYST_COUNTFLAG: u32 = 1 << 16;

impl SysTick for Processor {
    fn write_syst_rvr(&mut self, value: u32) {
        self.syst_rvr = value & 0x00ff_ffff;
    }

    fn write_syst_cvr(&mut self, _value: u32) {
        self.syst_cvr = 0;
        self.syst_csr &= SYST_COUNTFLAG ^ 0xffff_ffff;
    }

    fn write_syst_csr(&mut self, value: u32) {
        // is it an activation?
        if (self.syst_csr & SYST_ENABLE == 0) && (value & SYST_ENABLE == SYST_ENABLE) {
            // reload value -> counter value
            self.syst_cvr = self.syst_rvr & 0x00ff_ffff;
        }

        self.syst_csr &= 0b_111 ^ 0xffff_ffff;
        self.syst_csr |= value & 0b_111;
    }

    fn read_syst_csr(&self) -> u32 {
        self.syst_csr
    }

    fn read_syst_rvr(&self) -> u32 {
        self.syst_rvr
    }

    fn read_syst_cvr(&self) -> u32 {
        self.syst_cvr
    }

    fn read_syst_calib(&self) -> u32 {
        0
    }

    fn syst_step(&mut self) -> Option<Exception> {
        if (self.syst_csr & SYST_ENABLE) == SYST_ENABLE {
            self.syst_cvr = self.syst_cvr.saturating_sub(1);
            self.syst_cvr &= 0x00ff_ffff;

            // reach 0?
            if self.syst_cvr == 0 {
                // reload -> to counter value
                self.syst_cvr = self.syst_rvr & 0x00ff_ffff;
                self.syst_csr |= SYST_COUNTFLAG;
                if (self.syst_csr & SYST_TICKINT) == SYST_TICKINT {
                    return Some(Exception::SysTick);
                }
            }
        }
        None
    }
}
