pub mod internal;
pub mod ahblite;
pub mod busmatrix;

pub trait Bus {
    /// Reads a 32 bit value via the bus from the given address.
    ///
    fn read32(&mut self, addr: u32) -> u32;

    /// Reads a 16 bit value via the bus from the given address.
    ///
    fn read16(&mut self, addr: u32) -> u16;

    /// Reads a 8 bit value via the bus from the given address.
    ///
    fn read8(&mut self, addr: u32) -> u8;

    /// Writes a 32 bit value to the bus targeting the given address.
    ///
    fn write32(&mut self, addr: u32, value: u32);

    /// Writes a 8 bit value to the bus targeting the given address.
    ///
    fn write8(&mut self, addr: u32, value: u8);

    /// Checks if given address can be reached via the bus.
    ///
    fn in_range(&self, addr: u32) -> bool;
}