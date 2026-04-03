mod app;
mod semihost;
mod trace;

use zmu_cortex_m::DeviceBus;

fn create_device() -> Option<DeviceBus> {
    None
}

fn main() {
    app::main_with_device(
        "zmu-cortex-m0plus",
        "Cortex-M0+ emulator",
        "Load and run <EXECUTABLE> on a Cortex-M0+ target",
        create_device,
    );
}
