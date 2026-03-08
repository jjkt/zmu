#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <stdint.h>

/*
This test bench is used to test various ARM Cortex-M instructions.
Instruction tests are written using inline assembly so exact
mnemonics and behavior are preserved across compiler versions.
Avoid C-only instruction checks here, as compiler code generation
can vary and hide instruction-specific regressions.
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
#define HAVE_ARM_VFP 1
#else
#define HAVE_ARM_VFP 0
#endif

#if defined(__ARM_FP) && ((__ARM_FP & 0x8) != 0)
#define HAVE_ARM_FP64 1
#else
#define HAVE_ARM_FP64 0
#endif

/*
The toolchain exposes identical predefined macros for `fpv4-sp-d16` and
`fpv5-sp-d16` in this test bench setup, so use `__ARM_FP` double-precision
support as the best available proxy for the extra FP data-processing coverage
we currently exercise here (`VSEL` and `VRINTZ`).
*/
#if HAVE_ARM_FP64
#define HAVE_ARM_FP_EXTENDED_DATA_PROCESSING 1
#else
#define HAVE_ARM_FP_EXTENDED_DATA_PROCESSING 0
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

#if HAVE_ARM_VFP
float vabs_f32(float value)
{
    float result;

    asm volatile(
        "VABS.F32 %0, %1"
        : "=t"(result)
        : "t"(value));

    return result;
}

#if HAVE_ARM_FP64
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

#if HAVE_ARM_FP64
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

#if HAVE_ARM_FP64
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

int32_t vcvt_f32_s32(float a)
{
    int32_t result;
    union {
        float f;
        uint32_t u;
    } in = { .f = a };

    asm volatile(
        "vmov s0, %1\n\t"
        "vcvt.s32.f32 s0, s0\n\t"
        "vmov %0, s0"
        : "=r"(result)
        : "r"(in.u)
        : "s0");

    return result;
}

uint32_t vcvt_f32_u32(float a)
{
    uint32_t result;
    union {
        float f;
        uint32_t u;
    } in = { .f = a };

    asm volatile(
        "vmov s0, %1\n\t"
        "vcvt.u32.f32 s0, s0\n\t"
        "vmov %0, s0"
        : "=r"(result)
        : "r"(in.u)
        : "s0");

    return result;
}

float vcvt_s32_f32(int32_t a)
{
    union {
        float f;
        uint32_t u;
    } out;

    asm volatile(
        "vmov s0, %1\n\t"
        "vcvt.f32.s32 s0, s0\n\t"
        "vmov %0, s0"
        : "=r"(out.u)
        : "r"(a)
        : "s0");

    return out.f;
}

float vcvt_u32_f32(uint32_t a)
{
    union {
        float f;
        uint32_t u;
    } out;

    asm volatile(
        "vmov s0, %1\n\t"
        "vcvt.f32.u32 s0, s0\n\t"
        "vmov %0, s0"
        : "=r"(out.u)
        : "r"(a)
        : "s0");

    return out.f;
}

float vneg_f32(float value)
{
    float result;

    asm volatile(
        "VNEG.F32 %0, %1"
        : "=t"(result)
        : "t"(value));

    return result;
}

float vmul_f32(float a, float b)
{
    float result;

    asm volatile(
        "VMUL.F32 %0, %1, %2"
        : "=t"(result)
        : "t"(a), "t"(b));

    return result;
}

float vnmul_f32(float a, float b)
{
    float result;

    asm volatile(
        "VNMUL.F32 %0, %1, %2"
        : "=t"(result)
        : "t"(a), "t"(b));

    return result;
}

float vdiv_f32(float a, float b)
{
    float result;

    asm volatile(
        "VDIV.F32 %0, %1, %2"
        : "=t"(result)
        : "t"(a), "t"(b));

    return result;
}

float vfma_f32(float addend, float a, float b)
{
    float result = addend;

    asm volatile(
        "VFMA.F32 %0, %1, %2"
        : "+t"(result)
        : "t"(a), "t"(b));

    return result;
}

float vfnms_f32(float addend, float a, float b)
{
    float result = addend;

    asm volatile(
        "VFNMS.F32 %0, %1, %2"
        : "+t"(result)
        : "t"(a), "t"(b));

    return result;
}

#if HAVE_ARM_FP_EXTENDED_DATA_PROCESSING
float vrintz_f32(float value)
{
    float result;

    asm volatile(
        "VRINTZ.F32 %0, %1"
        : "=t"(result)
        : "t"(value));

    return result;
}

#endif

float vsqrt_f32(float value)
{
    float result;

    asm volatile(
        "VSQRT.F32 %0, %1"
        : "=t"(result)
        : "t"(value));

    return result;
}

#if HAVE_ARM_FP_EXTENDED_DATA_PROCESSING
float vseleq_f32(float when_equal, float when_not_equal, int lhs, int rhs)
{
    float result;

    asm volatile(
        "cmp %3, %4\n\t"
        "VSELEQ.F32 %0, %1, %2"
        : "=t"(result)
        : "t"(when_equal), "t"(when_not_equal), "r"(lhs), "r"(rhs)
        : "cc");

    return result;
}

