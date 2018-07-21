""" Module for autogenerating ARMv6-m decoder trees"""


INSTRUCTIONS = {
    #
    # ARM v6m
    #
    '1111001111101111 1000ddddssssssss':  'MRS_t1',
    '111100111000nnnn 10001000ssssssss':  'MSR_reg_t1',
    '1111001110111111 100011110110oooo':  'ISB_t1',
    '1111001110111111 100011110101oooo':  'DMB_t1',
    '1111001110111111 100011110100oooo':  'DSB_t1',
    '111101111111iiii 1010iiiiiiiiiiii':  'UDF_t2',

    # Data processing, modified immediate
    '11110x00000xnnnn 0xxxddddxxxxxxxx': 'AND_imm_t1',
    '11110x000001nnnn 0xxx1111xxxxxxxx': 'TST_imm_t1',
    # Data processing, (plain binary immediate)
    # Branches and misc control
    '11110Siiiiiiiiii 11j1Jiiiiiiiiiii': 'BL_t1',
    # Hint instructions
    # Misc control instructions


    #
    # load store multiple
    #
    '1110100010W0rrrr 0M0rrrrrrrrrrrrr': 'STMX_W_t2',
    '1110100010W1rrrr PM0rrrrrrrrrrrrr': 'LDM_W_t2',
    '1110100010111101 PM0rrrrrrrrrrrrr': 'POP_W_t2',
    '1111100001011101 tttt101100000100': 'POP_W_t3',
    '1110100100W0nnnn 0M0rrrrrrrrrrrrr': 'STMDB_t1',
    '1110100100101101 0M0rrrrrrrrrrrrr': 'PUSH_t2',
    '1111100001001101 tttt110100000100': 'PUSH_t3',
    '1110100100W1nnnn PM0rrrrrrrrrrrrr': 'LDMDB_t1',

    #
    # load store dual or exclusive, table branch
    #
    '111010000100nnnn ttttddddiiiiiiii': 'STREX_t1',
    '111010000101nnnn tttt1111iiiiiiii': 'LDREX_t1',
    '1110100PU1W0nnnn ttttTTTTiiiiiiii': 'STRD_imm_t1',
    '1110100PU1W1nnnn ttttTTTTiiiiiiii': 'LDRD_imm_t1',
    '1110100PU1W11111 ttttTTTTiiiiiiii': 'LDRD_lit_t1',
    '111010001100nnnn tttt11110100dddd': 'STREXB_t1',
    '111010001100nnnn tttt11110101dddd': 'STREXH_t1',
    '111010001101nnnn 111100000000mmmm': 'TBB_t1',
    '111010001101nnnn 111100000001mmmm': 'TBH_t1',
    '111010001101nnnn tttt111101001111': 'LDREXB_t1',
    '111010001101nnnn tttt111101011111': 'LDREXH_t1',

    #
    # load word
    #
    '111110001101nnnn ttttiiiiiiiiiiii': 'LDR_imm_t3',
    '111110000101nnnn tttt1PUWiiiiiiii': 'LDR_imm_t4',
    '111110000101nnnn tttt1110iiiiiiii': 'LDRT_t1',
    '111110000101nnnn tttt000000iimmmm': 'LDR_reg_t2',
    '11111000U1011111 ttttiiiiiiiiiiii': 'LDR_lit_t2',

    #
    # load halfword
    #
    '111110001011nnnn ttttiiiiiiiiiiii': 'LDRH_imm_t2',
    '111110000011nnnn tttt1PUWiiiiiiii': 'LDRH_imm_t3',
    '111110000011nnnn tttt1110iiiiiiii': 'LDRHT_t1',
    '11111000U0111111 ttttiiiiiiiiiiii': 'LDRH_lit_t1',
    '111110000011nnnn tttt000000iimmmm': 'LDRH_reg_t2',
    '111110011011nnnn ttttiiiiiiiiiiii': 'LDRSH_imm_t1',
    '111110010011nnnn tttt1PUWiiiiiiii': 'LDRSH_imm_t2',
    '111110010011nnnn tttt1110iiiiiiii': 'LDRSHT',
    '11111001U0111111 ttttiiiiiiiiiiii': 'LDRSH_lit_t1',
    '111110010011nnnn tttt000000iimmmm': 'LDRSH_reg_t2',

    #
    # load byte, memory hints
    #
    '11111000U0011111 ttttiiiiiiiiiiii': 'LDRB_lit_t1',  # tttt != 1111 (PLD)
    '111110001001nnnn ttttiiiiiiiiiiii': 'LDRB_imm_t2',  # nnnn != 1111, tttt != 1111
    # nnnn != 1111 (LDRB_lit), !(tttt == 1111, P = 1, U = 0, W = 0) (PLD)
    '111110000001nnnn tttt1PUWiiiiiiii': 'LDRB_imm_t3',
    '111110000001nnnn tttt1110iiiiiiii': 'LDRBT_t1',    # nnnn != 1111
    # nnnn != 1111 (LDRB_lit), tttt != 1111 (PLD)
    '111110000001nnnn tttt00000iimmmmm': 'LDRB_reg_t2',
    # nnnn != 1111 (PLI), tttt != 1111 (PLD)
    # tttt != 1111 (PLI), nnnn != 1111,
    '111110011001nnnn ttttiiiiiiiiiiii': 'LDRSB_imm_t1',
    # nnnn != 1111 (LDRSB_lit), !(tttt == 1111, P = 1, U = 0, W = 0) (PLI)
    '111110010001nnnn tttt1PUWiiiiiiii': 'LDRSB_imm_t2',
    # nnnn != 1111 (LDRSB_lit)
    '111110010001nnnn tttt1110iiiiiiii': 'LDRSBT_t1',
    '11111001U0011111 ttttiiiiiiiiiiii': 'LDRSB_lit_t1',  # tttt != 1111 (PLI)
    # tttt != 1111 (PLI), nnnn!= 1111 (LDRSB_lit)
    '111110010000nnnn tttt000000iimmmm': 'LDRSB_reg_t2',

    '111110001001nnnn 1111iiiiiiiiiiii': 'PLD_imm_t1',  # nnnn!= 1111 (PLD_lit)
    '111110000001nnnn 11111100iiiiiiii': 'PLD_imm_t2',  # nnnn!= 1111 (PLD_lit)

    # nnnn!= 1111 (PLD_lit)
    '111110011001nnnn 1111iiiiiiiiiiii': 'PLI_lit_imm_t1',
    # nnnn!= 1111 (PLD_lit)
    '111110010001nnnn 11111100iiiiiiii': 'PLI_lit_imm_t2',
    '11111001U0011111 1111iiiiiiiiiiii': 'PLI_lit_imm_t3',
    '111110010001nnnn 1111000000ssmmmm': 'PLI_reg_t1',  # nnnn != 1111

    # store single data item
    '111110001000nnnn ttttiiiiiiiiiiii': 'STRB_imm_t2',  # nnnn != 1111
    '111110000000nnnn tttt1PUWiiiiiiii': 'STRB_imm_t3',  # nnnn != 1111
    '111110000000nnnn tttt000000iimmmm': 'STRB_reg_t2',  # nnnn != 1111
    '111110001010nnnn ttttiiiiiiiiiiii': 'STRH_imm_t2',  # nnnn != 1111
    # nnnn != 1111 ||(P=0 &&W = 0) -> UDF, PUW != 110 STRHT
    '111110000010nnnn tttt1PUWiiiiiiii': 'STRH_imm_t3',
    '111110001100nnnn ttttiiiiiiiiiiii': 'STR_imm_t3',  # nnnn != 1111
    # nnnn != 1111 || (P = 0 && W = 0) UDF, (special rules) PUSH, (special rules)STRT
    '111110000100nnnn tttt1PUWiiiiiiii': 'STR_imm_t4',
    '111110000100nnnn tttt000000iimmmm': 'STR_reg_t2',  # nnnn != 1111

    # data processing, (shifted register)
    '11101010000Snnnn 0iiiddddiittmmmm': "AND_reg_t2",  # ddd == 1111, S == 1 -> TST
    '111010100001nnnn 0iii1111iittmmmm': "TST_reg_t2",  # ddd == 1111,
    '11101010001Snnnn 0iiiddddiittmmmm': "BIC_reg_t2",
    '11101010010Snnnn 0iiiddddiittmmmm': "ORR_reg_t2",  # nnnn == 1111 => MOV
    '11101010011Snnnn 0iiiddddiittmmmm': "ORN_reg_t2",  # nnnn == 1111 => MVN
    '11101010011S1111 0iiiddddiittmmmm': "MVN_reg_t2",
    '11101010100Snnnn 0iiiddddiittmmmm': "EOR_reg_t2",  # dddd = 1111 & S = 1 => TEQ
    '111010101001nnnn 0iii1111iittmmmm': "TEQ_reg_t1",  #
    '11101011000Snnnn 0iiiddddiittmmmm': "ADD_reg_t3",  #
    '111010110001nnnn 0iii1111iittmmmm': "CMN_reg_t2",  #
    '11101011010Snnnn 0iiiddddiittmmmm': "ADC_reg_t2",  #
    '11101011011Snnnn 0iiiddddiittmmmm': "SBC_reg_t2",  #
    '11101011101Snnnn 0iiiddddiittmmmm': "SUB_reg_t2",  #
    '111010111011nnnn 0iiiddddiittmmmm': "CMP_reg_t3",  #
    '11101011110Snnnn 0iiiddddiittmmmm': "RSB_reg_t2",  #

    '11101010010S1111 0000dddd0000mmmm': "MOV_reg_t2",  #
    '11101010010S1111 0iiiddddii00mmmm': "LSL_imm_t2",  # iiiii = 00000 => MOV_reg
    '11101010010S1111 0iiiddddii01mmmm': "LSR_imm_t2",  #
    '11101010010S1111 0iiiddddii10mmmm': "ASR_imm_t2",  #
    '11101010010S1111 0000dddd0011mmmm': "RRX_t1",  #
    '11101010010S1111 0iiiddddii11mmmm': "ROR_imm_t1",  #

    # data processing, register
    '11111010000Snnnn 1111dddd0000mmmm': "LSL_reg_t2",
    '11111010001Snnnn 1111dddd0000mmmm': "LSR_reg_t2",
    '11111010010Snnnn 1111dddd0000mmmm': "ASR_reg_t2",
    '11111010011Snnnn 1111dddd0000mmmm': "ROR_reg_t2",
    '1111101000001111 1111dddd10rrmmmm': "SXTH_t2",
    '1111101000011111 1111dddd10rrmmmm': "UXTH_t2",
    '1111101001001111 1111dddd10rrmmmm': "SXTB_t2",
    '1111101001011111 1111dddd10rrmmmm': "UXTB_t2",

    # miscellaneous operations
    '111110101001mmmm 1111dddd1000mmmm': "REV_t2",
    '111110101001mmmm 1111dddd1001mmmm': "REV16_t2",
    '111110101001mmmm 1111dddd1010mmmm': "RBIT_t1",
    '111110101001mmmm 1111dddd1011mmmm': "REVSH_t2",
    '111110101011mmmm 1111dddd1000mmmm': "CLZ_t1",

    # multiply, and multiply accumulate
    '111110110000nnnn aaaadddd0000mmmm': "MLA_t1",
    '111110110000nnnn 1111dddd0000mmmm': "MUL_t2",
    '111110110000nnnn aaaadddd0001mmmm': "MLS_t1",

    # long multiply, long multiply accumulate, and divide
    '111110111000nnnn llllhhhh0000mmmm': "SMULL_t1",
    '111110111001nnnn 1111dddd1111mmmm': "SDIV_t1",
    '111110111010nnnn llllhhhh0000mmmm': "UMULL_t1",
    '111110111011nnnn 1111dddd1111mmmm': "UDIV_t1",
    '111110111100nnnn llllhhhh0000mmmm': "SMLAL_t1",
    '111110111110nnnn llllhhhh0000mmmm': "UMLAL_t1",

    # coprocessor instructions
    '1110110PUNW0nnnn ccccppppiiiiiiii': "STC_t1",
    '1111110PUNW0nnnn ccccppppiiiiiiii': "STC2_t2",
    '1110110PUDW1nnnn ccccppppiiiiiiii': "LDC_imm_t1",
    '1111110PUDW1nnnn ccccppppiiiiiiii': "LDC2_imm_t2",
    '1110110PUDW11111 ccccppppiiiiiiii': "LDC_lit_t1",
    '1111110PUDW11111 ccccppppiiiiiiii': "LDC2_lit_t2",
    '111011000100TTTT ttttccccoooommmm': "MCRR_t1",
    '111111000100TTTT ttttccccoooommmm': "MCRR2_t2",
    '11101110oooonnnn ddddccccooo0mmmm': "CDP_t1",
    '11111110oooonnnn ddddccccooo0mmmm': "CDP2_t2",
    '11101110ooo0nnnn ttttccccooo0mmmm': "MCR_t1",
    '11111110ooo0nnnn ttttccccooo0mmmm': "MCR2_t2",
    '11101110ooo1nnnn ttttccccooo1mmmm': "MRC_t1",
    '11111110ooo1nnnn ttttccccooo1mmmm': "MRC2_t2",

}


