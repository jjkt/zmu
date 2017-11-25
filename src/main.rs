// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro. Don't forget to add `error-chain` in your
// `Cargo.toml`!
#[macro_use]
extern crate error_chain;

extern crate bit_field;
extern crate clap;
extern crate tabwriter;
extern crate zmu_cortex_m;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::io::prelude::*;
use std::io;
use std::time::Instant;
use tabwriter::TabWriter;

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

fn run_bin<T: Bus, R: Bus>(code: &mut T, sram: &mut R, trace: bool, instructions: Option<u64>) {
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


        while running && (count < max_instructions) {
            let pc = core.r[Reg::PC.value()];
            let thumb = core.fetch();
            let instruction = core.decode(&thumb);
            let count_before = count;
            core.step(&instruction, |imm32, r0, r1| {
                if imm32 == 0xab {
                    semihost_triggered = true;
                    semihost = (r0, r1);
                }
            });
            count += 1;

            writeln!(
                &mut trace_stdout,
                "{0:} 0x{1:08x}\t{2:}\t{3:}",
                count_before,
                pc,
                instruction,
                core
            ).unwrap();
          


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
                            elapsed.as_secs() * 100 + elapsed.subsec_nanos() as u64 / 100_000;

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
        trace_stdout.flush().unwrap();
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
                            elapsed.as_secs() * 100 + elapsed.subsec_nanos() as u64 / 100_000;

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

fn load_bin(filename: String, target: &mut [u8]) -> Result<()> {
    use std::fs::File;
    let mut f = File::open(filename).chain_err(|| "unable to open file")?;
    f.read(target).chain_err(|| "failed to read file")?;

    Ok(())
}

fn run(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("run", Some(run_matches)) => {
            let device = run_matches.value_of("device").unwrap_or("cortex-m0");
            //println!("Value for device: {}", device);
            let filename = run_matches.value_of("EXECUTABLE").unwrap();
            //println!("Using EXECUTABLE file: {}", filename);
            let mut ram_mem = vec![0; 32768];
            let mut flash_mem = [0; 32768];
            let instructions = match run_matches.value_of("instructions") {
                Some(instr) => Some(instr.parse::<u64>().unwrap()),
                None => None,
            };

            load_bin(filename.to_string(), &mut flash_mem)
                .chain_err(|| "loading of binary image failed")?;
            let mut hellow = FlashMemory::new(&mut flash_mem, 0x0);
            let mut ram = RAM::new(&mut ram_mem, 0x20000000);
            run_bin(
                &mut hellow,
                &mut ram,
                run_matches.is_present("trace"),
                instructions,
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
