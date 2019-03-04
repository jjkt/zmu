pub mod bits;
pub mod condition;
pub mod exception;
pub mod executor;
pub mod fault;
pub mod instruction;
pub mod operation;
pub mod register;

use crate::bus::Bus;
use crate::bus::BusStepResult;
use crate::core::bits::Bits;
use crate::core::condition::Condition;
use crate::core::exception::Exception;
use crate::core::executor::ExecuteResult;
use crate::core::executor::Executor;
use crate::core::instruction::Instruction;
use crate::core::operation::condition_test;
use crate::core::register::{Apsr, Control, Epsr, Ipsr, Reg, PSR};
use crate::decoder::{decode_16, decode_32, is_thumb32};
use crate::memory::flash::FlashMemory;
use crate::memory::ram::RAM;
use crate::peripheral::systick::SysTick;
use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;

use std::fmt;
use std::io;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ProcessorMode {
    ThreadMode,
    HandlerMode,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ThumbCode {
    Thumb32 { opcode: u32 },
    Thumb16 { half_word: u16 },
}

impl From<u16> for ThumbCode {
    fn from(value: u16) -> Self {
        ThumbCode::Thumb16 { half_word: value }
    }
}

impl From<u32> for ThumbCode {
    fn from(value: u32) -> Self {
        ThumbCode::Thumb32 { opcode: value }
    }
}
impl fmt::Display for ThumbCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThumbCode::Thumb16 { half_word } => write!(f, "0x{:x}", half_word),
            ThumbCode::Thumb32 { opcode } => write!(f, "0x{:x}", opcode),
        }
    }
}

pub struct Core {
    /* 13 of 32-bit general purpose registers. */
    pub r0_12: [u32; 13],

    msp: u32, //MSP, virtual reg r[13]
    psp: u32, //PSP, virtual reg r[13]
    lr: u32,
    pc: u32,

    pub cycle_count: u64,

    /* Processor state register, status flags. */
    pub psr: PSR,

    /* interrupt primary mask, a 1 bit mask register for
    global interrupt masking. */
    primask: bool,

    /* Control bits: currently used stack and execution privilege if core.mode == ThreadMode */
    control: Control,

    /* Processor mode: either handler or thread mode. */
    mode: ProcessorMode,

    /* Is the core simulation currently running or not.*/
    pub running: bool,

    /* One boolean per exception on the system: fixed priority system exceptions,
    configurable priority system exceptions and external exceptions. */
    pub exception_active: [bool; 64],

    itstate: u8,

    pub code: FlashMemory,
    pub sram: RAM,

    semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse>,

    pub cpuid: u32,
    pub icsr: u32,
    pub vtor: u32,
    pub aircr: u32,
    pub scr: u32,
    pub ccr: u32,
    pub shpr1: u32,
    pub shpr2: u32,
    pub shpr3: u32,
    pub shcsr: u32,
    pub cfsr: u32,
    pub hfsr: u32,
    pub dfsr: u32,
    pub mmfar: u32,
    pub bfar: u32,
    pub afsr: u32,
    pub cpacr: u32,

    pub fpccr: u32,
    pub fpcar: u32,
    pub fpdscr: u32,

    pub mvfr0: u32,
    pub mvfr1: u32,
    pub mvfr2: u32,

    pub ictr: u32,
    pub actlr: u32,

    pub nvic_interrupt_enabled: [u32; 16],
    pub nvic_interrupt_pending: [u32; 16],
    pub nvic_interrupt_active: [u32; 16],

    pub nvic_interrupt_priority: [u8; 124 * 4],

    pub dwt_ctrl: u32,
    pub dwt_cyccnt: u32,

    pub syst_rvr: u32,
    pub syst_cvr: u32,
    pub syst_csr: u32,

    pub itm_file: Option<Box<io::Write + 'static>>,
}