def main():
    """ My main function"""
    for key in sorted(INSTRUCTIONS.iterkeys()):
        numeric_low = key.replace('m', '0')
        numeric_low = numeric_low.replace('M', '0')
        numeric_low = numeric_low.replace('W', '0')
        numeric_low = numeric_low.replace('U', '0')
        numeric_low = numeric_low.replace('t', '0')
        numeric_low = numeric_low.replace('T', '0')
        numeric_low = numeric_low.replace('i', '0')
        numeric_low = numeric_low.replace('r', '0')
        numeric_low = numeric_low.replace('d', '0')
        numeric_low = numeric_low.replace('c', '0')
        numeric_low = numeric_low.replace('n', '0')
        numeric_low = numeric_low.replace('D', '0')
        numeric_low = numeric_low.replace('N', '0')
        numeric_low = numeric_low.replace('P', '0')
        numeric_low = numeric_low.replace('p', '0')
        numeric_low = numeric_low.replace('s', '0')
        numeric_low = numeric_low.replace('S', '0')
        numeric_low = numeric_low.replace('o', '0')
        numeric_low = numeric_low.replace('j', '0')
        numeric_low = numeric_low.replace('J', '0')
        numeric_low = numeric_low.replace('a', '0')
        numeric_low = numeric_low.replace('l', '0')
        numeric_low = numeric_low.replace('h', '0')
        numeric_low = numeric_low.replace(' ', '')

        numeric_high = key.replace('m', '1')
        numeric_high = numeric_high.replace('M', '1')
        numeric_high = numeric_high.replace('W', '1')
        numeric_high = numeric_high.replace('U', '1')
        numeric_high = numeric_high.replace('t', '1')
        numeric_high = numeric_high.replace('T', '1')
        numeric_high = numeric_high.replace('i', '1')
        numeric_high = numeric_high.replace('r', '1')
        numeric_high = numeric_high.replace('d', '1')
        numeric_high = numeric_high.replace('c', '1')
        numeric_high = numeric_high.replace('n', '1')
        numeric_high = numeric_high.replace('D', '1')
        numeric_high = numeric_high.replace('N', '1')
        numeric_high = numeric_high.replace('p', '1')
        numeric_high = numeric_high.replace('P', '1')
        numeric_high = numeric_high.replace('s', '1')
        numeric_high = numeric_high.replace('S', '1')
        numeric_high = numeric_high.replace('o', '1')
        numeric_high = numeric_high.replace('j', '1')
        numeric_high = numeric_high.replace('J', '1')
        numeric_high = numeric_high.replace('a', '1')
        numeric_high = numeric_high.replace('l', '1')
        numeric_high = numeric_high.replace('h', '1')
        numeric_high = numeric_high.replace(' ', '')
        number_low = int(numeric_low, 2)
        number_high = int(numeric_high, 2)

        print "{} ... {} => decode_{}(opcode),".format(number_low, number_high, INSTRUCTIONS[key])
    for value in sorted(INSTRUCTIONS.values()):
        print "#[allow(non_snake_case)]\n\
fn decode_{}(opcode: u32) -> Instruction {{ println!(\"{}\");\nInstruction::UDF {{imm32: 0,opcode: ThumbCode::from(opcode),}}}}\n".format(value,value)
    

main()
