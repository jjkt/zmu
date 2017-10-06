use bus::Bus;

pub struct InternalBus {}

const INTERNAL_BUS_START : u32 = 0xE0000000;
const INTERNAL_BUS_END : u32 = 0xF0000000;

impl InternalBus {
    pub fn new() -> InternalBus {
        InternalBus {}
    }
}

impl Bus for InternalBus {
    fn read16(&mut self, addr: u32) -> u16 {
        panic!("bus access fault read addr 0x{:x}", addr);
    }

    fn read32(&mut self, addr: u32) -> u32 {
        panic!("bus access fault read addr 0x{:x}", addr);
    }

    fn write32(&mut self, addr: u32, value: u32) {
        panic!("bus access fault write addr 0x{:x}", addr);
    }

    fn in_range(&self, addr: u32) -> bool {
        if (addr >= INTERNAL_BUS_START) && (addr < INTERNAL_BUS_END) {
            return true;
        }
        false
    }
}