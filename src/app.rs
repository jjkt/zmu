use anyhow::Context;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use clap::value_parser;
use goblin::Object;
use goblin::elf::program_header::pt_to_str;
use log::{debug, error, info};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

use crate::semihost::get_semihost_func;
use crate::trace::format_trace_entry;

use std::cmp;
use std::collections::HashMap;
use tabwriter::TabWriter;
use zmu_cortex_m::DeviceBus;
use zmu_cortex_m::Processor;
use zmu_cortex_m::core::fault::FaultTrapMode;
use zmu_cortex_m::memory::map::MemoryMapConfig;

use zmu_cortex_m::gdb::server::GdbServer;
use zmu_cortex_m::system::simulation::simulate;
use zmu_cortex_m::system::simulation::simulate_trace;

type DeviceFactory = fn() -> Option<DeviceBus>;

#[cfg(feature = "armv6m")]
const FAULT_TRAP_TARGETS: &[&str] = &["hardfault", "all"];

#[cfg(not(feature = "armv6m"))]
const FAULT_TRAP_TARGETS: &[&str] = &["hardfault", "memmanage", "busfault", "usagefault", "all"];

#[cfg(feature = "armv6m")]
const FAULT_TRAP_HELP: &str = "Enable trapping for a target: hardfault, all";

#[cfg(not(feature = "armv6m"))]
const FAULT_TRAP_HELP: &str =
    "Enable trapping for a target: hardfault, memmanage, busfault, usagefault, all";

#[cfg(feature = "armv6m")]
const FAULT_NO_TRAP_HELP: &str =
    "Disable trapping for a target: hardfault, all. Lockup still traps.";

#[cfg(not(feature = "armv6m"))]
const FAULT_NO_TRAP_HELP: &str = "Disable trapping for a target: hardfault, memmanage, busfault, usagefault, all. Lockup still traps.";

fn parse_fault_trap_target(
    target: &str,
) -> anyhow::Result<Vec<zmu_cortex_m::core::exception::Exception>> {
    use zmu_cortex_m::core::exception::Exception;

    #[cfg(feature = "armv6m")]
    let targets = match target {
        "all" | "hardfault" => vec![Exception::HardFault],
        _ => anyhow::bail!("unsupported trap target for armv6m build: {target}"),
    };

    #[cfg(not(feature = "armv6m"))]
    let targets = match target {
        "all" => vec![
            Exception::HardFault,
            Exception::MemoryManagementFault,
            Exception::BusFault,
            Exception::UsageFault,
        ],
        "hardfault" => vec![Exception::HardFault],
        "memmanage" => vec![Exception::MemoryManagementFault],
        "busfault" => vec![Exception::BusFault],
        "usagefault" => vec![Exception::UsageFault],
        _ => anyhow::bail!("unsupported trap target: {target}"),
    };

    Ok(targets)
}

fn resolve_fault_trap_mode(run_matches: &ArgMatches) -> anyhow::Result<FaultTrapMode> {
    let mut mode = FaultTrapMode::default();

    if run_matches.get_flag("fault-trap") {
        mode = FaultTrapMode::all();
    }

    if let Some(targets) = run_matches.get_many::<String>("trap") {
        for target in targets {
            for exception in parse_fault_trap_target(target)? {
                mode.set_trap(exception, true);
            }
        }
    }

    if let Some(targets) = run_matches.get_many::<String>("no-trap") {
        for target in targets {
            for exception in parse_fault_trap_target(target)? {
                mode.set_trap(exception, false);
            }
        }
    }

    Ok(mode)
}