float vselgt_f32(float when_gt, float when_not_gt, int lhs, int rhs)
{
    float result;

    asm volatile(
        "cmp %3, %4\n\t"
        "VSELGT.F32 %0, %1, %2"
        : "=t"(result)
        : "t"(when_gt), "t"(when_not_gt), "r"(lhs), "r"(rhs)
        : "cc");

    return result;
}
#endif

#if HAVE_ARM_FP_EXTENDED_DATA_PROCESSING
__attribute__((used, noinline)) void vsel_decode_probe_f32(void)
{
    asm volatile(
        "vseleq.f32 s0, s1, s2\n\t"
        "vselvs.f32 s31, s16, s30\n\t"
        "vselgt.f32 s15, s14, s15"
        :
        :
        : "s0", "s1", "s2", "s14", "s15", "s16", "s30", "s31");
}
#endif


#if HAVE_ARM_FP64
double vneg_f64(double value)
{
    double result;

    asm volatile(
        "VNEG.F64 %P0, %P1"
        : "=w"(result)
        : "w"(value));

    return result;
}

double vmul_f64(double a, double b)
{
    double result;

    asm volatile(
        "VMUL.F64 %P0, %P1, %P2"
        : "=w"(result)
        : "w"(a), "w"(b));

    return result;
}

double vnmul_f64(double a, double b)
{
    double result;

    asm volatile(
        "VNMUL.F64 %P0, %P1, %P2"
        : "=w"(result)
        : "w"(a), "w"(b));

    return result;
}

double vdiv_f64(double a, double b)
{
    double result;

    asm volatile(
        "VDIV.F64 %P0, %P1, %P2"
        : "=w"(result)
        : "w"(a), "w"(b));

    return result;
}

double vfma_f64(double addend, double a, double b)
{
    double result = addend;

    asm volatile(
        "VFMA.F64 %P0, %P1, %P2"
        : "+w"(result)
        : "w"(a), "w"(b));

    return result;
}

#if HAVE_ARM_FP_EXTENDED_DATA_PROCESSING
double vrintz_f64(double value)
{
    double result;

    asm volatile(
        "VRINTZ.F64 %P0, %P1"
        : "=w"(result)
        : "w"(value));

    return result;
}

double vsqrt_f64(double value)
{
    double result;

    asm volatile(
        "VSQRT.F64 %P0, %P1"
        : "=w"(result)
        : "w"(value));

    return result;
}

double vselge_f64(double when_ge, double when_lt, int lhs, int rhs)
{
    double result;

    asm volatile(
        "cmp %3, %4\n\t"
        "VSELGE.F64 %P0, %P1, %P2"
        : "=w"(result)
        : "w"(when_ge), "w"(when_lt), "r"(lhs), "r"(rhs)
        : "cc");

    return result;
}

#endif

int32_t vcvt_f64_s32(double a)
{
    int32_t result;

    asm volatile(
        "vcvt.s32.f64 s0, %P1\n\t"
        "vmov %0, s0"
        : "=r"(result)
        : "w"(a)
        : "s0");

    return result;
}
uint32_t vcvt_f64_u32(double a)
{
    uint32_t result;

    asm volatile(
        "vcvt.u32.f64 s0, %P1\n\t"
        "vmov %0, s0"
        : "=r"(result)
        : "w"(a)
        : "s0");

    return result;
}

#if HAVE_ARM_FP_EXTENDED_DATA_PROCESSING
__attribute__((used, noinline)) void vsel_decode_probe_f64(void)
{
    asm volatile(
        "vselge.f64 d7, d8, d9\n\t"
        "vselgt.f64 d15, d14, d13"
        :
        :
        : "d7", "d8", "d9", "d13", "d14", "d15");
}
#endif

double vcvt_s32_f64(int32_t a)
{
    double result;

    asm volatile(
        "vmov s0, %1\n\t"
        "vcvt.f64.s32 %P0, s0"
        : "=w"(result)
        : "r"(a)
        : "s0");

    return result;
}

double vcvt_u32_f64(uint32_t a)
{
    double result;

    asm volatile(
        "vmov s0, %1\n\t"
        "vcvt.f64.u32 %P0, s0"
        : "=w"(result)
        : "r"(a)
        : "s0");

    return result;
}

#endif

