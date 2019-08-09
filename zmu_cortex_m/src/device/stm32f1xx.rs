//!
//! STM32 F1 series microcontrollers peripheral support.
//!
//!

use crate::core::bits::Bits;

const PERIPH_BASE: u32 = 0x4000_0000;

const APB1PERIPH_BASE: u32 = PERIPH_BASE;
const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x10000;
const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x20000;

const TIM2_BASE: u32 = APB1PERIPH_BASE;
const TIM2_BASE_END: u32 = TIM2_BASE + 0x50;
const TIM3_BASE: u32 = APB1PERIPH_BASE + 0x0400;
const TIM3_BASE_END: u32 = TIM3_BASE + 0x50;
const TIM4_BASE: u32 = APB1PERIPH_BASE + 0x0800;
const TIM4_BASE_END: u32 = TIM4_BASE + 0x50;
const TIM5_BASE: u32 = APB1PERIPH_BASE + 0x0C00;
const TIM5_BASE_END: u32 = TIM5_BASE + 0x50;

const TIM6_BASE: u32 = APB1PERIPH_BASE + 0x1000;
const TIM6_BASE_END: u32 = TIM6_BASE + 0x50;
const TIM7_BASE: u32 = APB1PERIPH_BASE + 0x1400;
const TIM7_BASE_END: u32 = TIM7_BASE + 0x50;

const TIM12_BASE: u32 = APB1PERIPH_BASE + 0x1800;
const TIM12_BASE_END: u32 = TIM12_BASE + 0x50;
const TIM13_BASE: u32 = APB1PERIPH_BASE + 0x1C00;
const TIM13_BASE_END: u32 = TIM13_BASE + 0x50;
const TIM14_BASE: u32 = APB1PERIPH_BASE + 0x2000;
const TIM14_BASE_END: u32 = TIM14_BASE + 0x50;

const TIM1_BASE: u32 = APB2PERIPH_BASE + 0x2c00;
const TIM1_BASE_END: u32 = TIM1_BASE + 0x50;
const TIM8_BASE: u32 = APB2PERIPH_BASE + 0x3400;
const TIM8_BASE_END: u32 = TIM8_BASE + 0x50;

const TIM9_BASE: u32 = APB2PERIPH_BASE + 0x4c00;
const TIM9_BASE_END: u32 = TIM9_BASE + 0x50;
const TIM10_BASE: u32 = APB2PERIPH_BASE + 0x5000;
const TIM10_BASE_END: u32 = TIM10_BASE + 0x50;
const TIM11_BASE: u32 = APB2PERIPH_BASE + 0x5400;
const TIM11_BASE_END: u32 = TIM11_BASE + 0x50;

const AFIO_BASE: u32 = APB2PERIPH_BASE;
const AFIO_BASE_END: u32 = AFIO_BASE + 0x20;

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

#[allow(non_snake_case)]
struct AFIORegisters {
    EVCR: u32,
    MAPR: u32,
    EXTICR: [u32; 4],
    MAPR2: u32,
}

enum AdvancedControlTimerType {
    TIM1,
    TIM8,
}

enum GeneralPurposeTimerType {
    TIM2,
    TIM3,
    TIM4,
    TIM5,
}

enum GeneralPurposeTimer2Type {
    TIM9,
    TIM10,
    TIM11,
    TIM12,
    TIM13,
    TIM14,
}

enum BasicTimerType {
    TIM6,
    TIM7,
}

#[allow(non_snake_case)]
struct MinimumTimerRegisters {
    CR1: u32,
    DIER: u32,
    SR: u32,
    EGR: u32,
    CNT: u32,
    PSC: u32,
    ARR: u32,
}

#[allow(non_snake_case)]
struct BasicTimerRegisters {
    min: MinimumTimerRegisters,
    CR2: u32,
}

#[allow(non_snake_case)]
struct GeneralPurposeTimerRegisters {
    min: MinimumTimerRegisters,
    SMCR: u32,
    CCMR1: u32,
    CCER: u32,
    CCR1: u32,
}

#[allow(non_snake_case)]
struct GeneralPurposeTimer2Registers {
    gp: GeneralPurposeTimerRegisters,
    CR2: u32,
    CCMR2: u32,
    CCR2: u32,
    CCR3: u32,
    CCR4: u32,
    DCR: u32,
    DMAR: u32,
}

