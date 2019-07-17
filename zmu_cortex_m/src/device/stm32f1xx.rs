//!
//! STM32 F1 series microcontrollers peripheral support.
//!
//!

use crate::core::bits::Bits;

const PERIPH_BASE: u32 = 0x4000_0000;

//const APB1PERIPH_BASE: u32 = PERIPH_BASE;
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x10000;
const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x20000;

const GPIOA_BASE: u32 = APB2PERIPH_BASE + 0x0800;
const GPIOA_BASE_END: u32 = GPIOA_BASE + 0x18;

const GPIOB_BASE: u32 = APB2PERIPH_BASE + 0x0C00;
const GPIOB_BASE_END: u32 = GPIOB_BASE + 0x18;

const GPIOC_BASE: u32 = APB2PERIPH_BASE + 0x1000;
const GPIOC_BASE_END: u32 = GPIOC_BASE + 0x18;

const GPIOD_BASE: u32 = APB2PERIPH_BASE + 0x1400;
const GPIOD_BASE_END: u32 = GPIOD_BASE + 0x18;

const GPIOE_BASE: u32 = APB2PERIPH_BASE + 0x1800;
const GPIOE_BASE_END: u32 = GPIOE_BASE + 0x18;

const GPIOF_BASE: u32 = APB2PERIPH_BASE + 0x1C00;
const GPIOF_BASE_END: u32 = GPIOF_BASE + 0x18;

const GPIOG_BASE: u32 = APB2PERIPH_BASE + 0x2000;
const GPIOG_BASE_END: u32 = GPIOG_BASE + 0x18;

const RCC_BASE: u32 = AHBPERIPH_BASE + 0x1000;
const RCC_BASE_END: u32 = RCC_BASE + 0x24;
const FLASH_R_BASE: u32 = AHBPERIPH_BASE + 0x2000;
const FLASH_R_BASE_END: u32 = FLASH_R_BASE + 0x04;

use crate::bus::Bus;
use crate::core::fault::Fault;

#[allow(non_snake_case)]
struct RCCRegisters {
    ///
    /// 0
    ///
    CR: u32,
    ///
    /// 4
    ///
    CFGR: u32,
    ///
    /// 8
    ///
    CIR: u32,
    ///
    /// 0xc
    ///
    APB2RSTR: u32,
    ///
    /// 0x10
    ///
    APB1RSTR: u32,
    ///
    /// 0x14
    ///
    AHBENR: u32,
    ///
    /// 0x18
    ///
    APB2ENR: u32,
    ///
    /// 0x1c
    ///
    APB1ENR: u32,
    ///
    /// 0x20
    ///
    BDCR: u32,
    ///
    /// 0x24
    ///
    CSR: u32,
}

#[allow(non_snake_case)]
struct GPIORegisters {
    ///
    /// 0
    ///
    CRL: u32,
    ///
    /// 0x4
    ///
    CRH: u32,
    ///
    /// 0x8
    ///
    IDR: u32,
    ///
    /// 0xc
    ///
    ODR: u32,
    ///
    /// 0x10
    ///
    //BSRR: u32,
    ///
    /// 0x14
    ///
    //BRR: u32,
    ///
    /// 0x18
    ///
    LCKR: u32,
}

#[allow(non_snake_case)]
struct FLASHRegisters {
    ///
    /// 0
    ///
    ACR: u32,
}

///
///
pub struct Device {
    rcc: RCCRegisters,
    gpio: [GPIORegisters; 7],
    flash: FLASHRegisters,
}

impl Device {
    ///
    ///
    pub fn new() -> Self {
        println!("initialize stm32f1xx");
        Self {
            rcc: RCCRegisters {
                CR: 0x83,
                CFGR: 0,
                CIR: 0,
                APB2RSTR: 0,
                APB1RSTR: 0,
                AHBENR: 0,
                APB2ENR: 0,
                APB1ENR: 0,
                BDCR: 0,
                CSR: 0,
            },
            gpio: [
                GPIORegisters {
                    CRL: 0x4444_4444,
                    CRH: 0x4444_4444,
                    IDR: 0x0,
                    ODR: 0x0,
                    LCKR: 0x0,
                },
                GPIORegisters {
                    CRL: 0x4444_4444,
                    CRH: 0x4444_4444,
                    IDR: 0x0,
                    ODR: 0x0,
                    LCKR: 0x0,
                },
                GPIORegisters {
                    CRL: 0x4444_4444,
                    CRH: 0x4444_4444,
                    IDR: 0x0,
                    ODR: 0x0,
                    LCKR: 0x0,
                },
                GPIORegisters {
                    CRL: 0x4444_4444,
                    CRH: 0x4444_4444,
                    IDR: 0x0,
                    ODR: 0x0,
                    LCKR: 0x0,
                },
                GPIORegisters {
                    CRL: 0x4444_4444,
                    CRH: 0x4444_4444,
                    IDR: 0x0,
                    ODR: 0x0,
                    LCKR: 0x0,
                },
                GPIORegisters {
                    CRL: 0x4444_4444,
                    CRH: 0x4444_4444,
                    IDR: 0x0,
                    ODR: 0x0,
                    LCKR: 0x0,
                },
                GPIORegisters {
                    CRL: 0x4444_4444,
                    CRH: 0x4444_4444,
                    IDR: 0x0,
                    ODR: 0x0,
                    LCKR: 0x0,
                },
            ],
            flash: FLASHRegisters { ACR: 0x30 },
        }
    }
}

