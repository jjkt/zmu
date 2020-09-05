//!
//! Cortex system simulation framework
//!

use crate::core::bits::Bits;
use crate::core::fault::Fault;
use crate::core::register::BaseReg;
use crate::core::reset::Reset;
use crate::executor::Executor;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;
use crate::MemoryMapConfig;
use crate::Processor;
use std::io;
use std::time::Duration;
use std::time::Instant;

///
/// Various reasons for simulation to stop before completing fully
///
pub enum SimulationError {
    ///
    /// A fault was triggered and escalated to stop the simulation
    ///
    FaultTrap,
}

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

impl From<Fault> for SimulationError {
    fn from(_fault: Fault) -> Self {
        Self::FaultTrap
    }
}

///
/// Run simulation until processing gets terminated
///
pub fn simulate(
    code: &[u8],
    semihost_func: Box<dyn FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<dyn io::Write + 'static>>,
    map: Option<MemoryMapConfig>,
    flash_size: usize,
) -> Result<SimulationStatistics, SimulationError> {
    let mut processor = Processor::new();

    processor.itm(itm_file);
    processor.semihost(Some(semihost_func));
    processor.memory_map(map);
    processor.flash_memory(flash_size, code);
    //processor.ram_memory(ram_size);

    processor.cache_instructions();

    let start = Instant::now();
    processor.reset()?;
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

    Ok(SimulationStatistics {
        instruction_count: processor.instruction_count,
        cycle_count: processor.cycle_count,
        duration: end.duration_since(start),
    })
}

///
/// Run System simulation with tracing support
///
pub fn simulate_trace<F>(
    code: &[u8],
    mut trace_func: F,
    semihost_func: Box<dyn FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<dyn io::Write + 'static>>,
    map: Option<MemoryMapConfig>,
    flash_size: usize,
) -> Result<SimulationStatistics, SimulationError>
where
    F: FnMut(&Processor),
{
    let mut processor = Processor::new();
    processor.itm(itm_file);
    processor.semihost(Some(semihost_func));
    processor.memory_map(map);
    processor.flash_memory(flash_size, code);
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

    Ok(SimulationStatistics {
        instruction_count: processor.instruction_count,
        cycle_count: processor.cycle_count,
        duration: end.duration_since(start),
    })
}