fn run_bin(
    buffer: &[u8],
    trace: bool,
    option_trace_start: Option<u64>,
    itm_file: Option<Box<dyn io::Write + 'static>>,
    gdb: bool,
    fault_trap_mode: FaultTrapMode,
    device_factory: DeviceFactory,
) -> anyhow::Result<u32> {
    let res = Object::parse(buffer).unwrap();

    let elf = match res {
        Object::Elf(elf) => elf,
        _ => {
            anyhow::bail!("Unsupported file format.");
        }
    };

    debug!("Detected ELF file.");

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
    let flash_size = max_address - min_address;
    info!(
        "Auto configuring flash: address space is 0x{:x}..0x{:x}, size= {} bytes",
        flash_start_address, max_address, flash_size
    );
    let mut flash_mem = vec![0; flash_size];

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

    if gdb {
        let gdb = GdbServer::new(
            &flash_mem,
            device_factory(),
            semihost_func,
            if flash_start_address != 0 {
                Some(MemoryMapConfig::new(flash_start_address, 0, flash_size))
            } else {
                None
            },
            flash_size,
            fault_trap_mode,
        );

        let exit_code = gdb?.start()?;
        return Ok(exit_code);
    }

    let statistics = if trace {
        debug!("Configuring tracing.");

        let mut symboltable = HashMap::new();
        let mut trace_stdout = TabWriter::new(io::stdout()).minwidth(16).padding(1);

        for sym in &elf.syms {
            if sym.st_type() != goblin::elf::sym::STT_FILE
                && let Some(maybe_name) = elf.strtab.get_at(sym.st_name)
            {
                let name = maybe_name;
                let mut count = 0;
                let mut pos = sym.st_value as u32;
                while count <= sym.st_size {
                    symboltable.insert(pos & 0xffff_fffe, name);
                    pos += 2;
                    count += 2;
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
            device_factory(),
            tracefunc,
            semihost_func,
            itm_file,
            if flash_start_address != 0 {
                Some(MemoryMapConfig::new(flash_start_address, 0, flash_size))
            } else {
                None
            },
            flash_size,
            fault_trap_mode,
        )?
    } else {
        debug!("Starting simulation.");
        simulate(
            &flash_mem,
            device_factory(),
            semihost_func,
            itm_file,
            if flash_start_address != 0 {
                Some(MemoryMapConfig::new(flash_start_address, 0, flash_size))
            } else {
                None
            },
            flash_size,
            fault_trap_mode,
        )?
    };

    let duration_in_secs = statistics.duration.as_secs() as f64
        + (f64::from(statistics.duration.subsec_nanos()) / 1_000_000_000f64);
    let instructions_per_sec = statistics.instruction_count as f64 / duration_in_secs;

    let cycles_per_sec = statistics.cycle_count as f64 / duration_in_secs;

    debug!("Simulation done.");

    info!(
        "{:?}, {} instructions, {:.0} instructions/s, {:.0} modeled cycles/s ({:.2} Mcycles/s)",
        statistics.duration,
        statistics.instruction_count,
        instructions_per_sec,
        cycles_per_sec,
        cycles_per_sec / 1_000_000.0,
    );
    Ok(statistics.exit_code)
}

fn open_itm_file(filename: &str) -> Option<Box<dyn io::Write + 'static>> {
    let result = File::create(filename);

    match result {
        Ok(f) => Some(Box::new(f) as Box<dyn io::Write + 'static>),
        Err(_) => None,
    }
}

fn run(args: &ArgMatches, device_factory: DeviceFactory) -> anyhow::Result<u32> {
    let exit_code = match args.subcommand() {
        Some(("run", run_matches)) => {
            let filename = run_matches
                .get_one::<String>("EXECUTABLE")
                .context("filename missing")?;

            let trace_start = run_matches.get_one::<u64>("trace-start").copied();

            let itm_output = match run_matches.get_one::<String>("itm") {
                Some(filename) => open_itm_file(filename),
                None => None,
            };

            let buffer = {
                let mut v = Vec::new();
                let mut f = File::open(filename).context("unable to open file")?;
                f.read_to_end(&mut v).context("failed to read file")?;
                v
            };

            run_bin(
                &buffer,
                run_matches.get_flag("trace"),
                trace_start,
                itm_output,
                run_matches.get_flag("gdb"),
                resolve_fault_trap_mode(run_matches)?,
                device_factory,
            )?
        }
        Some((_, _)) => unreachable!(),
        None => unreachable!(),
    };

    Ok(exit_code)
}

fn build_command(bin_name: &'static str, about: &'static str, run_about: &'static str) -> Command {
    Command::new(bin_name)
        .bin_name(bin_name)
        .arg(
            Arg::new("verbosity")
                .short('v')
                .help("Increase message verbosity")
                .action(ArgAction::Count),
        )
        .about(about)
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .about(run_about)
                .arg(
                    Arg::new("trace")
                        .action(ArgAction::SetTrue)
                        .short('t')
                        .long("trace")
                        .help("Print instruction trace to stdout"),
                )
                .arg(
                    Arg::new("trace-start")
                        .long("trace-start")
                        .help("Instruction on which to start tracing")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(u64)),
                )
                .arg(
                    Arg::new("itm")
                        .long("itm")
                        .help("Name of file to which itm trace data is written to. ")
                        .num_args(1),
                )
                .arg(
                    Arg::new("EXECUTABLE")
                        .index(1)
                        .help("Set executable to load")
                        .required(true),
                )
                .arg(
                    Arg::new("ARGS")
                        .required(false)
                        .help("List of free arguments to pass to runtime as parameters")
                        .index(2)
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("fault-trap")
                        .action(ArgAction::SetTrue)
                        .long("fault-trap")
                        .help("Trap all architecturally visible fault exceptions (HardFault already traps by default; lockup always traps)")
                        .num_args(0),
                )
                .arg(
                    Arg::new("trap")
                        .long("trap")
                        .help(FAULT_TRAP_HELP)
                        .action(ArgAction::Append)
                        .value_parser(clap::builder::PossibleValuesParser::new(FAULT_TRAP_TARGETS))
                        .num_args(1),
                )
                .arg(
                    Arg::new("no-trap")
                        .long("no-trap")
                        .help(FAULT_NO_TRAP_HELP)
                        .action(ArgAction::Append)
                        .value_parser(clap::builder::PossibleValuesParser::new(FAULT_TRAP_TARGETS))
                        .num_args(1),
                )
                .arg(
                    Arg::new("gdb")
                        .action(ArgAction::SetTrue)
                        .long("gdb")
                        .help("Enable the gdb server")
                        .num_args(0),
                ),
        )
}