#[allow(non_snake_case)]
struct AdvancedControlTimerRegisters {
    gp: GeneralPurposeTimer2Registers,
    RCR: u32,
    BDTR: u32,
}

///
///
pub struct Device {
    afio: AFIORegisters,
    rcc: RCCRegisters,
    gpio: [GPIORegisters; 7],
    flash: FLASHRegisters,
    tim1_8: [AdvancedControlTimerRegisters; 2],
    tim2_5: [GeneralPurposeTimer2Registers; 4],
    tim6_7: [BasicTimerRegisters; 2],
    tim9_14: [GeneralPurposeTimerRegisters; 6],
}

impl Device {
    ///
    ///
    pub fn new() -> Self {
        println!("initialize stm32f1xx");
        Self {
            afio: AFIORegisters {
                EVCR: 0,
                MAPR: 0,
                EXTICR: [0; 4],
                MAPR2: 0,
            },
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
            tim1_8: [
                AdvancedControlTimerRegisters {
                    gp: GeneralPurposeTimer2Registers {
                        gp: GeneralPurposeTimerRegisters {
                            min: MinimumTimerRegisters {
                                CR1: 0x0,
                                CNT: 0x0,
                                DIER: 0x0,
                                EGR: 0x0,
                                PSC: 0x0,
                                SR: 0x0,
                                ARR: 0x0000_ffff,
                            },
                            CCER: 0x0,
                            CCMR1: 0x0,
                            CCR1: 0x0,
                            SMCR: 0x0,
                        },
                        CCMR2: 0x0,
                        CCR2: 0x0,
                        CCR3: 0x0,
                        CCR4: 0x0,
                        CR2: 0x0,
                        DMAR: 0x0,
                        DCR: 0x0,
                    },
                    BDTR: 0x0,
                    RCR: 0x0,
                },
                AdvancedControlTimerRegisters {
                    gp: GeneralPurposeTimer2Registers {
                        gp: GeneralPurposeTimerRegisters {
                            min: MinimumTimerRegisters {
                                CR1: 0x0,
                                CNT: 0x0,
                                DIER: 0x0,
                                EGR: 0x0,
                                PSC: 0x0,
                                SR: 0x0,
                                ARR: 0x0000_ffff,
                            },
                            CCER: 0x0,
                            CCMR1: 0x0,
                            CCR1: 0x0,
                            SMCR: 0x0,
                        },
                        CCMR2: 0x0,
                        CCR2: 0x0,
                        CCR3: 0x0,
                        CCR4: 0x0,
                        CR2: 0x0,
                        DMAR: 0x0,
                        DCR: 0x0,
                    },
                    BDTR: 0x0,
                    RCR: 0x0,
                },
            ],

            tim2_5: [
                GeneralPurposeTimer2Registers {
                    gp: GeneralPurposeTimerRegisters {
                        min: MinimumTimerRegisters {
                            CR1: 0x0,
                            CNT: 0x0,
                            DIER: 0x0,
                            EGR: 0x0,
                            PSC: 0x0,
                            SR: 0x0,
                            ARR: 0x0000_ffff,
                        },
                        CCER: 0x0,
                        CCMR1: 0x0,
                        CCR1: 0x0,
                        SMCR: 0x0,
                    },
                    CCMR2: 0x0,
                    CCR2: 0x0,
                    CCR3: 0x0,
                    CCR4: 0x0,
                    CR2: 0x0,
                    DMAR: 0x0,
                    DCR: 0x0,
                },
                GeneralPurposeTimer2Registers {
                    gp: GeneralPurposeTimerRegisters {
                        min: MinimumTimerRegisters {
                            CR1: 0x0,
                            CNT: 0x0,
                            DIER: 0x0,
                            EGR: 0x0,
                            PSC: 0x0,
                            SR: 0x0,
                            ARR: 0x0000_ffff,
                        },
                        CCER: 0x0,
                        CCMR1: 0x0,
                        CCR1: 0x0,
                        SMCR: 0x0,
                    },
                    CCMR2: 0x0,
                    CCR2: 0x0,
                    CCR3: 0x0,
                    CCR4: 0x0,
                    CR2: 0x0,
                    DMAR: 0x0,
                    DCR: 0x0,
                },
                GeneralPurposeTimer2Registers {
                    gp: GeneralPurposeTimerRegisters {
                        min: MinimumTimerRegisters {
                            CR1: 0x0,
                            CNT: 0x0,
                            DIER: 0x0,
                            EGR: 0x0,
                            PSC: 0x0,
                            SR: 0x0,
                            ARR: 0x0000_ffff,
                        },
                        CCER: 0x0,
                        CCMR1: 0x0,
                        CCR1: 0x0,
                        SMCR: 0x0,
                    },
                    CCMR2: 0x0,
                    CCR2: 0x0,
                    CCR3: 0x0,
                    CCR4: 0x0,
                    CR2: 0x0,
                    DMAR: 0x0,
                    DCR: 0x0,
                },
                GeneralPurposeTimer2Registers {
                    gp: GeneralPurposeTimerRegisters {
                        min: MinimumTimerRegisters {
                            CR1: 0x0,
                            CNT: 0x0,
                            DIER: 0x0,
                            EGR: 0x0,
                            PSC: 0x0,
                            SR: 0x0,
                            ARR: 0x0000_ffff,
                        },
                        CCER: 0x0,
                        CCMR1: 0x0,
                        CCR1: 0x0,
                        SMCR: 0x0,
                    },
                    CCMR2: 0x0,
                    CCR2: 0x0,
                    CCR3: 0x0,
                    CCR4: 0x0,
                    CR2: 0x0,
                    DMAR: 0x0,
                    DCR: 0x0,
                },
            ],
            tim6_7: [
                BasicTimerRegisters {
                    min: MinimumTimerRegisters {
                        CR1: 0x0,
                        CNT: 0x0,
                        DIER: 0x0,
                        EGR: 0x0,
                        PSC: 0x0,
                        SR: 0x0,
                        ARR: 0x0000_ffff,
                    },
                    CR2: 0x0,
                },
                BasicTimerRegisters {
                    min: MinimumTimerRegisters {
                        CR1: 0x0,
                        CNT: 0x0,
                        DIER: 0x0,
                        EGR: 0x0,
                        PSC: 0x0,
                        SR: 0x0,
                        ARR: 0x0000_ffff,
                    },
                    CR2: 0x0,
                },
            ],
            tim9_14: [
                GeneralPurposeTimerRegisters {
                    min: MinimumTimerRegisters {
                        CR1: 0x0,
                        CNT: 0x0,
                        DIER: 0x0,
                        EGR: 0x0,
                        PSC: 0x0,
                        SR: 0x0,
                        ARR: 0x0000_ffff,
                    },
                    CCER: 0x0,
                    CCMR1: 0x0,
                    CCR1: 0x0,
                    SMCR: 0x0,
                },
                GeneralPurposeTimerRegisters {
                    min: MinimumTimerRegisters {
                        CR1: 0x0,
                        CNT: 0x0,
                        DIER: 0x0,
                        EGR: 0x0,
                        PSC: 0x0,
                        SR: 0x0,
                        ARR: 0x0000_ffff,
                    },
                    CCER: 0x0,
                    CCMR1: 0x0,
                    CCR1: 0x0,
                    SMCR: 0x0,
                },
                GeneralPurposeTimerRegisters {
                    min: MinimumTimerRegisters {
                        CR1: 0x0,
                        CNT: 0x0,
                        DIER: 0x0,
                        EGR: 0x0,
                        PSC: 0x0,
                        SR: 0x0,
                        ARR: 0x0000_ffff,
                    },
                    CCER: 0x0,
                    CCMR1: 0x0,
                    CCR1: 0x0,
                    SMCR: 0x0,
                },
                GeneralPurposeTimerRegisters {
                    min: MinimumTimerRegisters {
                        CR1: 0x0,
                        CNT: 0x0,
                        DIER: 0x0,
                        EGR: 0x0,
                        PSC: 0x0,
                        SR: 0x0,
                        ARR: 0x0000_ffff,
                    },
                    CCER: 0x0,
                    CCMR1: 0x0,
                    CCR1: 0x0,
                    SMCR: 0x0,
                },
                GeneralPurposeTimerRegisters {
                    min: MinimumTimerRegisters {
                        CR1: 0x0,
                        CNT: 0x0,
                        DIER: 0x0,
                        EGR: 0x0,
                        PSC: 0x0,
                        SR: 0x0,
                        ARR: 0x0000_ffff,
                    },
                    CCER: 0x0,
                    CCMR1: 0x0,
                    CCR1: 0x0,
                    SMCR: 0x0,
                },
                GeneralPurposeTimerRegisters {
                    min: MinimumTimerRegisters {
                        CR1: 0x0,
                        CNT: 0x0,
                        DIER: 0x0,
                        EGR: 0x0,
                        PSC: 0x0,
                        SR: 0x0,
                        ARR: 0x0000_ffff,
                    },
                    CCER: 0x0,
                    CCMR1: 0x0,
                    CCR1: 0x0,
                    SMCR: 0x0,
                },
            ],
        }
    }
}

