#include <stdio.h>
// By Dik T. Winter at CWI
// rewritten in https://crypto.stanford.edu/pbc/notes/pi/code.html
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#if defined(__ARM_PCS_VFP) && defined(__VFP_FP__)
#define HAVE_ARM_VFP 1
#else
#define HAVE_ARM_VFP 0
#endif

int main()
{
    int r[2800 + 1];
    int i, k;
    int b, d;
    int c = 0;

    for (i = 0; i < 2801; i++)
    {
        r[i] = 2000;
    }

    for (k = 2800; k > 0; k -= 14)
    {
        d = 0;

        i = k;
        for (;;)
        {
            d += r[i] * 10000;
            b = 2 * i - 1;

            r[i] = d % b;
            d /= b;
            i--;
            if (i == 0)
                break;
            d *= i;
        }
        printf("%.4d", c + d / 10000);
        c = d % 10000;
    }

    return 0;
}

void SystemInit(void)
{

#if HAVE_ARM_VFP
#define SCB_CPACR (*(volatile uint32_t *)0xE000ED88u)
    // enable FPU
    SCB_CPACR |= (0xFu << 20);
    asm volatile("dsb 0xF" ::: "memory");
    asm volatile("isb 0xF" ::: "memory");
#endif
}

extern void initialise_monitor_handles(void);

void _start(void)
{
    initialise_monitor_handles();
    main();
    exit(0);
}

__attribute__((used)) void _fini(void) {}