use crate::bus::Bus;
use crate::core::exception::ExceptionHandling;
use crate::core::register::{BaseReg, PSR};
use crate::Processor;
use crate::ProcessorMode;

pub trait Reset {
    fn reset(&mut self);
}

impl Reset for Processor {
    //
    // Reset Exception
    //
    fn reset(&mut self) {
        // All basic registers to zero.
        self.r0_12[0] = 0;
        self.r0_12[1] = 0;
        self.r0_12[2] = 0;
        self.r0_12[3] = 0;
        self.r0_12[4] = 0;
        self.r0_12[5] = 0;
        self.r0_12[6] = 0;
        self.r0_12[7] = 0;
        self.r0_12[8] = 0;
        self.r0_12[9] = 0;
        self.r0_12[10] = 0;
        self.r0_12[11] = 0;

        // Main stack pointer is read via vector table
        let vtor = self.vtor;
        let sp = self.read32(vtor) & 0xffff_fffc;
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
        //TODOself.exceptions.clear();

        //self.event_reg.clear();

        self.itstate = 0;

        let reset_vector = self.read32(vtor + 4);

        self.execution_priority = self.get_execution_priority();

        self.blx_write_pc(reset_vector);
    }
}