trait RCC {
    fn rcc_write32(&mut self, offset: u32, value: u32) -> Result<(), Fault>;
    fn rcc_read32(&mut self, offset: u32) -> Result<u32, Fault>;
}

trait GPIO {
    fn gpio_write32(&mut self, index: usize, offset: u32, value: u32) -> Result<(), Fault>;
    fn gpio_read32(&mut self, index: usize, offset: u32) -> Result<u32, Fault>;
}

trait FLASH {
    fn flash_write32(&mut self, offset: u32, value: u32) -> Result<(), Fault>;
    fn flash_read32(&mut self, offset: u32) -> Result<u32, Fault>;
}

impl RCC for Device {
    fn rcc_write32(&mut self, offset: u32, value: u32) -> Result<(), Fault> {
        match offset {
            0x0 => {
                // Only RW bits can be modified by a write
                let rw_mask = 0b0000_0001_0000_1101_0000_0000_1111_1001;
                let masked_on = value & rw_mask;
                let masked_off = !value & rw_mask;

                self.rcc.CR |= masked_on;
                self.rcc.CR &= !masked_off;

                // PLLON -> PLL_RDY
                self.rcc.CR.set_bit(25, self.rcc.CR.get_bit(24));

                // HSEON -> HSE_RDY
                self.rcc.CR.set_bit(17, self.rcc.CR.get_bit(16));

                // HSION -> HSI_RDY
                self.rcc.CR.set_bit(1, self.rcc.CR.get_bit(0));
            }
            0x4 => self.rcc.CFGR = value,
            0x8 => self.rcc.CIR = value,
            0xc => self.rcc.APB2RSTR = value,
            0x10 => self.rcc.APB1RSTR = value,
            0x14 => self.rcc.AHBENR = value,
            0x18 => self.rcc.APB2ENR = value,
            0x1C => self.rcc.APB1ENR = value,
            0x20 => self.rcc.BDCR = value,
            0x24 => self.rcc.CSR = value,
            _ => return Err(Fault::DAccViol),
        }

        Ok(())
    }

    fn rcc_read32(&mut self, offset: u32) -> Result<u32, Fault> {
        let result = match offset {
            0x0 => self.rcc.CR,
            0x4 => self.rcc.CFGR,
            0x8 => self.rcc.CIR,
            0xc => self.rcc.APB2RSTR,
            0x10 => self.rcc.APB1RSTR,
            0x14 => self.rcc.AHBENR,
            0x18 => self.rcc.APB2ENR,
            0x1C => self.rcc.APB1ENR,
            0x20 => self.rcc.BDCR,
            0x24 => self.rcc.CSR,
            _ => return Err(Fault::DAccViol),
        };

        Ok(result)
    }
}

impl GPIO for Device {
    fn gpio_write32(&mut self, index: usize, offset: u32, value: u32) -> Result<(), Fault> {
        match offset {
            0x0 => self.gpio[index].CRL = value,
            0x4 => self.gpio[index].CRH = value,
            0x8 => (),
            0xc => self.gpio[index].ODR = value & 0xffff,
            0x10 => {
                let odr_reset_bits = value.get_bits(16..32);
                let odr_set_bits = value.get_bits(0..16);
                let odr = self.gpio[index].ODR;
                self.gpio[index].ODR = (odr | odr_reset_bits) & !odr;
                self.gpio[index].ODR |= odr_set_bits;
            }
            0x14 => {
                let odr_reset_bits = value.get_bits(0..16);
                let odr = self.gpio[index].ODR;
                self.gpio[index].ODR = (odr | odr_reset_bits) & !odr;
            }
            0x18 => self.gpio[index].LCKR = value & 0x1_ffff,
            _ => return Err(Fault::DAccViol),
        }
        Ok(())
    }

    fn gpio_read32(&mut self, index: usize, offset: u32) -> Result<u32, Fault> {
        let result = match offset {
            0x0 => self.gpio[index].CRL,
            0x4 => self.gpio[index].CRH,
            0x8 => self.gpio[index].IDR,
            0xc => self.gpio[index].ODR,
            0x10 => 0,
            0x14 => 0,
            0x18 => self.gpio[index].LCKR,
            _ => return Err(Fault::DAccViol),
        };

        Ok(result)
    }
}

impl FLASH for Device {
    fn flash_write32(&mut self, offset: u32, value: u32) -> Result<(), Fault> {
        match offset {
            0x0 => {
                // Only RW bits can be modified by a write
                let rw_mask = 0b1_1111;
                let masked_on = value & rw_mask;
                let masked_off = !value & rw_mask;

                self.flash.ACR |= masked_on;
                self.flash.ACR &= !masked_off;

                // PRFTBE -> PRFTBS
                self.flash.ACR.set_bit(5, self.flash.ACR.get_bit(4));

            }
            _ => return Err(Fault::DAccViol),
        }

        Ok(())
    }

