//!
//! Cortex System simulation
//!


use crate::system::simulation::SimulationError;
use crate::Processor;
use crate::MemoryMapConfig;
use crate::executor::Executor;
use crate::core::reset::Reset;
use crate::core::register::BaseReg;

use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;

///
/// Cortex ystem simulation framework
/// 
pub struct Simulation {
    /// processor state
    pub processor: Processor,
    ///
    pub exec_mode: SimulationExecMode,
    ///
    pub watchpoints: Vec<u32>,
    ///
    pub breakpoints: Vec<u32>,
}

///
/// 
/// 
pub enum SimulationEvent {
    ///
    DoneStep,
    ///
    #[allow(dead_code)]
    Halted,
    ///
    Break,
    ///
    #[allow(dead_code)]
    WatchWrite(u32),
    ///
    WatchRead(u32),
    ///
    Finalized(u32)
}

///
/// 
/// 
pub enum SimulationExecMode {
    ///
    Step,
    ///
    Continue,
    ///
    RangeStep(u32, u32),
}

impl Simulation {
    ///
    /// Prepare a new simulation instance
    /// 
    pub fn new(code: &[u8],
        semihost_func: Box<dyn FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
        // itm_file: Option<Box<dyn io::Write + 'static>>,
        map: Option<MemoryMapConfig>,
        flash_size: usize,    
    ) -> Result<Simulation, &'static str> {
        let mut processor = Processor::new();
        // processor.itm(itm_file);
        processor.semihost(Some(semihost_func));
        processor.memory_map(map);
        processor.flash_memory(flash_size, code);
        //processor.ram_memory(ram_size);
        processor.cache_instructions();
        processor.running = true; // running
        match processor.reset() {
            Ok(_) => {},
            Err(_) => return Err("Error resetting processor"),
        }

        Ok(Simulation {
            processor,
            exec_mode: SimulationExecMode::Step,
            watchpoints: Vec::new(),
            breakpoints: Vec::new(),
        })  
    }

    ///
    pub fn run(&mut self, mut poll_incomming_data: impl FnMut() -> bool ) -> SimulationRunEvent {
        match self.exec_mode {
            SimulationExecMode::Step => {
                return SimulationRunEvent::Event(self.step());
            },
            SimulationExecMode::Continue => {
                let mut cycles = 0;
                loop {
                    if cycles % 1024 == 0 {
                        if poll_incomming_data() {
                            return SimulationRunEvent::IncomingData;
                        }
                    }
                    cycles += 1;
                    let evt = self.step();
                    match evt {
                        SimulationEvent::DoneStep => {},
                        _ => return SimulationRunEvent::Event(evt),
                    };
                }
            },
            SimulationExecMode::RangeStep(start, end) => {
                let mut cycles = 0;
                loop {
                    if cycles % 1024 == 0 {
                        if poll_incomming_data() {
                            return SimulationRunEvent::IncomingData;
                        }
                    }
                    cycles += 1;

                    let evt = self.step();

                    match evt {
                        SimulationEvent::DoneStep => {
                            if !(start..end).contains(&self.processor.get_pc()) {
                                return SimulationRunEvent::Event(evt);
                            }
                        },
                        _ => return SimulationRunEvent::Event(evt),
                    };
                }
            },
        }
    }

    ///
    /// 
    /// 
    pub fn reset(&mut self) -> Result<(), SimulationError> {
        match self.processor.reset() {
            Ok(_) => Ok(()),
            Err(_) => Err(SimulationError::FaultTrap),
        }
    }

    ///
    /// 
    /// 
    pub fn step(&mut self) -> SimulationEvent {
        if self.processor.running {
            self.processor.step();
        } else if self.processor.sleeping {
            self.processor.step_sleep();
        } if self.breakpoints.contains(&self.processor.get_pc()) {
            return SimulationEvent::Break;
        } if self.watchpoints.contains(&self.processor.get_pc()) {
            return SimulationEvent::WatchRead(self.processor.get_pc());
        } if !self.processor.running && !self.processor.sleeping {
            return SimulationEvent::Finalized(self.processor.exit_code);
        } else {
            return SimulationEvent::DoneStep;
        }
    }
}

///
/// 
/// 

pub enum SimulationRunEvent {
    ///
    IncomingData,
    ///
    Event(SimulationEvent),
}
