use crate::bus::BusStepResult;

#[derive(Default)]
pub struct SystemControlAndID {
    icsr: u32,
    vtor: u32, // TODO

    //shpr1: u32,
    //shpr2: u32,
    shpr3: u32,
}

impl SystemControlAndID {
    pub fn read_icsr(&self) -> u32 {
        self.icsr
    }

    pub fn write_icsr(&mut self, value: u32) {
        self.icsr = value
    }

    pub fn read_vtor(&self) -> u32 {
        self.vtor
    }

    pub fn write_vtor(&mut self, value: u32) {
        self.vtor = value
    }

    pub fn write_shpr3(&mut self, value: u32) {
        self.shpr3 = value
    }

    pub fn step(&mut self) -> BusStepResult {
        BusStepResult::Nothing
    }
}