    fn flash_read32(&mut self, offset: u32) -> Result<u32, Fault> {
        let result = match offset {
            0x0 => self.flash.ACR,
            _ => return Err(Fault::DAccViol),
        };

        Ok(result)
    }
}

impl Bus for Device {
    fn read8(&self, bus_addr: u32) -> Result<u8, Fault> {
        println!("read8 0x{:x}", bus_addr);
        Ok(0)
    }

    fn read16(&self, bus_addr: u32) -> Result<u16, Fault> {
        println!("read16 0x{:x}", bus_addr);
        Ok(0)
    }

    fn read32(&mut self, bus_addr: u32) -> Result<u32, Fault> {
        println!("read32 0x{:x}", bus_addr);
        match bus_addr {
            RCC_BASE..=RCC_BASE_END => self.rcc_read32(bus_addr - RCC_BASE),
            GPIOA_BASE..=GPIOA_BASE_END => self.gpio_read32(0, bus_addr - GPIOA_BASE),
            GPIOB_BASE..=GPIOB_BASE_END => self.gpio_read32(1, bus_addr - GPIOB_BASE),
            GPIOC_BASE..=GPIOC_BASE_END => self.gpio_read32(2, bus_addr - GPIOC_BASE),
            GPIOD_BASE..=GPIOD_BASE_END => self.gpio_read32(3, bus_addr - GPIOD_BASE),
            GPIOE_BASE..=GPIOE_BASE_END => self.gpio_read32(4, bus_addr - GPIOE_BASE),
            GPIOF_BASE..=GPIOF_BASE_END => self.gpio_read32(5, bus_addr - GPIOF_BASE),
            GPIOG_BASE..=GPIOG_BASE_END => self.gpio_read32(6, bus_addr - GPIOG_BASE),
            FLASH_R_BASE..=FLASH_R_BASE_END => self.flash_read32(bus_addr - FLASH_R_BASE),
            _ => Err(Fault::DAccViol),
        }
    }

    fn write32(&mut self, addr: u32, value: u32) -> Result<(), Fault> {
        println!("write32 0x{:x}=0x{:x}", addr, value);
        match addr {
            RCC_BASE..=RCC_BASE_END => self.rcc_write32(addr - RCC_BASE, value),
            GPIOA_BASE..=GPIOA_BASE_END => self.gpio_write32(0, addr - GPIOA_BASE, value),
            GPIOB_BASE..=GPIOB_BASE_END => self.gpio_write32(1, addr - GPIOB_BASE, value),
            GPIOC_BASE..=GPIOC_BASE_END => self.gpio_write32(2, addr - GPIOC_BASE, value),
            GPIOD_BASE..=GPIOD_BASE_END => self.gpio_write32(3, addr - GPIOD_BASE, value),
            GPIOE_BASE..=GPIOE_BASE_END => self.gpio_write32(4, addr - GPIOE_BASE, value),
            GPIOF_BASE..=GPIOF_BASE_END => self.gpio_write32(5, addr - GPIOF_BASE, value),
            GPIOG_BASE..=GPIOG_BASE_END => self.gpio_write32(6, addr - GPIOG_BASE, value),
            FLASH_R_BASE..=FLASH_R_BASE_END => self.flash_write32(addr - FLASH_R_BASE, value),
            _ => Err(Fault::DAccViol),
        }
    }

    fn write16(&mut self, addr: u32, value: u16) -> Result<(), Fault> {
        println!("write16 0x{:x}=0x{:x}", addr, value);
        Ok(())
    }

    fn write8(&mut self, addr: u32, value: u8) -> Result<(), Fault> {
        println!("write8 0x{:x}=0x{:x}", addr, value);
        Ok(())
    }

    #[allow(unused)]
    fn in_range(&self, addr: u32) -> bool {
        addr >= PERIPH_BASE && addr < FLASH_R_BASE_END
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rcc_cr_init() {
        {
            let mut device = Device::new();
            assert_eq!(device.rcc_read32(0).unwrap(), 0x83);
        }
    }

    #[test]
    fn test_rcc_cr_write_all() -> Result<(), Fault> {
        {
            let mut device = Device::new();
            device.rcc_write32(0, 0xffff_ffff)?;
            assert_eq!(
                device.rcc_read32(0)?,
                0b0000_0011_0000_1111_0000_0000_1111_1011
            );
            Ok(())
        }
    }

    #[test]
    fn test_rcc_cr_hse_on() -> Result<(), Fault> {
        {
            let mut device = Device::new();
            // HSE_ON enables HSE_RDY
            device.rcc_write32(0, 0x10000)?;
            assert_eq!(device.rcc_read32(0)? & 0x30000, 0x30000);
            Ok(())
        }
    }

    #[test]
    fn test_rcc_cr_hse_off() -> Result<(), Fault> {
        {
            let mut device = Device::new();
            device.rcc_write32(0, 0x10000)?;
            device.rcc_write32(0, 0)?;
            assert_eq!(device.rcc_read32(0)? & 0x30000, 0);
            Ok(())
        }
    }

}
