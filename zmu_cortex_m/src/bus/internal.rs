use crate::bus::Bus;
use crate::bus::BusStepResult;
use crate::peripheral::itm::InstrumentationTraceMacrocell;
use crate::peripheral::scid::SystemControlAndID;
use crate::peripheral::systick::SysTick;
use std::io;

#[derive(Default)]
struct Dwt {
    pub ctrl: u32,
}

//#[derive(Default)]
pub struct InternalBus {
    syst: SysTick,
    scid: SystemControlAndID,
    dwt: Dwt,
    itm: InstrumentationTraceMacrocell,
}

impl InternalBus {
    pub fn new(itm_file: Option<Box<io::Write + 'static>>) -> InternalBus {
        InternalBus {
            syst: SysTick::default(),
            scid: SystemControlAndID::default(),
            dwt: Dwt { ctrl: 0x4000_0000 },
            itm: InstrumentationTraceMacrocell::new(itm_file),
        }
    }
}

const INTERNAL_BUS_START: u32 = 0xE000_0000;
const INTERNAL_BUS_END: u32 = 0xF000_0000;

impl Bus for InternalBus {
    fn read8(&self, addr: u32) -> u8 {
        panic!("byte access read to system area 0x{:x}", addr);
    }

    fn read16(&self, addr: u32) -> u16 {
        panic!("half-word access read to system area 0x{:x}", addr);
    }

    fn read32(&self, addr: u32) -> u32 {
        match addr {
            0xE000_0000 => self.itm.read_stim0(),

            0xE000_E010 => self.syst.read_syst_csr(),
            0xE000_E014 => self.syst.read_syst_rvr(),
            0xE000_E018 => self.syst.read_syst_cvr(),
            0xE000_E01C => self.syst.read_syst_calib(),

            0xE000_ED04 => self.scid.read_icsr(),
            0xE000_ED08 => self.scid.read_vtor(),
            0xE000_ED20 => self.scid.read_shpr3(),

            // DWT
            0xE000_1000 => self.dwt.ctrl,
            _ => panic!("bus access fault read addr 0x{:x}", addr),
        }
    }

    fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            0xE000_0000 | 0xE000_0004 | 0xE000_0008 | 0xE000_000C | 0xE000_0010 | 0xE000_0014
            | 0xE000_0018 | 0xE000_001C | 0xE000_0020 | 0xE000_0024 | 0xE000_0028 | 0xE000_002C
            | 0xE000_0030 | 0xE000_0034 | 0xE000_0038 | 0xE000_003C | 0xE000_0040 | 0xE000_0044
            | 0xE000_0048 | 0xE000_004C | 0xE000_0050 | 0xE000_0054 | 0xE000_0058 | 0xE000_005C
            | 0xE000_0060 | 0xE000_0064 | 0xE000_0068 | 0xE000_006C | 0xE000_0070 | 0xE000_0074
            | 0xE000_0078 | 0xE000_007C => self
                .itm
                .write_stim_u32(((addr - 0xE000_0000) >> 2) as u8, value),

            0xE000_1000 => self.dwt.ctrl = value,

            0xE000_ED04 => self.scid.write_icsr(value),
            0xE000_ED08 => self.scid.write_vtor(value),
            0xE000_ED20 => self.scid.write_shpr3(value),

            0xE000_E010 => self.syst.write_syst_csr(value),
            0xE000_E014 => self.syst.write_syst_rvr(value),
            0xE000_E018 => self.syst.write_syst_cvr(value),
            _ => panic!("bus access fault write addr 0x{:x}={}", addr, value),
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        match addr {
            0xE000_0000 | 0xE000_0004 | 0xE000_0008 | 0xE000_000C | 0xE000_0010 | 0xE000_0014
            | 0xE000_0018 | 0xE000_001C | 0xE000_0020 | 0xE000_0024 | 0xE000_0028 | 0xE000_002C
            | 0xE000_0030 | 0xE000_0034 | 0xE000_0038 | 0xE000_003C | 0xE000_0040 | 0xE000_0044
            | 0xE000_0048 | 0xE000_004C | 0xE000_0050 | 0xE000_0054 | 0xE000_0058 | 0xE000_005C
            | 0xE000_0060 | 0xE000_0064 | 0xE000_0068 | 0xE000_006C | 0xE000_0070 | 0xE000_0074
            | 0xE000_0078 | 0xE000_007C => self
                .itm
                .write_stim_u16(((addr - 0xE000_0000) >> 2) as u8, value),
            _ => panic!(
                "unsupported half-word access write to system area 0x{:x}->{}",
                addr, value
            ),
        }
    }

    fn write8(&mut self, addr: u32, value: u8) {
        match addr {
            0xE000_0000 | 0xE000_0004 | 0xE000_0008 | 0xE000_000C | 0xE000_0010 | 0xE000_0014
            | 0xE000_0018 | 0xE000_001C | 0xE000_0020 | 0xE000_0024 | 0xE000_0028 | 0xE000_002C
            | 0xE000_0030 | 0xE000_0034 | 0xE000_0038 | 0xE000_003C | 0xE000_0040 | 0xE000_0044
            | 0xE000_0048 | 0xE000_004C | 0xE000_0050 | 0xE000_0054 | 0xE000_0058 | 0xE000_005C
            | 0xE000_0060 | 0xE000_0064 | 0xE000_0068 | 0xE000_006C | 0xE000_0070 | 0xE000_0074
            | 0xE000_0078 | 0xE000_007C => self
                .itm
                .write_stim_u8(((addr - 0xE000_0000) >> 2) as u8, value),
            _ => panic!(
                "unsupported byte access write to system area 0x{:x}->{}",
                addr, value
            ),
        }
    }

    fn in_range(&self, addr: u32) -> bool {
        if (addr >= INTERNAL_BUS_START) && (addr < INTERNAL_BUS_END) {
            return true;
        }
        false
    }

    fn step(&mut self) -> BusStepResult {
        self.syst.step()
    }
}
