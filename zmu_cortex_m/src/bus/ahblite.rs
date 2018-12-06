use crate::bus::Bus;

pub struct AHBLite<'a, T: 'a + Bus, R: 'a + Bus> {
    code: &'a mut T,
    sram: &'a mut R,
}

impl<'a, T, R> AHBLite<'a, T, R>
where
    T: Bus,
    R: Bus,
{
    pub fn new(code: &'a mut T, sram: &'a mut R) -> AHBLite<'a, T, R> {
        AHBLite { code, sram }
    }
}

impl<'a, T, R> Bus for AHBLite<'a, T, R>
where
    T: Bus,
    R: Bus,
{
    fn read8(&self, addr: u32) -> u8 {
        if self.code.in_range(addr) {
            self.code.read8(addr)
        } else if self.sram.in_range(addr) {
            self.sram.read8(addr)
        } else {
            panic!("bus access fault read8 addr 0x{:x}", addr);
        }
    }

    fn read16(&self, addr: u32) -> u16 {
        if addr & 1 == 1 {
            panic!("unaliged read16 addr 0x{:x}", addr);
        }

        if self.code.in_range(addr) {
            self.code.read16(addr)
        } else if self.sram.in_range(addr) {
            self.sram.read16(addr)
        } else {
            panic!("bus access fault read16 addr 0x{:x}", addr);
        }
    }

    fn read32(&self, addr: u32) -> u32 {
        if addr & 3 != 0 {
            panic!("unaliged read32 addr 0x{:x}", addr);
        }
        if self.code.in_range(addr) {
            self.code.read32(addr)
        } else if self.sram.in_range(addr) {
            self.sram.read32(addr)
        } else {
            panic!("bus access fault read32 addr 0x{:x}", addr);
        }
    }

    fn write32(&mut self, addr: u32, value: u32) {
        if addr & 3 != 0 {
            panic!("unaliged write32 addr 0x{:x}", addr);
        }
        if self.code.in_range(addr) {
            self.code.write32(addr, value);
        } else if self.sram.in_range(addr) {
            self.sram.write32(addr, value);
        } else {
            panic!("bus access fault write addr 0x{:x}", addr);
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        if addr & 1 != 0 {
            panic!("unaligned write16 address 0x{:x}", addr);
        }
        if self.code.in_range(addr) {
            self.code.write16(addr, value);
        } else if self.sram.in_range(addr) {
            self.sram.write16(addr, value);
        } else {
            panic!("bus access fault write addr 0x{:x}", addr);
        }
    }

    fn write8(&mut self, addr: u32, value: u8) {
        if self.code.in_range(addr) {
            self.code.write8(addr, value);
        } else if self.sram.in_range(addr) {
            self.sram.write8(addr, value);
        } else {
            panic!("bus access fault write addr 0x{:x}", addr);
        }
    }
    #[allow(unused)]
    fn in_range(&self, addr: u32) -> bool {
        self.code.in_range(addr) || self.sram.in_range(addr)
    }
}
