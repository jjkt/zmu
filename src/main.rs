#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate clap;
extern crate goblin;
extern crate pad;
extern crate tabwriter;
extern crate zmu_cortex_m;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use goblin::Object;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

mod semihost;
mod trace;

use crate::semihost::get_semihost_func;
use crate::trace::format_trace_entry;

use std::collections::HashMap;
use tabwriter::TabWriter;
use zmu_cortex_m::system::simulation::TraceData;

use zmu_cortex_m::system::simulation::simulate;
use zmu_cortex_m::system::simulation::simulate_trace;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}

use crate::errors::*;

fn run_bin(
    buffer: &[u8],
    trace: bool,
    option_trace_start: Option<u64>,
    itm_file: Option<Box<io::Write + 'static>>,
) {
    let mut flash_mem = [0; 65536];
    let res = Object::parse(buffer).unwrap();
    let elf = match res {
        Object::Elf(elf) => elf,
        _ => {
            panic!("unsupported file format");
        }
    };

    for ph in elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD && ph.p_filesz > 0 {
            let dst_addr = ph.p_paddr as usize;
            let dst_end_addr = (ph.p_paddr + ph.p_filesz) as usize;

            let src_addr = ph.p_offset as usize;
            let src_end_addr = (ph.p_offset + ph.p_filesz) as usize;
            flash_mem[dst_addr..dst_end_addr].copy_from_slice(&buffer[src_addr..src_end_addr]);
        }
    }
    let trace_start = option_trace_start.unwrap_or(0);

    let start = Instant::now();
    let semihost_func = Box::new(get_semihost_func(start));

    let instruction_count = if trace {
        let mut symboltable = HashMap::new();
        let mut trace_stdout = TabWriter::new(io::stdout()).minwidth(16).padding(1);

        for sym in elf.syms {
            if sym.st_type() != goblin::elf::sym::STT_FILE {
                if let Some(maybe_name) = elf.strtab.get(sym.st_name) {
                    let name = maybe_name.unwrap_or("unknown");
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

        let tracefunc = |trace_data: &TraceData| {
            if trace_data.count >= trace_start {
                let trace_entry = format_trace_entry(trace_data, &symboltable);
                writeln!(&mut trace_stdout, "{}", trace_entry).unwrap();
                let _ = trace_stdout.flush();
            }
        };
        simulate_trace(&flash_mem, tracefunc, semihost_func, itm_file)
    } else {
        simulate(&flash_mem, semihost_func, itm_file)
    };

    let end = Instant::now();

    let duration = end.duration_since(start);

    println!(
        "\n{:?}, {} instructions, {} instructions per sec",
        duration,
        instruction_count,
        instruction_count as f64
            / (duration.as_secs() as f64 + (f64::from(duration.subsec_nanos()) / 1_000_000_000f64))
    );
}

fn open_itm_file(filename: &str) -> Option<Box<io::Write + 'static>> {
    let result = File::create(filename);

    match result {
        Ok(f) => Some(Box::new(f) as Box<io::Write + 'static>),
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
            );
        }
        ("", None) => panic!("No sub command found"),
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    Ok(())
}

fn main() {
    let args = App::new("zmu")
        .version("0.1")
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
                ),
        )
        .get_matches();

    if let Err(ref e) = run(&args) {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
