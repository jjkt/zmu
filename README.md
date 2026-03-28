# zmu - Emulator for Microcontroller Systems

zmu is a system-level emulator for microcontrollers, aimed at fast simulation of cores and peripherals. It currently targets ARM Cortex-M MCUs.

zmu supports Linux and Windows operating systems.

## Supported features
- Loading of ELF binaries
- Automatic flash sizing and address remapping from ELF `PT_LOAD` segments
- Relatively efficient simulation
  - Performance depends on the host and workload
  - Run with `-vv` to have zmu print measured throughput as `cycles_per_sec ~ X.XX Mhz`
  - Repeatable benchmark coverage exists via CoreMark in [test_coremark.sh](test_coremark.sh)
- Architectures:
  - arm-v6m
  - arm-v7m (partial support)
  - arm-v7em (partial support, including FP-enabled profiles)
- Core profiles exercised by the build and test scripts: Cortex-M0/M0+, Cortex-M3, Cortex-M4, Cortex-M4F, Cortex-M7
  - Pre-decoding of instructions for efficient simulation
  - Exception and fault handling, including configurable fault trapping
  - Processor sleep
- ARM semihosting support for console and feature-probe use cases:
  - open, close (`:tt` streams and `:semihosting-features`)
  - FLEN
  - ISTTY
  - writec, write, read
  - seek, clock
  - exception -> exit
  - exit extended
  - errno
- Floating-point instruction/register support is implemented and exercised by `cm4f`, `cm7-d16`, and `cm7-sp-d16` test targets
- ITM
  - (TPIU) write stimulus register data to a file, in framed format
  - STIM0 .. STIM31 supported
- DWT
  - Cycle counter
- Core peripherals: NVIC, SCB, SysTick
- Device models: generic Cortex-M system and STM32F103, selected in the binary layer rather than inside `zmu_cortex_m`
- Instruction trace
- GDB Server
  - continue / run control
  - single stepping
  - range stepping
  - software breakpoints
  - monitor `reset`

## Missing / Planned features
The detailed backlog and missing architecture, floating-point, peripheral, platform, and tooling work lives in [doc/todo.md](doc/todo.md).


## Dependencies

You have to install Rust.

```sh
curl https://sh.rustup.rs -sSf | sh
```

Follow the install menu, then run the following command in the terminal used for compilation

```sh
source ~/.cargo/env
```


## How to Compile

```sh
cargo build --release
```

By default, this builds the `zmu-cortex-m4f` product binary.

To build the full product matrix:

```sh
chmod +x buildall.sh
./buildall.sh
```
The product binaries are first-class Cargo bin targets and are generated in `./target/release/`.

Examples:

```sh
cargo build --release --no-default-features --features cortex-m0 --bin zmu-cortex-m0
cargo build --release --no-default-features --features cortex-m4f --bin zmu-cortex-m4f
cargo build --release --no-default-features --features stm32f103 --bin zmu-stm32f103
```

## Testing

Install support for compiling for cortex-M targets: (needed for rust based tests)

```sh
rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

You need ARM compiler to compile some of the examples. Get the latest compilers from [ARM website](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads). Some Linux distros (like Ubuntu) have prebuilt packages for this.
```sh
'sudo apt install gcc-arm-none-eabi'

```

For Arch Linux, use arm-none-eabi-*. Arch Linux does not package the CSTARTUP files used by the GCC based tests, so you need to manually download them from the arm developer site anyways.

For reproducible local installs (and for testing multiple versions), use the helper script:

```sh
bash scripts/install_arm_gcc.sh --version 15.2.rel1
bash scripts/install_arm_gcc.sh --version 14.2.rel1
```

You can also choose the install location:

```sh
bash scripts/install_arm_gcc.sh --version 15.2.rel1 --install-dir "$HOME/.local/gcc-arm/15.2.rel1"
```


Set the environmental variable GCC_HOME to point to the home of arm-none-eabi-gcc. The default location is /usr

```sh
export GCC_HOME=/usr
chmod +x testall.sh
./testall.sh
```

To switch GCC versions quickly, point GCC_HOME to the version you want to test:

```sh
export GCC_HOME="$HOME/.local/arm-gnu-toolchain/15.2.rel1"
./test_gcc.sh