impl Core {
    pub fn new(
        itm_file: Option<Box<io::Write + 'static>>,
        code: &[u8],
        semihost_func: Box<FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
    ) -> Core {
        let mut core = Core {
            mode: ProcessorMode::ThreadMode,
            vtor: 0,
            psr: PSR { value: 0 },
            primask: false,
            control: Control {
                n_priv: false,
                sp_sel: false,
            },
            r0_12: [0; 13],
            pc: 0,
            msp: 0,
            psp: 0,
            lr: 0,
            code: FlashMemory::new(0, 65536),
            sram: RAM::new_with_fill(0x2000_0000, 128 * 1024, 0xcd),
            itm_file: itm_file,
            running: true,
            cycle_count: 0,
            exception_active: [false; 64],
            itstate: 0,
            semihost_func: semihost_func,
            cpuid: 0,
            icsr: 0,
            aircr: 0,
            scr: 0,
            ccr: 0,
            shpr1: 0,
            shpr2: 0,
            shpr3: 0,
            shcsr: 0,
            cfsr: 0,
            dfsr: 0,
            hfsr: 0,
            mmfar: 0,
            bfar: 0,
            afsr: 0,
            cpacr: 0,

            fpccr: 0,
            fpcar: 0,
            fpdscr: 0,
            mvfr0: 0,
            mvfr1: 0,
            mvfr2: 0,

            ictr: 0,
            actlr: 0,

            dwt_ctrl: 0x4000_0000,
            dwt_cyccnt: 0,

            nvic_interrupt_enabled: [0; 16],
            nvic_interrupt_pending: [0; 16],
            nvic_interrupt_active: [0; 16],
            nvic_interrupt_priority: [0; 124 * 4],

            //nvic_exception_pending: 0,
            //nvic_exception_active: 0,
            syst_rvr: 0,
            syst_cvr: 0,
            syst_csr: 0,
        };
        core.code.load(code);

        /*let mut internal_bus = SystemRegion::new(itm_file);
        let mut ahb = AHBLite::new(&mut flash_memory, &mut ram_memory);
        let mut bussi = BusMatrix::new(&mut internal_bus, &mut ahb);*/

        core
    }

    pub fn condition_passed(&mut self) -> bool {
        let itstate = self.itstate;

        if itstate != 0 {
            let cond = u16::from(itstate.get_bits(4..8));
            condition_test(
                Condition::from_u16(cond).unwrap_or(Condition::AL),
                &self.psr,
            )
        } else {
            true
        }
    }

    pub fn condition_passed_b(&mut self, cond: Condition) -> bool {
        condition_test(cond, &self.psr)
    }

    pub fn integer_zero_divide_trapping_enabled(&mut self) -> bool {
        true
    }

    pub fn branch_write_pc(&mut self, address: u32) {
        self.set_pc(address & 0xffff_fffe);
    }

    //
    // interworking branch
    //
    pub fn blx_write_pc(&mut self, address: u32) {
        self.psr.set_t((address & 1) == 1);
        self.branch_write_pc(address);
    }

    //
    // interworking branch
    //
    pub fn bx_write_pc(&mut self, address: u32) {
        if self.mode == ProcessorMode::HandlerMode && (address.get_bits(28..32) == 0b1111) {
            self.exception_return(address.get_bits(0..28));
        } else {
            self.blx_write_pc(address);
        }
    }

    //
    // alias for bx_write_pc
    //
    pub fn load_write_pc(&mut self, address: u32) {
        self.bx_write_pc(address);
    }

    //
    // Getter for registers
    //
    pub fn get_r(&self, r: Reg) -> u32 {
        match r {
            Reg::R0
            | Reg::R1
            | Reg::R2
            | Reg::R3
            | Reg::R4
            | Reg::R5
            | Reg::R6
            | Reg::R7
            | Reg::R8
            | Reg::R9
            | Reg::R10
            | Reg::R11
            | Reg::R12 => {
                let reg: usize = From::from(r);
                self.r0_12[reg]
            }
            Reg::SP => {
                if self.control.sp_sel {
                    self.psp
                } else {
                    self.msp
                }
            }
            Reg::LR => self.lr,
            Reg::PC => self.pc + 4,
        }
    }
    //
    // Setter for registers
    //
    pub fn set_r(&mut self, r: Reg, value: u32) {
        match r {
            Reg::R0
            | Reg::R1
            | Reg::R2
            | Reg::R3
            | Reg::R4
            | Reg::R5
            | Reg::R6
            | Reg::R7
            | Reg::R8
            | Reg::R9
            | Reg::R10
            | Reg::R11
            | Reg::R12 => {
                let reg: usize = From::from(r);
                self.r0_12[reg] = value;
            }
            Reg::SP => {
                if self.control.sp_sel {
                    self.set_psp(value)
                } else {
                    self.set_msp(value)
                }
            }
            Reg::LR => {
                self.lr = value;
            }
            Reg::PC => panic!("use branch commands instead"),
        };
    }

