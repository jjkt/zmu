#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

/*
This bench is intentionally small and fault-focused.
Each ELF is built with a single FAULT_CASE value so the host-side test runner can
verify trap/no-trap behaviour for the currently reachable fault classes, plus
lockup handling.
*/

#define FAULT_CASE_USAGE 1
#define FAULT_CASE_MEMMANAGE 2
#define FAULT_CASE_HARDFAULT 3
#define FAULT_CASE_LOCKUP 4
#define FAULT_CASE_FORCED_HARDFAULT 5
#define FAULT_CASE_EXCEPTION_RETURN_INVSTATE 6
#define FAULT_CASE_EXCEPTION_RETURN_INVSTATE_FORCED_HARDFAULT 7
#define FAULT_CASE_SOFTWARE_PEND_SVCALL 8
#define FAULT_CASE_SOFTWARE_PEND_USAGEFAULT 9
#define FAULT_CASE_SOFTWARE_PEND_MEMMANAGE 10
#define FAULT_CASE_SOFTWARE_PEND_BUSFAULT 11

#ifndef FAULT_CASE
#define FAULT_CASE FAULT_CASE_USAGE
#endif

#ifndef __ARM_ARCH
#define __ARM_ARCH 0
#endif

#define SCB_SHCSR (*(volatile uint32_t *)0xE000ED24u)
#define SCB_CFSR (*(volatile uint32_t *)0xE000ED28u)
#define SCB_HFSR (*(volatile uint32_t *)0xE000ED2Cu)
#define SCB_MMFAR (*(volatile uint32_t *)0xE000ED34u)

#define SHCSR_MEMFAULTACT (1u << 0)
#define SHCSR_BUSFAULTACT (1u << 1)
#define SHCSR_USGFAULTACT (1u << 3)
#define SHCSR_SVCALLACT (1u << 7)
#define SHCSR_USGFAULTPENDED (1u << 12)
#define SHCSR_MEMFAULTPENDED (1u << 13)
#define SHCSR_BUSFAULTPENDED (1u << 14)
#define SHCSR_SVCALLPENDED (1u << 15)
#define SHCSR_MEMFAULTENA (1u << 16)
#define SHCSR_BUSFAULTENA (1u << 17)
#define SHCSR_USGFAULTENA (1u << 18)
#define CFSR_UNDEFINSTR (1u << 16)
#define CFSR_INVSTATE (1u << 17)
#define HFSR_FORCED (1u << 30)
#define XPSR_T (1u << 24)

static volatile uint32_t fault_marker;
static volatile uint32_t fault_stage;

__attribute__((noinline, noreturn)) static void trigger_udf(void)
{
    asm volatile("udf #0");

    /* Should never be reached. */
    for (;;)
    {
    }
}

__attribute__((noinline, noreturn)) static void trigger_memmanage(void)
{
    *(volatile uint32_t *)0x00000000u = 0x46544d4du; /* 'FTMM' */

    for (;;)
    {
    }
}

#if __ARM_ARCH >= 7 && (FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE || FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE_FORCED_HARDFAULT)
__attribute__((naked)) void SVC_Handler(void)
{
    asm volatile(
        "mrs r0, msp\n"
        "ldr r1, [r0, #28]\n"
        "bic r1, r1, %0\n"
        "str r1, [r0, #28]\n"
        "bx lr\n"
        :
        : "I"(XPSR_T)
        : "r0", "r1", "memory");
}

#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_SVCALL
void SVC_Handler(void)
{
    fault_marker = 0x53564654u; /* 'SVFT' */
    printf("SVC_Handler marker=0x%08lx shcsr=0x%08lx\n",
           (unsigned long)fault_marker,
           (unsigned long)SCB_SHCSR);

#if __ARM_ARCH >= 7
    if ((SCB_SHCSR & SHCSR_SVCALLPENDED) != 0 || (SCB_SHCSR & SHCSR_SVCALLACT) == 0)
    {
        exit(96);
    }
#else
    if ((SCB_SHCSR & SHCSR_SVCALLPENDED) != 0)
    {
        exit(96);
    }
#endif

    exit(0);
}

#endif

