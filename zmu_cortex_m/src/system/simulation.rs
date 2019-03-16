//!
//! Cortex system simulation framework
//!

use crate::core::bits::Bits;
use crate::core::executor::Executor;
use crate::core::register::BaseReg;
use crate::core::reset::Reset;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;
use crate::Processor;
use std::io;
use std::time::Duration;
use std::time::Instant;

///
/// Statistical information on the simulation run.
///
///
pub struct SimulationStatistics {
    ///
    /// Total number of instructions executed (taken, or not taken).
    ///
    pub instruction_count: u64,

    ///
    /// Number of system clock cycles simulated.
    ///
    pub cycle_count: u64,

    ///
    /// Wallclock time spent for the simulation
    ///
    pub duration: Duration,
}

///
/// Run simulation until processing gets terminated
///
pub fn simulate(
    code: &[u8],
    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<io::Write + 'static>>,
) -> SimulationStatistics {
    let mut processor = Processor::new(itm_file, code, semihost_func);

    processor.reset().unwrap();
    processor.cache_instructions();

    let start = Instant::now();
    processor.reset().unwrap();

    processor.state.set_bit(0, true); // running

    while processor.state & 1 == 1 {
        while processor.state == 0b01 {
            //running, !sleeping
            processor.step();
        }

        while processor.state == 0b11 {
            //running, sleeping
            processor.step_sleep();
        }
    }
    let end = Instant::now();

    SimulationStatistics {
        instruction_count: processor.instruction_count,
        cycle_count: processor.cycle_count,
        duration: end.duration_since(start),
    }
}

///
/// Run System simulation with tracing support
///
pub fn simulate_trace<F>(
    code: &[u8],
    mut trace_func: F,
    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<io::Write + 'static>>,
) -> SimulationStatistics
where
    F: FnMut(&Processor),
{
    let mut processor = Processor::new(itm_file, code, semihost_func);

    processor.reset().unwrap();
    processor.cache_instructions();

    let start = Instant::now();

    processor.reset().unwrap();

    processor.state.set_bit(0, true); // running

    while processor.state & 1 == 1 {
        while processor.state == 0b01 {
            //running, !sleeping
            processor.last_pc = processor.get_pc();
            processor.step();
            trace_func(&processor);
        }
        processor.last_pc = processor.get_pc();
        while processor.state == 0b11 {
            //running, sleeping
            processor.step_sleep();
        }
    }
    let end = Instant::now();

    SimulationStatistics {
        instruction_count: processor.instruction_count,
        cycle_count: processor.cycle_count,
        duration: end.duration_since(start),
    }
}
