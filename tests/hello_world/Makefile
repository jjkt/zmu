
CC=arm-none-eabi-gcc
CFLAGS= --specs=rdimon.specs -mthumb -g -nostartfiles -T link.ld  
LIBS=-lc -lrdimon
STARTUP_PATH=/usr/share/gcc-arm-embedded/samples/startup

all: hello_world-cm0.elf hello_world-cm3.elf hello_world-cm4.elf

hello_world-cm0.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m0 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM0.S -o $@

hello_world-cm3.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m3 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM3.S -o $@

hello_world-cm4.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m4 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM4.S -o $@

clean:
	rm -f *.elf
