extern crate bit_field;
extern crate clap;
extern crate tabwriter;
extern crate zmu_cortex_m;

use std::process;
use clap::{App, AppSettings, SubCommand};
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io;
use tabwriter::TabWriter;

use zmu_cortex_m::semihosting::semihost_return;
use zmu_cortex_m::semihosting::{decode_semihostcmd, SemihostingCommand, SemihostingResponse,
                                SysExceptionReason};
use zmu_cortex_m::core::Core;
use zmu_cortex_m::core::register::Reg;
use zmu_cortex_m::bus::Bus;

use zmu_cortex_m::memory::ram::RAM;
use zmu_cortex_m::memory::flash::FlashMemory;


pub fn run_bin<T: Bus, R: Bus>(code: &mut T, sram: &mut R) {
    let mut internal_bus = zmu_cortex_m::bus::internal::InternalBus::default();
    let mut ahb = zmu_cortex_m::bus::ahblite::AHBLite::new(code, sram);

    let mut bus = zmu_cortex_m::bus::busmatrix::BusMatrix::new(&mut internal_bus, &mut ahb);

    let mut core = Core::new(&mut bus);
    let mut running = true;
    let mut semihost = (0, 0);
    let mut semihost_triggered = false;
    let mut count = 0;
    let mut trace_stdout = TabWriter::new(io::stdout()).minwidth(16).padding(1);

    core.reset();
    while running {
        let pc = core.r[Reg::PC.value()];
        let thumb = core.fetch();
        let instruction = core.decode(&thumb).unwrap();
        let count_before = count;
        core.step(&instruction, |imm32, r0, r1| if imm32 == 0xab {
            semihost_triggered = true;
            semihost = (r0, r1);
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
        trace_stdout.flush().unwrap();


        if semihost_triggered {
            let (r0, r1) = semihost;
            let semihost_cmd = decode_semihostcmd(r0, r1, &mut core);

            let semihost_response = match semihost_cmd {
                SemihostingCommand::SysOpen { name, mode } => {
                    println!("SEMIHOST: SYS_OPEN('{}',{})", name, mode);
                    SemihostingResponse::SysOpen { result: Ok(1) }
                }
                SemihostingCommand::SysClose { handle } => {
                    println!("SEMIHOST: SYS_CLOSE({})", handle);
                    SemihostingResponse::SysClose { success: true }
                }
                SemihostingCommand::SysWrite { handle, data } => {
                    if handle == 1 {
                        print!("{}", data[0] as char);
                    } else {
                        println!("SEMIHOST: SYS_WRITE({}, data.len={})", handle, data.len());
                    }
                    SemihostingResponse::SysWrite { result: Ok(0) }
                }
                SemihostingCommand::SysException { reason } => {
                    println!("SEMIHOST: EXCEPTION({:?})", reason);

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



fn main() {
    let matches = App::new("zmu")
        .version("1.0")
        .about("a Low level emulator for microcontrollers")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Load and run <EXECUTABLE>")
                .args_from_usage(
                    "-d, --device=[DEVICE] 'Use specific device'
                              \
                              <EXECUTABLE>         'Set executable to load'",
                ),
        )
        .subcommand(SubCommand::with_name("devices").about("List available devices"))
        .get_matches();

    match matches.subcommand() {
        ("run", Some(run_matches)) => {
            let device = run_matches.value_of("DEVICE").unwrap_or("cortex-m0");
            println!("Value for device: {}", device);
            let filename = run_matches.value_of("EXECUTABLE").unwrap();
            println!("Using EXECUTABLE file: {}", filename);
            let mut ram_mem = vec![0; 32768];
            let mut flash_mem = [0; 8192];

            let file = File::open(filename);

            match file {
                Ok(f) => {
                    let mut buf_reader = BufReader::new(f);

                    match buf_reader.read(&mut flash_mem) {
                        Ok(_) => {
                            let mut hellow = FlashMemory::new(&mut flash_mem, 0x0);
                            let mut ram = RAM::new(&mut ram_mem, 0x20000000);
                            run_bin(&mut hellow, &mut ram);
                        }
                        Err(_) => {
                            println!("Failed to read file {}", filename);
                        }
                    };
                }
                Err(_) => {
                    println!("Failed to open file {}", filename);
                }
            };
        }
        ("devices", Some(_)) => {
            println!("cortex-m0");
            process::exit(0);
        }
        ("", None) => panic!("No sub command found"),
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}
