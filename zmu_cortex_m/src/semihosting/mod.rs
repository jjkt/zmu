use crate::bus::Bus;
use crate::core::register::Reg;
use crate::core::Core;

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
    SysOpen { name: String, mode: u32 },
    SysClose { handle: u32 },
    SysFlen { handle: u32 },
    SysWrite { handle: u32, data: Vec<u8> },
    SysException { reason: SysExceptionReason },
    SysClock,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SemihostingResponse {
    SysOpen { result: Result<u32, i32> },
    SysClose { success: bool },
    SysFlen { result: Result<u32, u32> },
    SysWrite { result: Result<u32, u32> },
    SysException { success: bool, stop: bool },
    SysClock { result: Result<u32, i32> },
}

pub fn decode_semihostcmd<T: Bus>(r0: u32, r1: u32, core: &mut Core<T>) -> SemihostingCommand {
    match r0 {
        1 => {
            let argument_block = r1;

            let mut string_ptr = core.bus.read32(argument_block);
            let mode = core.bus.read32(argument_block + 4);
            let mut filename_len = core.bus.read32(argument_block + 8);
            let mut string_bytes: Vec<u8> = Vec::new();

            while filename_len > 0 {
                string_bytes.push(core.bus.read8(string_ptr));
                string_ptr += 1;
                filename_len -= 1;
            }

            SemihostingCommand::SysOpen {
                name: String::from_utf8(string_bytes).unwrap(),
                mode: mode,
            }
        }
        2 => SemihostingCommand::SysClose { handle: r1 },
        5 => {
            let params_ptr = r1;
            let handle = core.bus.read32(params_ptr);
            let mut memoryptr = core.bus.read32(params_ptr + 4);
            let mut len = core.bus.read32(params_ptr + 8);

            let mut data: Vec<u8> = Vec::new();

            // :tt console output
            while len > 0 {
                data.push(core.bus.read8(memoryptr));
                memoryptr += 1;
                len -= 1;
            }

            SemihostingCommand::SysWrite {
                handle: handle,
                data: data,
            }
        }
        12 => SemihostingCommand::SysFlen{handle: r1 },
        16 => SemihostingCommand::SysClock,
        24 => SemihostingCommand::SysException {
            reason: SysExceptionReason::from_u32(r1),
        },
        _ => {
            panic!("unknown semihosting command");
        }
    }
}

#[allow(unused)]
pub fn semihost_return<T: Bus>(core: &mut Core<T>, response: &SemihostingResponse) {
    match *response {
        SemihostingResponse::SysOpen { result } => match result {
            Ok(handle) => core.set_r(Reg::R0, handle),
            Err(error_code) => core.set_r(Reg::R0, error_code as u32),
        },
        SemihostingResponse::SysFlen { result } => match result {
            Ok(size) => core.set_r(Reg::R0, size),
            Err(error_code) => core.set_r(Reg::R0, (-1_i32) as u32),
        },
        SemihostingResponse::SysException { success, stop } => {
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
        SemihostingResponse::SysWrite { result } => match result {
            Ok(_) => core.set_r(Reg::R0, 0),
            Err(unwritten_bytes) => core.set_r(Reg::R0, unwritten_bytes as u32),
        },
        SemihostingResponse::SysClock { result } => match result {
            Ok(centiseconds) => core.set_r(Reg::R0, centiseconds),
            Err(error_code) => core.set_r(Reg::R0, error_code as u32),
        },
    }
}
