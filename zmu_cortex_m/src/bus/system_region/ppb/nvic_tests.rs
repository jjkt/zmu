use crate::bus::system_region::ppb::nvic::NVIC;
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

/*#[test]
fn test_nvic_set_pend() {

    ppb.nvic_set_pend(Exception::MemoryManagementFault);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::BusFault);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::UsageFault);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::Reserved4);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::Reserved5);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::Reserved6);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::DebugMonitor);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::SVCall);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::Reserved8);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::Reserved9);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::PendSV);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::SysTick);
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);

    ppb.nvic_set_pend(Exception::Interrupt { n: 0 });
    // TODO: cannot be masked away?
    assert_eq!(ppb.nvic_step(), BusStepResult::Nothing);
}
*/
