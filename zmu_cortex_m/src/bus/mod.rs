pub mod system_region;
use crate::core::exception::Exception;
use crate::core::Core;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BusStepResult {
    Nothing,
    Exception { exception: Exception },
}

pub trait Bus {
    /// Reads a 32 bit value via the bus from the given address.
    ///
    fn read32(&self, addr: u32) -> u32;

    /// Reads a 16 bit value via the bus from the given address.
    ///
    fn read16(&self, addr: u32) -> u16;

    /// Reads a 8 bit value via the bus from the given address.
    ///
    fn read8(&self, addr: u32) -> u8;

    /// Writes a 32 bit value to the bus targeting the given address.
    ///
    fn write32(&mut self, addr: u32, value: u32);

    /// Writes a 16 bit value to the bus targeting the given address.
    ///
    fn write16(&mut self, addr: u32, value: u16);

    /// Writes a 8 bit value to the bus targeting the given address.
    ///
    fn write8(&mut self, addr: u32, value: u8);

    /// Checks if given address can be reached via the bus.
    ///
    fn in_range(&self, addr: u32) -> bool;

    //
    // drives bus connected devices state
    //
    fn step(&mut self) -> BusStepResult;
}

impl Bus for Core {
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
        /*
        FIXME: LDR{S}H{T}, STRH{T} support non-halfword aligned access.
        FIXME: TBH support non-hw aligned access
        FIXME: LDR{T}, STR{T} support non-hw aligned access

        if addr & 1 == 1 {
            panic!("unaliged read16 addr 0x{:x}", addr);
        }*/

        if self.code.in_range(addr) {
            self.code.read16(addr)
        } else if self.sram.in_range(addr) {
            self.sram.read16(addr)
        } else {
            panic!("bus access fault read16 addr 0x{:x}", addr);
        }
    }

    fn read32(&self, addr: u32) -> u32 {
        /*
        FIXME: LDR{S}H{T}, STRH{T} support non-halfword aligned access.
        FIXME: TBH support non-hw aligned access
        FIXME: LDR{T}, STR{T} support non-hw aligned access
        if addr & 3 != 0 {
            panic!("unaliged read32 addr 0x{:x}", addr);
        }
        */
        if self.code.in_range(addr) {
            self.code.read32(addr)
        } else if self.sram.in_range(addr) {
            self.sram.read32(addr)
        } else {
            panic!("bus access fault read32 addr 0x{:x}", addr);
        }
    }

    fn write32(&mut self, addr: u32, value: u32) {
        /*
        if addr & 3 != 0 {
            panic!("unaliged write32 addr 0x{:x}", addr);
        }
        */
        if self.code.in_range(addr) {
            self.code.write32(addr, value);
        } else if self.sram.in_range(addr) {
            self.sram.write32(addr, value);
        } else {
            panic!("bus access fault write addr 0x{:x}", addr);
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        /*        if addr & 1 != 0 {
            panic!("unaligned write16 address 0x{:x}", addr);
        }*/
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

    fn step(&mut self) -> BusStepResult {
        BusStepResult::Nothing
    }
}
