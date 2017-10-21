extern crate clap;
extern crate bit_field;
extern crate zmu_cortex_m;

use std::process;
use clap::{App, SubCommand, AppSettings};
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

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

    core.reset();
    while running {

        let pc = core.r[Reg::PC.value()];
        let thumb = core.fetch();
        let instruction = core.decode(&thumb).unwrap();
        println!("{} 0x{:x}: {}", count, pc, instruction);
        core.step(instruction, |imm32, r0, r1| if imm32 == 0xab {
            semihost_triggered = true;
            semihost = (r0, r1);
        });
        count += 1;

        /*println!(" PC:{:08X} PSR:{:08X} Z={}, C={} R0:{:08X} R1:{:08X} R2:{:08X} R3:{:08X} R4:{:08X} R5:{:08X} \
                  R6:{:08X} R7:{:08X} R8:{:08X} R9:{:08X} R10:{:08X} R11:{:08X} R12:{:08X} SP:{:08X} LR:{:08X} ",
                 self.r[Reg::PC.value()],
                 self.psr.value,
                 self.psr.get_z(),
                 self.psr.get_c(),
                 self.r[Reg::R0.value()],
                 self.r[Reg::R1.value()],
                 self.r[Reg::R2.value()],
                 self.r[Reg::R3.value()],
                 self.r[Reg::R4.value()],
                 self.r[Reg::R5.value()],
                 self.r[Reg::R6.value()],
                 self.r[Reg::R7.value()],
                 self.r[Reg::R8.value()],
                 self.r[Reg::R9.value()],
                 self.r[Reg::R10.value()],
                 self.r[Reg::R11.value()],
                 self.r[Reg::R12.value()],
                 self.r[Reg::SP.value()],
                 self.r[Reg::LR.value()],
                 );*/

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
        .subcommand(SubCommand::with_name("run")
            .about("Load and run <EXECUTABLE>")
            .args_from_usage("-d, --device=[DEVICE] 'Use specific device'
                              \
                              <EXECUTABLE>         'Set executable to load'"))
        .subcommand(SubCommand::with_name("devices").about("List available devices"))
        .get_matches();

    match matches.subcommand() {
        ("run", Some(run_matches)) => {
            let device = run_matches.value_of("DEVICE").unwrap_or("cortex-m0");
            println!("Value for device: {}", device);
            let filename = run_matches.value_of("EXECUTABLE").unwrap();
            println!("Using EXECUTABLE file: {}", filename);
            let mut ram_mem = vec![0; 1024 + 8];
            let mut flash_mem = [0; 2048];

            let file = File::open(filename);

            match file {
                Ok(f) => {
                    let mut buf_reader = BufReader::new(f);

                    match buf_reader.read(&mut flash_mem)
                    {
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
                Err(_) => {println!("Failed to open file {}", filename);}
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
