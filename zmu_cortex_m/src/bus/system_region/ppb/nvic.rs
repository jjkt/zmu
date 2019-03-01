use crate::bus::system_region::ppb::PrivatePeripheralBus;
use crate::bus::BusStepResult;
use crate::core::exception::Exception;

use crate::bus::system_region::ppb::ExceptionState;

pub trait NVIC {
    fn write_iser(&mut self, index: u8, value: u32);
    fn read_iser(&mut self, index: u8) -> u32;
    fn write_icer(&mut self, index: u8, value: u32);
    fn read_icer(&mut self, index: u8) -> u32;
    fn write_ispr(&mut self, index: u8, value: u32);
    fn read_ispr(&mut self, index: u8) -> u32;
    fn write_icpr(&mut self, index: u8, value: u32);
    fn read_icpr(&mut self, index: u8) -> u32;
    fn read_iapr(&mut self, index: u8) -> u32;
    fn write_iapr(&mut self, index: u8, value: u32);
    fn write_ipr(&mut self, index: u8, value: u32);
    fn read_ipr(&mut self, index: u8) -> u32;
    fn write_ipr_u8(&mut self, index: u8, value: u8);
    fn nvic_set_pend(&mut self, exception: Exception);
    fn nvic_step(&mut self) -> BusStepResult;
}

impl NVIC for PrivatePeripheralBus {
    fn write_iser(&mut self, _index: u8, _value: u32) {
        //println!("write iser index {} => 0x{:x}", index, value);
    }

    fn read_iser(&mut self, _index: u8) -> u32 {
        //TODO
        0
    }

    fn write_icer(&mut self, _index: u8, _value: u32) {
        //println!("write iser index {} => 0x{:x}", index, value);
    }

    fn read_icer(&mut self, _index: u8) -> u32 {
        //TODO
        0
    }

    fn write_ispr(&mut self, index: u8, value: u32) {
        //println!("write ispr index {} => 0x{:x}", index, value);
        // set interrupt (index*32) + value (bits) to pending
        let mut mask = value;
        let mut cnt = ((index * 32) as usize) + 16;
        while mask > 0 {
            if (mask & 1) == 1 {
                match self.exception_state[cnt] {
                    ExceptionState::Inactive => {
                        self.exception_state[cnt] = ExceptionState::Pending;
                        self.pending_count += 1;
                    }
                    ExceptionState::Active => {
                        self.exception_state[cnt] = ExceptionState::ActivePending
                    }
                    _ => (),
                }

                //println!("set irq {} pending", cnt);
            }
            mask >>= 1;
            cnt += 1;
        }
    }

    fn read_ispr(&mut self, _index: u8) -> u32 {
        //TODO
        0
    }

    fn write_icpr(&mut self, _index: u8, _value: u32) {}

    fn read_icpr(&mut self, _index: u8) -> u32 {
        //TODO
        0
    }

    fn read_iapr(&mut self, _index: u8) -> u32 {
        //TODO
        0
    }

    fn write_iapr(&mut self, _index: u8, _value: u32) {
        //TODO
    }

    fn write_ipr(&mut self, _index: u8, _value: u32) {
        //println!("write ipr index {} => 0x{:x}", index, value);
    }

    fn read_ipr(&mut self, _index: u8) -> u32 {
        //TODO
        0
    }

    fn write_ipr_u8(&mut self, _index: u8, _value: u8) {
        //println!("write ipr index (u8) {} => 0x{:x} ", index, value);
    }

    fn nvic_set_pend(&mut self, exception: Exception) {
        let exception_number: u8 = exception.into();

        let index = exception_number as usize;

        match self.exception_state[index] {
            ExceptionState::Inactive => {
                self.exception_state[index] = ExceptionState::Pending;
                self.pending_count += 1;
            }
            ExceptionState::Active => self.exception_state[index] = ExceptionState::ActivePending,
            _ => (),
        }
    }

    fn nvic_step(&mut self) -> BusStepResult {
        if self.pending_count > 0 {
            // TODO: should select the interrupt by priority scheme:
            // first fixed priority interrupts in order,
            // then ones with configurable priority

            for (irq, state) in self.exception_state.iter_mut().enumerate() {
                if *state == ExceptionState::Pending {
                    *state = ExceptionState::Active;
                    self.pending_count -= 1;
                    return BusStepResult::Exception {
                        exception: (irq as u8).into(),
                    };
                }
            }
        }
        BusStepResult::Nothing
    }
}
