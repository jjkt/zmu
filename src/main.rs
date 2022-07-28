#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate clap;
extern crate goblin;
extern crate pad;
extern crate tabwriter;
extern crate zmu_cortex_m;

#[macro_use]
extern crate log;
extern crate stderrlog;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use goblin::elf::program_header::pt_to_str;
use goblin::Object;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

mod semihost;
mod trace;

use crate::semihost::get_semihost_func;
use crate::trace::format_trace_entry;

use std::cmp;
use std::collections::HashMap;
use tabwriter::TabWriter;
use zmu_cortex_m::memory::map::MemoryMapConfig;
use zmu_cortex_m::Processor;

use zmu_cortex_m::system::simulation::simulate_trace;
use zmu_cortex_m::system::simulation::{simulate, SimulationError};

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}

use crate::errors::*;
use error_chain::State;

impl From<SimulationError> for errors::Error {
    fn from(_error: SimulationError) -> Self {
        errors::Error(ErrorKind::Msg("trap".to_string()), State::default())
    }
}

fn run_bin(
    buffer: &[u8],
    trace: bool,
    option_trace_start: Option<u64>,
    itm_file: Option<Box<dyn io::Write + 'static>>,
) -> Result<()> {
    let res = Object::parse(buffer).unwrap();

    let elf = match res {
        Object::Elf(elf) => elf,
        _ => {
            bail!("Unsupported file format.");
        }
    };

    debug!("Detected ELF file.");

    // auto detection of required flash size:
    // loop 1: determine lower bound and upper bound

    let mut min_address = 0xffff_ffff;
    let mut max_address = 0;
    debug!("Determining ELF code sections");
    for ph in &elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD && ph.p_filesz > 0 {
            let dst_addr = ph.p_paddr as usize;
            let dst_end_addr = (ph.p_paddr + ph.p_filesz) as usize;

            debug!(
                "PT_LOAD section at 0x{:08x} - 0x{:08x} (size = {} bytes)",
                dst_addr, dst_end_addr, ph.p_filesz
            );
            min_address = cmp::min(dst_addr, min_address);
            max_address = cmp::max(dst_end_addr, max_address);
        } else {
            debug!(
                "ignoring section : {} (size = {} bytes)",
                pt_to_str(ph.p_type),
                ph.p_filesz
            );
        }
    }

    let flash_start_address = min_address as u32;
    let flash_size = (max_address - min_address) as usize;
    info!(
        "Auto configuring flash: address space is 0x{:x}..0x{:x}, size= {} bytes",
        flash_start_address, max_address, flash_size
    );
    let mut flash_mem = vec![0; flash_size];

    // loop 2: load data by offset
    for ph in &elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD && ph.p_filesz > 0 {
            let dst_addr = (ph.p_paddr - u64::from(flash_start_address)) as usize;
            let dst_end_addr =
                ((ph.p_paddr + ph.p_filesz) - u64::from(flash_start_address)) as usize;

            let src_addr = ph.p_offset as usize;
            let src_end_addr = (ph.p_offset + ph.p_filesz) as usize;

            flash_mem[dst_addr..dst_end_addr].copy_from_slice(&buffer[src_addr..src_end_addr]);
        }
    }

    let trace_start = option_trace_start.unwrap_or(0);
    let semihost_func = Box::new(get_semihost_func(Instant::now()));

    let statistics = if trace {
        debug!("Configuring tracing.");

        let mut symboltable = HashMap::new();
        let mut trace_stdout = TabWriter::new(io::stdout()).minwidth(16).padding(1);

        for sym in &elf.syms {
            if sym.st_type() != goblin::elf::sym::STT_FILE {
                if let Some(maybe_name) = elf.strtab.get_at(sym.st_name) {
                    let name = maybe_name;
                    let mut count = 0;
                    let mut pos = sym.st_value as u32;
                    while count <= sym.st_size {
                        // Align addresses to 2 byte alignment
                        symboltable.insert(pos & 0xffff_fffe, name);
                        pos += 2;
                        count += 2;
                    }
                }
            }
        }

        let tracefunc = |processor: &Processor| {
            if processor.instruction_count >= trace_start {
                let trace_entry = format_trace_entry(processor, &symboltable);
                writeln!(&mut trace_stdout, "{}", trace_entry).unwrap();
                let _ = trace_stdout.flush();
            }
        };
        debug!("Starting simulation with trace.");

        simulate_trace(
            &flash_mem,
            tracefunc,
            semihost_func,
            itm_file,
            if flash_start_address != 0 {
                Some(MemoryMapConfig::new(flash_start_address, 0, flash_size))
            } else {
                None
            },
            flash_size,
        )?
    } else {
        debug!("Starting simulation.");
        simulate(
            &flash_mem,
            semihost_func,
            itm_file,
            if flash_start_address != 0 {
                Some(MemoryMapConfig::new(flash_start_address, 0, flash_size))
            } else {
                None
            },
            flash_size,
        )?
    };

    let duration_in_secs = statistics.duration.as_secs() as f64
        + (f64::from(statistics.duration.subsec_nanos()) / 1_000_000_000f64);
    let instructions_per_sec = statistics.instruction_count as f64 / duration_in_secs;

    let cycles_per_sec = statistics.cycle_count as f64 / duration_in_secs;

    debug!("Simulation done.");

    info!(
        "{:?}, {} instructions, {:.0} instructions per sec, {:.0} cycles_per_sec ~ {:.2} Mhz",
        statistics.duration,
        statistics.instruction_count,
        instructions_per_sec,
        cycles_per_sec,
        cycles_per_sec / 1_000_000.0,
    );
    Ok(())
}

