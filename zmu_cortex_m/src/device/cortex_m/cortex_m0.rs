use crate::bus::ahblite::AHBLite;
use crate::bus::busmatrix::BusMatrix;
use crate::bus::internal::InternalBus;
use crate::core::instruction::Instruction;
use crate::core::Core;
use crate::core::ThumbCode;
use crate::memory::flash::FlashMemory;
use crate::memory::ram::RAM;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;

pub fn cortex_m0_simulate<F>(code: &[u8], mut semihost_func: F) -> u64
where
    F: FnMut(&SemihostingCommand) -> SemihostingResponse,
{
    let mut flash_memory = FlashMemory::new(0, 32768);
    let mut ram_memory = RAM::new_with_fill(0x2000_0000, 128 * 1024, 0xcd);

    flash_memory.load(code);

    let mut internal_bus = InternalBus::new();
    let mut ahb = AHBLite::new(&mut flash_memory, &mut ram_memory);
    let mut bus = BusMatrix::new(&mut internal_bus, &mut ahb);

    let mut core = Core::new(&mut bus);
    let mut count = 0;
    core.reset();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions

    {
        let mut pc = 0;

        while pc < (code.len() as u32) {
            core.set_pc(pc);
            let thumb = core.fetch();
            let instruction = core.decode(&thumb);
            instruction_cache.push(instruction);
            pc += 2;
        }
    }

    core.reset();

    while core.running {
        let pc = core.get_pc();
        let instruction = &instruction_cache[(pc >> 1) as usize];
        core.step(
            instruction,
            |semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                semihost_func(semihost_cmd)
            },
        );
        count += 1;
    }

    count
}

pub fn cortex_m0_simulate_trace<F, G>(code: &[u8], mut trace_func: F, mut semihost_func: G) -> u64
where
    F: FnMut(&ThumbCode, u64, u32, &Instruction, [u32; 13], u32),
    G: FnMut(&SemihostingCommand) -> SemihostingResponse,
{
    let mut flash_memory = FlashMemory::new(0, 32768);
    let mut ram_memory = RAM::new_with_fill(0x2000_0000, 128 * 1024, 0xcd);

    flash_memory.load(code);

    let mut internal_bus = InternalBus::new();
    let mut ahb = AHBLite::new(&mut flash_memory, &mut ram_memory);
    let mut bus = BusMatrix::new(&mut internal_bus, &mut ahb);

    let mut core = Core::new(&mut bus);
    let mut count = 0;
    core.reset();

    let mut instruction_cache = Vec::new();
    // pre-cache the decoded instructions

    {
        let mut pc = 0;

        while pc < (code.len() as u32) {
            core.set_pc(pc);
            let thumb = core.fetch();
            let instruction = core.decode(&thumb);
            instruction_cache.push(instruction);
            pc += 2;
        }
    }

    core.reset();

    while core.running {
        let pc = core.get_pc();
        let instruction = &instruction_cache[(pc >> 1) as usize];
        let opcode = core.fetch();
        core.step(
            instruction,
            |semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                semihost_func(semihost_cmd)
            },
        );
        trace_func(
            &opcode,
            core.cycle_count,
            pc,
            instruction,
            core.r0_12,
            core.psr.value,
        );
        count += 1;
    }

    count
}
