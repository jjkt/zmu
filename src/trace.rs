extern crate zmu_cortex_m;

use pad::PadStr;
use std::collections::HashMap;
use zmu_cortex_m::core::register::{Apsr, PSR};
use zmu_cortex_m::core::ThumbCode;
use zmu_cortex_m::system::simulation::TraceData;

pub fn format_trace_entry(trace_data: &TraceData, symboltable: &HashMap<u32, &str>) -> String {
    let opcode_str = match trace_data.opcode {
        ThumbCode::Thumb32 { opcode } => format!("{:08X}", opcode).with_exact_width(8),
        ThumbCode::Thumb16 { half_word } => format!("{:04X}", half_word).with_exact_width(8),
    };

    let instruction_str = format!("{}", trace_data.instruction).with_exact_width(32);
    let symbol = symboltable
        .get(&trace_data.pc)
        .unwrap_or(&"")
        .with_exact_width(20);

    let psr = PSR {
        value: trace_data.psr_value,
    };

    format!(
        "{0:}  {1:} {2:08X}  {3:}  {4:} {5:}{6:}{7:}{8:}{9:} r0:{10:08x} 1:{11:08x} 2:{12:08x} 3:{13:08x} 4:{14:08x} 5:{15:08x} 6:{16:08x} 7:{17:08x} 8:{18:08x} 9:{19:08x} 10:{20:08x} 11:{21:08x} 12:{22:08x}",
        opcode_str, instruction_str, trace_data.pc, symbol, trace_data.count,
        if psr.get_q() {'Q'} else {'q'},
        if psr.get_v() {'V'} else {'v'},
        if psr.get_c() {'C'} else {'c'},
        if psr.get_z() {'Z'} else {'z'},
        if psr.get_n() {'N'} else {'n'},
        trace_data.r0_12[0], trace_data.r0_12[1], trace_data.r0_12[2], trace_data.r0_12[3], trace_data.r0_12[4], trace_data.r0_12[5], trace_data.r0_12[6], trace_data.r0_12[7], trace_data.r0_12[8], trace_data.r0_12[9], trace_data.r0_12[10], trace_data.r0_12[11], trace_data.r0_12[12],
    )
}
