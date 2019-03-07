/*use crate::bus::system_region::ppb::nvic::NVIC;
use crate::bus::system_region::ppb::PrivatePeripheralBus;
use crate::bus::BusStepResult;
use crate::core::exception::Exception;
use std::io;
use std::io::Result;
use std::io::Write;

struct TestWriter {}

impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test_nvic_new() {
    let mut ppb = PrivatePeripheralBus::new(Some(Box::new(TestWriter {})));
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);
}

#[test]
fn test_nvic_set_pend_reset() {
    let mut ppb = PrivatePeripheralBus::new(Some(Box::new(TestWriter {})));;
    ppb.nvic_set_pend(Exception::Reset);
    assert_eq!(
        ppb.nvic_step(),
        BusStepResult::Exception {
            exception: Exception::Reset
        }
    );
}

#[test]
fn test_nvic_set_pend_nmi() {
    let mut ppb = PrivatePeripheralBus::new(Some(Box::new(TestWriter {})));;
    ppb.nvic_set_pend(Exception::NMI);
    assert_eq!(
        ppb.nvic_step(),
        BusStepResult::Exception {
            exception: Exception::NMI
        }
    );
}

#[test]
fn test_nvic_set_pend_hardfault() {
    let mut ppb = PrivatePeripheralBus::new(Some(Box::new(TestWriter {})));;
    ppb.nvic_set_pend(Exception::HardFault);
    assert_eq!(
        ppb.nvic_step(),
        BusStepResult::Exception {
            exception: Exception::HardFault
        }
    );
}

#[test]
fn test_nvic_set_pend_reset_hf_nmi_priority() {
    let mut ppb = PrivatePeripheralBus::new(Some(Box::new(TestWriter {})));;
    ppb.nvic_set_pend(Exception::Reset);
    ppb.nvic_set_pend(Exception::NMI);
    ppb.nvic_set_pend(Exception::HardFault);
    assert_eq!(
        ppb.nvic_step(),
        BusStepResult::Exception {
            exception: Exception::Reset
        }
    );
}

#[test]
fn test_nvic_set_pend_nmi_hf_priority() {
    let mut ppb = PrivatePeripheralBus::new(Some(Box::new(TestWriter {})));;
    ppb.nvic_set_pend(Exception::NMI);
    ppb.nvic_set_pend(Exception::HardFault);
    assert_eq!(
        ppb.nvic_step(),
        BusStepResult::Exception {
            exception: Exception::NMI
        }
    );
}

#[test]
fn test_nvic_set_pend_reset_hr_priority() {
    let mut ppb = PrivatePeripheralBus::new(Some(Box::new(TestWriter {})));;
    ppb.nvic_set_pend(Exception::Reset);
    ppb.nvic_set_pend(Exception::HardFault);
    assert_eq!(
        ppb.nvic_step(),
        BusStepResult::Exception {
            exception: Exception::Reset
        }
    );
}

#[test]
fn test_nvic_set_pend_do_not_activate_again() {
    let mut ppb = PrivatePeripheralBus::new(Some(Box::new(TestWriter {})));;
    ppb.nvic_set_pend(Exception::Reset);
    ppb.nvic_step();

    // should not trigger again before irq is handled
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);
}

*/