trait AFIO {
    fn afio_write32(&mut self, offset: u32, value: u32) -> Result<(), Fault>;
    fn afio_read32(&mut self, offset: u32) -> Result<u32, Fault>;
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

trait TIM {
    fn tim_advanced_control_timer_write32(
        &mut self,
        timer: AdvancedControlTimerType,
        offset: u32,
        value: u32,
    ) -> Result<(), Fault>;
    fn tim_advanced_control_timer_read32(
        &mut self,
        timer: AdvancedControlTimerType,
        offset: u32,
    ) -> Result<u32, Fault>;
    fn tim_general_purpose_timer_write32(
        &mut self,
        timer: GeneralPurposeTimerType,
        offset: u32,
        value: u32,
    ) -> Result<(), Fault>;
    fn tim_general_purpose_timer_read32(
        &mut self,
        timer: GeneralPurposeTimerType,
        offset: u32,
    ) -> Result<u32, Fault>;
    fn tim_general_purpose_timer2_write32(
        &mut self,
        timer: GeneralPurposeTimer2Type,
        offset: u32,
        value: u32,
    ) -> Result<(), Fault>;
    fn tim_general_purpose_timer2_read32(
        &mut self,
        timer: GeneralPurposeTimer2Type,
        offset: u32,
    ) -> Result<u32, Fault>;
    fn tim_basic_timer_write32(
        &mut self,
        timer: BasicTimerType,
        offset: u32,
        value: u32,
    ) -> Result<(), Fault>;
    fn tim_basic_timer_read32(&mut self, timer: BasicTimerType, offset: u32) -> Result<u32, Fault>;
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

impl AFIO for Device {
    fn afio_write32(&mut self, offset: u32, value: u32) -> Result<(), Fault> {
        match offset {
            0x0 => self.afio.EVCR = value,
            0x4 => self.afio.MAPR = value,
            0x8 => self.afio.EXTICR[0] = value,
            0xc => self.afio.EXTICR[1] = value,
            0x10 => self.afio.EXTICR[2] = value,
            0x14 => self.afio.EXTICR[3] = value,
            //0x18 => self.rcc.APB2ENR = value,
            0x1C => self.afio.MAPR = value,
            _ => return Err(Fault::DAccViol),
        }

        Ok(())
    }

