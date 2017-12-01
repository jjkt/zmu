#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate bit_field;
extern crate clap;
extern crate tabwriter;
extern crate zmu_cortex_m;
extern crate goblin;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::io::prelude::*;
use std::io;
use std::time::Instant;
use std::fs::File;
use tabwriter::TabWriter;
use goblin::Object;

use zmu_cortex_m::semihosting::semihost_return;
use zmu_cortex_m::semihosting::{decode_semihostcmd, SemihostingCommand, SemihostingResponse,
                                SysExceptionReason};
use zmu_cortex_m::core::Core;
use zmu_cortex_m::core::register::Reg;
use zmu_cortex_m::bus::Bus;

use zmu_cortex_m::memory::ram::RAM;
use zmu_cortex_m::memory::flash::FlashMemory;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;

fn run_bin<T: Bus, R: Bus>(
    code: &mut T,
    sram: &mut R,
    trace: bool,
    instructions: Option<u64>,
    option_trace_start: Option<u64>,
) {
    let mut internal_bus = zmu_cortex_m::bus::internal::InternalBus::default();
    let mut ahb = zmu_cortex_m::bus::ahblite::AHBLite::new(code, sram);

    let mut bus = zmu_cortex_m::bus::busmatrix::BusMatrix::new(&mut internal_bus, &mut ahb);

    let mut core = Core::new(&mut bus);
    let mut running = true;
    let mut semihost = (0, 0);
    let mut semihost_triggered = false;

    core.reset();
    let start = Instant::now();
    let max_instructions = instructions.unwrap_or(0xffff_ffff_ffff_ffff);
    let mut count = 0;

    if trace {
        let mut trace_stdout = TabWriter::new(io::stdout()).minwidth(16).padding(1);
        let trace_start = option_trace_start.unwrap_or(0);


        while running && (count < max_instructions) {
            let pc = core.get_r(&Reg::PC);
            let thumb = core.fetch();
            let instruction = core.decode(&thumb);
            core.step(&instruction, |imm32, r0, r1| {
                if imm32 == 0xab {
                    semihost_triggered = true;
                    semihost = (r0, r1);
                }
            });
            if count >= trace_start {
                writeln!(
                    &mut trace_stdout,
                    "{0:} {1:}        0x{2:04x}: \t{3:}\t{4:}",
                    count,
                    count + 1,
                    pc,
                    instruction,
                    core
                ).unwrap();

                /*    writeln!(
                &mut trace_stdout,
                "{0:} {1:}\t0x{2:x}: \t{3:}",
                count,
                count+1,
                pc,
                instruction
            ).unwrap();*/


                trace_stdout.flush().unwrap();
            }
            count += 1;



            if semihost_triggered {
                let (r0, r1) = semihost;
                let semihost_cmd = decode_semihostcmd(r0, r1, &mut core);

                let semihost_response = match semihost_cmd {
                    SemihostingCommand::SysOpen { .. } => {
                        //println!("SEMIHOST: SYS_OPEN('{}',{})", name, mode);
                        SemihostingResponse::SysOpen { result: Ok(1) }
                    }
                    SemihostingCommand::SysClose { .. } => {
                        //println!("SEMIHOST: SYS_CLOSE({})", handle);
                        SemihostingResponse::SysClose { success: true }
                    }
                    SemihostingCommand::SysWrite { handle, data } => {
                        if handle == 1 {
                            print!("{}", String::from_utf8(data).unwrap());
                        } else {
                            //println!("SEMIHOST: SYS_WRITE({}, data.len={})", handle, data.len());
                        }
                        SemihostingResponse::SysWrite { result: Ok(0) }
                    }
                    SemihostingCommand::SysClock { .. } => {
                        let elapsed = start.elapsed();
                        let in_cs =
                            elapsed.as_secs() * 100 + elapsed.subsec_nanos() as u64 / 10_000_000;


                        //println!("SEMIHOST: SYS_OPEN('{}',{})", name, mode);
                        SemihostingResponse::SysClock {
                            result: Ok(in_cs as u32),
                        }
                    }
                    SemihostingCommand::SysException { reason } => {
                        //println!("SEMIHOST: EXCEPTION({:?})", reason);

                        if reason == SysExceptionReason::ADPStoppedApplicationExit {
                            running = false;
                        }

                        SemihostingResponse::SysException { success: true }
                    }
                };

                semihost_return(&mut core, &semihost_response);
                semihost_triggered = false;
            }
        }
    } else {
        while running && (count < max_instructions) {
            let thumb = core.fetch();
            let instruction = core.decode(&thumb);
            core.step(&instruction, |imm32, r0, r1| {
                if imm32 == 0xab {
                    semihost_triggered = true;
                    semihost = (r0, r1);
                }
            });
            count += 1;

            if semihost_triggered {
                let (r0, r1) = semihost;
                let semihost_cmd = decode_semihostcmd(r0, r1, &mut core);

                let semihost_response = match semihost_cmd {
                    SemihostingCommand::SysOpen { .. } => {
                        //println!("SEMIHOST: SYS_OPEN('{}',{})", name, mode);
                        SemihostingResponse::SysOpen { result: Ok(1) }
                    }
                    SemihostingCommand::SysClose { .. } => {
                        //println!("SEMIHOST: SYS_CLOSE({})", handle);
                        SemihostingResponse::SysClose { success: true }
                    }
                    SemihostingCommand::SysWrite { handle, data } => {
                        if handle == 1 {
                            print!("{}", String::from_utf8(data).unwrap());
                        } else {
                            //println!("SEMIHOST: SYS_WRITE({}, data.len={})", handle, data.len());
                        }
                        SemihostingResponse::SysWrite { result: Ok(0) }
                    }
                    SemihostingCommand::SysClock { .. } => {
                        let elapsed = start.elapsed();
                        let in_cs =
                            elapsed.as_secs() * 100 + elapsed.subsec_nanos() as u64 / 10_000_000;

                        //println!("SEMIHOST: SYS_OPEN('{}',{})", name, mode);
                        SemihostingResponse::SysClock {
                            result: Ok(in_cs as u32),
                        }
                    }
                    SemihostingCommand::SysException { reason } => {
                        //println!("SEMIHOST: EXCEPTION({:?})", reason);

                        if reason == SysExceptionReason::ADPStoppedApplicationExit {
                            running = false;
                        }

                        SemihostingResponse::SysException { success: true }
                    }
                };

                semihost_return(&mut core, &semihost_response);
                semihost_triggered = false;
            }
        }
    }
}