    pub fn set_msp(&mut self, value: u32) {
        self.msp = value;
    }

    pub fn set_psp(&mut self, value: u32) {
        self.psp = value;
    }
    pub fn get_msp(&self) -> u32 {
        self.msp
    }

    pub fn get_psp(&self) -> u32 {
        self.psp
    }

    pub fn add_pc(&mut self, value: u32) {
        self.pc += value;
    }

    pub fn get_pc(&mut self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, value: u32) {
        self.pc = value
    }

    //
    // Add value to register
    //
    pub fn add_r(&mut self, r: Reg, value: u32) {
        match r {
            Reg::R0
            | Reg::R1
            | Reg::R2
            | Reg::R3
            | Reg::R4
            | Reg::R5
            | Reg::R6
            | Reg::R7
            | Reg::R8
            | Reg::R9
            | Reg::R10
            | Reg::R11
            | Reg::R12 => {
                let reg: usize = From::from(r);
                self.r0_12[reg] += value;
            }
            Reg::SP => {
                if self.control.sp_sel {
                    self.psp += value
                } else {
                    self.msp += value
                }
            }
            Reg::LR => self.lr += value,
            Reg::PC => self.pc += value,
        };
    }
    //
    // Substract value from register
    //
    pub fn sub_r(&mut self, r: Reg, value: u32) {
        match r {
            Reg::R0
            | Reg::R1
            | Reg::R2
            | Reg::R3
            | Reg::R4
            | Reg::R5
            | Reg::R6
            | Reg::R7
            | Reg::R8
            | Reg::R9
            | Reg::R10
            | Reg::R11
            | Reg::R12 => {
                let reg: usize = From::from(r);
                self.r0_12[reg] -= value;
            }
            Reg::SP => {
                if self.control.sp_sel {
                    self.psp -= value
                } else {
                    self.msp -= value
                }
            }
            Reg::LR => self.lr -= value,
            Reg::PC => self.pc -= value,
        };
    }

    //
    // Reset Exception
    //
    pub fn reset(&mut self) {
        // All basic registers to zero.
        self.r0_12[0] = 0;
        self.r0_12[1] = 0;
        self.r0_12[2] = 0;
        self.r0_12[3] = 0;
        self.r0_12[4] = 0;
        self.r0_12[5] = 0;
        self.r0_12[6] = 0;
        self.r0_12[7] = 0;
        self.r0_12[8] = 0;
        self.r0_12[9] = 0;
        self.r0_12[10] = 0;
        self.r0_12[11] = 0;

        // Main stack pointer is read via vector table
        let vtor = self.vtor;
        let sp = self.read32(vtor) & 0xffff_fffc;
        self.set_msp(sp);

        // Process stack pointer to zero
        self.set_psp(0);

        // Link Register
        self.lr = 0;

        // Mode
        self.mode = ProcessorMode::ThreadMode;

        // Apsr, ipsr
        self.psr = PSR { value: 0 };
        self.primask = false;
        self.control.sp_sel = false;
        self.control.n_priv = false;

        //TODO self.scs.reset();
        //TODOself.exceptions.clear();

        //self.event_reg.clear();

        self.itstate = 0;

        let reset_vector = self.read32(vtor + 4);

        self.blx_write_pc(reset_vector);
    }

    pub fn set_itstate(&mut self, state: u8) {
        self.itstate = state;
    }

    pub fn it_advance(&mut self) {
        if self.itstate != 0 {
            if self.itstate.get_bits(0..3) == 0 {
                self.itstate = 0;
            } else {
                let it = self.itstate.get_bits(0..5);
                self.itstate.set_bits(0..5, (it << 1) & 0b11111);
            }
        }
    }

    pub fn in_it_block(&mut self) -> bool {
        self.itstate.get_bits(0..4) != 0
    }

    pub fn last_in_it_block(&mut self) -> bool {
        self.itstate.get_bits(0..4) == 0b1000
    }

    fn return_address(&self, exception_type: Exception, return_address: u32) -> u32 {
        match exception_type {
            Exception::NMI => return_address,
            Exception::HardFault => return_address,
            Exception::MemoryManagementFault => return_address,
            Exception::BusFault => return_address,
            Exception::UsageFault => return_address - 4,
            Exception::SVCall => return_address,
            Exception::DebugMonitor => return_address,
            Exception::PendSV => return_address,
            Exception::SysTick => return_address,
            Exception::Interrupt { .. } => return_address,
            _ => panic!("unsupported exception"),
        }
    }

