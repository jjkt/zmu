#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    printf("hello, world\n");
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