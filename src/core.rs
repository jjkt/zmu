use memory::Fetch;
use executor::execute;
use decoder::decode_16;
use decoder::decode_32;
use decoder::is_thumb32;
use register::Reg;

pub enum ProcessorMode {
    ThreadMode,
    HandlerMode,
}

pub struct Core<'a, T: Fetch + 'a> {
    pub msp: u32,
    pub psp: u32,
    pub r: [u32; 16],

    pub apsr: u32,
    pub ipsr: u32,
    pub epsr: u32,

    pub primask: u32,
    pub control: u32,

    pub mode: ProcessorMode,
    pub memory: &'a mut T,
}

impl<'a, T: Fetch> Core<'a, T> {
    pub fn new(memory: &'a mut T) -> Core<'a, T> {
        Core {
            mode: ProcessorMode::ThreadMode,
            msp: 0,
            psp: 0,
            apsr: 0,
            ipsr: 0,
            epsr: 0,
            primask: 0,
            control: 0,
            r: [0; 16],
            memory: memory,
        }
    }

    pub fn reset(&mut self) {
        let reset_vector = self.memory.fetch32(4);
        println!("\nRESET");

        self.r[Reg::PC.value()] = reset_vector & 0xfffffffe;
        self.epsr = (reset_vector & 1) << 24;
        self.msp = self.memory.fetch32(0);
    }

    //
    // fetch, decode and execute single instruction
    //
    pub fn run(&mut self) {
        //TODO: print only changed registers
        //TODO: print APSR
        //TODO: print memory accesses
        println!("PC:{:08X} APSR:{:08X} LR:{:08X} R0:{:08X} R1:{:08X} R2:{:08X} R3:{:08X} R4:{:08X} R5:{:08X} \
                  R6:{:08X} R7:{:08X} R8:{:08X} R9:{:08X} R10:{:08X} R11:{:08X} R12:{:08X} SP:{:08X}",
                 self.r[Reg::PC.value()],
                 self.apsr,
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

        let hw = self.memory.fetch16(self.r[Reg::PC.value()]);

        let op = match is_thumb32(hw) {
            true => {
                let hw2 = self.memory.fetch16(self.r[Reg::PC.value()] + 2);
                //println!("pc 0x{:X} = 0x{:X}, 0x{:X}",
                //         self.r[Reg::PC.value()],
                //         hw,
                //         hw2);
                decode_32(hw, hw2)
            }
            false => {
                //println!("pc 0x{:X} = 0x{:X}", self.r[Reg::PC.value()], hw);
                decode_16(hw)
            }
        };

        execute(self, op);
    }
}