    fn push_stack(&mut self, exception_type: Exception, return_address: u32) {
        const FRAME_SIZE: u32 = 0x20;

        //TODO FP extensions
        //TODO forcealign
        // forces 8 byte alignment on the stack
        let forcealign = true;
        let spmask = ((forcealign as u32) << 2) ^ 0xFFFF_FFFF;

        let (frameptr, frameptralign) =
            if self.control.sp_sel && self.mode == ProcessorMode::ThreadMode {
                let align = (self.psp.get_bit(2) & forcealign) as u32;
                self.set_psp((self.psp - FRAME_SIZE) & spmask);
                (self.psp, align)
            } else {
                let align = self.msp.get_bit(2) as u32;
                self.set_msp((self.msp - FRAME_SIZE) & spmask);
                (self.msp, align)
            };

        let r0 = self.get_r(Reg::R0);
        let r1 = self.get_r(Reg::R1);
        let r2 = self.get_r(Reg::R2);
        let r3 = self.get_r(Reg::R3);
        let r12 = self.get_r(Reg::R12);
        let lr = self.get_r(Reg::LR);

        let ret_addr = self.return_address(exception_type, return_address);

        self.write32(frameptr, r0);
        self.write32(frameptr + 0x4, r1);
        self.write32(frameptr + 0x8, r2);
        self.write32(frameptr + 0xc, r3);
        self.write32(frameptr + 0x10, r12);
        self.write32(frameptr + 0x14, lr);
        self.write32(frameptr + 0x18, ret_addr);
        let xpsr = (self.psr.value & 0b1111_1111_1111_1111_1111_1101_1111_1111)
            | (frameptralign << 9) as u32;
        self.write32(frameptr + 0x1c, xpsr);

        if self.mode == ProcessorMode::HandlerMode {
            self.lr = 0xFFFF_FFF1;
        } else if !self.control.sp_sel {
            self.lr = 0xFFFF_FFF9;
        } else {
            self.lr = 0xFFFF_FFFD;
        }
    }

    fn pop_stack(&mut self, frameptr: u32, exc_return: u32) {
        //TODO: fp extensions

        const FRAME_SIZE: u32 = 0x20;

        //let forcealign = ccr.stkalign;
        let forcealign = true;

        self.set_r(Reg::R0, self.read32(frameptr));
        self.set_r(Reg::R1, self.read32(frameptr + 0x4));
        self.set_r(Reg::R2, self.read32(frameptr + 0x8));
        self.set_r(Reg::R3, self.read32(frameptr + 0xc));
        self.set_r(Reg::R12, self.read32(frameptr + 0x10));
        self.set_r(Reg::LR, self.read32(frameptr + 0x14));
        let pc = self.read32(frameptr + 0x18);
        let psr = self.read32(frameptr + 0x1c);

        self.branch_write_pc(pc);

        let spmask = ((psr.get_bit(9) && forcealign) as u32) << 2;

        match exc_return.get_bits(0..4) {
            0b0001 | 0b1001 => {
                let msp = self.get_msp();
                self.set_msp((msp + FRAME_SIZE) | spmask);
            }
            0b1101 => {
                let psp = self.get_psp();
                self.set_psp((psp + FRAME_SIZE) | spmask);
            }
            _ => {
                panic!("wrong exc return");
            }
        }
        self.psr.value.set_bits(27..32, psr.get_bits(27..32));
        self.psr.value.set_bits(0..9, psr.get_bits(0..9));
        self.psr.value.set_bits(10..16, psr.get_bits(10..16));
        self.psr.value.set_bits(24..27, psr.get_bits(24..27));
    }

    pub fn exception_taken(&mut self, exception: Exception) {
        self.control.sp_sel = false;
        self.mode = ProcessorMode::HandlerMode;
        self.psr.set_exception_number(exception.into());
        self.exception_active[usize::from(u8::from(exception))] = true;

        // SetEventRegister();
        // InstructionSynchronizationBarrier();
        let vtor = self.vtor;
        let offset = u32::from(u8::from(exception)) * 4;
        let start = self.read32(vtor + offset);
        self.blx_write_pc(start);
    }

    pub fn exception_entry(&mut self, exception: Exception, return_address: u32) {
        self.push_stack(exception, return_address);
        self.exception_taken(exception);
    }

