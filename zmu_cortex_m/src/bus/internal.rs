use crate::bus::Bus;

#[derive(Default)]
struct SysTick {
    pub rvr: u32,
    pub cvr: u32,
    pub csr: u32,
}

#[derive(Default)]
struct Dwt {
    pub ctrl: u32,
}

#[derive(Default)]
pub struct InternalBus {
    shpr3: u32, // RW, 0xe000_ed20, reset value = SBZ (systick, pendsv bits are zero)
    vtor: u32,  // RW, 0xe000_ed08, reset value = 0
    icsr: u32,  // Interrupt Control and State Register RW, 0xe000_ed04, reset value = 0
    syst: SysTick,
    dwt: Dwt, /*
    ACTLR: u32, // RW, 0xe000_e008, reset value = implementation defined
    
    SYST_CSR: u32, // RW, 0xe000_e010, reset value = 0 or 4, 
    SYST_RVR: u32, // RW, 0xe000_e014, reset value = unknown, 
    SYST_CVR: u32, // RW, 0xe000_e018, reset value = unknown, 
    SYST_CALIB: u32, // R0, 0xe000_e01C, reset value = implementation_defined, 
    
    CPUID: u32, // RO, 0xe000_ed00, reset value = implementation defined
    
    AIRCR: u32, // RW, 0xe000_ed0c, reset value = bits [10:8] = 0b000
    SCR: u32,   // RW, 0xe000_ed10, reset value = bits [4,2,1] = 0b000
    CCR: u32,   // RO, 0xe000_ed14, reset value = bits [9:3] = 0b111111
    SHPR2: u32, // RW, 0xe000_ed1c, reset value = SBZ (svcall priority is zero)
    SHCSR: u32, // RW, 0xe000_ed24, reset value = 0
    DFSR: u32,  // RW, 0xe000_ed30, reset value = 0*/

/*DWT_CTRL	Read/write	0xE0001000	0x00000000	See DWT Control Register
DWT_CYCCNT	Read/write	0xE0001004	0x00000000	See DWT Current PC Sampler Cycle Count Register
DWT_CPICNT	Read/write	0xE0001008	-	See DWT CPI Count Register
DWT_EXCCNT	Read/write	0xE000100C	-	See DWT Exception Overhead Count Register
DWT_SLEEPCNT	Read/write	0xE0001010	-	See DWT Sleep Count Register
DWT_LSUCNT	Read/write	0xE0001014	-	See DWT LSU Count Register
DWT_FOLDCNT	Read/write	0xE0001018	-	See DWT Fold Count Register
DWT_PCSR	Read-only	0xE000101C	-	See DWT Program Counter Sample Register
DWT_COMP0	Read/write	0xE0001020	-	See DWT Comparator Registers
DWT_MASK0	Read/write	0xE0001024	-	See DWT Mask Registers 0-3
DWT_FUNCTION0	Read/write	0xE0001028	0x00000000	See DWT Function Registers 0-3
DWT_COMP1	Read/write	0xE0001030	-	See DWT Comparator Registers
DWT_MASK1	Read/write	0xE0001034	-	See DWT Mask Registers 0-3
DWT_FUNCTION1	Read/write	0xE0001038	0x00000000	See DWT Function Registers 0-3
DWT_COMP2	Read/write	0xE0001040	-	See DWT Comparator Registers
DWT_MASK2	Read/write	0xE0001044	-	See DWT Mask Registers 0-3
DWT_FUNCTION2	Read/write	0xE0001048	0x00000000	See DWT Function Registers 0-3
DWT_COMP3	Read/write	0xE0001050	-	See DWT Comparator Registers
DWT_MASK3	Read/write	0xE0001054 	-	See DWT Mask Registers 0-3
DWT_FUNCTION3	Read/write	0xE0001058 	0x00000000	See DWT Function Registers 0-3
PID4	Read-only	0xE0001FD0	0x04	Value 0x04
PID5	Read-only	0xE0001FD4	0x00	Value 0x00
PID6	Read-only	0xE0001FD8	0x00	Value 0x00
PID7	Read-only	0xE0001FDC	0x00	Value 0x00
PID0	Read-only	0xE0001FE0	0x02	Value 0x02
PID1	Read-only	0xE0001FE4	0xB0	Value 0xB0
PID2	Read-only	0xE0001FE8	0x1B	Value 0x1B
PID3	Read-only	0xE0001FEC	0x00	Value 0x00
CID0	Read-only	0xE0001FF0	0x0D	Value 0x0D
CID1	Read-only	0xE0001FF4	0xE0	Value 0xE0
CID2	Read-only	0xE0001FF8	0x05	Value 0x05
CID3	Read-only	0xE0001FFC	0xB1	Value 0xB1*/
}

impl InternalBus {
    fn read_shpr3(&self) -> u32 {
        self.shpr3
    }

    fn write_shpr3(&mut self, value: u32) {
        self.shpr3 = value
    }

    fn write_syst_rvr(&mut self, value: u32) {
        self.syst.rvr = value
    }

    fn write_syst_cvr(&mut self, value: u32) {
        self.syst.cvr = value
    }

    fn write_syst_csr(&mut self, value: u32) {
        self.syst.csr = value
    }

    fn write_vtor(&mut self, value: u32) {
        self.vtor = value
    }

    pub fn new() -> InternalBus {
        InternalBus {
            vtor: 0,
            shpr3: 0,
            syst: SysTick::default(),
            dwt: Dwt { ctrl: 0x4000_0000 },
            icsr: 0,
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
            0xE000_ED20 => self.read_shpr3(),
            // DWT
            0xE000_1000 => self.dwt.ctrl,
            _ => panic!("bus access fault read addr 0x{:x}", addr),
        }
    }

    fn write32(&mut self, addr: u32, value: u32) {
        match addr {
            0xE000_1000 => self.dwt.ctrl = value,
            0xE000_ED04 => {
                println!("change of ICSR {:x} -> {:x}", self.icsr, value);
                self.icsr = value;
            }
            0xE000_ED08 => self.write_vtor(value),
            0xE000_ED20 => self.write_shpr3(value),
            0xE000_E010 => self.write_syst_csr(value),
            0xE000_E014 => self.write_syst_rvr(value),
            0xE000_E018 => self.write_syst_cvr(value),
            _ => panic!("bus access fault write addr 0x{:x}={}", addr, value),
        }
    }

    fn write16(&mut self, addr: u32, value: u16) {
        panic!(
            "half-word access write to system area 0x{:x}->{}",
            addr, value
        );
    }

    fn write8(&mut self, addr: u32, value: u8) {
        panic!("byte access write to system area 0x{:x}->{}", addr, value);
    }

    fn in_range(&self, addr: u32) -> bool {
        if (addr >= INTERNAL_BUS_START) && (addr < INTERNAL_BUS_END) {
            return true;
        }
        false
    }
}
