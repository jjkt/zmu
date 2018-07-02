use bus::Bus;

pub struct BusMatrix<'a, T: 'a + Bus, R: 'a + Bus> {
    intr: &'a mut T,
    extr: &'a mut R,
}

impl<'a, T, R> BusMatrix<'a, T, R>
where
    T: Bus,
    R: Bus,
{
    pub fn new(intr: &'a mut T, extr: &'a mut R) -> BusMatrix<'a, T, R> {
        BusMatrix {
            intr: intr,
            extr: extr,
        }
    }
}

impl<'a, T, R> Bus for BusMatrix<'a, T, R>
where
    T: Bus,
    R: Bus,
{
    fn read8(&self, addr: u32) -> u8 {
        if self.extr.in_range(addr) {
            return self.extr.read8(addr);
        } else if self.intr.in_range(addr) {
            return self.intr.read8(addr);
        }

        panic!("read out of bus range");
    }

    fn read16(&self, addr: u32) -> u16 {
        if self.extr.in_range(addr) {
            return self.extr.read16(addr);
        } else if self.intr.in_range(addr) {
            return self.intr.read16(addr);
        }

        panic!("read out of bus range");
    }

    fn read32(&self, addr: u32) -> u32 {
        if self.extr.in_range(addr) {
            return self.extr.read32(addr);
        } else if self.intr.in_range(addr) {
            return self.intr.read32(addr);
        }

        panic!("read out of bus range");
    }

    fn write32(&mut self, addr: u32, value: u32) {
        if self.extr.in_range(addr) {
            self.extr.write32(addr, value);
        } else if self.intr.in_range(addr) {
            self.intr.write32(addr, value);
        } else {
            panic!("write out of bus range");
        }
    }
    fn write8(&mut self, addr: u32, value: u8) {
        if self.extr.in_range(addr) {
            self.extr.write8(addr, value);
        } else if self.intr.in_range(addr) {
            self.intr.write8(addr, value);
        } else {
            panic!("write out of bus range");
        }
    }
    fn write16(&mut self, addr: u32, value: u16) {
        if self.extr.in_range(addr) {
            self.extr.write16(addr, value);
        } else if self.intr.in_range(addr) {
            self.intr.write16(addr, value);
        } else {
            panic!("write out of bus range");
        }
    }
    #[allow(unused)]
    fn in_range(&self, addr: u32) -> bool {
        true
    }
}
