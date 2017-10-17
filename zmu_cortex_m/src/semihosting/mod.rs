use bus::Bus;
use core::Core;
use core::register::Reg;


#[derive(PartialEq, Debug)]
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
}

impl SysExceptionReason {
    pub fn from_u32(reason: u32) -> Option<SysExceptionReason> {
        match reason {
            0x20000 => Some(SysExceptionReason::ADPStoppedBranchThroughZero),
            0x20001 => Some(SysExceptionReason::ADPStoppedUndefinedInstr),
            0x20002 => Some(SysExceptionReason::ADPStoppedSoftwareInterrupt),
            0x20003 => Some(SysExceptionReason::ADPStoppedPrefetchAbort),
            0x20004 => Some(SysExceptionReason::ADPStoppedDataAbort),
            0x20005 => Some(SysExceptionReason::ADPStoppedAddressException),
            0x20006 => Some(SysExceptionReason::ADPStoppedIRQ),
            0x20007 => Some(SysExceptionReason::ADPStoppedFIQ),
            0x20020 => Some(SysExceptionReason::ADPStoppedBreakPoint),
            0x20021 => Some(SysExceptionReason::ADPStoppedWatchPoint),
            0x20022 => Some(SysExceptionReason::ADPStoppedStepComplete),
            0x20023 => Some(SysExceptionReason::ADPStoppedRunTimeErrorUnknown),
            0x20024 => Some(SysExceptionReason::ADPStoppedInternalError),
            0x20025 => Some(SysExceptionReason::ADPStoppedUserInterruption),
            0x20026 => Some(SysExceptionReason::ADPStoppedApplicationExit),
            0x20027 => Some(SysExceptionReason::ADPStoppedStackOverflow),
            0x20028 => Some(SysExceptionReason::ADPStoppedDivisionByZero),
            0x20029 => Some(SysExceptionReason::ADPStoppedOSSpecific),
            _ => None,
        }
    }
}

pub enum SemihostingCommand {
    SysOpen { name: String, mode: u32 },
    SysClose { handle: u32 },
    SysWrite { handle: u32, data: Vec<u8> },
    SysException { reason: SysExceptionReason },
}

pub enum SemihostingResponse {
    SysOpen { result: Result<u32, i32> },
    SysClose { success: bool },
    SysWrite { result: Result<u32, u32> },
    SysException { success: bool },
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
                string_ptr = string_ptr + 1;
                filename_len = filename_len - 1;
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
                memoryptr = memoryptr + 1;
                len = len - 1;
            }

            SemihostingCommand::SysWrite {
                handle: handle,
                data: data,
            }
        }
        24 => SemihostingCommand::SysException {
            reason: SysExceptionReason::from_u32(r1).unwrap(),
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
            Ok(handle) => core.r[Reg::R0.value()] = handle,
            Err(error_code) => core.r[Reg::R0.value()] = error_code as u32,
        },
        SemihostingResponse::SysException { success } => { /* no core operation needed. */ }
        SemihostingResponse::SysClose { success } => if success {
            core.r[Reg::R0.value()] = 0;
        } else {
            core.r[Reg::R0.value()] = (-1_i32) as u32;
        },
        SemihostingResponse::SysWrite { result } => match result {
            Ok(_) => core.r[Reg::R0.value()] = 0,
            Err(unwritten_bytes) => core.r[Reg::R0.value()] = unwritten_bytes as u32,
        },
    }
}
