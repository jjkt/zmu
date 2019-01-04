
CC=arm-none-eabi-gcc
CFLAGS= -O2 --specs=rdimon.specs -mthumb -g -nostartfiles -T link.ld  
LIBS=-lc -lrdimon
STARTUP_PATH=/usr/share/gcc-arm-embedded/samples/startup

#
# cortex-m0 cortex-m0.small-multiply cortex-m0plus
# cortex-m0plus.small-multiply cortex-m1 cortex-m1.small-multiply 
# cortex-m23 cortex-m3 cortex-m33 cortex-m33+nodsp cortex-m4 cortex-m7
#

all: hello_world-cm0.elf hello_world-cm0p.elf hello_world-cm3.elf hello_world-cm4.elf hello_world-cm4f.elf

hello_world-cm0.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m0 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM0.S -o $@

hello_world-cm0p.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m0plus $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM0.S -o $@

hello_world-cm3.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m3 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM3.S -o $@

hello_world-cm4.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m4 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM4.S -o $@

hello_world-cm4f.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM4.S -o $@

clean:
	rm -f *.elf
