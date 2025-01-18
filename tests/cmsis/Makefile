
#
# Input
# TARGET: cm0, cm0p, cm3, cm4, cm4f, cm7-d16, cm7-sp-d16
# GCC_HOME: path to the GCC ARM toolchain
# RUNNER: path to the runner script

# Compilation tools
CC := $(GCC_HOME)/bin/arm-none-eabi-gcc
ARMAR := $(GCC_HOME)/bin/arm-none-eabi-ar

# Compilation flags
CFLAGS_LIB := -Wsign-compare \
 -Wdouble-promotion \
 -Ofast -ffast-math \
 -DNDEBUG \
 -std=c11 \
 -mfp16-format=ieee \
 -Wall -Wextra -Werror \
 -fshort-enums

CFLAGS= -DSEMIHOSTING -ffunction-sections -mfp16-format=ieee -fdata-sections -std=c11 -Ofast -ffast-math -flax-vector-conversions -DDISABLEFLOAT16  --specs=rdimon.specs --specs=nano.specs -mthumb -Wl,--gc-sections -g -nostartfiles -T link.ld -D__STARTUP_CLEAR_BSS

# libraries to link
LIBS=-lCMSISDSP-$(TARGET) -lc -lrdimon -lm
 
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

# Path to CMSIS_5
CMSIS_5 := CMSIS_5

# Path to CMSIS_DSP
CMSIS_DSP := CMSIS-DSP

# Path to CMSIS Core includes for Cortex-M
# For low end Cortex-A, use Core_A
# For high end Cortex-A (aarch64), don't use CMSIS 
# Core Includes (Refer to the CMSIS-DSP README to 
# know how to build in that case)
CMSIS_CORE_INCLUDES := $(CMSIS_5)/CMSIS/Core/Include 

# Sources
SRCS := $(CMSIS_DSP)/Source/BasicMathFunctions/BasicMathFunctions.c \
 $(CMSIS_DSP)/Source/CommonTables/CommonTables.c \
 $(CMSIS_DSP)/Source/InterpolationFunctions/InterpolationFunctions.c \
 $(CMSIS_DSP)/Source/BayesFunctions/BayesFunctions.c \
 $(CMSIS_DSP)/Source/MatrixFunctions/MatrixFunctions.c \
 $(CMSIS_DSP)/Source/ComplexMathFunctions/ComplexMathFunctions.c \
 $(CMSIS_DSP)/Source/QuaternionMathFunctions/QuaternionMathFunctions.c \
 $(CMSIS_DSP)/Source/ControllerFunctions/ControllerFunctions.c \
 $(CMSIS_DSP)/Source/SVMFunctions/SVMFunctions.c \
 $(CMSIS_DSP)/Source/DistanceFunctions/DistanceFunctions.c \
 $(CMSIS_DSP)/Source/StatisticsFunctions/StatisticsFunctions.c \
 $(CMSIS_DSP)/Source/FastMathFunctions/FastMathFunctions.c \
 $(CMSIS_DSP)/Source/SupportFunctions/SupportFunctions.c \
 $(CMSIS_DSP)/Source/FilteringFunctions/FilteringFunctions.c \
 $(CMSIS_DSP)/Source/TransformFunctions/TransformFunctions.c \
 $(CMSIS_DSP)/Source/WindowFunctions/WindowFunctions.c
 
# Includes
DSP_INCLUDES := $(CMSIS_DSP)/Include \
  $(CMSIS_DSP)/PrivateInclude 

# Compilation flags for include folders
INC_FLAGS := $(addprefix -I,$(DSP_INCLUDES))
INC_FLAGS += $(addprefix -I,$(CMSIS_CORE_INCLUDES))
CFLAGS_LIB += $(INC_FLAGS) $(XCFLAGS)

CFLAGS += $(INC_FLAGS) $(XCFLAGS)

# Output folder for build products
BUILDDIR := ./builddir-$(TARGET)
OBJECTS := $(SRCS:%=$(BUILDDIR)/%.o)
EXAMPLES_DIR := CMSIS-DSP/Examples/ARM

# Select startup file based on TARGET variable:
# TARGET             startupfile
# ---------------------------------------
# cm0                startup_ARMCM0.S
# cm0p               startup_ARMCM0.S
# cm3                startup_ARMCM3.S
# cm4                startup_ARMCM4.S
# cm4f               startup_ARMCM4.S
# cm7-d16            startup_ARMCM7.S
# cm7-sp-d16         startup_ARMCM7.S
ifeq ($(TARGET),cm0)
	STARTUP_FILE := startup_ARMCM0.S
	HANDLER_FILE := handlerCM0.S
else ifeq ($(TARGET),cm0p)
	STARTUP_FILE := startup_ARMCM0.S
	HANDLER_FILE := handlerCM0.S
else ifeq ($(TARGET),cm3)
	STARTUP_FILE := startup_ARMCM3.S
	HANDLER_FILE := handlerCM3.S