export GCC_HOME="$HOME/.local/arm-gnu-toolchain/14.2.rel1"
./test_gcc.sh
```

## Usage

- Product-oriented binaries are emitted as `zmu-cortex-m0`, `zmu-cortex-m0plus`, `zmu-cortex-m3`, `zmu-cortex-m4`, `zmu-cortex-m4f`, `zmu-cortex-m7-d16`, `zmu-cortex-m7-sp-d16`, and `zmu-stm32f103`.

### Run an ELF binary
```sh
./target/release/zmu-cortex-m0 run tests/hello_world/hello_world-cm0.elf
hello, world
```

### Run with tracing
```sh
./target/release/zmu-cortex-m3 run -t tests/minimal/minimal-cm3.elf | head -3
4906      ldr r1, [pc, #+24]               00000074  Reset_Handler         2 qvczn r0:00000000 1:00001c84 2:00000000 3:00000000 4:00000000 5:00000000 6:00000000 7:00000000 8:00000000 9:00000000 10:00000000 11:00000000 12:00000000
4A07      ldr r2, [pc, #+28]               00000076  Reset_Handler         4 qvczn r0:00000000 1:00001c84 2:20000000 3:00000000 4:00000000 5:00000000 6:00000000 7:00000000 8:00000000 9:00000000 10:00000000 11:00000000 12:00000000
4B07      ldr r3, [pc, #+28]               00000078  Reset_Handler         6 qvczn r0:00000000 1:00001c84 2:20000000 3:20000854 4:00000000 5:00000000 6:00000000 7:00000000 8:00000000 9:00000000 10:00000000 11:00000000 12:00000000
```

### Fault behavior and trap control

By default, `HardFault` traps stop execution, while configurable faults such as `UsageFault` and `MemoryManagementFault` run their handlers unless you explicitly trap them.

This ARMv7-M test binary executes an undefined instruction and reaches the installed `UsageFault_Handler` when trapping is not enabled:

```sh
./target/release/zmu-cortex-m3 run tests/fault-test-bench/fault-usage-cm3.elf
UsageFault_Handler marker=0x55534654
```

To stop immediately when the fault is raised instead, enable trapping for all architecturally visible faults or select individual fault classes with `--trap` and `--no-trap`:

```sh
./target/release/zmu-cortex-m3 run --fault-trap tests/fault-test-bench/fault-usage-cm3.elf
fault trap: fault=UndefInstr, exception=UsageFault, ...
```

Useful combinations:

```sh
./target/release/zmu-cortex-m3 run --trap usagefault tests/fault-test-bench/fault-usage-cm3.elf
./target/release/zmu-cortex-m3 run --trap memmanage tests/fault-test-bench/fault-memmanage-cm3.elf
./target/release/zmu-cortex-m0 run --no-trap all tests/fault-test-bench/fault-lockup-cm0.elf
```

On ARMv6-M builds, only `HardFault` is architecturally visible, so `--trap` and `--no-trap` accept `hardfault` and `all` only.

### Run with `--itm`

Install the decoder once:

```sh
cargo install itm
```

Then pipe zmu's ITM stream to `itmdump`:

```sh
./target/release/zmu-cortex-m3 run --itm /dev/stdout tests/hello_world_itm/hello_world_itm-cm3.elf | ~/.cargo/bin/itmdump
Hello, world!
```

### "Hello, world" example with Arm GCC + semihosting

```c
#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    printf("hello, world\n");
}

void SystemInit(void) { }

extern void initialise_monitor_handles(void);

void _start(void)
{
    initialise_monitor_handles();
    main();
    exit(0);
}


__attribute__((used))
void _fini(void) { }
```

Compile the code with GCC:
```sh
arm-none-eabi-gcc -O2 --specs=rdimon.specs -mthumb -g -nostartfiles -T link.ld   -mcpu=cortex-m0 -lc -lrdimon main.c /usr/share/gcc-arm-embedded/samples/startup/startup_ARMCM0.S -o hello_world-cm0.elf
```

Run the emulator:
```sh
./target/release/zmu-cortex-m0 run tests/hello_world/hello_world-cm0.elf
hello, world
```

Run the GDB Server:
```sh
./target/release/zmu-cortex-m0 run --gdb tests/hello_world/hello_world-cm0.elf
Starting GDB Server on port 9001 ...
```

On a separate terminal start the gdb client:
```text
$ gdb-multiarch tests/hello_world/hello_world-cm0.elf
...
Reading symbols from tests/hello_world/hello_world-cm0.elf...
(gdb) target remote localhost:9001
Remote debugging using localhost:9001
Reset_Handler ()
    at /usr/share/doc/gcc-arm-none-eabi/examples/startup/startup_ARMCM0.S:150
150             ldr     r1, =__etext
(gdb) b main
Breakpoint 1 at 0x5c: file main.c, line 6.
(gdb) c
Continuing.

Breakpoint 1, main () at main.c:6
6           printf("hello, world\n");
(gdb)
```