__attribute__((noinline, noreturn)) static void trigger_software_pending(void)
{
#if FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_SVCALL
    SCB_SHCSR |= SHCSR_SVCALLPENDED;
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_USAGEFAULT
    SCB_SHCSR |= SHCSR_USGFAULTPENDED;
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_MEMMANAGE
    SCB_SHCSR |= SHCSR_MEMFAULTPENDED;
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_BUSFAULT
    SCB_SHCSR |= SHCSR_BUSFAULTPENDED;
#else
    exit(3);
#endif

    for (;;)
    {
        asm volatile("nop");
    }
}

#if __ARM_ARCH >= 7
__attribute__((noinline, noreturn)) static void trigger_exception_return_invstate(void)
{
    asm volatile("svc #0");

    for (;;)
    {
    }
}
#endif

static const char *fault_case_name(void)
{
#if FAULT_CASE == FAULT_CASE_USAGE
    return "usagefault";
#elif FAULT_CASE == FAULT_CASE_MEMMANAGE
    return "memmanage";
#elif FAULT_CASE == FAULT_CASE_HARDFAULT
    return "hardfault";
#elif FAULT_CASE == FAULT_CASE_LOCKUP
    return "lockup";
#elif FAULT_CASE == FAULT_CASE_FORCED_HARDFAULT
    return "forced-hardfault";
#elif FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE
    return "exception-return-invstate";
#elif FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE_FORCED_HARDFAULT
    return "exception-return-invstate-forced-hardfault";
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_SVCALL
    return "software-pend-svcall";
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_USAGEFAULT
    return "software-pend-usagefault";
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_MEMMANAGE
    return "software-pend-memmanage";
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_BUSFAULT
    return "software-pend-busfault";
#else
    return "unknown";
#endif
}

__attribute__((noreturn)) static void trigger_selected_fault(void)
{
#if FAULT_CASE == FAULT_CASE_USAGE
    trigger_udf();
#elif FAULT_CASE == FAULT_CASE_MEMMANAGE
    trigger_memmanage();
#elif FAULT_CASE == FAULT_CASE_HARDFAULT
    trigger_udf();
#elif FAULT_CASE == FAULT_CASE_LOCKUP
    trigger_udf();
#elif FAULT_CASE == FAULT_CASE_FORCED_HARDFAULT
    trigger_udf();
#elif FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE
    trigger_exception_return_invstate();
#elif FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE_FORCED_HARDFAULT
    trigger_exception_return_invstate();
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_SVCALL
    trigger_software_pending();
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_USAGEFAULT
    trigger_software_pending();
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_MEMMANAGE
    trigger_software_pending();
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_BUSFAULT
    trigger_software_pending();
#else
#error Unsupported FAULT_CASE
#endif
}

static void enable_selected_fault_handler(void)
{
#if __ARM_ARCH >= 7
#if FAULT_CASE == FAULT_CASE_USAGE
    SCB_SHCSR |= SHCSR_USGFAULTENA;
#elif FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE
    SCB_SHCSR |= SHCSR_USGFAULTENA;
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_USAGEFAULT
    SCB_SHCSR |= SHCSR_USGFAULTENA;
#elif FAULT_CASE == FAULT_CASE_MEMMANAGE
    SCB_SHCSR |= SHCSR_MEMFAULTENA;
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_MEMMANAGE
    SCB_SHCSR |= SHCSR_MEMFAULTENA;
#elif FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_BUSFAULT
    SCB_SHCSR |= SHCSR_BUSFAULTENA;
#endif
#endif
}

#if __ARM_ARCH >= 7
void UsageFault_Handler(void)
{
    fault_marker = 0x55534654u; /* 'USFT' */
    printf("UsageFault_Handler marker=0x%08lx cfsr=0x%08lx shcsr=0x%08lx\n",
           (unsigned long)fault_marker,
           (unsigned long)SCB_CFSR,
           (unsigned long)SCB_SHCSR);

    if (FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE)
    {
        if ((SCB_CFSR & CFSR_INVSTATE) == 0 || (SCB_SHCSR & SHCSR_USGFAULTACT) == 0)
        {
            exit(97);
        }
    }

    if (FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_USAGEFAULT)
    {
        if (SCB_CFSR != 0 || (SCB_SHCSR & SHCSR_USGFAULTPENDED) != 0 || (SCB_SHCSR & SHCSR_USGFAULTACT) == 0)
        {
            exit(96);
        }
    }

    exit(0);
}

