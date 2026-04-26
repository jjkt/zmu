#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#if defined(__ARM_PCS_VFP) && defined(__VFP_FP__)
#define HAVE_ARM_VFP 1
#else
#define HAVE_ARM_VFP 0
#endif

void SystemInit(void)
{
#if HAVE_ARM_VFP
#define SCB_CPACR (*(volatile uint32_t *)0xE000ED88u)
    // enable FPU
    SCB_CPACR |= (0xFu << 20);
#endif
}

extern void initialise_monitor_handles(void);
extern int main(void);

void _start(void)
{
    initialise_monitor_handles();
    main();
    exit(0);
}

__attribute__((used)) void _fini(void) {}

void hard_fault_handler_c(unsigned int *args)
{
    printf("hardfault!\n");
    exit(0);
}

void bus_fault_handler_c(unsigned int *args)
{
    printf("busfault!\n");
    exit(0);
}
