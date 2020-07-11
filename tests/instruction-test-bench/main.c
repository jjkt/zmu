#include <stdio.h>
#include <stdlib.h>

#if __ARM_ARCH >= 7
unsigned int bfc_0_32(int value)
{
    asm volatile (
        "bfc     %[value], 0, 32"
        : [value] "+r" (value));
    return value;
}
#endif
//  [lsb] "I" (lsb), [width] "I" (width)
int main(void)
{
    
#if __ARM_ARCH >= 7
    printf("bfc(0xffffffff, 0, 32) = 0x%08x\n", bfc_0_32(0xffffffff));
#endif    
}

void SystemInit(void)
{
}

extern void initialise_monitor_handles(void);

void _start(void)
{
    initialise_monitor_handles();
    main();
    exit(0);
}

__attribute__((used))
void _fini(void) { }