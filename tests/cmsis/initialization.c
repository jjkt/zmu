#include <stdlib.h>
#include <stdio.h>

void SystemInit(void)
{
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

