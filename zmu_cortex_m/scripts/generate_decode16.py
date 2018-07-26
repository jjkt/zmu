""" Module for autogenerating ARMv6-m decoder trees"""


INSTRUCTIONS = {

    '00000...........': 'MOV_reg_t2_LSL_imm_t1',
    '00001...........': 'LSR_imm_t1',
    '00010...........': 'ASR_imm_t1',
    '0001100.........': 'ADD_reg_t1',
    '0001101.........': 'SUB_reg_t1',
    '0001110.........': 'ADD_imm_t1',
    '0001111.........': 'SUB_imm_t1',
    '00100...........': 'MOV_imm_t1',
    '00101...........': 'CMP_imm_t1',
    '00110...........': 'ADD_imm_t2',
    '00111...........': 'SUB_imm_t2',
    '0100000000......': 'AND_reg_t1',
    '0100000001......': 'EOR_reg_t1',
    '0100000010......': 'LSL_reg_t1',
    '0100000011......': 'LSR_reg_t1',
    '0100000100......': 'ASR_reg_t1',
    '0100000101......': 'ADC_reg_t1',
    '0100000110......': 'SBC_reg_t1',
    '0100000111......': 'ROR_reg_t1',
    '0100001000......': 'TST_reg_t1',
    '0100001001......': 'RSB_imm_t1',
    '0100001010......': 'CMP_reg_t1',
    '0100001011......': 'CMN_reg_t1',
    '0100001100......': 'ORR_reg_t1',
    '0100001101......': 'MUL_t1',
    '0100001110......': 'BIC_reg_t1',
    '0100001111......': 'MVN_reg_t1',
    '01000100........': 'ADD_reg_t2_ADD_SP_reg',
    '01000101........': 'CMP_reg_t2',
    '01000110........': 'MOV_reg_t1',
    '010001110....000': 'BX_t1',
    '010001111....000': 'BLX_t1',
    '01001...........': 'LDR_lit_t1',
    '0101000.........': 'STR_reg_t1',
    '0101001.........': 'STRH_reg_t1',
    '0101010.........': 'STRB_reg_t1',
    '0101011.........': 'LDRSB_reg_t1',
    '0101100.........': 'LDR_reg_t1',
    '0101101.........': 'LDRH_reg_t1',
    '0101110.........': 'LDRB_reg_t1',
    '0101111.........': 'LDRSH_reg_t1',
    '01100...........': 'STR_imm_t1',
    '01101...........': 'LDR_imm_t1',
    '01110...........': 'STRB_imm_t1',
    '01111...........': 'LDRB_imm_t1',
    '10000...........': 'STRH_imm_t1',
    '10001...........': 'LDRH_imm_t1',
    '10010...........': 'STR_imm_t2',
    '10011...........': 'LDR_imm_t2',
    '10100...........': 'ADR_t1',
    '10101...........': 'ADD_SP_imm_t1',
    '101100000.......': 'ADD_SP_imm_t2',
    '101100001.......': 'SUB_SP_imm_t1',
    '1011001000......': 'SXTH_t1',
    '1011001001......': 'SXTB_t1',
    '1011001010......': 'UXTH_t1',
    '1011001011......': 'UXTB_t1',
    '1011010.........': 'PUSH_t1',
    '10110110011.0010': 'CPS_t1',
    '1011.0.1........': 'CBZ_t1',
    '1011101000......': 'REV_t1',
    '1011101001......': 'REV16_t1',
    '1011101011......': 'REVSH_t1',
    '1011110.........': 'POP_reg_t1',
    '10111110........': 'BKPT_t1',
    '10111111........': 'IT_t1',
    '1011111100000000': 'NOP_t1',
    '1011111100010000': 'YIELD_t1',
    '1011111100100000': 'WFE_t1',
    '1011111100110000': 'WFI_t1',
    '1011111101000000': 'SEV_t1',
    '11000...........': 'STM_t1',
    '11001...........': 'LDM_t1',
    '1101............': 'B_t1_SVC_t1',
    '11100...........': 'B_t2',

}


def main():
    """ My main function"""

    #
    # finding a decoder match:
    # - go through the list of bitmasks in the order of specificity
    #   - test first the ones that have most bits set
    #   - first one to match is the one
    #
    maskstrings = sorted(INSTRUCTIONS.iterkeys(),
                         key=lambda string: string.count('.'))
    onemasks = [key.replace('0', '1')
                for key in maskstrings]
    onemasks = [int(key.replace('.', '0'), 2)
                for key in onemasks]
    resultmasks = [int(key.replace('.', '0'), 2)
                   for key in maskstrings]

    for i in range(len(onemasks)):
        onemask = onemasks[i]
        result = resultmasks[i]
        instr = INSTRUCTIONS[maskstrings[i]]
        print '{} if (opcode & 0x{:x}) == 0x{:x} {{ decode_{}(opcode)}}'.format('' if i == 0 else 'else', onemask, result, instr)


main()
