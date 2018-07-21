""" Module for autogenerating ARMv6-m decoder trees"""


INSTRUCTIONS = {
    #
    # ARM v6m
    #
    '1111001111101111 1000xxxxxxxxxxxx':  'MRS_t1',
    '111100111000xxxx 10001000xxxxxxxx':  'MSR_reg_t1',
    '1111001110111111 100011110110xxxx':  'ISB_t1',
    '1111001110111111 100011110101xxxx':  'DMB_t1',
    '1111001110111111 100011110100xxxx':  'DSB_t1',
    '111101111111xxxx 1010xxxxxxxxxxxx':  'UDF_t2',

    # Data processing, modified immediate
    '11110x00000xxxxx 0xxxxxxxxxxxxxxx': 'AND_imm_t1',
    '11110x000001xxxx 0xxx1111xxxxxxxx': 'TST_imm_t1',
    # Data processing, (plain binary immediate)
    # Branches and misc control
    '11110xxxxxxxxxxx 11x1xxxxxxxxxxxx': 'BL_t1',
    # Hint instructions
    # Misc control instructions


    #
    # load store multiple
    #
    '1110100010x0xxxx 0x0xxxxxxxxxxxxx': 'STMX_W_t2',
    '1110100010x1xxxx xx0xxxxxxxxxxxxx': 'LDM_W_t2',
    '1110100010111101 xx0xxxxxxxxxxxxx': 'POP_W_t2',
    '1111100001011101 xxxx101100000100': 'POP_W_t3',
    '1110100100x0xxxx 0x0xxxxxxxxxxxxx': 'STMDB_t1',
    '1110100100101101 0x0xxxxxxxxxxxxx': 'PUSH_t2',
    '1111100001001101 xxxx110100000100': 'PUSH_t3',
    '1110100100x1xxxx xx0xxxxxxxxxxxxx': 'LDMDB_t1',

    #
    # load store dual or exclusive, table branch
    #
    '111010000100xxxx xxxxxxxxxxxxxxxx': 'STREX_t1',
    '111010000101xxxx xxxx1111xxxxxxxx': 'LDREX_t1',
    '1110100xx1x0xxxx xxxxxxxxxxxxxxxx': 'STRD_imm_t1',
    '1110100xx1x1xxxx xxxxxxxxxxxxxxxx': 'LDRD_imm_t1',
    '1110100xx1x11111 xxxxxxxxxxxxxxxx': 'LDRD_lit_t1',
    '111010001100xxxx xxxx11110100xxxx': 'STREXB_t1',
    '111010001100xxxx xxxx11110101xxxx': 'STREXH_t1',
    '111010001101xxxx 111100000000xxxx': 'TBB_t1',
    '111010001101xxxx 111100000001xxxx': 'TBH_t1',
    '111010001101xxxx xxxx111101001111': 'LDREXB_t1',
    '111010001101xxxx xxxx111101011111': 'LDREXH_t1',

    #
    # load word
    #
    '111110001101xxxx xxxxxxxxxxxxxxxx': 'LDR_imm_t3',
    '111110000101xxxx xxxx1xxxxxxxxxxx': 'LDR_imm_t4',
    '111110000101xxxx xxxx1110xxxxxxxx': 'LDRT_t1',
    '111110000101xxxx xxxx000000xxxxxx': 'LDR_reg_t2',
    '11111000x1011111 xxxxxxxxxxxxxxxx': 'LDR_lit_t2',

    #
    # load halfword
    #
    '111110001011xxxx xxxxxxxxxxxxxxxx': 'LDRH_imm_t2',
    '111110000011xxxx xxxx1xxxxxxxxxxx': 'LDRH_imm_t3',
    '111110000011xxxx xxxx1110xxxxxxxx': 'LDRHT_t1',
    '11111000x0111111 xxxxxxxxxxxxxxxx': 'LDRH_lit_t1',
    '111110000011xxxx xxxx000000xxxxxx': 'LDRH_reg_t2',
    '111110011011xxxx xxxxxxxxxxxxxxxx': 'LDRSH_imm_t1',
    '111110010011xxxx xxxx1xxxxxxxxxxx': 'LDRSH_imm_t2',
    '111110010011xxxx xxxx1110xxxxxxxx': 'LDRSHT',
    '11111001x0111111 xxxxxxxxxxxxxxxx': 'LDRSH_lit_t1',
    '111110010011xxxx xxxx000000xxxxxx': 'LDRSH_reg_t2',

    #
    # load byte, memory hints
    #
    '11111000x0011111 xxxxxxxxxxxxxxxx': 'LDRB_lit_t1',  # tttt != 1111 (PLD)
    '111110001001xxxx xxxxxxxxxxxxxxxx': 'LDRB_imm_t2',  # nnnn != 1111, tttt != 1111
    '111110000001xxxx xxxx1xxxxxxxxxxx': 'LDRB_imm_t3',
    '111110000001xxxx xxxx1110xxxxxxxx': 'LDRBT_t1',    # nnnn != 1111
    '111110000001xxxx xxxx00000xxxxxxx': 'LDRB_reg_t2',
    '111110011001xxxx xxxxxxxxxxxxxxxx': 'LDRSB_imm_t1',
    '111110010001xxxx xxxx1xxxxxxxxxxx': 'LDRSB_imm_t2',
    '111110010001xxxx xxxx1110xxxxxxxx': 'LDRSBT_t1',
    '11111001x0011111 xxxxxxxxxxxxxxxx': 'LDRSB_lit_t1',  # tttt != 1111 (PLI)
    '111110010000xxxx xxxx000000xxxxxx': 'LDRSB_reg_t2',

    '111110001001xxxx 1111xxxxxxxxxxxx': 'PLD_imm_t1',  # nnnn!= 1111 (PLD_lit)
    '111110000001xxxx 11111100xxxxxxxx': 'PLD_imm_t2',  # nnnn!= 1111 (PLD_lit)

    '111110011001xxxx 1111xxxxxxxxxxxx': 'PLI_lit_imm_t1',
    '111110010001xxxx 11111100xxxxxxxx': 'PLI_lit_imm_t2',
    '11111001x0011111 1111xxxxxxxxxxxx': 'PLI_lit_imm_t3',
    '111110010001xxxx 1111000000xxxxxx': 'PLI_reg_t1',  # nnnn != 1111

    # store single data item
    '111110001000xxxx xxxxxxxxxxxxxxxx': 'STRB_imm_t2',  # nnnn != 1111
    '111110000000xxxx xxxx1xxxxxxxxxxx': 'STRB_imm_t3',  # nnnn != 1111
    '111110000000xxxx xxxx000000xxxxxx': 'STRB_reg_t2',  # nnnn != 1111
    '111110001010xxxx xxxxxxxxxxxxxxxx': 'STRH_imm_t2',  # nnnn != 1111
    '111110000010xxxx xxxx1xxxxxxxxxxx': 'STRH_imm_t3',
    '111110001100xxxx xxxxxxxxxxxxxxxx': 'STR_imm_t3',  # nnnn != 1111
    '111110000100xxxx xxxx1xxxxxxxxxxx': 'STR_imm_t4',
    '111110000100xxxx xxxx000000xxxxxx': 'STR_reg_t2',  # nnnn != 1111

    # data processing, (shifted register)
    '11101010000xxxxx 0xxxxxxxxxxxxxxx': "AND_reg_t2",  # ddd == 1111, S == 1 -> TST
    '111010100001xxxx 0xxx1111xxxxxxxx': "TST_reg_t2",  # ddd == 1111,
    '11101010001xxxxx 0xxxxxxxxxxxxxxx': "BIC_reg_t2",
    '11101010010xxxxx 0xxxxxxxxxxxxxxx': "ORR_reg_t2",  # nnnn == 1111 => MOV
    '11101010011xxxxx 0xxxxxxxxxxxxxxx': "ORN_reg_t2",  # nnnn == 1111 => MVN
    '11101010011x1111 0xxxxxxxxxxxxxxx': "MVN_reg_t2",
    '11101010100xxxxx 0xxxxxxxxxxxxxxx': "EOR_reg_t2",  # dddd = 1111 & S = 1 => TEQ
    '111010101001xxxx 0xxx1111xxxxxxxx': "TEQ_reg_t1",  #
    '11101011000xxxxx 0xxxxxxxxxxxxxxx': "ADD_reg_t3",  #
    '111010110001xxxx 0xxx1111xxxxxxxx': "CMN_reg_t2",  #
    '11101011010xxxxx 0xxxxxxxxxxxxxxx': "ADC_reg_t2",  #
    '11101011011xxxxx 0xxxxxxxxxxxxxxx': "SBC_reg_t2",  #
    '11101011101xxxxx 0xxxxxxxxxxxxxxx': "SUB_reg_t2",  #
    '111010111011xxxx 0xxxxxxxxxxxxxxx': "CMP_reg_t3",  #
    '11101011110xxxxx 0xxxxxxxxxxxxxxx': "RSB_reg_t2",  #

    '11101010010x1111 0000xxxx0000xxxx': "MOV_reg_t2",  #
    '11101010010x1111 0xxxxxxxxx00xxxx': "LSL_imm_t2",  # iiiii = 00000 => MOV_reg
    '11101010010x1111 0xxxxxxxxx01xxxx': "LSR_imm_t2",  #
    '11101010010x1111 0xxxxxxxxx10xxxx': "ASR_imm_t2",  #
    '11101010010x1111 0000xxxx0011xxxx': "RRX_t1",  #
    '11101010010x1111 0xxxxxxxxx11xxxx': "ROR_imm_t1",  #

    # data processing, register
    '11111010000xxxxx 1111xxxx0000xxxx': "LSL_reg_t2",
    '11111010001xxxxx 1111xxxx0000xxxx': "LSR_reg_t2",
    '11111010010xxxxx 1111xxxx0000xxxx': "ASR_reg_t2",
    '11111010011xxxxx 1111xxxx0000xxxx': "ROR_reg_t2",
    '1111101000001111 1111xxxx10xxxxxx': "SXTH_t2",
    '1111101000011111 1111xxxx10xxxxxx': "UXTH_t2",
    '1111101001001111 1111xxxx10xxxxxx': "SXTB_t2",
    '1111101001011111 1111xxxx10xxxxxx': "UXTB_t2",

    # miscellaneous operations
    '111110101001xxxx 1111xxxx1000xxxx': "REV_t2",
    '111110101001xxxx 1111xxxx1001xxxx': "REV16_t2",
    '111110101001xxxx 1111xxxx1010xxxx': "RBIT_t1",
    '111110101001xxxx 1111xxxx1011xxxx': "REVSH_t2",
    '111110101011xxxx 1111xxxx1000xxxx': "CLZ_t1",

    # multiply, and multiply accumulate
    '111110110000xxxx xxxxxxxx0000xxxx': "MLA_t1",
    '111110110000xxxx 1111xxxx0000xxxx': "MUL_t2",
    '111110110000xxxx xxxxxxxx0001xxxx': "MLS_t1",

    # long multiply, long multiply accumulate, and divide
    '111110111000xxxx xxxxxxxx0000xxxx': "SMULL_t1",
    '111110111001xxxx 1111xxxx1111xxxx': "SDIV_t1",
    '111110111010xxxx xxxxxxxx0000xxxx': "UMULL_t1",
    '111110111011xxxx 1111xxxx1111xxxx': "UDIV_t1",
    '111110111100xxxx xxxxxxxx0000xxxx': "SMLAL_t1",
    '111110111110xxxx xxxxxxxx0000xxxx': "UMLAL_t1",

    # coprocessor instructions
    '1110110xxxx0xxxx xxxxxxxxxxxxxxxx': "STC_t1",
    '1111110xxxx0xxxx xxxxxxxxxxxxxxxx': "STC2_t2",
    '1110110xxxx1xxxx xxxxxxxxxxxxxxxx': "LDC_imm_t1",
    '1111110xxxx1xxxx xxxxxxxxxxxxxxxx': "LDC2_imm_t2",
    '1110110xxxx11111 xxxxxxxxxxxxxxxx': "LDC_lit_t1",
    '1111110xxxx11111 xxxxxxxxxxxxxxxx': "LDC2_lit_t2",
    '111011000100xxxx xxxxxxxxxxxxxxxx': "MCRR_t1",
    '111111000100xxxx xxxxxxxxxxxxxxxx': "MCRR2_t2",
    '11101110xxxxxxxx xxxxxxxxxxx0xxxx': "CDP_t1",
    '11111110xxxxxxxx xxxxxxxxxxx0xxxx': "CDP2_t2",
    '11101110xxx0xxxx xxxxxxxxxxx0xxxx': "MCR_t1",
    '11111110xxx0xxxx xxxxxxxxxxx0xxxx': "MCR2_t2",
    '11101110xxx1xxxx xxxxxxxxxxx1xxxx': "MRC_t1",
    '11111110xxx1xxxx xxxxxxxxxxx1xxxx': "MRC2_t2",

}


def main():
    """ My main function"""
    for key in reversed(sorted(INSTRUCTIONS.iterkeys())):
        numeric_low = key.replace('x', '0')
        numeric_low = numeric_low.replace(' ', '')

        numeric_high = key.replace('x', '1')
        numeric_high = numeric_high.replace(' ', '')
        #number_low = int(numeric_low, 2)
        #number_high = int(numeric_high, 2)

        print "0b{} ... 0b{} => decode_{}(opcode),".format(numeric_low, numeric_high, INSTRUCTIONS[key])
#    for value in sorted(INSTRUCTIONS.values()):
#        print "#[allow(non_snake_case)]\n\
# fn decode_{}(opcode: u32) -> Instruction {{ println!(\"{}\");\nInstruction::UDF {{imm32: 0,opcode: ThumbCode::from(opcode),}}}}\n".format(value,value)


main()
