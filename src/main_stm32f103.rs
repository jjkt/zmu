mod app;
mod device;
mod semihost;
mod trace;

use zmu_cortex_m::DeviceBus;

fn create_device() -> Option<DeviceBus> {
    Some(Box::new(device::stm32f1xx::Device::new()))
}

fn main() {
    app::main_with_device(
        "zmu-stm32f103",
        "STM32F103 emulator",
        "Load and run <EXECUTABLE> on an STM32F103 target",
        create_device,
    );
}
