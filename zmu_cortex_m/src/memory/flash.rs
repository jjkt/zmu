use bus::Bus;
use byteorder::{ByteOrder, LittleEndian};

pub struct FlashMemory {
    start_address: u32,
    data: Box<[u8]>,
}

impl FlashMemory {
    pub fn new(start_address: u32, size: usize) -> FlashMemory {
        let data = vec![0u8; size].into_boxed_slice();

        FlashMemory {
            start_address: start_address,
            data: data,
        }
    }

    pub fn load(&mut self, new_data: &[u8]) {
        self.data.copy_from_slice(new_data);
    }
}

impl Bus for FlashMemory {
    fn read8(&self, addr: u32) -> u8 {
        let a = addr - self.start_address;
        self.data[a as usize]
    }
    fn read16(&self, addr: u32) -> u16 {
        let a = (addr - self.start_address) as usize;

        LittleEndian::read_u16(&self.data[a..a + 2])
    }

    fn read32(&self, addr: u32) -> u32 {
        let a = (addr - self.start_address) as usize;
        LittleEndian::read_u32(&self.data[a..a + 4])
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
    let _mem = FlashMemory::new(0, 1024);
}

#[test]
fn test_load() {
    let mut mem = FlashMemory::new(0, 1024);
    mem.load(&vec![42u8; 1024]);
    assert_eq!(mem.read8(0), 42);
    assert_eq!(mem.read16(0), (42 << 8) + 42);
    assert_eq!(mem.read32(0), (42 << 24) + (42 << 16) + (42 << 8) + 42);
}

#[test]
fn test_in_range() {
    {
        /* no offset */
        let mem = FlashMemory::new(0, 1024);
        assert!(mem.in_range(0));
        assert!(mem.in_range(1023));
        assert!(!mem.in_range(1024));
        assert!(!mem.in_range(0xFFFF_FFFF));
    }

    {
        /* offset of 0x8000_0000 */
        let mem = FlashMemory::new(0x8000_0000, 1024);
        assert!(mem.in_range(0x8000_0000));
        assert!(mem.in_range(0x8000_0001));
        assert!(!mem.in_range(0x8000_0000 + 1024));
        assert!(!mem.in_range(0x8000_0000 + 0xffff));
    }
}
