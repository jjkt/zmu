#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

/*
This test bench is used to test various ARM Cortex-M instructions.
If you want to test exact instruction, use inline assembly.
If you want to test a general concept, use C code. The latter
has downside of unpredictable compiler code generation.
*/

/*

Potentially useful defines lookup:

arm-none-eabi-gcc ... -mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-sp-d16

#define __VFP_FP__ 1
#define __ARM_PCS_VFP 1
#define __ARM_ARCH_PROFILE 77   
#define __ARM_ARCH_7EM__ 1
#define __ARM_FEATURE_DSP 1


*/

#if defined(__ARM_PCS_VFP) && defined(__VFP_FP__)
    // Hard floating-point is enabled, and VFP instructions are available
    #define HARD_FLOATING_POINT_ENABLED 1
#else
    // Hard floating-point is not enabled or VFP support is absent
    #define HARD_FLOATING_POINT_ENABLED 0
#endif


#if __ARM_ARCH >= 7
unsigned int bfc_0_32(int value)
{
    asm volatile(
        "bfc     %[value], 0, 32"
        : [value] "+r"(value));
    return value;
}
unsigned int bfc_0_16(int value)
{
    asm volatile(
        "bfc     %[value], 0, 16"
        : [value] "+r"(value));
    return value;
}
unsigned int bfc_15_16(int value)
{
    asm volatile(
        "bfc     %[value], 15, 16"
        : [value] "+r"(value));
    return value;
}

void bfc(void)
{
    assert(bfc_0_32(0xffffffff) == 0xffffffff);
    printf("bfc(0xffffffff, 0, 32) = 0x%08x\n", bfc_0_32(0xffffffff));
    printf("bfc(0xffffffff, 0, 16) = 0x%08x\n", bfc_0_16(0xffffffff));
    printf("bfc(0xffffffff, 15, 16) = 0x%08x\n", bfc_15_16(0xffffffff));
}

#endif

#if HARD_FLOATING_POINT_ENABLED
float vabs(float value)
{
    float result;

    asm volatile(
        "VABS.F32 %0, %1"
        : "=t"(result)
        : "t"(value));

    return result;
}

void floating_point(void)
{
    // Try to generate floating-point data-processing instructions
    // VABS, VADD, VCMP, VCVT, VDIV, VFMA, VFNMA, VMAXNM
    // VMLA, VMOV, VMOV, VMUL, VNEG, VNMLA, VRINTA, VRINTZ
    // VSEL, VSQRT, VSUB

    assert(vabs(-1.0f) == 1.0f);
    assert(vabs(-42.0f) == 42.0f);
    assert(vabs(0.0f) == 0.0f);
    assert(vabs(1.0f) == 1.0f);
}
#endif
int main(void)
{

#if __ARM_ARCH >= 7
    bfc();
#endif

#if HARD_FLOATING_POINT_ENABLED
    floating_point();
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

__attribute__((used)) void _fini(void) {}


void __assert_func( const char *filename, int line, const char *assert_func, const char *expr )
{
    printf("assert_failed: %s:%d\n", filename, line);
    exit(1);
}