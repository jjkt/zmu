use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Seek, SeekFrom};
use bus::Bus;


pub struct RAM<'a> {
    start_address: u32,
    access: Cursor<&'a mut [u8]>,
    size: usize,
}

impl<'a> RAM<'a> {
    pub fn new(data: &'a mut [u8], start_address: u32) -> RAM<'a> {

        let len = data.len();

        RAM {
            start_address: start_address,
            access: Cursor::new(data),
            size: len,
        }
    }
}

impl<'a> Bus for RAM<'a> {
    fn read16(&mut self, addr: u32) -> u16 {
        self.access.seek(SeekFrom::Start((addr - self.start_address) as u64)).unwrap();
        let value = self.access.read_u16::<LittleEndian>().unwrap();
        print!("RAM R16 [0x{:x}] => 0x{:x}\n", addr, value);
        value
    }

    fn read32(&mut self, addr: u32) -> u32 {
        self.access.seek(SeekFrom::Start((addr - self.start_address) as u64)).unwrap();
        let value = self.access.read_u32::<LittleEndian>().unwrap();
        print!("RAM R32 [0x{:x}] => 0x{:x}\n", addr, value);
        value
    }

    fn write32(&mut self, addr: u32, value: u32) {
        self.access.seek(SeekFrom::Start((addr - self.start_address) as u64)).unwrap();
        self.access.write_u32::<LittleEndian>(value).unwrap();
        print!("RAM W32 [0x{:x}] <= 0x{:x}\n", addr, value);
    }

    fn in_range(&self, addr: u32) -> bool {
        if (addr >= self.start_address) && (addr <= (self.start_address + self.size as u32)) {
            return true;
        }
        false
    }
}
