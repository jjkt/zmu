//!
//! Processor Reset logic
//!

use crate::bus::Bus;
use crate::core::exception::ExceptionHandling;
use crate::core::fault::Fault;
use crate::core::register::{BaseReg, PSR};
use crate::Processor;
use crate::ProcessorMode;

/// Trait for processor reset
pub trait Reset {
    ///
    /// Reset Processor
    ///
    fn reset(&mut self) -> Result<(), Fault>;
}

impl Reset for Processor {
    fn reset(&mut self) -> Result<(), Fault> {
        // All basic registers to zero.
        for r in self.r0_12.iter_mut() {
            *r = 0;
        }

        // Main stack pointer is read via vector table
        let vtor = self.vtor;
        let sp = self.read32(vtor)? & 0xffff_fffc;
        self.set_msp(sp);

        // Process stack pointer to zero
        self.set_psp(0);

        // Link Register
        self.lr = 0;

        // Mode
        self.mode = ProcessorMode::ThreadMode;

        // Apsr, ipsr
        self.psr = PSR { value: 0 };
        self.primask = false;
        self.faultmask = false;
        self.basepri = 0;
        self.control.sp_sel = false;
        self.control.n_priv = false;

        //TODO self.scs.reset();
        self.exceptions_reset();

        //self.event_reg.clear();

        self.itstate = 0;
        self.execution_priority = self.get_execution_priority();

        let reset_vector = self.read32(vtor + 4)?;
        self.blx_write_pc(reset_vector);
        Ok(())
    }
}
