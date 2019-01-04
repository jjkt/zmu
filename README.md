# zmu - emulator for microcontrollers

zmu is an instruction level emulator for microcontrollers, aiming for high speed simulation of core and peripherals. Currently targets ARM Cortex MCUs.

zmu supports linux and windows operating systems.

## Supported features
- Architectures: arm-v6m, arm-v7m (in progress), arm-v7me (in progress)
    - notably missing: floating point support and most of the DSP extensions
- Cores (in progress): Cortex-m0/m0+, Cortex-m3, Cortex-m4, Cortex-M4f
    - notably missing: full exception and interrupt support
- ARM semihosting, semihosting extensions:
    - open, close (streams only)
    - FLEN 
    - ISTTY
    - write, read
    - seek, clock, exception -> exit
    - errno



## "Hello, world" example with semihosting

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