void MemManage_Handler(void)
{
    fault_marker = 0x4d4d4654u; /* 'MMFT' */
    printf("MemManage_Handler marker=0x%08lx cfsr=0x%08lx mmfar=0x%08lx shcsr=0x%08lx\n",
           (unsigned long)fault_marker,
           (unsigned long)SCB_CFSR,
           (unsigned long)SCB_MMFAR,
           (unsigned long)SCB_SHCSR);

    if (FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_MEMMANAGE)
    {
        if (SCB_CFSR != 0 || (SCB_SHCSR & SHCSR_MEMFAULTPENDED) != 0 || (SCB_SHCSR & SHCSR_MEMFAULTACT) == 0)
        {
            exit(96);
        }
    }

    exit(0);
}

void BusFault_Handler(void)
{
    fault_marker = 0x42534654u; /* 'BSFT' */
    printf("BusFault_Handler marker=0x%08lx cfsr=0x%08lx shcsr=0x%08lx\n",
           (unsigned long)fault_marker,
           (unsigned long)SCB_CFSR,
           (unsigned long)SCB_SHCSR);

    if (FAULT_CASE == FAULT_CASE_SOFTWARE_PEND_BUSFAULT)
    {
        if (SCB_CFSR != 0 || (SCB_SHCSR & SHCSR_BUSFAULTPENDED) != 0 || (SCB_SHCSR & SHCSR_BUSFAULTACT) == 0)
        {
            exit(96);
        }
        exit(0);
    }

    exit(99);
}

void HardFault_Handler(void)
{
    if (FAULT_CASE == FAULT_CASE_FORCED_HARDFAULT)
    {
        fault_marker = 0x48444654u; /* 'HDFT' */
        printf("HardFault_Handler marker=0x%08lx case=%s cfsr=0x%08lx hfsr=0x%08lx\n",
               (unsigned long)fault_marker,
               fault_case_name(),
               (unsigned long)SCB_CFSR,
               (unsigned long)SCB_HFSR);

        if ((SCB_CFSR & CFSR_UNDEFINSTR) == 0 || (SCB_HFSR & HFSR_FORCED) == 0)
        {
            exit(98);
        }

        exit(0);
    }

    if (FAULT_CASE == FAULT_CASE_EXCEPTION_RETURN_INVSTATE_FORCED_HARDFAULT)
    {
        fault_marker = 0x48444654u; /* 'HDFT' */
        printf("HardFault_Handler marker=0x%08lx case=%s cfsr=0x%08lx hfsr=0x%08lx\n",
               (unsigned long)fault_marker,
               fault_case_name(),
               (unsigned long)SCB_CFSR,
               (unsigned long)SCB_HFSR);

        if ((SCB_CFSR & CFSR_INVSTATE) == 0 || (SCB_HFSR & HFSR_FORCED) == 0)
        {
            exit(97);
        }

        exit(0);
    }

    fault_marker = 0x48444621u; /* 'HDF!' */
    printf("HardFault_Handler unexpected marker=0x%08lx case=%s\n",
           (unsigned long)fault_marker,
           fault_case_name());
    exit(99);
}
#else
void HardFault_Handler(void)
{
#if FAULT_CASE == FAULT_CASE_LOCKUP
    fault_stage += 1u;
    fault_marker = 0x4c4f434bu; /* 'LOCK' */
    printf("HardFault_Handler stage=%lu case=%s forcing nested hardfault\n",
           (unsigned long)fault_stage,
           fault_case_name());
    fflush(stdout);
    trigger_udf();
#else
    fault_marker = 0x48444654u; /* 'HDFT' */
    printf("HardFault_Handler marker=0x%08lx case=%s\n",
           (unsigned long)fault_marker,
           fault_case_name());
    exit(0);
#endif
}
#endif

int main(void)
{
    printf("fault-test: case=%s arch=%d\n", fault_case_name(), __ARM_ARCH);
    fflush(stdout);

    enable_selected_fault_handler();
    trigger_selected_fault();
}

void SystemInit(void)
{
}

extern void initialise_monitor_handles(void);

void _start(void)
{
    initialise_monitor_handles();
    main();
    exit(2);
}

__attribute__((used)) void _fini(void) {}
