#include <stdlib.h>
int main(void)
{
    return 0;
}


void SystemInit(void)
{
}

void _start(void)
{
    main();
    exit(0);
}

__attribute__((used))
void _fini(void) { }