pub mod internal;
pub mod ahblite;
pub mod busmatrix;

pub trait Bus {
    fn read32(&mut self, addr: u32) -> u32;
    fn read16(&mut self, addr: u32) -> u16;
    fn write32(&mut self, addr: u32, value: u32);
    fn in_range(&self, addr: u32) -> bool;
}