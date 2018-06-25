JIT emulator design
===================

Need a way to map each JIT'd target instruction into series of host instructions.
-> all jump destinations need to be recalculated, absolute and relative.
-> simplistic approach would be aligning all generated code into same sizes of code blocks ,filling with NOPs as needed.

registers
--------------

x64-64 cpu has 16 general purpose 64-bit registers, so there is plenty of space to
have almost direct register mapping, instead of simulating registers in memory.

| x86    |   thumbv2 |  
|------- | --------- |
| eax    | r0        |
| ebx    | r1        |
| ecx    | r2        |
| edx    | r3        |
| esi    | r4        |
| edi    | r5        |
| r8d    | r6        |
| r9d    | r7        |
| r10    | r8, r9    |
| r11    | r10, r11  |
| r12d   | r12       |
| r13d   | r13 (SP)  |
| r14d   | r14 (LR)  |
| r15d   | r15 (PC)  |

rsp, => stack pointer, should not be messed with so that PUSH and POP still work
rbp, => base pointer, do not mess with

condition flags
--------------------

There's a nice mapping that could be possibly directly utilized.

| x86    |   thumbv2 |  
|------- | --------- |
| SF     |  n (negative) |
| ZF     |  z (zero)  |
| CF     |  c (carry) |
| OF     |  v (singed overflow)       |



Intrinsics:
-----------
some intrinsics functions are needed and should be called with c calling convention
-> there needs to be "c like functions" that then call the rust function via a bridge.
example __bkpt() 

Instructions
-----------------

Example with "ADD" instruction:

| thumbv2          | x86       |description             |  
|----------------- | --------- | ---------------------- |
| ADDS R2, R1, R3  | ADD EBX, EDX;  MOV ECX, EBX ; 01 d3 89 d9 | r2 = r1 + r3           |
But for example same variant without "S" would need to save the condition flags first with PUSHF/POPF (9c .. 9d). Also, if x64 registers are needed, special form
is needed for encoding.
To do this, either something like LLVM could be used or just "x86asm" create to create instructions on the fly

Something like:

        writer.write2(Mnemonic::PUSHF); 
        writer.write2(Mnemonic::ADD, Operand::Direct(Reg::EBX), Operand::Direc(Reg::EDX)); 
        writer.write2(Mnemonic::MOV, Operand::Direct(Reg::ECX), Operand::Direc(Reg::EBX)); 
        writer.write2(Mnemonic::POPF); 




HowTo Execute JIT'd code in rust:
-------------------

https://github.com/danburkert/memmap-rs/blob/master/src/lib.rs

        use std::mem;
        mmap[0] = 0xB8; // mov eax, 0xAB
        mmap[1] = 0xAB;
        mmap[2] = 0x00;
        mmap[3] = 0x00;
        mmap[4] = 0x00;
        mmap[5] = 0xC3; // ret

        let mmap = mmap.make_exec().expect("make_exec");

        let jitfn: extern "C" fn() -> u8 = unsafe { mem::transmute(mmap.as_ptr()) };
        assert_eq!(jitfn(), 0xab);
