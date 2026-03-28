#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

/*
This bench is intentionally small and fault-focused.
Each ELF is built with a single FAULT_CASE value so the host-side test runner can
verify trap/no-trap behaviour for the currently reachable fault classes, plus
lockup handling.
*/

#define FAULT_CASE_USAGE      1
#define FAULT_CASE_MEMMANAGE  2
#define FAULT_CASE_HARDFAULT  3
#define FAULT_CASE_LOCKUP     4

#ifndef FAULT_CASE
#define FAULT_CASE FAULT_CASE_USAGE
#endif

static volatile uint32_t fault_marker;
static volatile uint32_t fault_stage;

__attribute__((noinline, noreturn)) static void trigger_udf(void)
{
    asm volatile("udf #0");

    /* Should never be reached. */
    for (;;) {
    }
}

__attribute__((noinline, noreturn)) static void trigger_memmanage(void)
{
    *(volatile uint32_t *)0x00000000u = 0x46544d4du; /* 'FTMM' */

    for (;;) {
    }
}

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
#else
#error Unsupported FAULT_CASE
#endif
}

#if __ARM_ARCH >= 7
void UsageFault_Handler(void)
{
    fault_marker = 0x55534654u; /* 'USFT' */
    printf("UsageFault_Handler marker=0x%08lx\n", (unsigned long)fault_marker);
    exit(0);
}

void MemManage_Handler(void)
{
    fault_marker = 0x4d4d4654u; /* 'MMFT' */
    printf("MemManage_Handler marker=0x%08lx\n", (unsigned long)fault_marker);
    exit(0);
}

void HardFault_Handler(void)
{
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
