#File : core_portme.mak

# check the expected dir for the startup file
ifneq ($(wildcard $(GCC_HOME)/share/gcc-arm-embedded/samples/startup/*),)
	STARTUP_PATH := $(GCC_HOME)/share/gcc-arm-embedded/samples/startup
else ifneq ($(wildcard $(GCC_HOME)/share/gcc-arm-none-eabi/samples/startup/*),)
	STARTUP_PATH := $(GCC_HOME)/share/gcc-arm-none-eabi/samples/startup
else ifneq ($(wildcard $(GCC_HOME)/share/doc/gcc-arm-none-eabi/examples/startup/*),)
	STARTUP_PATH := $(GCC_HOME)/share/doc/gcc-arm-none-eabi/examples/startup
else
	$(error startup dir not found !)
endif

# Flag : OUTFLAG
#	Use this flag to define how to to get an executable (e.g -o)
OUTFLAG= -o
# Flag : CC
#	Use this flag to define compiler to use
CC = $(GCC_HOME)/bin/arm-none-eabi-gcc
# Flag : CFLAGS
#	Use this flag to define compiler options. Note, you can add compiler options from the command line using XCFLAGS="other flags"
PORT_CFLAGS = -O3 --specs=rdimon.specs -mthumb -g -nostartfiles -T $(PORT_DIR)/link.ld -D__STARTUP_CLEAR_BSS
FLAGS_STR = "$(PORT_CFLAGS) $(XCFLAGS) $(XLFLAGS) $(LFLAGS_END)"
CFLAGS = $(PORT_CFLAGS) -I$(PORT_DIR) -I. -DCOMPILER_FLAGS=\"$(FLAGS_STR)\"
#Flag : LFLAGS_END
#	Define any libraries needed for linking or other flags that should come at the end of the link line (e.g. linker scripts). 
#	Note : On certain platforms, the default clock_gettime implementation is supported but requires linking of librt.
LFLAGS_END = -lc -lrdimon
# Flag : PORT_SRCS
# 	Port specific source files can be added here
# Detect CPU type from XCFLAGS to select the right startup file and emulator
ifeq ($(findstring cortex-m0,$(XCFLAGS)),cortex-m0)
	STARTUP_FILE = startup_ARMCM0.S
	ZMU = ../../../target/release/zmu-armv6m
else ifeq ($(findstring cortex-m3,$(XCFLAGS)),cortex-m3)
	STARTUP_FILE = startup_ARMCM3.S
	ZMU = ../../../target/release/zmu-armv7m
else ifeq ($(findstring cortex-m4,$(XCFLAGS)),cortex-m4)
	STARTUP_FILE = startup_ARMCM4.S
	ZMU = ../../../target/release/zmu-armv7m
else
	STARTUP_FILE = startup_ARMCM4.S
	ZMU = ../../../target/release/zmu-armv7m
endif
PORT_SRCS = $(PORT_DIR)/core_portme.c $(STARTUP_PATH)/$(STARTUP_FILE)
# Flag : LOAD
#	For a simple port, we assume self hosted compile and run, no load needed.

# Flag : RUN
#	For a simple port, we assume self hosted compile and run, simple invocation of the executable

#For native compilation and execution
LOAD = echo Loading done
RUN = $(ZMU) run 

OEXT = .o
EXE = .elf

# Target : port_pre% and port_post%
# For the purpose of this simple port, no pre or post steps needed.

.PHONY : port_prebuild port_postbuild port_prerun port_postrun port_preload port_postload
port_pre% port_post% : 

# FLAG : OPATH
# Path to the output folder. Default - current folder.
OPATH = ./
MKDIR = mkdir -p

