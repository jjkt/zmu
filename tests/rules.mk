
#common rules used by all tests

# testing the requirements
ifndef GCC_HOME
    $(error GCC_HOME is undefined)
endif

CC=arm-none-eabi-gcc
CFLAGS= -O2 --specs=rdimon.specs -mthumb -g -nostartfiles -T link.ld -D__STARTUP_CLEAR_BSS
LIBS=-lc -lrdimon

# check the expected dir for the startup file
ifneq ($(wildcard $(GCC_HOME)/share/gcc-arm-embedded/samples/startup/*),)
	STARTUP_PATH := $(GCC_HOME)/share/gcc-arm-embedded/samples/startup
else ifneq ($(wildcard $(GCC_HOME)/share/gcc-arm-none-eabi/samples/startup/*),)
	STARTUP_PATH := $(GCC_HOME)/share/gcc-arm-none-eabi/samples/startup
else
	$(error startup dir not found !)
endif

all: $(PROG)-cm0.elf $(PROG)-cm0p.elf $(PROG)-cm3.elf $(PROG)-cm4.elf $(PROG)-cm4f.elf

$(PROG)-cm0.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m0 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM0.S -o $@

$(PROG)-cm0p.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m0plus $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM0.S -o $@

$(PROG)-cm3.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m3 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM3.S -o $@

$(PROG)-cm4.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m4 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM4.S -o $@

$(PROG)-cm4f.elf:
	$(CC) $(CFLAGS) -mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16 $(LIBS) main.c $(STARTUP_PATH)/startup_ARMCM4.S -o $@

clean:
	rm -f *.elf