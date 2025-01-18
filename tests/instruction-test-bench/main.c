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

#if __ARM_FP & 0x8
#define HARD_FLOATING_POINT_DOUBLE_PRECISION 0 // TODO change to 1 once 64 bit ops are tested
#else
#define HARD_FLOATING_POINT_DOUBLE_PRECISION 0
#endif

#else
// Hard floating-point is not enabled or VFP support is absent
#define HARD_FLOATING_POINT_ENABLED 0
#define HARD_FLOATING_POINT_DOUBLE_PRECISION 0
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
float vabs_f32(float value)
{
    float result;

    asm volatile(
        "VABS.F32 %0, %1"
        : "=t"(result)
        : "t"(value));

    return result;
}

#if HARD_FLOATING_POINT_DOUBLE_PRECISION
double vabs_f64(double value)
{
    double result;

    asm volatile(
        "VABS.F64 %P0, %P1"
        : "=w"(result)
        : "w"(value));

    return result;
}
#endif

float vadd_f32(float a, float b)
{
    float result;

    asm volatile(
        "VADD.F32 %0, %1, %2"
        : "=t"(result)
        : "t"(a), "t"(b));

    return result;
}

#if HARD_FLOATING_POINT_DOUBLE_PRECISION
double vadd_f64(double a, double b)
{
    double result;

    asm volatile(
        "VADD.F64 %P0, %P1, %P2"
        : "=w"(result)
        : "w"(a), "w"(b));

    return result;
}
#endif

float vsub_f32(float a, float b)
{
    float result;

    asm volatile(
        "VSUB.F32 %0, %1, %2"
        : "=t"(result)
        : "t"(a), "t"(b));

    return result;
}

#if HARD_FLOATING_POINT_DOUBLE_PRECISION
double vsub_f64(double a, double b)
{
    double result;

    asm volatile(
        "VSUB.F64 %P0, %P1, %P2"
        : "=w"(result)
        : "w"(a), "w"(b));

    return result;
}
#endif

void floating_point(void)
{
    // Try to generate floating-point data-processing instructions
    // TODO: VCVT, VDIV, VFMA, VFNMA, VMAXNM
    // VMLA, VMOV, VMOV, VMUL, VNEG, VNMLA, VRINTA, VRINTZ
    // VSEL, VSQRT

    // VABS.F32, VABS.F64
    assert(vabs_f32(-1.0f) == 1.0f);
    assert(vabs_f32(-42.0f) == 42.0f);
    assert(vabs_f32(0.0f) == 0.0f);
    assert(vabs_f32(1.0f) == 1.0f);

#if HARD_FLOATING_POINT_DOUBLE_PRECISION
    assert(vabs_f64(-1.0) == 1.0);
    assert(vabs_f64(-42.0) == 42.0);
    assert(vabs_f64(0.0) == 0.0);
    assert(vabs_f64(1.0) == 1.0);
#endif

    // VADD.F32 VADD.F64, VSUB.F32, VSUB.F64
    assert(vadd_f32(1.0f, 2.0f) == 3.0f);
    assert(vadd_f32(-1.0f, 2.0f) == 1.0f);
    assert(vadd_f32(-1.0f, -2.0f) == -3.0f);

#if HARD_FLOATING_POINT_DOUBLE_PRECISION
    assert(vadd_f64(1.0, 2.0) == (1.0 + 2.0));
    assert(vadd_f64(-1.0, 2.0) == (-1.0 + 2.0));
    assert(vadd_f64(-1.0, -2.0) == (-1.0 + -2.0));
#endif

    assert(vsub_f32(1.0f, 2.0f) == (1.0f - 2.0f));
    assert(vsub_f32(-1.0f, 2.0f) == (-1.0f - 2.0f));
    assert(vsub_f32(-1.0f, -2.0f) == (-1.0f - -2.0f));

#if HARD_FLOATING_POINT_DOUBLE_PRECISION
    assert(vsub_f64(1.0, 2.0) == (1.0 - 2.0));
    assert(vsub_f64(-1.0, 2.0) == (-1.0 - 2.0));
    assert(vsub_f64(-1.0, -2.0) == (-1.0 - -2.0));
#endif
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

void __assert_func(const char *filename, int line, const char *assert_func, const char *expr)
{
    printf("assert_failed: %s:%d\n", filename, line);
    exit(1);
}