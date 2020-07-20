use crate::core::fault::Fault;
use crate::core::operation::add_with_carry;
use crate::core::operation::shift;

use crate::executor::conditional_setflags;
use crate::executor::ExecuteResult;
use crate::executor::ExecutorHelper;

use crate::core::instruction::AdcRegParams;
use crate::core::register::Apsr;
use crate::core::register::BaseReg;
use crate::Processor;

pub trait AdcReg {
    fn exec_adc_reg(&mut self, params: &AdcRegParams) -> Result<ExecuteResult, Fault>;
}

impl AdcReg for Processor {
    fn exec_adc_reg(&mut self, params: &AdcRegParams) -> Result<ExecuteResult, Fault> {
        if self.condition_passed() {
            let c = self.psr.get_c();
            let shifted = shift(
                self.get_r(params.rm),
                params.shift_t,
                params.shift_n as usize,
                c,
            );
            let (result, carry, overflow) = add_with_carry(self.get_r(params.rn), shifted, c);
            self.set_r(params.rd, result);

            if conditional_setflags(params.setflags, self.in_it_block()) {
                self.psr.set_n(result);
                self.psr.set_z(result);
                self.psr.set_c(carry);
                self.psr.set_v(overflow);
            }

            return Ok(ExecuteResult::Taken { cycles: 1 });
        }
        Ok(ExecuteResult::NotTaken)
    }
}
