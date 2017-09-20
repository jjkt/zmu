use std::io::Cursor;
use std::io::SeekFrom;
use std::io::Seek;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};


pub trait Fetch {
    fn fetch32(&mut self, addr: u32) -> u32;
    fn fetch16(&mut self, addr: u32) -> u16;
    fn write32(&mut self, addr: u32, value: u32);
}

pub struct SystemMemory<'a> {
    access: Cursor<&'a mut [u8]>,
}

impl<'a> SystemMemory<'a> {
    pub fn new(bin: &mut [u8]) -> SystemMemory {
        SystemMemory { access: Cursor::new(bin) }
    }
}

impl<'a> Fetch for SystemMemory<'a> {
    fn fetch16(&mut self, addr: u32) -> u16 {
        self.access.seek(SeekFrom::Start(addr as u64)).unwrap();
        self.access.read_u16::<LittleEndian>().unwrap()
    }

    fn fetch32(&mut self, addr: u32) -> u32 {
        self.access.seek(SeekFrom::Start(addr as u64)).unwrap();
        self.access.read_u32::<LittleEndian>().unwrap()
    }

    fn write32(&mut self, addr: u32, value: u32) {
        self.access.seek(SeekFrom::Start(addr as u64)).unwrap();
        self.access.write_u32::<LittleEndian>(value).unwrap()
    }
}
