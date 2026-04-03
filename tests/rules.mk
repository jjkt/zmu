# common rules used by all tests
# testing the requirements
ifndef GCC_HOME
    $(error GCC_HOME is undefined)
endif

CC=$(GCC_HOME)/bin/arm-none-eabi-gcc
CFLAGS= -O2 --specs=rdimon.specs -u _printf_float -mthumb -g -nostartfiles -T link.ld -D__STARTUP_CLEAR_BSS
LIBS=-lc -lrdimon

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

TEST_CORES_ARMV6M := cm0 cm0p
TEST_CORES_ARMV7M := cm3
TEST_CORES_ARMV7EM := cm4 cm4f cm7-d16 cm7-sp-d16
ALL_TEST_CORES := $(TEST_CORES_ARMV6M) $(TEST_CORES_ARMV7M) $(TEST_CORES_ARMV7EM)

TEST_BUILD_FLAGS_cm0 := -mcpu=cortex-m0
TEST_BUILD_FLAGS_cm0p := -mcpu=cortex-m0plus
TEST_BUILD_FLAGS_cm3 := -mcpu=cortex-m3
TEST_BUILD_FLAGS_cm4 := -mcpu=cortex-m4
TEST_BUILD_FLAGS_cm4f := -mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16
TEST_BUILD_FLAGS_cm7-d16 := -mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-d16
TEST_BUILD_FLAGS_cm7-sp-d16 := -mcpu=cortex-m7 -mfloat-abi=hard -mfpu=fpv5-sp-d16

TEST_STARTUP_cm0 := startup_ARMCM0.S
TEST_STARTUP_cm0p := startup_ARMCM0.S
TEST_STARTUP_cm3 := startup_ARMCM3.S
TEST_STARTUP_cm4 := startup_ARMCM4.S
TEST_STARTUP_cm4f := startup_ARMCM4.S
TEST_STARTUP_cm7-d16 := startup_ARMCM7.S
TEST_STARTUP_cm7-sp-d16 := startup_ARMCM7.S

TEST_RUNNER_cm0 := zmu-cortex-m0
TEST_RUNNER_cm0p := zmu-cortex-m0plus
TEST_RUNNER_cm3 := zmu-cortex-m3
TEST_RUNNER_cm4 := zmu-cortex-m4
TEST_RUNNER_cm4f := zmu-cortex-m4f
TEST_RUNNER_cm7-d16 := zmu-cortex-m7-d16
TEST_RUNNER_cm7-sp-d16 := zmu-cortex-m7-sp-d16

FAULT_TEST_ARMV6M_CORES := $(TEST_CORES_ARMV6M)
FAULT_TEST_ARMV7_CORES := $(TEST_CORES_ARMV7M) $(TEST_CORES_ARMV7EM)

define build_test_elf
$1:
	$(CC) $(CFLAGS) $2 $(LIBS) main.c $(STARTUP_PATH)/$3 -o $$@
endef

ifdef PROG

all: $(foreach core,$(ALL_TEST_CORES),$(PROG)-$(core).elf)

$(foreach core,$(ALL_TEST_CORES),$(eval $(call build_test_elf,$(PROG)-$(core).elf,$(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))

endif

clean:
	rm -f *.elf