void floating_point(void)
{
    // Try to generate floating-point data-processing instructions
    // Remaining TODOs here: VFNMA, VMAXNM, VMLA, VNMLA, VRINTA

    // Unary data-processing instructions: VABS, VNEG
    assert(vabs_f32(-1.0f) == 1.0f);
    assert(vabs_f32(-42.0f) == 42.0f);
    assert(vabs_f32(0.0f) == 0.0f);
    assert(vabs_f32(1.0f) == 1.0f);

    assert(vneg_f32(1.0f) == -1.0f);
    assert(vneg_f32(-42.0f) == 42.0f);

#if HAVE_ARM_FP64
    assert(vabs_f64(-1.0) == 1.0);
    assert(vabs_f64(-42.0) == 42.0);
    assert(vabs_f64(0.0) == 0.0);
    assert(vabs_f64(1.0) == 1.0);

    assert(vneg_f64(1.5) == -1.5);
    assert(vneg_f64(-42.0) == 42.0);
#endif

    // Basic arithmetic: VADD, VSUB, VMUL, VNMUL, VDIV
    assert(vadd_f32(1.0f, 2.0f) == 3.0f);
    assert(vadd_f32(-1.0f, 2.0f) == 1.0f);
    assert(vadd_f32(-1.0f, -2.0f) == -3.0f);

    assert(vsub_f32(1.0f, 2.0f) == (1.0f - 2.0f));
    assert(vsub_f32(-1.0f, 2.0f) == (-1.0f - 2.0f));
    assert(vsub_f32(-1.0f, -2.0f) == (-1.0f - -2.0f));

    assert(vmul_f32(2.0f, 3.0f) == 6.0f);
    assert(vnmul_f32(2.0f, 3.0f) == -6.0f);
    assert(vdiv_f32(6.0f, 2.0f) == 3.0f);

#if HAVE_ARM_FP64
    assert(vadd_f64(1.0, 2.0) == (1.0 + 2.0));
    assert(vadd_f64(-1.0, 2.0) == (-1.0 + 2.0));
    assert(vadd_f64(-1.0, -2.0) == (-1.0 + -2.0));

    assert(vsub_f64(1.0, 2.0) == (1.0 - 2.0));
    assert(vsub_f64(-1.0, 2.0) == (-1.0 - 2.0));
    assert(vsub_f64(-1.0, -2.0) == (-1.0 - -2.0));

    assert(vmul_f64(1.5, 2.0) == 3.0);
    assert(vnmul_f64(1.5, 2.0) == -3.0);
    assert(vdiv_f64(9.0, 4.5) == 2.0);
#endif

    // Fused arithmetic: VFMA, VFNMS
    assert(vfma_f32(1.0f, 2.0f, 3.0f) == 7.0f);
    assert(vfnms_f32(10.0f, 2.0f, 3.0f) == -4.0f);

#if HAVE_ARM_FP64
    assert(vfma_f64(1.0, 2.0, 3.0) == 7.0);
#endif

    // Rounding, selection, and square root: VRINTZ, VSEL, VSQRT
    assert(vsqrt_f32(4.0f) == 2.0f);

#if HAVE_ARM_FP_EXTENDED_DATA_PROCESSING
    assert(vrintz_f32(1.75f) == 1.0f);
    assert(vrintz_f32(-2.9f) == -2.0f);
    assert(vseleq_f32(1.0f, 2.0f, 5, 5) == 1.0f);
    assert(vseleq_f32(1.0f, 2.0f, 5, 6) == 2.0f);
    assert(vselgt_f32(1.0f, 2.0f, 6, 5) == 1.0f);
    assert(vselgt_f32(1.0f, 2.0f, 5, 6) == 2.0f);
#endif

#if HAVE_ARM_FP64
    assert(vsqrt_f64(4.0) == 2.0);

#if HAVE_ARM_FP_EXTENDED_DATA_PROCESSING
    assert(vrintz_f64(-2.9) == -2.0);
    assert(vselge_f64(1.0, 2.0, 6, 5) == 1.0);
    assert(vselge_f64(1.0, 2.0, 4, 5) == 2.0);
#endif
#endif

    // Conversions: floating-point to integer
    assert(vcvt_f32_s32(42.0f) == 42);
    assert(vcvt_f32_s32(-42.0f) == -42);
    assert(vcvt_f32_u32(42.0f) == 42);
    assert(vcvt_f32_u32(-42.0f) == 0);

#if HAVE_ARM_FP64
    assert(vcvt_f64_s32(42.0) == 42);
    assert(vcvt_f64_s32(-42.0) == -42);
    assert(vcvt_f64_u32(42.0) == 42);
    assert(vcvt_f64_u32(-42.0) == 0);
#endif

    // Conversions: integer to floating-point
    assert(vcvt_s32_f32(42) == 42.0f);
    assert(vcvt_s32_f32(-42) == -42.0f);
    assert(vcvt_u32_f32(42) == 42.0f);
    assert(vcvt_u32_f32(0) == 0.0f);

#if HAVE_ARM_FP64
    assert(vcvt_s32_f64(42) == 42.0);
    assert(vcvt_s32_f64(-42) == -42.0);
    assert(vcvt_u32_f64(42) == 42.0);
    assert(vcvt_u32_f64(0) == 0.0);
#endif

}
#endif
int main(void)
{

#if __ARM_ARCH >= 7
    bfc();
#endif

#if HAVE_ARM_VFP
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