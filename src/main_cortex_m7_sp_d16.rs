mod app;
mod semihost;
mod trace;

use zmu_cortex_m::DeviceBus;

fn create_device() -> Option<DeviceBus> {
    None
}

fn main() {
    app::main_with_device(
        "zmu-cortex-m7-sp-d16",
        "Cortex-M7 SP-D16 emulator",
        "Load and run <EXECUTABLE> on a Cortex-M7 SP-D16 target",
        create_device,
    );
}
