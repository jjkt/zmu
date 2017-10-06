
pub mod condition;
pub mod executor;
pub mod instruction;
pub mod operation;
pub mod register;

use bus::Bus;
use core::executor::execute;
use decoder::{decode_16, decode_32, is_thumb32};
use core::register::{Reg, PSR, Epsr};

pub enum ProcessorMode {
    ThreadMode,
    HandlerMode,
}


pub struct Core<'a, T: Bus + 'a> {
    pub msp: u32,
    pub psp: u32,
    pub r: [u32; 16],

    pub psr: PSR,

    pub primask: u32,
    pub control: u32,

    pub mode: ProcessorMode,
    pub bus: &'a mut T,
}

impl<'a, T: Bus> Core<'a, T> {
    pub fn new(bus: &'a mut T) -> Core<'a, T> {
        Core {
            mode: ProcessorMode::ThreadMode,
            msp: 0,
            psp: 0,
            psr: PSR { value: 0 },
            primask: 0,
            control: 0,
            r: [0; 16],
            bus: bus,
        }
    }

    pub fn reset(&mut self) {
        let reset_vector = self.bus.read32(4);
        println!("\nRESET");

        self.r[Reg::PC.value()] = reset_vector & 0xfffffffe;
        self.psr.set_t((reset_vector & 1) == 1);
        self.msp = self.bus.read32(0);
    }

    //
    // fetch, decode and execute single instruction
    //
    pub fn run(&mut self) {
        println!("PC:{:08X} APSR:{:08X} LR:{:08X} R0:{:08X} R1:{:08X} R2:{:08X} R3:{:08X} R4:{:08X} R5:{:08X} \
                  R6:{:08X} R7:{:08X} R8:{:08X} R9:{:08X} R10:{:08X} R11:{:08X} R12:{:08X} SP:{:08X}",
                 self.r[Reg::PC.value()],
                 self.psr.value,
                 self.r[Reg::LR.value()],
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
                 );

        let hw = self.bus.read16(self.r[Reg::PC.value()]);

        let op = match is_thumb32(hw) {
            true => {
                let hw2 = self.bus.read16(self.r[Reg::PC.value()] + 2);
                decode_32(hw, hw2)
            }
            false => decode_16(hw),
        };

        execute(self, op);
    }
}