pub fn main_with_device(
    bin_name: &'static str,
    about: &'static str,
    run_about: &'static str,
    device_factory: DeviceFactory,
) {
    let cmd = build_command(bin_name, about, run_about).get_matches();

    let verbose = cmd.get_count("verbosity") as usize;

    stderrlog::new()
        .module(module_path!())
        .verbosity(verbose)
        .init()
        .unwrap();

    let result = run(&cmd, device_factory);
    match result {
        Ok(exit_code) => {
            std::process::exit(exit_code as i32);
        }
        Err(ref e) => {
            error!("{e}");

            ::std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build_command;

    #[cfg(not(feature = "armv6m"))]
    use super::resolve_fault_trap_mode;

    #[cfg(not(feature = "armv6m"))]
    use zmu_cortex_m::core::exception::Exception;

    #[cfg(not(feature = "armv6m"))]
    fn run_matches(args: &[&str]) -> clap::ArgMatches {
        build_command("zmu-test", "test", "test run")
            .try_get_matches_from(args)
            .expect("argument parsing should succeed")
            .remove_subcommand()
            .expect("run subcommand expected")
            .1
    }

    #[cfg(not(feature = "armv6m"))]
    #[test]
    fn test_resolve_fault_trap_mode_enables_usagefault_trap() {
        let run_matches = run_matches(&["zmu-test", "run", "--trap", "usagefault", "firmware.elf"]);

        let mode = resolve_fault_trap_mode(&run_matches).expect("fault trap mode should resolve");

        assert!(mode.should_trap(Exception::HardFault));
        assert!(mode.should_trap(Exception::UsageFault));
        assert!(!mode.should_trap(Exception::MemoryManagementFault));
        assert!(!mode.should_trap(Exception::BusFault));
    }

    #[cfg(not(feature = "armv6m"))]
    #[test]
    fn test_resolve_fault_trap_mode_no_trap_usagefault_overrides_fault_trap() {
        let run_matches = run_matches(&[
            "zmu-test",
            "run",
            "--fault-trap",
            "--no-trap",
            "usagefault",
            "firmware.elf",
        ]);

        let mode = resolve_fault_trap_mode(&run_matches).expect("fault trap mode should resolve");

        assert!(mode.should_trap(Exception::HardFault));
        assert!(mode.should_trap(Exception::MemoryManagementFault));
        assert!(mode.should_trap(Exception::BusFault));
        assert!(!mode.should_trap(Exception::UsageFault));
    }

    #[cfg(not(feature = "armv6m"))]
    #[test]
    fn test_resolve_fault_trap_mode_fault_trap_enables_all_fault_targets() {
        let run_matches = run_matches(&["zmu-test", "run", "--fault-trap", "firmware.elf"]);

        let mode = resolve_fault_trap_mode(&run_matches).expect("fault trap mode should resolve");

        assert!(mode.should_trap(Exception::HardFault));
        assert!(mode.should_trap(Exception::MemoryManagementFault));
        assert!(mode.should_trap(Exception::BusFault));
        assert!(mode.should_trap(Exception::UsageFault));
    }

    #[cfg(feature = "armv6m")]
    #[test]
    fn test_command_rejects_usagefault_trap_target_on_armv6m() {
        let result = build_command("zmu-test", "test", "test run").try_get_matches_from([
            "zmu-test",
            "run",
            "--trap",
            "usagefault",
            "firmware.elf",
        ]);

        assert!(result.is_err());
    }
}
