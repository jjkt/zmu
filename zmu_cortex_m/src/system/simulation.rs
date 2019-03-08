//!
//! Cortex system simulation framework
//!

use crate::core::executor::Executor;
use crate::core::fetch::Fetch;
use crate::core::instruction::instruction_size;
use crate::core::instruction::Instruction;
use crate::core::register::BaseReg;
use crate::core::reset::Reset;
use crate::core::thumb::ThumbCode;
use crate::decoder::Decoder;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;
use crate::Processor;
use std::io;

///
/// Data sent via tracing API's
///
pub struct TraceData {
    /// Executed instruction
    pub instruction: Instruction,
    /// opcode of the instruction that was executed
    pub opcode: ThumbCode,
    /// sequential number of instructions executed at this point
    pub count: u64,
    /// Program Counter of the instruction
    pub pc: u32,
    /// Contents of the registers r0-r12 after instruction was executed
    pub r0_12: [u32; 13],
    /// Content of PSR after instruction was executed
    pub psr_value: u32,
}

///
/// Run simulation until processing gets terminated
///
pub fn simulate(
    code: &[u8],
    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<io::Write + 'static>>,
) -> u64 {
    let mut processor = Processor::new(itm_file, code, semihost_func);
    let mut count = 0;
    processor.reset().unwrap();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions
    {
        let mut pc = 0;

        while pc < (code.len() as u32) {
            processor.set_pc(pc);
            let thumb = processor.fetch().unwrap();
            let instruction = processor.decode(thumb);
            instruction_cache.push((instruction, instruction_size(&instruction)));
            pc += 2;
        }
    }

    processor.reset().unwrap();

    while processor.running {
        let pc = processor.get_pc();
        let (instruction, instruction_size) = &instruction_cache[(pc >> 1) as usize];
        processor.step(instruction, *instruction_size);
        count += 1;
    }

    count
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
    F: FnMut(&TraceData),
{
    let mut processor = Processor::new(itm_file, code, semihost_func);
    let mut count = 0;
    processor.reset().unwrap();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions

    {
        let mut pc = 0;

        while pc < (code.len() as u32) {
            processor.set_pc(pc);
            let thumb = processor.fetch().unwrap();
            let instruction = processor.decode(thumb);
            instruction_cache.push((thumb, instruction, instruction_size(&instruction)));
            pc += 2;
        }
    }

    processor.reset().unwrap();

    while processor.running {
        let pc = processor.get_pc();
        let (opcode, instruction, instruction_size) = &instruction_cache[(pc >> 1) as usize];
        processor.step(instruction, *instruction_size);

        let trace_data = TraceData {
            opcode: *opcode,
            count: processor.cycle_count,
            pc: pc,
            instruction: *instruction,
            r0_12: processor.r0_12,
            psr_value: processor.psr.value,
        };
        trace_func(&trace_data);
        count += 1;
    }

    count
}
