//!
//! A Trait for representing a Cortex armv6-m exceptions.
//!
//!

pub enum Exception {
    Reset,
    NMI,
    HardFault,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    SVCall,
    Reserved8,
    Reserved9,
    PendSV,
    SysTick,
    Interrupt { n: u32 },
}

impl From<Exception> for u32 {
    fn from(value: Exception) -> Self {
        match value {
            Exception::Reset => 1,
            Exception::NMI => 2,
            Exception::HardFault => 3,
            Exception::Reserved1 => 4,
            Exception::Reserved2 => 5,
            Exception::Reserved3 => 6,
            Exception::Reserved4 => 7,
            Exception::Reserved5 => 8,
            Exception::Reserved6 => 9,
            Exception::Reserved7 => 10,
            Exception::SVCall => 11,
            Exception::Reserved8 => 12,
            Exception::Reserved9 => 13,
            Exception::PendSV => 14,
            Exception::SysTick => 15,
            Exception::Interrupt { n } => 16 + n,
        }
    }
}
