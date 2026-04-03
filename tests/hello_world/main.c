#include <stdio.h>
#include <stdlib.h>

static void emit_hello_world(void)
{
    printf("hello, world\n");
}

int main(void)
{
    emit_hello_world();
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