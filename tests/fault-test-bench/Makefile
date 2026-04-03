include ../rules.mk

.DEFAULT_GOAL := all

all: \
	$(foreach core,$(FAULT_TEST_ARMV6M_CORES),fault-hardfault-$(core).elf fault-lockup-$(core).elf fault-software-pend-svcall-$(core).elf) \
	$(foreach core,$(FAULT_TEST_ARMV7_CORES),fault-usage-$(core).elf fault-memmanage-$(core).elf fault-forced-hardfault-$(core).elf fault-exception-return-invstate-$(core).elf fault-exception-return-invstate-forced-hardfault-$(core).elf fault-software-pend-svcall-$(core).elf fault-software-pend-usagefault-$(core).elf fault-software-pend-memmanage-$(core).elf fault-software-pend-busfault-$(core).elf)

$(foreach core,$(FAULT_TEST_ARMV6M_CORES),$(eval $(call build_test_elf,fault-hardfault-$(core).elf,-DFAULT_CASE=3 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV6M_CORES),$(eval $(call build_test_elf,fault-lockup-$(core).elf,-DFAULT_CASE=4 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV6M_CORES),$(eval $(call build_test_elf,fault-software-pend-svcall-$(core).elf,-DFAULT_CASE=8 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-usage-$(core).elf,-DFAULT_CASE=1 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-memmanage-$(core).elf,-DFAULT_CASE=2 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-forced-hardfault-$(core).elf,-DFAULT_CASE=5 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-exception-return-invstate-$(core).elf,-DFAULT_CASE=6 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-exception-return-invstate-forced-hardfault-$(core).elf,-DFAULT_CASE=7 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-software-pend-svcall-$(core).elf,-DFAULT_CASE=8 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-software-pend-usagefault-$(core).elf,-DFAULT_CASE=9 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-software-pend-memmanage-$(core).elf,-DFAULT_CASE=10 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))
$(foreach core,$(FAULT_TEST_ARMV7_CORES),$(eval $(call build_test_elf,fault-software-pend-busfault-$(core).elf,-DFAULT_CASE=11 $(TEST_BUILD_FLAGS_$(core)),$(TEST_STARTUP_$(core)))))

.PHONY: print-fault-core-metadata

print-fault-core-metadata:
	@$(foreach core,$(FAULT_TEST_ARMV6M_CORES),printf '%s %s %s\n' '$(core)' '$(TEST_RUNNER_$(core))' 'armv6m';)
	@$(foreach core,$(FAULT_TEST_ARMV7_CORES),printf '%s %s %s\n' '$(core)' '$(TEST_RUNNER_$(core))' 'armv7';)