else ifeq ($(TARGET),cm4)
	STARTUP_FILE := startup_ARMCM4.S
	HANDLER_FILE := handlerCM3.S
else ifeq ($(TARGET),cm4f)
	STARTUP_FILE := startup_ARMCM4.S
	HANDLER_FILE := handlerCM3.S
else ifeq ($(TARGET),cm7-d16)
	STARTUP_FILE := startup_ARMCM7.S
	HANDLER_FILE := handlerCM3.S
else ifeq ($(TARGET),cm7-sp-d16)
	STARTUP_FILE := startup_ARMCM7.S
	HANDLER_FILE := handlerCM3.S
else
	$(error TARGET not supported)
endif

EXES := $(BUILDDIR)/arm_bayes_example.elf \
$(BUILDDIR)/arm_class_marks_example.elf \
$(BUILDDIR)/arm_convolution_example.elf \
$(BUILDDIR)/arm_dotproduct_example.elf \
$(BUILDDIR)/arm_fft_bin_example.elf \
$(BUILDDIR)/arm_graphic_equalizer_example.elf \
$(BUILDDIR)/arm_linear_interp_example.elf \
$(BUILDDIR)/arm_matrix_example.elf \
$(BUILDDIR)/arm_variance_example.elf

# forever loop on cm3
#$(BUILDDIR)/arm_fir_example.elf \ (ok on CM0)
#$(BUILDDIR)/arm_signal_converge_example.elf \ (ok on CM0)
# $(BUILDDIR)/arm_sin_cos_example.elf \ (ok on CM0)

# forever loop on cm4f
#$(BUILDDIR)/arm_svm_example.elf \


#
# for each EXES, run RUNNER and ignore if it fails
#
.PHONY: run
run : $(EXES)
	$(foreach exe,$(EXES),echo running $(RUNNER) $(exe);$(RUNNER) $(exe) || true;)


all: $(EXES)

$(BUILDDIR)/arm_bayes_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_bayes_example/arm_bayes_example_f32.c \
	$(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_class_marks_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_class_marks_example/arm_class_marks_example_f32.c \
	$(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_convolution_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_convolution_example/arm_convolution_example_f32.c \
	$(EXAMPLES_DIR)/arm_convolution_example/math_helper.c $(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_dotproduct_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_dotproduct_example/arm_dotproduct_example_f32.c \
	$(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_fft_bin_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_fft_bin_example/arm_fft_bin_example_f32.c \
	$(EXAMPLES_DIR)/arm_fft_bin_example/arm_fft_bin_data.c $(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_fir_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_fir_example/arm_fir_example_f32.c \
	$(EXAMPLES_DIR)/arm_fir_example/arm_fir_data.c $(EXAMPLES_DIR)/arm_fir_example/math_helper.c \
	$(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_graphic_equalizer_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_graphic_equalizer_example/arm_graphic_equalizer_example_q31.c \
	$(EXAMPLES_DIR)/arm_graphic_equalizer_example/arm_graphic_equalizer_data.c \
	$(EXAMPLES_DIR)/arm_graphic_equalizer_example/math_helper.c $(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_linear_interp_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_linear_interp_example/arm_linear_interp_example_f32.c \
	$(EXAMPLES_DIR)/arm_linear_interp_example/arm_linear_interp_data.c \
	$(EXAMPLES_DIR)/arm_linear_interp_example/math_helper.c $(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_matrix_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_matrix_example/arm_matrix_example_f32.c \
	$(EXAMPLES_DIR)/arm_matrix_example/math_helper.c $(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_signal_converge_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_signal_converge_example/arm_signal_converge_example_f32.c \
	$(EXAMPLES_DIR)/arm_signal_converge_example/arm_signal_converge_data.c \
	$(EXAMPLES_DIR)/arm_signal_converge_example/math_helper.c $(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_sin_cos_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_sin_cos_example/arm_sin_cos_example_f32.c \
	$(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_svm_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_svm_example/arm_svm_example_f32.c \
	$(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

$(BUILDDIR)/arm_variance_example.elf: $(BUILDDIR)/libCMSISDSP-$(TARGET).a initialization.c $(HANDLER_FILE)
	$(CC) $(CFLAGS) initialization.c $(EXAMPLES_DIR)/arm_variance_example/arm_variance_example_f32.c \
	$(STARTUP_PATH)/$(STARTUP_FILE) $(HANDLER_FILE) -L$(BUILDDIR) $(LIBS) -o $@ 

# Build rules
$(BUILDDIR)/libCMSISDSP-$(TARGET).a: $(OBJECTS)
	$(ARMAR) -rc $@ $(OBJECTS)
	
$(BUILDDIR)/%.c.o: %.c
	mkdir -p $(dir $@)
	$(CC) -c $(CFLAGS_LIB) $(CPPFLAGS) $< -o $@

clean:
	rm -f $(BUILDDIR)/libCMSISDSP-$(TARGET).a $(OBJECTS)