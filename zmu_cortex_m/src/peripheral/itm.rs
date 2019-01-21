use crate::bus::BusStepResult;

#[derive(Default)]
pub struct InstrumentationTraceMacrocell {}

impl InstrumentationTraceMacrocell {
    pub fn read_stim0(&self) -> u32 {
        // return 0 if fifo is full, 1 otherwise
        1
    }

    pub fn write_stim0_u32(&self, value: u32) {
        println!("STIM0 write 0x{:x}", value);

    }

    pub fn write_stim0_u16(&self, value: u16) {
        println!("STIM0 write 0x{:x}", value);
    }

    pub fn write_stim0_u8(&self, value: u8) {
        println!("STIM0 write 0x{:x}", value);
    }

    pub fn step(&mut self) -> BusStepResult {
        BusStepResult::Nothing
    }
}