fn open_itm_file(filename: &str) -> Option<Box<dyn io::Write + 'static>> {
    let result = File::create(filename);

    match result {
        Ok(f) => Some(Box::new(f) as Box<dyn io::Write + 'static>),
        Err(_) => None,
    }
}

fn run(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        ("run", Some(run_matches)) => {
            let filename = run_matches
                .value_of("EXECUTABLE")
                .chain_err(|| "filename missing")?;

            let trace_start = match run_matches.value_of("trace-start") {
                Some(instr) => Some(
                    instr
                        .parse::<u64>()
                        .chain_err(|| "invalid trace start point")?,
                ),
                None => None,
            };

            let itm_output = match run_matches.value_of("itm") {
                Some(filename) => open_itm_file(filename),
                None => None,
            };

            let buffer = {
                let mut v = Vec::new();
                let mut f = File::open(&filename).chain_err(|| "unable to open file")?;
                f.read_to_end(&mut v).chain_err(|| "failed to read file")?;
                v
            };

            run_bin(
                &buffer,
                run_matches.is_present("trace"),
                trace_start,
                itm_output,
            )?;
        }
        ("", None) => bail!("No sub command found"),
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    Ok(())
}

fn main() {
    let args = App::new("zmu")
        .version(crate_version!())
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity"),
        )
        .about("a Low level emulator for microcontrollers")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("run")
                .about("Load and run <EXECUTABLE>")
                .arg(
                    Arg::with_name("trace")
                        .short("t")
                        .long("trace")
                        .help("Print instruction trace to stdout"),
                )
                .arg(
                    Arg::with_name("trace-start")
                        .long("trace-start")
                        .help("Instruction on which to start tracing")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("itm")
                        .long("itm")
                        .help("Name of file to which itm trace data is written to. ")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("EXECUTABLE")
                        .index(1)
                        .help("Set executable to load")
                        .required(true),
                )
                .arg(
                    Arg::with_name("ARGS")
                        .required(false)
                        .help("List of free arguments to pass to runtime as parameters")
                        .index(2)
                        .multiple(true),
                ),
        )
        .get_matches();

    let verbose = args.occurrences_of("verbosity") as usize;

    stderrlog::new()
        .module(module_path!())
        .verbosity(verbose)
        .init()
        .unwrap();

    if let Err(ref e) = run(&args) {
        error!("error: {}", e);

        for e in e.iter().skip(1) {
            error!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            error!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
