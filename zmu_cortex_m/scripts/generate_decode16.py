""" Module for autogenerating ARMv6-m decoder trees"""

INSTRUCTIONS = {
    '00000nnnnnmmmddd': 'MOV_reg_t2_LSL_imm_t1',
    '00001iiiiimmmddd': 'LSR_imm_t1',
    '00010iiiiimmmddd': 'ASR_imm_t1',
    '0001100mmmnnnddd': 'ADD_reg_t1',
    '0001101mmmnnnddd': 'SUB_reg_t1',
    '0001110iiinnnddd': 'ADD_imm_t1',
    '0001111iiinnnddd': 'SUB_imm_t1',
    '00100dddiiiiiiii': 'MOV_imm_t1',
    '00101nnniiiiiiii': 'CMP_imm_t1',
    '00110dddiiiiiiii': 'ADD_imm_t2',
    '00111dddiiiiiiii': 'SUB_imm_t2',
    '0100000000mmmddd': 'AND_reg_t1',
    '0100000001mmmddd': 'EOR_reg_t1',
    '0100000010mmmddd': 'LSL_reg_t1',
    '0100000011mmmddd': 'LSR_reg_t1',
    '0100000100mmmddd': 'ASR_reg_t1',
    '0100000101mmmddd': 'ADC_reg_t1',
    '0100000110mmmddd': 'SBC_reg_t1',
    '0100000111mmmddd': 'ROR_reg_t1',
    '0100001000mmmnnn': 'TST_reg_t1',
    '0100001001nnnddd': 'RSB_imm_t1',
    '0100001010mmmnnn': 'CMP_reg_t1',
    '0100001011mmmnnn': 'CMN_reg_t1',
    '0100001100mmmddd': 'ORR_reg_t1',
    '0100001101nnnddd': 'MUL_t1',
    '0100001110mmmddd': 'BIC_reg_t1',
    '0100001111mmmddd': 'MVN_reg_t1',
    '01000100dmmmmddd': 'ADD_reg_t2_ADD_SP_reg',
    '01000101Nmmmmnnn': 'CMP_reg_t2',
    '01000110Dmmmmddd': 'MOV_reg_t1',
    '010001110mmmm000': 'BX_t1',
    '010001111mmmm000': 'BLX_t1',
    '01001tttiiiiiiii': 'LDR_lit_t1',
    '0101000mmmnnnttt': 'STR_reg_t1',
    '0101001mmmnnnttt': 'STRH_reg_t1',
    '0101010mmmnnnttt': 'STRB_reg_t1',
    '0101011mmmnnnttt': 'LDRSB_reg_t1',
    '0101100mmmnnnttt': 'LDR_reg_t1',
    '0101101mmmnnnttt': 'LDRH_reg_t1',
    '0101110mmmnnnttt': 'LDRB_reg_t1',
    '0101111mmmnnnttt': 'LDRSH_reg_t1',
    '01100iiiiinnnttt': 'STR_imm_t1',
    '01101iiiiinnnttt': 'LDR_imm_t1',
    '01110iiiiinnnttt': 'STRB_imm_t1',
    '01111iiiiinnnttt': 'LDRB_imm_t1',
    '10000iiiiinnnttt': 'STRH_imm_t1',
    '10001iiiiinnnttt': 'LDRH_imm_t1',
    '10010tttiiiiiiii': 'STR_imm_t2',
    '10011tttiiiiiiii': 'LDR_imm_t2',
    '10100dddiiiiiiii': 'ADR_t1',
    '10101dddiiiiiiii': 'ADD_SP_imm_t1',
    '101100000iiiiiii': 'ADD_SP_imm_t2',
    '101100001iiiiiii': 'SUB_SP_imm_t1',
    '1011001000mmmddd': 'SXTH_t1',
    '1011001001mmmddd': 'SXTB_t1',
    '1011001010mmmddd': 'UXTH_t1',
    '1011001011mmmddd': 'UXTB_t1',
    '1011010mrrrrrrrr': 'PUSH_t1',
    '10110110011i0010': 'CPS_t1',
    '1011101000mmmddd': 'REV_t1',
    '1011101001mmmddd': 'REV16_t1',
    '1011101011mmmddd': 'REVSH_t1',
    '1011110prrrrrrrr': 'POP_reg_t1',
    '10111110iiiiiiii': 'BKPT_t1',
    '1011111100000000': 'NOP_t1',
    '1011111100010000': 'YIELD_t1',
    '1011111100100000': 'WFE_t1',
    '1011111100110000': 'WFI_t1',
    '1011111101000000': 'SEV_t1',
    '11000nnnrrrrrrrr': 'STM_t1',
    '11001nnnrrrrrrrr': 'LDM_t1',
    '1101cccciiiiiiii': 'B_t1_SVC_t1',
    '11100iiiiiiiiiii': 'B_t2',
}


def main():
    """ My main function"""
    for key in sorted(INSTRUCTIONS.iterkeys()):
        numeric_low = key.replace('m', '0')
        numeric_low = numeric_low.replace('t', '0')
        numeric_low = numeric_low.replace('i', '0')
        numeric_low = numeric_low.replace('r', '0')
        numeric_low = numeric_low.replace('d', '0')
        numeric_low = numeric_low.replace('c', '0')
        numeric_low = numeric_low.replace('n', '0')
        numeric_low = numeric_low.replace('D', '0')
        numeric_low = numeric_low.replace('N', '0')
        numeric_low = numeric_low.replace('p', '0')

        numeric_high = key.replace('m', '1')
        numeric_high = numeric_high.replace('t', '1')
        numeric_high = numeric_high.replace('i', '1')
        numeric_high = numeric_high.replace('r', '1')
        numeric_high = numeric_high.replace('d', '1')
        numeric_high = numeric_high.replace('c', '1')
        numeric_high = numeric_high.replace('n', '1')
        numeric_high = numeric_high.replace('D', '1')
        numeric_high = numeric_high.replace('N', '1')
        numeric_high = numeric_high.replace('p', '1')
        number_low = int(numeric_low, 2)
        number_high = int(numeric_high, 2)
        
        print "{} ... {} => decode_{}(opcode),".format(number_low, number_high, INSTRUCTIONS[key])

# MRS_t1,        "1111001111101111", "1000ddddssssssss" # 32 bit
# MSR_reg_t1,    "111100111000nnnn", "10001000ssssssss" # 32 bit
# ISB,           "1111001110111111", "100011110110oooo" # 32 bit
# BL,            "11110siiiiiiiiii", "11j1Jiiiiiiiiiii" #32 bit
# DMB_t1,        "1111001110111111", "100011110101oooo" # 32 bit
# DSB_t1,        "1111001110111111", "100011110100oooo" # 32 bit
# ISB_t1,        "1111001110111111", "100011110110oooo" # 32 bit
# UDF_t2,        "111101111111iiii", "1010iiiiiiiiiiii",