    fn exception_active_bit_count(&self) -> usize {
        self.exception_active
            .iter()
            .fold(0, |acc, &x| acc + (x as usize))
    }

    fn deactivate(&mut self, returning_exception_number: u8) {
        self.exception_active[returning_exception_number as usize] = false;
        if self.psr.get_exception_number() != 0b10 {
            //TODO
            //self.faultmask0 = 0;
        }
    }

    fn invalid_exception_return(&mut self, returning_exception_number: u8, exc_return: u32) {
        self.deactivate(returning_exception_number);
        //ufsr.invpc = true;
        self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
        self.exception_taken(Exception::UsageFault);
    }

    pub fn exception_return(&mut self, exc_return: u32) {
        assert!(self.mode == ProcessorMode::HandlerMode);

        let returning_exception_number = self.psr.get_exception_number();
        let nested_activation = self.exception_active_bit_count();

        if !self.exception_active[returning_exception_number as usize] {
            self.invalid_exception_return(returning_exception_number, exc_return);
            return;
        } else {
            let frameptr;
            match exc_return.get_bits(0..4) {
                0b0001 => {
                    // return to handler
                    frameptr = self.get_msp();
                    self.mode = ProcessorMode::HandlerMode;
                    self.control.sp_sel = false;
                }
                0b1001 => {
                    // returning to thread using main stack
                    if nested_activation == 0
                    /*&& !self.ccr.nonbasethreadena*/
                    {
                        self.invalid_exception_return(returning_exception_number, exc_return);
                        return;
                    } else {
                        frameptr = self.get_msp();
                        self.mode = ProcessorMode::ThreadMode;
                        self.control.sp_sel = false;
                    }
                }
                0b1101 => {
                    // returning to thread using process stack
                    if nested_activation == 0
                    /*&& !self.ccr.nonbasethreadena*/
                    {
                        self.invalid_exception_return(returning_exception_number, exc_return);
                        return;
                    } else {
                        frameptr = self.get_psp();
                        self.mode = ProcessorMode::ThreadMode;
                        self.control.sp_sel = true;
                    }
                }
                _ => {
                    self.invalid_exception_return(returning_exception_number, exc_return);
                    return;
                }
            }

            self.deactivate(returning_exception_number);
            self.pop_stack(frameptr, exc_return);
            if self.mode == ProcessorMode::HandlerMode && self.psr.get_exception_number() == 0 {
                //ufsr.invpc = true;
                self.push_stack(Exception::UsageFault, exc_return); // to negate pop_stack
                self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
                self.exception_taken(Exception::UsageFault);
                return;
            }

            if self.mode == ProcessorMode::ThreadMode && self.psr.get_exception_number() != 0 {
                //ufsr.invpc = true;
                self.push_stack(Exception::UsageFault, exc_return); // to negate pop_stack
                self.set_r(Reg::LR, (0b1111 << 28) + exc_return);
                self.exception_taken(Exception::UsageFault);
                return;
            }

            //self.clear_exclusive_local(processor_id());
            //self.set_event_register();
            //self.instruction_synchronization_barrier();
            /*if self.mode == ProcessorMode::ThreadMode && !nested_activation && scr.sleeponexit{
                self.sleep_on_exit();
            }*/
        }
    }

    // Fetch next Thumb2-coded instruction from current
    // PC location. Depending on instruction type, fetches
    // one or two half-words.
    pub fn fetch(&mut self) -> ThumbCode {
        let hw = self.read16(self.pc);

        if is_thumb32(hw) {
            let hw2 = self.read16(self.pc + 2);
            ThumbCode::Thumb32 {
                opcode: (u32::from(hw) << 16) + u32::from(hw2),
            }
        } else {
            ThumbCode::Thumb16 { half_word: hw }
        }
    }

    // Decode ThumbCode into Instruction
    pub fn decode(&self, code: ThumbCode) -> Instruction {
        match code {
            ThumbCode::Thumb32 { opcode } => decode_32(opcode),
            ThumbCode::Thumb16 { half_word } => decode_16(half_word),
        }
    }

