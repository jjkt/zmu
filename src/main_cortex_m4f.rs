mod app;
mod semihost;
mod trace;

use zmu_cortex_m::DeviceBus;

fn create_device() -> Option<DeviceBus> {
    None
}

fn main() {
    app::main_with_device(
        "zmu-cortex-m4f",
        "Cortex-M4F emulator",
        "Load and run <EXECUTABLE> on a Cortex-M4F target",
        create_device,
    );
}
