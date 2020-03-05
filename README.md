# zmu - Emulator for Microcontroller Systems

zmu is an system level emulator for microcontrollers, aiming for high speed simulation of core and peripherals. Currently targets ARM Cortex MCUs.

zmu supports Linux and Windows operating systems.

## Supported features
- Loading of ELF binaries
- Relatively efficient Simulation
    - Intel Core i7-2630QM @ 2.8 Ghz can simulate 40-50 Mhz Cortex-m4 in realtime
- Architectures:
    - arm-v6m,
    - arm-v7m (partial support)
    - arm-v7me (partial support)
- Cores (in progress): Cortex-m0/m0+, Cortex-m3, Cortex-m4
    - Pre-decoding of instructions for efficient simulation
    - Exception and fault handling
    - Processor sleep
- ARM semihosting, supported semihosting extensions:
    - open, close (streams only)
    - FLEN
    - ISTTY
    - write, read
    - seek, clock, exception -> exit
    - errno
- ITM
    - (TPIU) write stimulus register data to a file, in framed format
    - STIM0 .. STIM31 supported
- DWT
    - Cycle counter
- Instruction trace

## Missing / Planned features
- Time simulation / sync to real time
- Some instructions are not yet properly supported
    - ~20 instructions missing: BFC, CDP, CLREX, LDMDB, ...
    - Full v7m + DSP exensions support
    - Full v7me + floats (m4f)
- ARM Cortex peripherals
    - NVIC (partial support available)
    - MPU
- Semihosting: filesystem access
- System Simulation:
    - device profiles, eg stm32 device support
    - board profiles, external peripheral simulation

## Depedencies

You have to install RUST. Assuming Ubuntu 18.04.

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

You need ARM compiler to compile some of the examples. Get the latest compilers from [ARM website](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)

Set the environmental variable GCC_HOME to point to the home of arm-none-eabi-gcc. The default location is /usr

```sh
export GCC_HOME=/usr
chmod +x testall.sh
./testall.sh
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


### "RTFM" examples with rust
Zmu can already run many of the [cortex-m-rtfm](https://github.com/japaric/cortex-m-rtfm) examples directly.

Here are few example runs:

message.rs
```
$./target/release/zmu-armv7m run ./tests/cortex-m-rtfm/target/thumbv7m-none-eabi/debug/examples/message
foo
bar(0)
baz(1, 2)
foo
bar(1)
baz(2, 3)
^C
```

resource.rs
```
$./target/release/zmu-armv7m run ./tests/cortex-m-rtfm/target/thumbv7m-none-eabi/debug/examples/resource
UART0: SHARED = 1
UART1: SHARED = 2
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