    /*    fn set_pend(&mut self, exception: Exception) {
            match exception {
                Exception::Reset => {
                    self.nvic_exception_pending.set_bit(0, true);
                }
                Exception::NMI => {
                    self.nvic_exception_pending.set_bit(1, true);
                }
                Exception::HardFault => {
                    self.nvic_exception_pending.set_bit(2, true);
                }
                Exception::MemoryManagementFault => {
                    self.nvic_exception_pending.set_bit(3, true);
                }
                Exception::BusFault => {
                    self.nvic_exception_pending.set_bit(4, true);
                }
                Exception::UsageFault => {
                    self.nvic_exception_pending.set_bit(5, true);
                }
                Exception::Reserved4 => {
                    self.nvic_exception_pending.set_bit(6, true);
                }
                Exception::Reserved5 => {
                    self.nvic_exception_pending.set_bit(7, true);
                }
                Exception::Reserved6 => {
                    self.nvic_exception_pending.set_bit(8, true);
                }
                Exception::DebugMonitor => {
                    self.nvic_exception_pending.set_bit(9, true);
                }
                Exception::SVCall => {
                    self.nvic_exception_pending.set_bit(10, true);
                }
                Exception::Reserved8 => {
                    self.nvic_exception_pending.set_bit(11, true);
                }
                Exception::Reserved9 => {
                    self.nvic_exception_pending.set_bit(12, true);
                }
                Exception::PendSV => {
                    self.nvic_exception_pending.set_bit(13, true);
                }
                Exception::SysTick => {
                    self.nvic_exception_pending.set_bit(14, true);
                }
                Exception::Interrupt { n } => {
                    let index = n / 32;
                    let bit = n % 32;
                    self.nvic_interrupt_pending[index as usize].set_bit(bit as usize, true);
                }
            }
        }
    */

    /*
            // setting PRIMASK to 1 raises execution priority to "0"
            // setting BASEPRI changes the priority level required for exception pre-emption
            // Has effect only if BASEPRI < current unmasked priority
            // FAULTMASK 1 raises masked execution priority to "-1"


            // When no exception is active, software in Thread or handler mode is executing
            // at a execution priority (max supported priority + 1)
            // this is the "base level of execution"

            // from base level, the execution priority is the greatest urgency of:
            // - base level of execution priority
            // - greatest urgency of all active exceptions, including any that the current exception pre-empted
            // - the impact of PRIMASK, FAULTMASK, BASEPRI

            // Reset, NMI or HardFault pending?
            if (self.nvic_exception_pending & 0b111) != 0 {
                if self.nvic_exception_active.get_bit(0) {
                    return BusStepResult::Nothing;
                }

                if self.nvic_exception_pending.get_bit(0) {
                    self.nvic_exception_active.set_bit(0, true);
                    return BusStepResult::Exception {
                        exception: Exception::Reset,
                    };
                }

                if self.nvic_exception_active.get_bit(1) {
                    return BusStepResult::Nothing;
                }

                if self.nvic_exception_pending.get_bit(1) {
                    self.nvic_exception_active.set_bit(1, true);
                    return BusStepResult::Exception {
                        exception: Exception::NMI,
                    };
                }
                if self.nvic_exception_active.get_bit(2) {
                    return BusStepResult::Nothing;
                }

                if self.nvic_exception_pending.get_bit(2) {
                    self.nvic_exception_active.set_bit(2, true);
                    return BusStepResult::Exception {
                        exception: Exception::HardFault,
                    };
                }
            }

            // check the priorities from shrp1, shrp2, shrp3

    */
    // Run single instruction on core
    pub fn step(&mut self, instruction: &Instruction, instruction_size: usize) {
        let in_it_block = self.in_it_block();

        match self.execute(instruction) {
            ExecuteResult::Fault { .. } => {
                // all faults are mapped to hardfaults on armv6m
                let pc = self.get_pc();

                //TODO: set pending, not exception entry directly
                self.exception_entry(Exception::HardFault, pc);
            }
            ExecuteResult::NotTaken => {
                self.add_pc(instruction_size as u32);
                self.cycle_count += 1;
                if in_it_block {
                    self.it_advance();
                }
            }
            ExecuteResult::Branched { cycles } => {
                self.cycle_count += cycles;
            }
            ExecuteResult::Taken { cycles } => {
                self.add_pc(instruction_size as u32);
                self.cycle_count += cycles;
                if in_it_block {
                    self.it_advance();
                }
            }
        }

        //
        // run bus connected devices forward
        //
        // FIXME: Bus->Nvic needs also to see exception_active array?
        // or actually better: resolve the exception taking here, just utilize
        // nvic information?

        //
        // Resolve first the need
        //

        if let BusStepResult::Exception { exception } = self.syst_step() {
            let pc = self.get_pc();
            self.exception_entry(exception, pc);

            //self.set_exception_pending(exception);
        }
    }
}

