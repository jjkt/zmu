.syntax unified
.cpu cortex-m0
.thumb

.global HardFault_Handler
.global BusFault_Handler
.extern hard_fault_handler_c
.extern bus_fault_handler_c

HardFault_Handler:
  MOVS R0, #4
  MOV R1, LR
  TST R0, R1
  BNE hf_get_psp
  MRS R0, MSP
  B hf_send
hf_get_psp:  
  MRS R0, PSP
hf_send:
  LDR R2,=hard_fault_handler_c
  BX R2

BusFault_Handler:
  MOVS R0, #4
  MOV R1, LR
  TST R0, R1
  BNE bf_get_psp
  MRS R0, MSP
  B bf_send
bf_get_psp:  
  MRS R0, PSP
bf_send:
  LDR R2,=bus_fault_handler_c
  BX R2
