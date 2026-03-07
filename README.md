# zmu - Emulator for Microcontroller Systems

zmu is an system level emulator for microcontrollers, aiming for high speed simulation of core and peripherals. Currently targets ARM Cortex MCUs.

zmu supports Linux and Windows operating systems.

## Supported features
- Loading of ELF binaries
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
    - Exception and fault handling
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
- Device models: generic Cortex-M system and STM32F103
- Instruction trace
- GDB Server
    - continue / run control
    - single stepping
    - range stepping
    - software breakpoints
    - monitor `reset`

## Missing / Planned features
The detailed backlog and missing architecture, floating-point, peripheral, platform, and tooling work lives in [doc/todo.md](doc/todo.md).


## Depedencies

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
chmod +x buildall.sh
./buildall.sh
```
The executables are genereated in the dir ```./target/release/```.

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

- ```zmu-armv6m``` runs the zmu with support for armv6m instructions.
- ```zmu-armv7m``` runs the zmu with support for armv7m instructions.

### Run an ELF binary
```
$./target/release/zmu-armv6m run tests/hello_world/hello_world-cm0.elf
hello, world
```

### Run with tracing
```
$./target/release/zmu-armv7m run -t tests/minimal/minimal-cm3.elf | head -3
4906      ldr r1, [pc, #+24]               00000074  Reset_Handler         2 qvczn r0:00000000 1:00001c84 2:00000000 3:00000000 4:00000000 5:00000000 6:00000000 7:00000000 8:00000000 9:00000000 10:00000000 11:00000000 12:00000000
4A07      ldr r2, [pc, #+28]               00000076  Reset_Handler         4 qvczn r0:00000000 1:00001c84 2:20000000 3:00000000 4:00000000 5:00000000 6:00000000 7:00000000 8:00000000 9:00000000 10:00000000 11:00000000 12:00000000
4B07      ldr r3, [pc, #+28]               00000078  Reset_Handler         6 qvczn r0:00000000 1:00001c84 2:20000000 3:20000854 4:00000000 5:00000000 6:00000000 7:00000000 8:00000000 9:00000000 10:00000000 11:00000000 12:00000000
```

### Run with ITM trace via itmdump

Following example uses the [itmdump](https://docs.rs/itm/0.3.1/itm/) tool and embedded rustbook examples to show how to dump itm trace prints to stdout from the zmu. To install itmdump, you need to run ```cargo install itmdump```.

```
$./target/release/zmu-armv7m run --itm /dev/stdout tests/rustbook/target/thumbv7m-none-eabi/debug/examples/itm | itmdump
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
```
arm-none-eabi-gcc -O2 --specs=rdimon.specs -mthumb -g -nostartfiles -T link.ld   -mcpu=cortex-m0 -lc -lrdimon main.c /usr/share/gcc-arm-embedded/samples/startup/startup_ARMCM0.S -o hello_world-cm0.elf
```

Run the emulator:
```
$zmu run tests/hello_world/hello_world-cm0.elf
hello, world
```

Run the GDB Server:
```
$zmu run --gdb tests/hello_world/hello_world-cm0.elf
Starting GDB Server on port 9001 ...
```

On a separate terminal start the gdb client:
```
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
