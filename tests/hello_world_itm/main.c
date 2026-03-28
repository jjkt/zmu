#include <stdint.h>
#include <stdlib.h>

#define ITM_PORT0_U8 (*(volatile uint8_t *)0xE0000000u)

static void itm_write_char(char ch)
{
    ITM_PORT0_U8 = (uint8_t)ch;
}

static void emit_hello_world(void)
{
    static const char message[] = "Hello, world!\n";

    for (const char *cursor = message; *cursor != '\0'; ++cursor) {
        itm_write_char(*cursor);
    }
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