impl fmt::Display for Core {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC:{:08X} {}{}{}{}{} R0:{:08X} R1:{:08X} R2:{:08X} R3:{:08X} R4:{:08X} R5:{:08X} \
                  R6:{:08X} R7:{:08X} R8:{:08X} R9:{:08X} R10:{:08X} R11:{:08X} R12:{:08X} SP:{:08X} LR:{:08X}",
                 self.get_r(Reg::PC),
                 if self.psr.get_z() {'Z'} else {'z'},
                 if self.psr.get_n() {'N'} else {'n'},
                 if self.psr.get_c() {'C'} else {'c'},
                 if self.psr.get_v() {'V'} else {'v'},
                 if self.psr.get_q() {'Q'} else {'q'},
                 self.get_r(Reg::R0),
                 self.get_r(Reg::R1),
                 self.get_r(Reg::R2),
                 self.get_r(Reg::R3),
                 self.get_r(Reg::R4),
                 self.get_r(Reg::R5),
                 self.get_r(Reg::R6),
                 self.get_r(Reg::R7),
                 self.get_r(Reg::R8),
                 self.get_r(Reg::R9),
                 self.get_r(Reg::R10),
                 self.get_r(Reg::R11),
                 self.get_r(Reg::R12),
                 self.get_r(Reg::SP),
                 self.get_r(Reg::LR))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bus::Bus;
    use std::io::Result;
    use std::io::Write;
    struct TestWriter {}

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_push_stack() {
        const STACK_START: u32 = 0x2000_0100;
        let code = [0; 65536];
        let mut core = Core::new(
            Some(Box::new(TestWriter {})),
            &code,
            Box::new(
                |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                    panic!("shoud not happen")
                },
            ),
        );

        // arrange
        let lr = {
            //    if self.control.sp_sel && self.mode == ProcessorMode::ThreadMode {
            core.control.sp_sel = false;
            //core.mode = ProcessorMode::ThreadMode;
            core.set_r(Reg::R0, 42);
            core.set_r(Reg::R1, 43);
            core.set_r(Reg::R2, 44);
            core.set_r(Reg::R3, 45);
            core.set_r(Reg::R12, 46);
            core.set_r(Reg::LR, 47);
            core.set_psp(0);
            core.set_msp(STACK_START);
            core.psr.value = 0xffff_ffff;

            // act
            core.push_stack(Exception::HardFault, 99);

            assert_eq!(core.msp, STACK_START - 32);
            core.get_r(Reg::LR)
        };

        // values pushed on to stack
        assert_eq!(core.read32(STACK_START - 0x20), 42);
        assert_eq!(core.read32(STACK_START - 0x20 + 4), 43);
        assert_eq!(core.read32(STACK_START - 0x20 + 8), 44);
        assert_eq!(core.read32(STACK_START - 0x20 + 12), 45);
        assert_eq!(core.read32(STACK_START - 0x20 + 16), 46);
        assert_eq!(core.read32(STACK_START - 0x20 + 20), 47);
        assert_eq!(core.read32(STACK_START - 0x20 + 24), 99);
        assert_eq!(
            core.read32(STACK_START - 0x20 + 28),
            0b1111_1111_1111_1111_1111_1101_1111_1111
        );
        assert_eq!(lr, 0xffff_fff9);
    }

    #[test]
    fn test_exception_taken() {
        // Arrange
        let code = [0; 65536];
        let mut core = Core::new(
            Some(Box::new(TestWriter {})),
            &code,
            Box::new(
                |_semihost_cmd: &SemihostingCommand| -> SemihostingResponse {
                    panic!("shoud not happen")
                },
            ),
        );

        core.control.sp_sel = true;
        core.mode = ProcessorMode::ThreadMode;
        core.psr.value = 0xffff_ffff;

        // Act
        core.exception_taken(Exception::BusFault);

        // Assert
        assert_eq!(core.control.sp_sel, false);
        assert_eq!(core.mode, ProcessorMode::HandlerMode);
        assert_eq!(core.psr.get_exception_number(), Exception::BusFault.into());
        assert_eq!(
            core.exception_active[u8::from(Exception::BusFault) as usize],
            true
        );
    }

}
