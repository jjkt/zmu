extern crate zmu_cortex_m;

use pad::PadStr;
use std::collections::HashMap;
use zmu_cortex_m::core::fetch::Fetch;
use zmu_cortex_m::core::register::{Apsr, PSR};
use zmu_cortex_m::core::thumb::ThumbCode;
use zmu_cortex_m::decoder::Decoder;
use zmu_cortex_m::Processor;

pub fn format_trace_entry(processor: &Processor, symboltable: &HashMap<u32, &str>) -> String {
    let pc = processor.last_pc;

    let thumb = processor.fetch(pc).unwrap();
    let instruction = processor.decode(thumb);

    let opcode_str = match thumb {
        ThumbCode::Thumb32 { opcode } => format!("{:08X}", opcode).with_exact_width(8),
        ThumbCode::Thumb16 { opcode } => format!("{:04X}", opcode).with_exact_width(8),
    };

    let instruction_str = format!("{}", instruction).with_exact_width(32);

    let symbol = symboltable.get(&pc).unwrap_or(&"").with_exact_width(20);

    let psr = PSR {
        value: processor.psr.value,
    };

    format!(
        "{0:}  {1:} {2:08X}  {3:}  {4:} {5:}{6:}{7:}{8:}{9:} r0:{10:08x} 1:{11:08x} 2:{12:08x} 3:{13:08x} 4:{14:08x} 5:{15:08x} 6:{16:08x} 7:{17:08x} 8:{18:08x} 9:{19:08x} 10:{20:08x} 11:{21:08x} 12:{22:08x} msp:{23:08x} psp:{24:08x} lr:{25:08x}",
        opcode_str, instruction_str, pc, symbol, processor.instruction_count,
        if psr.get_q() {'Q'} else {'q'},
        if psr.get_v() {'V'} else {'v'},
        if psr.get_c() {'C'} else {'c'},
        if psr.get_z() {'Z'} else {'z'},
        if psr.get_n() {'N'} else {'n'},
        processor.r0_12[0],
        processor.r0_12[1], processor.r0_12[2], processor.r0_12[3], processor.r0_12[4], processor.r0_12[5],
        processor.r0_12[6], processor.r0_12[7], processor.r0_12[8], processor.r0_12[9], processor.r0_12[10],
        processor.r0_12[11],
        processor.r0_12[12],processor.msp,
        processor.psp,
        processor.lr
    )
}
