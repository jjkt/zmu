use bus::Bus;

pub struct InternalBus {}

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
        if (addr >= 0xE0000000) && (addr < 0xF0000000) {
            return true;
        }
        false
    }
}