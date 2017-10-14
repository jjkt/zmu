
pub mod condition;
pub mod executor;
pub mod instruction;
pub mod operation;
pub mod register;

use bus::Bus;
use core::executor::execute;
use decoder::{decode_16, decode_32, is_thumb32};
use core::register::{Reg, PSR, Epsr, Apsr};

pub enum ProcessorMode {
    ThreadMode,
    HandlerMode,
}


pub struct Core<'a, T: Bus + 'a> {
    pub r: [u32; 16],

    pub psr: PSR,

    pub primask: u32,
    pub control: u32,

    pub mode: ProcessorMode,
    pub bus: &'a mut T,
    instruction_count: u32,
    running : bool
}

impl<'a, T: Bus> Core<'a, T> {
    pub fn new(bus: &'a mut T) -> Core<'a, T> {
        Core {
            mode: ProcessorMode::ThreadMode,
            psr: PSR { value: 0 },
            primask: 0,
            control: 0,
            r: [0; 16],
            bus: bus,
            instruction_count: 0,
            running : true
        }
    }


    //
    // Getter for Stack pointer.
    // Depending on the control more, the SP is MSP or PSP
    //
    pub fn get_sp(&self) -> u32 {
        self.r[Reg::SP.value()]
    }

    //
    // Setter for Stack pointer.
    // Depending on the control more, the SP is MSP or PSP
    //
    pub fn set_sp(&mut self, value: u32) {
        self.r[Reg::SP.value()] = value;
    }

    //
    // Setter for Stack pointer.
    // Depending on the control more, the SP is MSP or PSP
    //
    pub fn bkpt(&mut self, imm32: u32) {
        // See: ARM Compiler toolchain Developing Software for ARM Processors
        // Semihosting
        if imm32 == 0xab {
            match self.r[Reg::R0.value()] {
                1 => {
                    // SYS_OPEN
                    // R1 = pointer to argument block
                    // - u32: pointer to null terminated string (file name)
                    // - u32: opening mode
                    // - u32: len of file name
                    // file name ":tt" means console output or console input stream
                    //

                    // return nonzero value for handle
                    self.r[Reg::R0.value()] = 1;
                }
                2 => {
                    // SYS_CLOSE
                    // -u32: handle to close
                    // return 0 on success
                    self.r[Reg::R0.value()] = 0;
                }
                3 => {
                    // SYS_WRITEC
                    // -u32: pointer to character to write
                    // no return value
                }
                4 => {
                    // SYS_WRITE0
                    // Write null terminated string to console
                    // -u32: pointer to string
                    // no return value
                }
                5 => {
                    // SYS_WRITE
                    // write to file
                    // R1 = pointer to write parmeters
                    // parameters = 
                    //  {handle u32,
                    //   memoryptr u32,
                    //   len u32}
                    // 
                    // return code in reg R0. 0 == no error
                    self.r[Reg::R0.value()] = 0;
                }
                24 => {
                    // report exception
                    // R1 = reason / type
                    // ADP_Stopped_ApplicationExit 	0x20026
                    // 
                    // return code in reg R0. 0 == no error
                    if self.r[Reg::R1.value()] == 0x20026
                    {
                    self.running = false
                    }
                }
                _ => {}
            }
        }
    }

    //
    // Reset the cpu core
    //
    pub fn reset(&mut self) {
        let reset_vector = self.bus.read32(4);
        println!("\nRESET");

        self.r[Reg::PC.value()] = reset_vector & 0xffff_fffe;
        self.psr.set_t((reset_vector & 1) == 1);
        let sp = self.bus.read32(0);
        self.set_sp(sp);
    }

    //
    // fetch, decode and execute single instruction
    //
    pub fn run(&mut self) -> bool{

        let hw = self.bus.read16(self.r[Reg::PC.value()]);

        let op = if is_thumb32(hw) {
            let hw2 = self.bus.read16(self.r[Reg::PC.value()] + 2);
            decode_32(hw, hw2)
        } else {
            decode_16(hw)
        };

        print!("{} 0x{:x}: ", self.instruction_count, self.r[Reg::PC.value()]);
        execute(self, op);
        println!(" PC:{:08X} PSR:{:08X} Z={}, C={} R0:{:08X} R1:{:08X} R2:{:08X} R3:{:08X} R4:{:08X} R5:{:08X} \
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
                 );
        self.instruction_count = self.instruction_count + 1;
        self.running
    }
}
