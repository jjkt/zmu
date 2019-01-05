use crate::bus::BusStepResult;

#[derive(Default)]
pub struct SysTick {
    rvr: u32,
    cvr: u32,
    csr: u32,
}

const SYST_ENABLE: u32 = 1;
const SYST_TICKINT: u32 = 1 << 1;
const SYST_COUNTFLAG: u32 = 1 << 16;

impl SysTick {
    pub fn write_syst_rvr(&mut self, value: u32) {
        self.rvr = value & 0x00ff_ffff;
    }

    pub fn write_syst_cvr(&mut self, _value: u32) {
        self.cvr = 0;
        self.csr &= SYST_COUNTFLAG ^ 0xffff_ffff;
    }

    pub fn write_syst_csr(&mut self, value: u32) {
        // is it an activation?
        if (self.csr & SYST_ENABLE == 0) && (value & SYST_ENABLE == SYST_ENABLE) {
            // reload value -> counter value
            self.cvr = self.rvr & 0x00ff_ffff;
        }

        self.csr &= 0b_111 ^ 0xffff_ffff;
        self.csr |= value & 0b_111;
    }

    pub fn read_syst_csr(&self) -> u32 {
        self.csr
    }

    pub fn read_syst_rvr(&self) -> u32 {
        self.rvr
    }

    pub fn read_syst_cvr(&self) -> u32 {
        self.cvr
    }

    pub fn read_syst_calib(&self) -> u32 {
        0
    }

    pub fn step(&mut self) -> BusStepResult {
        if (self.csr & SYST_ENABLE) == SYST_ENABLE {
            self.cvr = self.cvr.saturating_sub(1000);
            self.cvr &= 0x00ff_ffff;

            // reach 0?
            if self.cvr == 0 {
                // reload -> to counter value
                self.cvr = self.rvr & 0x00ff_ffff;
                self.csr |= SYST_COUNTFLAG;
                if (self.csr & SYST_TICKINT) == SYST_TICKINT {
                    return BusStepResult::Exception {
                        exception_number: 15,
                    };
                }
            }
        }
        BusStepResult::Nothing
    }
}
