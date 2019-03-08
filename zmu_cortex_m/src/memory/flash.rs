//!
//! Flash Memory simulation
//!
//!

use crate::bus::Bus;
use crate::core::fault::Fault;
use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug)]
/// Flash memory with configurable start address and data content
pub struct FlashMemory {
    start_address: u32,
    data: Box<[u8]>,
}

impl FlashMemory {
    /// make a flash data instance with given start address, size and data content
    pub fn new(start_address: u32, size: usize, new_data: &[u8]) -> FlashMemory {
        let mut data = vec![0u8; size].into_boxed_slice();
        data.copy_from_slice(new_data);

        FlashMemory {
            start_address: start_address,
            data: data,
        }
    }
}

impl Bus for FlashMemory {
    fn read8(&self, addr: u32) -> u8 {
        let a = addr - self.start_address;
        self.data[a as usize]
    }
    fn read16(&self, addr: u32) -> Result<u16, Fault> {
        let a = (addr - self.start_address) as usize;

        Ok(LittleEndian::read_u16(&self.data[a..a + 2]))
    }

    fn read32(&self, addr: u32) -> Result<u32, Fault> {
        let a = (addr - self.start_address) as usize;
        Ok(LittleEndian::read_u32(&self.data[a..a + 4]))
    }

    fn write32(&mut self, addr: u32, value: u32) {
        panic!(
            "trying to write to flash memory add 0x{:x} = 0x{}",
            addr, value
        );
    }
    fn write16(&mut self, addr: u32, value: u16) {
        panic!(
            "trying to write to flash memory add 0x{:x} = 0x{}",
            addr, value
        );
    }
    fn write8(&mut self, addr: u32, value: u8) {
        panic!(
            "trying to write to flash memory add 0x{:x} = 0x{}",
            addr, value
        );
    }

    fn in_range(&self, addr: u32) -> bool {
        if (addr >= self.start_address) && (addr < (self.start_address + self.data.len() as u32)) {
            return true;
        }
        false
    }
}

#[test]
fn test_new() {
    // should be able to make new instance of memory
    let data = [0; 1024];
    let _mem = FlashMemory::new(0, 1024, &data);
}

#[test]
fn test_load() {
    let mem = FlashMemory::new(0, 1024, &vec![42u8; 1024]);
    assert_eq!(mem.read8(0), 42);
    assert_eq!(mem.read16(0), (42 << 8) + 42);
    assert_eq!(mem.read32(0), (42 << 24) + (42 << 16) + (42 << 8) + 42);
}

#[test]
fn test_in_range() {
    {
        /* no offset */
        let mem = FlashMemory::new(0, 1024, &vec![0u8; 1024]);
        assert!(mem.in_range(0));
        assert!(mem.in_range(1023));
        assert!(!mem.in_range(1024));
        assert!(!mem.in_range(0xFFFF_FFFF));
    }

    {
        /* offset of 0x8000_0000 */
        let mem = FlashMemory::new(0x8000_0000, 1024, &vec![0u8; 1024]);
        assert!(mem.in_range(0x8000_0000));
        assert!(mem.in_range(0x8000_0001));
        assert!(!mem.in_range(0x8000_0000 + 1024));
        assert!(!mem.in_range(0x8000_0000 + 0xffff));
    }
}
