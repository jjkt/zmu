ifndef GCC_HOME
    $(error GCC_HOME is undefined)
endif

CC=$(GCC_HOME)/bin/arm-none-eabi-gcc
CFLAGS= -O2 --specs=rdimon.specs -u _printf_float -mthumb -g -nostartfiles -T link.ld -D__STARTUP_CLEAR_BSS
LIBS=-lc -lrdimon

ifneq ($(wildcard $(GCC_HOME)/share/gcc-arm-embedded/samples/startup/*),)
	STARTUP_PATH := $(GCC_HOME)/share/gcc-arm-embedded/samples/startup
else ifneq ($(wildcard $(GCC_HOME)/share/gcc-arm-none-eabi/samples/startup/*),)
	STARTUP_PATH := $(GCC_HOME)/share/gcc-arm-none-eabi/samples/startup
else ifneq ($(wildcard $(GCC_HOME)/share/doc/gcc-arm-none-eabi/examples/startup/*),)
	STARTUP_PATH := $(GCC_HOME)/share/doc/gcc-arm-none-eabi/examples/startup
else
$(error startup dir not found !)
endif

all: fault-usage-cm3.elf fault-memmanage-cm3.elf fault-hardfault-cm0.elf fault-lockup-cm0.elf fault-forced-hardfault-cm3.elf

fault-usage-cm3.elf:
	$(CC) $(CFLAGS) -DFAULT_CASE=1 -mcpu=cortex-m3 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM3.S -o $@

fault-memmanage-cm3.elf:
	$(CC) $(CFLAGS) -DFAULT_CASE=2 -mcpu=cortex-m3 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM3.S -o $@

fault-hardfault-cm0.elf:
	$(CC) $(CFLAGS) -DFAULT_CASE=3 -mcpu=cortex-m0 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM0.S -o $@

fault-lockup-cm0.elf:
	$(CC) $(CFLAGS) -DFAULT_CASE=4 -mcpu=cortex-m0 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM0.S -o $@

fault-forced-hardfault-cm3.elf:
	$(CC) $(CFLAGS) -DFAULT_CASE=5 -mcpu=cortex-m3 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM3.S -o $@

clean:
	rm -f fault-*.elf
