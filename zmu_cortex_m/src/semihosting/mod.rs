use crate::bus::Bus;
use crate::core::register::BaseReg;
use crate::core::register::Reg;
use crate::Processor;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SysExceptionReason {
    ADPStoppedBranchThroughZero,
    ADPStoppedUndefinedInstr,
    ADPStoppedSoftwareInterrupt,
    ADPStoppedPrefetchAbort,
    ADPStoppedDataAbort,
    ADPStoppedAddressException,
    ADPStoppedIRQ,
    ADPStoppedFIQ,
    ADPStoppedBreakPoint,
    ADPStoppedWatchPoint,
    ADPStoppedStepComplete,
    ADPStoppedRunTimeErrorUnknown,
    ADPStoppedInternalError,
    ADPStoppedUserInterruption,
    ADPStoppedApplicationExit,
    ADPStoppedStackOverflow,
    ADPStoppedDivisionByZero,
    ADPStoppedOSSpecific,
    ADPStopped,
}

impl SysExceptionReason {
    pub fn from_u32(reason: u32) -> SysExceptionReason {
        match reason {
            0x20000 => SysExceptionReason::ADPStoppedBranchThroughZero,
            0x20001 => SysExceptionReason::ADPStoppedUndefinedInstr,
            0x20002 => SysExceptionReason::ADPStoppedSoftwareInterrupt,
            0x20003 => SysExceptionReason::ADPStoppedPrefetchAbort,
            0x20004 => SysExceptionReason::ADPStoppedDataAbort,
            0x20005 => SysExceptionReason::ADPStoppedAddressException,
            0x20006 => SysExceptionReason::ADPStoppedIRQ,
            0x20007 => SysExceptionReason::ADPStoppedFIQ,
            0x20020 => SysExceptionReason::ADPStoppedBreakPoint,
            0x20021 => SysExceptionReason::ADPStoppedWatchPoint,
            0x20022 => SysExceptionReason::ADPStoppedStepComplete,
            0x20023 => SysExceptionReason::ADPStoppedRunTimeErrorUnknown,
            0x20024 => SysExceptionReason::ADPStoppedInternalError,
            0x20025 => SysExceptionReason::ADPStoppedUserInterruption,
            0x20026 => SysExceptionReason::ADPStoppedApplicationExit,
            0x20027 => SysExceptionReason::ADPStoppedStackOverflow,
            0x20028 => SysExceptionReason::ADPStoppedDivisionByZero,
            0x20029 => SysExceptionReason::ADPStoppedOSSpecific,
            _ => SysExceptionReason::ADPStopped,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum SemihostingCommand {
    SysOpen {
        name: String,
        mode: u32,
    },
    SysClose {
        handle: u32,
    },
    SysSeek {
        handle: u32,
        position: u32,
    },
    SysFlen {
        handle: u32,
    },
    SysIstty {
        handle: u32,
    },
    SysWrite {
        handle: u32,
        data: Vec<u8>,
    },
    SysRead {
        handle: u32,
        memoryptr: u32,
        len: u32,
    },
    SysException {
        reason: SysExceptionReason,
    },
    SysExitExtended {
        reason: SysExceptionReason,
        subcode: u32,
    },
    SysClock,
    SysErrno,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SemihostingResponse {
    SysOpen {
        result: Result<u32, i32>,
    },
    SysClose {
        success: bool,
    },
    SysFlen {
        result: Result<u32, i32>,
    },
    SysIstty {
        result: Result<u32, i32>,
    },
    SysSeek {
        success: bool,
    },
    SysWrite {
        result: Result<u32, i32>,
    },
    SysRead {
        result: Result<(u32, Vec<u8>, u32), i32>,
    },
    SysException {
        success: bool,
        stop: bool,
    },
    SysExitExtended {
        success: bool,
        stop: bool,
    },
    SysClock {
        result: Result<u32, i32>,
    },
    SysErrno {
        result: u32,
    },
}

const SYS_OPEN: u32 = 0x01;
const SYS_CLOSE: u32 = 0x02;
const SYS_WRITE: u32 = 0x05;
const SYS_READ: u32 = 0x06;
const SYS_ISTTY: u32 = 0x09;
const SYS_SEEK: u32 = 0x0a;
const SYS_FLEN: u32 = 0x0c;
const SYS_CLOCK: u32 = 0x10;
const SYS_ERRNO: u32 = 0x13;
const SYS_EXIT: u32 = 0x18;
const SYS_EXIT_EXTENDED: u32 = 0x20;

pub fn decode_semihostcmd(r0: u32, r1: u32, core: &mut Processor) -> SemihostingCommand {
    match r0 {
        SYS_OPEN => {
            let argument_block = r1;

            let mut string_ptr = core.read32(argument_block);
            let mode = core.read32(argument_block + 4);
            let mut filename_len = core.read32(argument_block + 8);
            let mut string_bytes: Vec<u8> = Vec::new();

            while filename_len > 0 {
                string_bytes.push(core.read8(string_ptr));
                string_ptr += 1;
                filename_len -= 1;
            }

            SemihostingCommand::SysOpen {
                name: String::from_utf8(string_bytes).unwrap(),
                mode: mode,
            }
        }
        SYS_CLOSE => {
            let params_ptr = r1;
            let handle = core.read32(params_ptr);
            SemihostingCommand::SysClose { handle: handle }
        }
        SYS_WRITE => {
            let params_ptr = r1;
            let handle = core.read32(params_ptr);
            let mut memoryptr = core.read32(params_ptr + 4);
            let mut len = core.read32(params_ptr + 8);

            let mut data: Vec<u8> = Vec::new();

            // :tt console output
            while len > 0 {
                data.push(core.read8(memoryptr));
                memoryptr += 1;
                len -= 1;
            }
            SemihostingCommand::SysWrite {
                handle: handle,
                data: data,
            }
        }
        SYS_READ => {
            let params_ptr = r1;
            let handle = core.read32(params_ptr);
            let memoryptr = core.read32(params_ptr + 4);
            let len = core.read32(params_ptr + 8);

            SemihostingCommand::SysRead {
                handle: handle,
                memoryptr: memoryptr,
                len: len,
            }
        }
        SYS_FLEN => {
            let params_ptr = r1;
            let handle = core.read32(params_ptr);

            SemihostingCommand::SysFlen { handle: handle }
        }
        SYS_ISTTY => {
            let params_ptr = r1;
            let handle = core.read32(params_ptr);

            SemihostingCommand::SysIstty { handle: handle }
        }
        SYS_SEEK => {
            let params_ptr = r1;
            let handle = core.read32(params_ptr);
            let position = core.read32(params_ptr + 4);

            SemihostingCommand::SysSeek {
                handle: handle,
                position: position,
            }
        }
        SYS_CLOCK => SemihostingCommand::SysClock,
        SYS_ERRNO => SemihostingCommand::SysErrno,
        SYS_EXIT_EXTENDED => {
            let params_ptr = r1;
            let reason = SysExceptionReason::from_u32(core.read32(params_ptr));
            let subcode = core.read32(params_ptr + 4);

            SemihostingCommand::SysExitExtended {
                reason: reason,
                subcode: subcode,
            }
        }
        SYS_EXIT => SemihostingCommand::SysException {
            reason: SysExceptionReason::from_u32(r1),
        },
        _ => {
            panic!("unknown semihosting command {}", r0);
        }
    }
}

#[allow(unused)]
pub fn semihost_return(core: &mut Processor, response: &SemihostingResponse) {
    match *response {
        SemihostingResponse::SysOpen { result } => match result {
            Ok(handle) => core.set_r(Reg::R0, handle),
            Err(error_code) => core.set_r(Reg::R0, error_code as u32),
        },
        SemihostingResponse::SysFlen { result } => match result {
            Ok(size) => core.set_r(Reg::R0, size),
            Err(error_code) => core.set_r(Reg::R0, error_code as u32),
        },
        SemihostingResponse::SysIstty { result } => match result {
            Ok(response) => core.set_r(Reg::R0, response),
            Err(error_code) => core.set_r(Reg::R0, error_code as u32),
        },
        SemihostingResponse::SysException { success, stop } => {
            if success {
                core.running = !stop
            }
        }
        SemihostingResponse::SysExitExtended { success, stop } => {
            if success {
                core.running = !stop
            }
        }
        SemihostingResponse::SysClose { success } => {
            if success {
                core.set_r(Reg::R0, 0);
            } else {
                core.set_r(Reg::R0, (-1_i32) as u32);
            }
        }
        SemihostingResponse::SysSeek { success } => {
            if success {
                core.set_r(Reg::R0, 0);
            } else {
                core.set_r(Reg::R0, (-1_i32) as u32);
            }
        }
        SemihostingResponse::SysWrite { result } => match result {
            Ok(_) => core.set_r(Reg::R0, 0),
            Err(unwritten_bytes) => core.set_r(Reg::R0, unwritten_bytes as u32),
        },
        SemihostingResponse::SysRead { ref result } => match result {
            Ok((memoryptr, data, diff)) => {
                let mut addr = *memoryptr;
                for x in data {
                    core.write8(addr, *x);
                    addr += 1;
                }
                core.set_r(Reg::R0, *diff);
            }
            Err(error_code) => core.set_r(Reg::R0, *error_code as u32),
        },
        SemihostingResponse::SysClock { result } => match result {
            Ok(centiseconds) => core.set_r(Reg::R0, centiseconds),
            Err(error_code) => core.set_r(Reg::R0, error_code as u32),
        },
        SemihostingResponse::SysErrno { result } => {
            core.set_r(Reg::R0, result);
        }
    }
}