fn run(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("run", Some(run_matches)) => {
            let device = run_matches.value_of("device").unwrap_or("cortex-m0");
            let filename = run_matches.value_of("EXECUTABLE").unwrap();
            let mut ram_mem = vec![0; 32768];
            let mut flash_mem = [0; 32768];
            let instructions = match run_matches.value_of("instructions") {
                Some(instr) => Some(instr.parse::<u64>().unwrap()),
                None => None,
            };
            let trace_start = match run_matches.value_of("trace_start") {
                Some(instr) => Some(instr.parse::<u64>().unwrap()),
                None => None,
            };

            let buffer = { let mut v = Vec::new(); let mut f = File::open(&filename).chain_err(|| "unable to open file")?; 
                            f.read_to_end(&mut v).chain_err(|| "failed to read file")?; v};
            let res = Object::parse(&buffer).unwrap();
            match res {
                Object::Elf(elf) => {
                    //println!("elf: {:#?}", &elf);
                    for ph in elf.program_headers {
                        if ph.p_type == goblin::elf::program_header::PT_LOAD {
                        println!("load: {} bytes from offset 0x{:x} to addr 0x{:x}", ph.p_filesz, ph.p_offset, ph.p_paddr);
                        let dst_addr = ph.p_paddr as usize;
                        let dst_end_addr = (ph.p_paddr+ph.p_filesz) as usize;

                        let src_addr = ph.p_offset as usize;
                        let src_end_addr = (ph.p_offset+ph.p_filesz) as usize;
                        flash_mem[dst_addr..dst_end_addr].copy_from_slice(&buffer[src_addr..src_end_addr]);
                        }
                    }
                },
                _ => {
                    panic!("unsupported file format");
                }
            }


            let mut hellow = FlashMemory::new(&mut flash_mem, 0x0);
            let mut ram = RAM::new(&mut ram_mem, 0x20000000);
            run_bin(
                &mut hellow,
                &mut ram,
                run_matches.is_present("trace"),
                instructions,
                trace_start,
            );
        }
        ("devices", Some(_)) => {
            println!("cortex-m0");
        }
        ("", None) => panic!("No sub command found"),
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    Ok(())
}

fn main() {
    let args = App::new("zmu")
        .version("1.0")
        .about("a Low level emulator for microcontrollers")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Load and run <EXECUTABLE>")
                .arg(
                    Arg::with_name("device")
                        .short("d")
                        .long("device")
                        .help("Use specific device")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("trace")
                        .short("t")
                        .long("trace")
                        .help("Print instruction trace to stdout"),
                )
                .arg(
                    Arg::with_name("instructions")
                        .short("n")
                        .long("max_instructions")
                        .help("Max number of instructions to run")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("trace_start")
                        .long("trace_start")
                        .help("Instruction on which to start tracing")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("EXECUTABLE")
                        .index(1)
                        .help("Set executable to load")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("devices").about("List available devices"))
        .get_matches();

    if let Err(ref e) = run(&args) {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
