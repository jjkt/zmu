 
all:
	arm-none-eabi-gcc -lc --specs=rdimon.specs -mcpu=cortex-m0 -mthumb -g -nostartfiles -lrdimon -T link.ld  -D__STARTUP_CLEAR_BSS main.c /usr/share/gcc-arm-embedded/samples/startup/startup_ARMCM0.S -o minimal.elf

clean:
	rm -f *.o
