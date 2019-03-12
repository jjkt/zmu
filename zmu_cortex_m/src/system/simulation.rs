//!
//! Cortex system simulation framework
//!

use crate::core::executor::Executor;
use crate::core::reset::Reset;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;
use crate::Processor;
use std::io;

///
/// Run simulation until processing gets terminated
///
pub fn simulate(
    code: &[u8],
    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<io::Write + 'static>>,
) -> u64 {
    let mut processor = Processor::new(itm_file, code, semihost_func);

    processor.reset().unwrap();
    processor.cache_instructions();
    processor.reset().unwrap();

    while processor.running {
        while !processor.sleeping && processor.running {
            processor.tick();
        }
        while processor.sleeping && processor.running {
            processor.sleep_tick();
        }
    }
    processor.instruction_count
}

///
/// Run System simulation with tracing support
///
pub fn simulate_trace<F>(
    code: &[u8],
    mut trace_func: F,
    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<io::Write + 'static>>,
) -> u64
where
    F: FnMut(&Processor),
{
    let mut processor = Processor::new(itm_file, code, semihost_func);

    processor.reset().unwrap();
    processor.cache_instructions();
    processor.reset().unwrap();

    while processor.running {
        while !processor.sleeping && processor.running {
            processor.tick();
            trace_func(&processor);
        }
        while processor.sleeping && processor.running {
            processor.sleep_tick();
        }
    }

    processor.instruction_count
}