    fn afio_read32(&mut self, offset: u32) -> Result<u32, Fault> {
        let result = match offset {
            0x0 => self.afio.EVCR,
            0x4 => self.afio.MAPR,
            0x8 => self.afio.EXTICR[0],
            0xc => self.afio.EXTICR[1],
            0x10 => self.afio.EXTICR[2],
            0x14 => self.afio.EXTICR[3],
            //0x18 => self.rcc.APB2ENR,
            0x1C => self.afio.MAPR2,
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

impl TIM for Device {
    fn tim_advanced_control_timer_write32(
        &mut self,
        timer: AdvancedControlTimerType,
        offset: u32,
        value: u32,
    ) -> Result<(), Fault> {
        let index = match timer {
            AdvancedControlTimerType::TIM1 => 0,
            AdvancedControlTimerType::TIM8 => 1,
        };

        match offset {
            0x0 => self.tim1_8[index].gp.gp.min.CR1 = value,
            _ => return Err(Fault::DAccViol),
        }

        Ok(())
    }

    fn tim_advanced_control_timer_read32(
        &mut self,
        timer: AdvancedControlTimerType,
        offset: u32,
    ) -> Result<u32, Fault> {
        let index = match timer {
            AdvancedControlTimerType::TIM1 => 0,
            AdvancedControlTimerType::TIM8 => 1,
        };

        let result = match offset {
            0x0 => self.tim1_8[index].gp.gp.min.CR1,
            _ => return Err(Fault::DAccViol),
        };

        Ok(result)
    }

    fn tim_general_purpose_timer_write32(
        &mut self,
        timer: GeneralPurposeTimerType,
        offset: u32,
        value: u32,
    ) -> Result<(), Fault> {
        let index = match timer {
            GeneralPurposeTimerType::TIM2 => 0,
            GeneralPurposeTimerType::TIM3 => 1,
            GeneralPurposeTimerType::TIM4 => 2,
            GeneralPurposeTimerType::TIM5 => 3,
        };

        match offset {
            0x0 => self.tim2_5[index].gp.min.CR1 = value & 0xffff,
            0x18 => self.tim2_5[index].gp.CCMR1 = value & 0xffff,
            0x1c => self.tim2_5[index].CCMR2 = value & 0xffff,
            0x28 => self.tim2_5[index].gp.min.PSC = value & 0xffff,
            0x2c => self.tim2_5[index].gp.min.ARR = value & 0xffff,
            _ => return Err(Fault::DAccViol),
        }

        Ok(())
    }

    fn tim_general_purpose_timer_read32(
        &mut self,
        timer: GeneralPurposeTimerType,
        offset: u32,
    ) -> Result<u32, Fault> {
        let index = match timer {
            GeneralPurposeTimerType::TIM2 => 0,
            GeneralPurposeTimerType::TIM3 => 1,
            GeneralPurposeTimerType::TIM4 => 2,
            GeneralPurposeTimerType::TIM5 => 3,
        };

        let result = match offset {
            0x0 => self.tim2_5[index].gp.min.CR1,
            0x18 => self.tim2_5[index].gp.CCMR1,
            0x1c => self.tim2_5[index].CCMR2,
            0x28 => self.tim2_5[index].gp.min.PSC,
            0x2C => self.tim2_5[index].gp.min.ARR,
            _ => return Err(Fault::DAccViol),
        };

        Ok(result)
    }

    fn tim_general_purpose_timer2_write32(
        &mut self,
        timer: GeneralPurposeTimer2Type,
        offset: u32,
        value: u32,
    ) -> Result<(), Fault> {
        let index = match timer {
            GeneralPurposeTimer2Type::TIM9 => 0,
            GeneralPurposeTimer2Type::TIM10 => 1,
            GeneralPurposeTimer2Type::TIM11 => 2,
            GeneralPurposeTimer2Type::TIM12 => 4,
            GeneralPurposeTimer2Type::TIM13 => 5,
            GeneralPurposeTimer2Type::TIM14 => 6,
        };

        match offset {
            0x0 => self.tim9_14[index].min.CR1 = value,
            _ => return Err(Fault::DAccViol),
        }

        Ok(())
    }

    fn tim_general_purpose_timer2_read32(
        &mut self,
        timer: GeneralPurposeTimer2Type,
        offset: u32,
    ) -> Result<u32, Fault> {
        let index = match timer {
            GeneralPurposeTimer2Type::TIM9 => 0,
            GeneralPurposeTimer2Type::TIM10 => 1,
            GeneralPurposeTimer2Type::TIM11 => 2,
            GeneralPurposeTimer2Type::TIM12 => 4,
            GeneralPurposeTimer2Type::TIM13 => 5,
            GeneralPurposeTimer2Type::TIM14 => 6,
        };

        let result = match offset {
            0x0 => self.tim9_14[index].min.CR1,
            _ => return Err(Fault::DAccViol),
        };

        Ok(result)
    }

    fn tim_basic_timer_write32(
        &mut self,
        timer: BasicTimerType,
        offset: u32,
        value: u32,
    ) -> Result<(), Fault> {
        let index = match timer {
            BasicTimerType::TIM6 => 0,
            BasicTimerType::TIM7 => 1,
        };

        match offset {
            0x0 => self.tim6_7[index].min.CR1 = value,
            _ => return Err(Fault::DAccViol),
        }

        Ok(())
    }

    fn tim_basic_timer_read32(&mut self, timer: BasicTimerType, offset: u32) -> Result<u32, Fault> {
        let index = match timer {
            BasicTimerType::TIM6 => 0,
            BasicTimerType::TIM7 => 1,
        };

        let result = match offset {
            0x0 => self.tim6_7[index].min.CR1,
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
            AFIO_BASE..=AFIO_BASE_END => self.afio_read32(bus_addr - AFIO_BASE),
            RCC_BASE..=RCC_BASE_END => self.rcc_read32(bus_addr - RCC_BASE),
            GPIOA_BASE..=GPIOA_BASE_END => self.gpio_read32(0, bus_addr - GPIOA_BASE),
            GPIOB_BASE..=GPIOB_BASE_END => self.gpio_read32(1, bus_addr - GPIOB_BASE),
            GPIOC_BASE..=GPIOC_BASE_END => self.gpio_read32(2, bus_addr - GPIOC_BASE),
            GPIOD_BASE..=GPIOD_BASE_END => self.gpio_read32(3, bus_addr - GPIOD_BASE),
            GPIOE_BASE..=GPIOE_BASE_END => self.gpio_read32(4, bus_addr - GPIOE_BASE),
            GPIOF_BASE..=GPIOF_BASE_END => self.gpio_read32(5, bus_addr - GPIOF_BASE),
            GPIOG_BASE..=GPIOG_BASE_END => self.gpio_read32(6, bus_addr - GPIOG_BASE),
            FLASH_R_BASE..=FLASH_R_BASE_END => self.flash_read32(bus_addr - FLASH_R_BASE),
            TIM1_BASE..=TIM1_BASE_END => self.tim_advanced_control_timer_read32(
                AdvancedControlTimerType::TIM1,
                bus_addr - TIM1_BASE,
            ),
            TIM2_BASE..=TIM2_BASE_END => self.tim_general_purpose_timer_read32(
                GeneralPurposeTimerType::TIM2,
                bus_addr - TIM2_BASE,
            ),
            TIM3_BASE..=TIM3_BASE_END => self.tim_general_purpose_timer_read32(
                GeneralPurposeTimerType::TIM3,
                bus_addr - TIM3_BASE,
            ),
            TIM4_BASE..=TIM4_BASE_END => self.tim_general_purpose_timer_read32(
                GeneralPurposeTimerType::TIM4,
                bus_addr - TIM4_BASE,
            ),
            TIM5_BASE..=TIM5_BASE_END => self.tim_general_purpose_timer_read32(
                GeneralPurposeTimerType::TIM5,
                bus_addr - TIM5_BASE,
            ),
            TIM6_BASE..=TIM6_BASE_END => {
                self.tim_basic_timer_read32(BasicTimerType::TIM6, bus_addr - TIM6_BASE)
            }
            TIM7_BASE..=TIM7_BASE_END => {
                self.tim_basic_timer_read32(BasicTimerType::TIM7, bus_addr - TIM7_BASE)
            }
            TIM8_BASE..=TIM8_BASE_END => self.tim_advanced_control_timer_read32(
                AdvancedControlTimerType::TIM8,
                bus_addr - TIM8_BASE,
            ),
            TIM9_BASE..=TIM9_BASE_END => self.tim_general_purpose_timer2_read32(
                GeneralPurposeTimer2Type::TIM9,
                bus_addr - TIM9_BASE,
            ),
            TIM10_BASE..=TIM10_BASE_END => self.tim_general_purpose_timer2_read32(
                GeneralPurposeTimer2Type::TIM10,
                bus_addr - TIM10_BASE,
            ),
            TIM11_BASE..=TIM11_BASE_END => self.tim_general_purpose_timer2_read32(
                GeneralPurposeTimer2Type::TIM11,
                bus_addr - TIM11_BASE,
            ),
            TIM12_BASE..=TIM12_BASE_END => self.tim_general_purpose_timer2_read32(
                GeneralPurposeTimer2Type::TIM12,
                bus_addr - TIM12_BASE,
            ),
            TIM13_BASE..=TIM13_BASE_END => self.tim_general_purpose_timer2_read32(
                GeneralPurposeTimer2Type::TIM13,
                bus_addr - TIM13_BASE,
            ),
            TIM14_BASE..=TIM14_BASE_END => self.tim_general_purpose_timer2_read32(
                GeneralPurposeTimer2Type::TIM14,
                bus_addr - TIM14_BASE,
            ),
            _ => Err(Fault::DAccViol),
        }
    }

    fn write32(&mut self, addr: u32, value: u32) -> Result<(), Fault> {
        println!("write32 0x{:x}=0x{:x}", addr, value);
        match addr {
            AFIO_BASE..=AFIO_BASE_END => self.afio_write32(addr - AFIO_BASE, value),
            RCC_BASE..=RCC_BASE_END => self.rcc_write32(addr - RCC_BASE, value),
            GPIOA_BASE..=GPIOA_BASE_END => self.gpio_write32(0, addr - GPIOA_BASE, value),
            GPIOB_BASE..=GPIOB_BASE_END => self.gpio_write32(1, addr - GPIOB_BASE, value),
            GPIOC_BASE..=GPIOC_BASE_END => self.gpio_write32(2, addr - GPIOC_BASE, value),
            GPIOD_BASE..=GPIOD_BASE_END => self.gpio_write32(3, addr - GPIOD_BASE, value),
            GPIOE_BASE..=GPIOE_BASE_END => self.gpio_write32(4, addr - GPIOE_BASE, value),
            GPIOF_BASE..=GPIOF_BASE_END => self.gpio_write32(5, addr - GPIOF_BASE, value),
            GPIOG_BASE..=GPIOG_BASE_END => self.gpio_write32(6, addr - GPIOG_BASE, value),
            FLASH_R_BASE..=FLASH_R_BASE_END => self.flash_write32(addr - FLASH_R_BASE, value),
            TIM1_BASE..=TIM1_BASE_END => self.tim_advanced_control_timer_write32(
                AdvancedControlTimerType::TIM1,
                addr - TIM1_BASE,
                value,
            ),
            TIM2_BASE..=TIM2_BASE_END => self.tim_general_purpose_timer_write32(
                GeneralPurposeTimerType::TIM2,
                addr - TIM2_BASE,
                value,
            ),
            TIM3_BASE..=TIM3_BASE_END => self.tim_general_purpose_timer_write32(
                GeneralPurposeTimerType::TIM3,
                addr - TIM3_BASE,
                value,
            ),
            TIM4_BASE..=TIM4_BASE_END => self.tim_general_purpose_timer_write32(
                GeneralPurposeTimerType::TIM4,
                addr - TIM4_BASE,
                value,
            ),
            TIM5_BASE..=TIM5_BASE_END => self.tim_general_purpose_timer_write32(
                GeneralPurposeTimerType::TIM5,
                addr - TIM5_BASE,
                value,
            ),
            TIM6_BASE..=TIM6_BASE_END => {
                self.tim_basic_timer_write32(BasicTimerType::TIM6, addr - TIM6_BASE, value)
            }
            TIM7_BASE..=TIM7_BASE_END => {
                self.tim_basic_timer_write32(BasicTimerType::TIM7, addr - TIM7_BASE, value)
            }
            TIM8_BASE..=TIM8_BASE_END => self.tim_advanced_control_timer_write32(
                AdvancedControlTimerType::TIM8,
                addr - TIM8_BASE,
                value,
            ),
            TIM9_BASE..=TIM9_BASE_END => self.tim_general_purpose_timer2_write32(
                GeneralPurposeTimer2Type::TIM9,
                addr - TIM9_BASE,
                value,
            ),
            TIM10_BASE..=TIM10_BASE_END => self.tim_general_purpose_timer2_write32(
                GeneralPurposeTimer2Type::TIM10,
                addr - TIM10_BASE,
                value,
            ),
            TIM11_BASE..=TIM11_BASE_END => self.tim_general_purpose_timer2_write32(
                GeneralPurposeTimer2Type::TIM11,
                addr - TIM11_BASE,
                value,
            ),
            TIM12_BASE..=TIM12_BASE_END => self.tim_general_purpose_timer2_write32(
                GeneralPurposeTimer2Type::TIM12,
                addr - TIM12_BASE,
                value,
            ),
            TIM13_BASE..=TIM13_BASE_END => self.tim_general_purpose_timer2_write32(
                GeneralPurposeTimer2Type::TIM13,
                addr - TIM13_BASE,
                value,
            ),
            TIM14_BASE..=TIM14_BASE_END => self.tim_general_purpose_timer2_write32(
                GeneralPurposeTimer2Type::TIM14,
                addr - TIM14_BASE,
                value,
            ),
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
