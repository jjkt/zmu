use std::cmp::min;
use std::io;
use std::io::prelude::*;
use std::time::Instant;

use zmu_cortex_m::semihosting::{SemihostingCommand, SemihostingResponse, SysExceptionReason};

const TT_HANDLE_STDIN: u32 = 1;
const TT_HANDLE_STDOUT: u32 = 2;
const TT_HANDLE_STDERR: u32 = 3;
const SEMIHOST_FEATURES_HANDLE: u32 = 4;

/*
 byte 0: SHFB_MAGIC_0 0x53
 byte 1: SHFB_MAGIC_1 0x48
 byte 2: SHFB_MAGIC_2 0x46
 byte 3: SHFB_MAGIC_3 0x42
 byte 4: feature bits
*/
static FEATURE_DATA: [u8; 5] = [0x53, 0x48, 0x46, 0x42, 3];

pub fn get_semihost_func(start: Instant) -> impl FnMut(&SemihostingCommand) -> SemihostingResponse {
    let mut semihost_features_position: u32 = 0;

    move |semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
        match semihost_cmd {
            SemihostingCommand::SysOpen { name, mode } => {
                // println!("opening stream '{}' in mode '{}'", name, mode);
                if name == ":tt" {
                    match mode {
                        0..=3 => SemihostingResponse::SysOpen {
                            result: Ok(TT_HANDLE_STDIN),
                        },
                        4..=7 => SemihostingResponse::SysOpen {
                            result: Ok(TT_HANDLE_STDOUT),
                        },
                        8..=11 => SemihostingResponse::SysOpen {
                            result: Ok(TT_HANDLE_STDERR),
                        },
                        _ => SemihostingResponse::SysOpen {
                            result: Ok(TT_HANDLE_STDOUT),
                        },
                    }
                } else if name == ":semihosting-features" {
                    SemihostingResponse::SysOpen {
                        result: Ok(SEMIHOST_FEATURES_HANDLE),
                    }
                } else {
                    SemihostingResponse::SysOpen { result: Err(-1) }
                }
            }
            SemihostingCommand::SysClose { handle } => {
                // println!("closing handle '{}'", handle);
                if *handle == SEMIHOST_FEATURES_HANDLE {
                    semihost_features_position = 0;
                }

                SemihostingResponse::SysClose { success: true }
            }
            SemihostingCommand::SysFlen { handle } => {
                // println!("filelen for handle '{}'", handle);

                if *handle == TT_HANDLE_STDIN || *handle == TT_HANDLE_STDOUT {
                    SemihostingResponse::SysFlen { result: Ok(0) }
                } else if *handle == SEMIHOST_FEATURES_HANDLE {
                    SemihostingResponse::SysFlen { result: Ok(5) }
                } else {
                    SemihostingResponse::SysFlen { result: Err(-1) }
                }
            }
            SemihostingCommand::SysIstty { handle } => {
                // println!("istty query for handle '{}'", handle);

                if *handle == TT_HANDLE_STDIN
                    || *handle == TT_HANDLE_STDOUT
                    || *handle == TT_HANDLE_STDERR
                {
                    SemihostingResponse::SysIstty { result: Ok(1) }
                } else if *handle == SEMIHOST_FEATURES_HANDLE {
                    SemihostingResponse::SysIstty { result: Ok(0) }
                } else {
                    SemihostingResponse::SysIstty { result: Err(-1) }
                }
            }
            SemihostingCommand::SysWrite { handle, ref data } => {
                // println!("write: handle={}, data={:?}", handle, data);
                if *handle == TT_HANDLE_STDOUT {
                    let text = &**data;
                    print!("{}", String::from_utf8_lossy(text));
                    io::stdout().flush().expect("Could not flush stdout");
                    SemihostingResponse::SysWrite { result: Ok(0) }
                } else {
                    SemihostingResponse::SysWrite { result: Err(-1) }
                }
            }
            SemihostingCommand::SysRead {
                handle,
                memoryptr,
                len,
            } => {
                /*println!(
                    "read: handle={}, memoryptr=0x{:x}, len={}",
                    handle, memoryptr, len
                );*/
                if *handle == SEMIHOST_FEATURES_HANDLE {
                    let max_size = min(FEATURE_DATA.len() as u32, *len);

                    let read_slice = &FEATURE_DATA[(semihost_features_position as usize)
                        ..((semihost_features_position + max_size) as usize)];

                    let data = read_slice.to_vec();
                    let data_len = data.len();
                    let diff = ((*len as usize) - data_len) as u32;

                    /*println!(
                        "read response: memoryptr=0x{:x}, data={:?}, bytes not read = {}",
                        memoryptr, data, diff
                    );*/

                    semihost_features_position += data.len() as u32;

                    SemihostingResponse::SysRead {
                        result: Ok((*memoryptr, data, diff)),
                    }
                } else {
                    SemihostingResponse::SysRead { result: Err(-1) }
                }
            }
            SemihostingCommand::SysSeek { handle, position } => {
                /*println!("seek: handle={}, position={}", handle, position);*/

                if *handle == SEMIHOST_FEATURES_HANDLE {
                    if *position < 5 {
                        semihost_features_position = *position;
                        SemihostingResponse::SysSeek { success: true }
                    } else {
                        SemihostingResponse::SysSeek { success: false }
                    }
                } else {
                    SemihostingResponse::SysSeek { success: false }
                }
            }
            SemihostingCommand::SysClock { .. } => {
                // println!("sysclock");
                let elapsed = start.elapsed();
                let in_cs =
                    elapsed.as_secs() * 100 + u64::from(elapsed.subsec_nanos()) / 10_000_000;

                SemihostingResponse::SysClock {
                    result: Ok(in_cs as u32),
                }
            }
            SemihostingCommand::SysException { ref reason } => {
                // println!("sysexception {:?}", reason);
                let stop = match reason {
                    SysExceptionReason::ADPStoppedApplicationExit
                    | SysExceptionReason::ADPStopped => true,
                    _ => false,
                };

                SemihostingResponse::SysException {
                    success: true,
                    stop,
                }
            }
            SemihostingCommand::SysExitExtended { ref reason, .. } => {
                // println!("sys exit {:?}", reason);

                SemihostingResponse::SysExitExtended {
                    success: true,
                    stop: reason == &SysExceptionReason::ADPStoppedApplicationExit,
                }
            }
            SemihostingCommand::SysErrno { .. } => {
                // println!("syserrno");

                SemihostingResponse::SysErrno { result: 0 }
            }
        }
    }
}
