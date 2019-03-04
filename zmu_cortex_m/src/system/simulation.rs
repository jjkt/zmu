use crate::core::executor::Executor;
use crate::core::fetch::Fetch;
use crate::core::instruction::instruction_size;
use crate::core::instruction::Instruction;
use crate::core::register::BaseReg;
use crate::core::reset::Reset;
use crate::core::thumb::ThumbCode;
use crate::core::Processor;
use crate::decoder::Decoder;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;
use std::io;

pub struct TraceData {
    pub opcode: ThumbCode,
    pub count: u64,
    pub pc: u32,
    pub instruction: Instruction,
    pub r0_12: [u32; 13],
    pub psr_value: u32,
}

pub fn simulate(
    code: &[u8],
    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<io::Write + 'static>>,
) -> u64 {
    let mut core = Processor::new(itm_file, code, semihost_func);
    let mut count = 0;
    core.reset();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions

    {
        let mut pc = 0;

        while pc < (code.len() as u32) {
            core.set_pc(pc);
            let thumb = core.fetch();
            let instruction = core.decode(thumb);
            instruction_cache.push((instruction, instruction_size(&instruction)));
            pc += 2;
        }
    }

    core.reset();

    while core.running {
        let pc = core.get_pc();
        let (instruction, instruction_size) = &instruction_cache[(pc >> 1) as usize];
        core.step(instruction, *instruction_size);

        count += 1;
    }

    count
}

pub fn simulate_trace<F>(
    code: &[u8],
    mut trace_func: F,
    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    itm_file: Option<Box<io::Write + 'static>>,
) -> u64
where
    F: FnMut(&TraceData),
{
    let mut core = Processor::new(itm_file, code, semihost_func);
    let mut count = 0;
    core.reset();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions

    {
        let mut pc = 0;

        while pc < (code.len() as u32) {
            core.set_pc(pc);
            let thumb = core.fetch();
            let instruction = core.decode(thumb);
            instruction_cache.push((thumb, instruction, instruction_size(&instruction)));
            pc += 2;
        }
    }

    core.reset();

    while core.running {
        let pc = core.get_pc();
        let (opcode, instruction, instruction_size) = &instruction_cache[(pc >> 1) as usize];
        core.step(instruction, *instruction_size);

        let trace_data = TraceData {
            opcode: *opcode,
            count: core.cycle_count,
            pc: pc,
            instruction: *instruction,
            r0_12: core.r0_12,
            psr_value: core.psr.value,
        };
        trace_func(&trace_data);
        count += 1;
    }

    count
}
