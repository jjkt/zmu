pub mod ahblite;
pub mod busmatrix;
pub mod internal;

pub enum BusStepResult {
    Nothing,
    Exception { exception_number: u8 },
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
