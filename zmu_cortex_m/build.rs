use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

fn generate_decode(
    dest_path: &Path,
    func_name: &str,
    undefined_else: &str,
    instructions: &HashMap<&str, &str>,
    bits: u32,
) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(dest_path)?;

    //
    // finding a decoder match:
    // - go through the list of bitmasks in the order of specificity
    // - test first the ones that have most bits set
    // - first one to match is the one
    //

    // collect mask keys to string vector
    let mut maskstrings: Vec<&str> = instructions.keys().copied().collect();

    // sort by number of dots in the string
    maskstrings.sort_by_key(|a| a.matches(".").count());

    let onemasks: Vec<u32> = maskstrings
        .iter()
        .map(|key| {
            let key = key.replace("0", "1");
            let key = key.replace(".", "0");
            u32::from_str_radix(&key, 2).unwrap()
        })
        .collect();

    let resultmasks: Vec<u32> = maskstrings
        .iter()
        .map(|key| {
            let key = key.replace(".", "0");
            u32::from_str_radix(&key, 2).unwrap()
        })
        .collect();

    writeln!(file, "/// automatically generated decoder function")?;
    writeln!(file, "pub fn {} -> Instruction {{", func_name)?;

    for i in 0..onemasks.len() {
        let onemask = onemasks[i];
        let result = resultmasks[i];
        let instr = instructions[maskstrings[i]];
        if (bits == 32 && onemask == 0xffff_ffff) || (bits == 16 && onemask == 0xffff) {
            if bits == 32 {
                writeln!(
                    file,
                    "{} if opcode == 0x{:04x}_{:04x} {{ decode_{}(opcode)}}",
                    if i == 0 { "" } else { "else" },
                    result >> 16,    // high 16 bits
                    result & 0xffff, // low 16 bits
                    instr
                )?;
            } else {
                writeln!(
                    file,
                    "{} if opcode == 0x{:04x} {{ decode_{}(opcode)}}",
                    if i == 0 { "" } else { "else" },
                    result,
                    instr
                )?;
            }
        } else if bits == 32 {
            writeln!(
                file,
                "{} if (opcode & 0x{:04x}_{:04x}) == 0x{:04x}_{:04x} {{ decode_{}(opcode)}}",
                if i == 0 { "" } else { "else" },
                onemask >> 16,    // high 16 bits
                onemask & 0xffff, // low 16 bits
                result >> 16,     // high 16 bits
                result & 0xffff,  // low 16 bits
                instr
            )?;
        } else {
            writeln!(
                file,
                "{} if (opcode & 0x{:04x}) == 0x{:04x} {{ decode_{}(opcode)}}",
                if i == 0 { "" } else { "else" },
                onemask,
                result,
                instr
            )?;
        }
    }

    writeln!(file, "else {{ {} }}", undefined_else)?;
    writeln!(file, "}}")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let instructions_thumb32 = HashMap::from([
        ("11111110...1...............1....", "MRC2_t2"),
        ("11111110...0...............0....", "MCR2_t2"),
        ("11111110...................0....", "CDP2_t2"),
        ("111111000100....................", "MCRR2_t2"),
        ("1111110....11111................", "LDC2_lit_t2"),
        ("1111110....1....................", "LDC2_imm_t2"),
        ("1111110....0....................", "STC2_t2"),
        ("111110111110............0000....", "UMLAL_t1"),
        ("111110111100............0000....", "SMLAL_t1"),
        ("111110111011....1111....1111....", "UDIV_t1"),
        ("111110101000....1111....0100....", "UADD8_t1"),
        ("111110101010....1111....1000....", "SEL_t1"),
        ("111110111010............0000....", "UMULL_t1"),
        ("111110111001....1111....1111....", "SDIV_t1"),
        ("111110110001....1111....00......", "SMUL_t1"),
        ("111110111000............0000....", "SMULL_t1"),
        ("111110110000....1111....0000....", "MUL_t2"),
        ("111110110000............0001....", "MLS_t1"),
        ("111110110000............0000....", "MLA_t1"),
        ("111110110001............00......", "SMLA_t1"),
        ("111110101011....1111....1000....", "CLZ_t1"),
        ("111110101001....1111....1011....", "REVSH_t2"),
        ("111110101001....1111....1010....", "RBIT_t1"),
        ("111110101001....1111....1001....", "REV16_t2"),
        ("111110101001....1111....1000....", "REV_t2"),
        ("11111010011.....1111....0000....", "ROR_reg_t2"),
        ("11111010010111111111....10......", "UXTB_t2"),
        ("11111010010011111111....10......", "SXTB_t2"),
        ("11111010010.....1111....0000....", "ASR_reg_t2"),
        ("11111010001.....1111....0000....", "LSR_reg_t2"),
        ("11111010000111111111....10......", "UXTH_t2"),
        ("11111010000011111111....10......", "SXTH_t2"),
        ("111110100101....1111....10......", "UXTAB_t1"),
        ("11111010000.....1111....0000....", "LSL_reg_t2"),
        ("111110011011....................", "LDRSH_imm_t1"),
        ("111110011001....1111............", "PLI_lit_imm_t1"),
        ("111110011001....................", "LDRSB_imm_t1"),
        ("111110010011........1110........", "LDRSHT"),
        ("111110010011........1...........", "LDRSH_imm_t2"),
        ("111110010011........000000......", "LDRSH_reg_t2"),
        ("111110010001....11111100........", "PLI_lit_imm_t2"),
        ("111110010001....1111000000......", "PLI_reg_t1"),
        ("111110010001........1110........", "LDRSBT_t1"),
        ("111110010001........1...........", "LDRSB_imm_t2"),
        ("111110010001........000000......", "LDRSB_reg_t2"),
        ("11111001.0111111................", "LDRSH_lit_t1"),
        ("11111001.00111111111............", "PLI_lit_imm_t3"),
        ("11111001.0011111................", "LDRSB_lit_t1"),
        ("111110001101....................", "LDR_imm_t3"),
        ("111110001100....................", "STR_imm_t3"),
        ("111110001011....................", "LDRH_imm_t2"),
        ("111110001010....................", "STRH_imm_t2"),
        ("111110001001....1111............", "PLD_imm_t1"),
        ("11111000.00111111111............", "PLD_lit_t1"),
        ("111110000001....1111000000......", "PLD_reg_t1"),
        ("111110001001....................", "LDRB_imm_t2"),
        ("111110001000....................", "STRB_imm_t2"),
        ("1111100001011101....101100000100", "POP_t3"),
        ("111110000101........1110........", "LDRT_t1"),
        ("111110000101........1...........", "LDR_imm_t4"),
        ("111110000101........000000......", "LDR_reg_t2"),
        ("1111100001001101....110100000100", "PUSH_t3"),
        ("111110000100........1...........", "STR_imm_t4"),
        ("111110000100........000000......", "STR_reg_t2"),
        ("111110000011........1110........", "LDRHT_t1"),
        ("111110000011........1...........", "LDRH_imm_t3"),
        ("111110000011........000000......", "LDRH_reg_t2"),
        ("111110000010........1...........", "STRH_imm_t3"),
        ("111110000001....11111100........", "PLD_imm_t2"),
        ("111110000001........1110........", "LDRBT_t1"),
        ("111110000001........1...........", "LDRB_imm_t3"),
        ("111110000001........00000.......", "LDRB_reg_t2"),
        ("111110000010........000000......", "STRH_reg_t2"),
        ("111110000000........1...........", "STRB_imm_t3"),
        ("111110000000........000000......", "STRB_reg_t2"),
        ("11111000.1011111................", "LDR_lit_t2"),
        ("11111000.0111111................", "LDRH_lit_t1"),
        ("11111000.0011111................", "LDRB_lit_t1"),
        ("111101111111....1010............", "UDF_t2"),
        ("11110011111011111000............", "MRS_t1"),
        ("111100111100....0.........0.....", "UBFX_t1"),
        ("111100110100....0.........0.....", "SBFX_t1"),
        ("1111001110111111100011110110....", "ISB_t1"),
        ("1111001110111111100011110101....", "DMB_t1"),
        ("1111001110111111100011110100....", "DSB_t1"),
        ("1111001110111111100011110010....", "CLREX_t1"),
        ("11110011101011111000000011110000", "DBG_t1"),
        ("11110011101011111000000000000100", "SEV_t2"),
        ("11110011101011111000000000000011", "WFI_t2"),
        ("11110011101011111000000000000010", "WFE_t2"),
        ("11110011101011111000000000000001", "YIELD_t2"),
        ("11110011101011111000000000000000", "NOP_t2"),
        ("111100111000....10001000........", "MSR_reg_t1"),
        ("1111001110.0....0.........0.....", "USAT_t1"),
        ("11110011011011110.........0.....", "BFC_t1"),
        ("111100110110....0.........0.....", "BFI_t1"),
        ("111100110100....0.........0.....", "SBFX_t1"),
        ("1111001100.0....0.........0.....", "SSAT_t1"),
        ("11110.101100....0...............", "MOVT_t1"),
        ("11110.10101011110...............", "ADR_t2"),
        ("11110.101010....0...............", "SUB_imm_t4"),
        ("11110.100100....0...............", "MOV_imm_t3"),
        ("11110.10000011110...............", "ADR_t3"),
        ("11110.100000....0...............", "ADD_imm_t4"),
        ("11110.01110.....0...............", "RSB_imm_t2"),
        ("11110.011011....0...1111........", "CMP_imm_t2"),
        ("11110.01101.....0...............", "SUB_imm_t3"),
        ("11110.01011.....0...............", "SBC_imm_t1"),
        ("11110.01101.11010...............", "SUB_SP_imm_t2"),
        ("11110.10101011010...............", "SUB_SP_imm_t3"),
        ("11110.01010.....0...............", "ADC_imm_t1"),
        ("11110.010001....0...1111........", "CMN_imm_t1"),
        ("11110.01000.....0...............", "ADD_imm_t3"),
        ("11110.001001....0...1111........", "TEQ_imm_t1"),
        ("11110.00100.....0...............", "EOR_imm_t1"),
        ("11110.00011.11110...............", "MVN_imm_t1"),
        ("11110.00011.....0...............", "ORN_imm_t1"),
        ("11110.00010.....0...............", "ORR_imm_t1"),
        ("11110.00010.11110...............", "MOV_imm_t2"),
        ("11110.000001....0...1111........", "TST_imm_t1"),
        ("11110.00001.....0...............", "BIC_imm_t1"),
        ("11110.00000.....0...............", "AND_imm_t1"),
        ("11110...........11.1............", "BL_t1"),
        ("11110...........10.1............", "B_t4"),
        ("11110...........10.0............", "B_t3"),
        ("11101110...1...............1....", "MRC_t1"),
        ("11101110...0...............0....", "MCR_t1"),
        ("11101110...................0....", "CDP_t1"),
        ("111011000100....................", "MCRR_t1"),
        ("1110110....11111................", "LDC_lit_t1"),
        ("1110110....1....................", "LDC_imm_t1"),
        ("1110110....0....................", "STC_t1"),
        ("11101011110.....0...............", "RSB_reg_t1"),
        ("111010111011....0...1111........", "CMP_reg_t3"),
        ("11101011101.....0...............", "SUB_reg_t2"),
        ("11101011011.....0...............", "SBC_reg_t2"),
        ("11101011010.....0...............", "ADC_reg_t2"),
        ("111010110001....0...1111........", "CMN_reg_t2"),
        ("11101011000.....0...............", "ADD_reg_t3"),
        ("111010101001....0...1111........", "TEQ_reg_t1"),
        ("11101010100.....0...............", "EOR_reg_t2"),
        ("11101010011.11110...............", "MVN_reg_t2"),
        ("11101010011.....0...............", "ORN_reg_t1"),
        ("11101010010.11110000....0011....", "RRX_t1"),
        ("11101010010.11110000....0000....", "MOV_reg_t3"),
        ("11101010010.11110.........11....", "ROR_imm_t1"),
        ("11101010010.11110.........10....", "ASR_imm_t2"),
        ("11101010010.11110.........01....", "LSR_imm_t2"),
        ("11101010010.11110.........00....", "LSL_imm_t2"),
        ("11101010010.....0...............", "ORR_reg_t2"),
        ("11101010001.....0...............", "BIC_reg_t2"),
        ("111010100001....0...1111........", "TST_reg_t2"),
        ("11101010000.....0...............", "AND_reg_t2"),
        ("11101001001011010.0.............", "PUSH_t2"),
        ("1110100100.1......0.............", "LDMDB_t1"),
        ("1110100100.0....0.0.............", "STMDB_t1"),
        ("111010001101....111100000001....", "TBH_t1"),
        ("111010001101....111100000000....", "TBB_t1"),
        ("111010001101........111101011111", "LDREXH_t1"),
        ("111010001101........111101001111", "LDREXB_t1"),
        ("111010001100........11110101....", "STREXH_t1"),
        ("111010001100........11110100....", "STREXB_t1"),
        ("1110100010111101..0.............", "POP_t2"),
        ("1110100010.1......0.............", "LDM_t2"),
        ("1110100010.0....0.0.............", "STM_t2"),
        ("111010000101........1111........", "LDREX_t1"),
        ("111010000100....................", "STREX_t1"),
        ("1110100..1.11111................", "LDRD_lit_t1"),
        ("1110100..1.1....................", "LDRD_imm_t1"),
        ("1110100..1.0....................", "STRD_imm_t1"),
        ("111011101.110000....101.11.0....", "VABS_t1"),
        ("111011100.11........101..0.0....", "VADD_t1"),
        ("111011100.11........101..1.0....", "VSUB_t1"),
        ("111011101.110100....101..1.0....", "VCMP_t1"),
        ("111011101.110101....101..1.0....", "VCMP_t2"),
        ("111011101.111.......101..1.0....", "VCVT_t1"),
        //("111011101.111.1.....101..1.0....": "VCVT_fx_t1"),
        //("111011101.110111....101.11.0....": "VCVT_ds_t1"),
        //("111011101.11001.....101..1.0....": "VCVTB"),
        //("111011101.00........101..0.0....": "VDIV"),
        //("111011101.10........101....0....": "VFMAS"),
        //("111011101.01........101....0....": "VFNMAS"),
        ("1110111011110001....101000010000", "VMRS"),
        ("1110110....0........1011.......0", "VSTM_t1"),
        ("1110110....0........1010........", "VSTM_t2"),
        ("11101101..01........1011........", "VLDR_t1"),
        ("11101101..01........1010........", "VLDR_t2"),
        ("111011010.101101....1011........", "VPUSH_t1"),
        ("111011010.101101....1010........", "VPUSH_t2"),
        ("111011001.111101....1011........", "VPOP_t1"),
        ("111011001.111101....1010........", "VPOP_t2"),
        ("11101101..00........1011........", "VSTR_t1"),
        ("11101101..00........1010........", "VSTR_t2"),
        ("111011101.11........101.0000....", "VMOV_imm"),
        ("111011101.110000....101.01.0....", "VMOV_reg"),
        ("1110111000.0........1011.0010000", "VMOV_cr_scalar"),
        ("1110111000.1........1011.0010000", "VMOV_scalar_cr"),
        ("11101110000.........1010.0010000", "VMOV_cr_sp"),
        ("11101100010.........101000.1....", "VMOV_cr2_sp2"),
        ("11101100010.........101100.1....", "VMOV_cr2_dp"),
    ]);

    let instructions_thumb16 = HashMap::from([
        ("00000...........", "MOV_reg_t2_LSL_imm_t1"),
        ("00001...........", "LSR_imm_t1"),
        ("00010...........", "ASR_imm_t1"),
        ("0001100.........", "ADD_reg_t1"),
        ("0001101.........", "SUB_reg_t1"),
        ("0001110.........", "ADD_imm_t1"),
        ("0001111.........", "SUB_imm_t1"),
        ("00100...........", "MOV_imm_t1"),
        ("00101...........", "CMP_imm_t1"),
        ("00110...........", "ADD_imm_t2"),
        ("00111...........", "SUB_imm_t2"),
        ("0100000000......", "AND_reg_t1"),
        ("0100000001......", "EOR_reg_t1"),
        ("0100000010......", "LSL_reg_t1"),
        ("0100000011......", "LSR_reg_t1"),
        ("0100000100......", "ASR_reg_t1"),
        ("0100000101......", "ADC_reg_t1"),
        ("0100000110......", "SBC_reg_t1"),
        ("0100000111......", "ROR_reg_t1"),
        ("0100001000......", "TST_reg_t1"),
        ("0100001001......", "RSB_imm_t1"),
        ("0100001010......", "CMP_reg_t1"),
        ("0100001011......", "CMN_reg_t1"),
        ("0100001100......", "ORR_reg_t1"),
        ("0100001101......", "MUL_t1"),
        ("0100001110......", "BIC_reg_t1"),
        ("0100001111......", "MVN_reg_t1"),
        ("01000100........", "ADD_reg_t2"),
        ("01000100.1101...", "ADD_reg_sp_t1"),
        ("010001001....101", "ADD_reg_sp_t2"),
        ("01000101........", "CMP_reg_t2"),
        ("01000110........", "MOV_reg_t1"),
        ("010001110....000", "BX_t1"),
        ("010001111....000", "BLX_t1"),
        ("01001...........", "LDR_lit_t1"),
        ("0101000.........", "STR_reg_t1"),
        ("0101001.........", "STRH_reg_t1"),
        ("0101010.........", "STRB_reg_t1"),
        ("0101011.........", "LDRSB_reg_t1"),
        ("0101100.........", "LDR_reg_t1"),
        ("0101101.........", "LDRH_reg_t1"),
        ("0101110.........", "LDRB_reg_t1"),
        ("0101111.........", "LDRSH_reg_t1"),
        ("01100...........", "STR_imm_t1"),
        ("01101...........", "LDR_imm_t1"),
        ("01110...........", "STRB_imm_t1"),
        ("01111...........", "LDRB_imm_t1"),
        ("10000...........", "STRH_imm_t1"),
        ("10001...........", "LDRH_imm_t1"),
        ("10010...........", "STR_imm_t2"),
        ("10011...........", "LDR_imm_t2"),
        ("10100...........", "ADR_t1"),
        ("10101...........", "ADD_SP_imm_t1"),
        ("101100000.......", "ADD_SP_imm_t2"),
        ("101100001.......", "SUB_SP_imm_t1"),
        ("1011001000......", "SXTH_t1"),
        ("1011001001......", "SXTB_t1"),
        ("1011001010......", "UXTH_t1"),
        ("1011001011......", "UXTB_t1"),
        ("1011010.........", "PUSH_t1"),
        ("10110110011.0010", "CPS_t1"),
        ("1011.0.1........", "CBZ_t1"),
        ("1011101000......", "REV_t1"),
        ("1011101001......", "REV16_t1"),
        ("1011101011......", "REVSH_t1"),
        ("1011110.........", "POP_reg_t1"),
        ("10111110........", "BKPT_t1"),
        ("10111111........", "IT_t1"),
        ("1011111100000000", "NOP_t1"),
        ("1011111100010000", "YIELD_t1"),
        ("1011111100100000", "WFE_t1"),
        ("1011111100110000", "WFI_t1"),
        ("1011111101000000", "SEV_t1"),
        ("11000...........", "STM_t1"),
        ("11001...........", "LDM_t1"),
        ("1101............", "B_t1_SVC_t1"),
        ("11100...........", "B_t2"),
    ]);

    let dest_path_16 = Path::new(&out_dir).join("decode_16.rs");
    let dest_path_32 = Path::new(&out_dir).join("decode_32.rs");

    generate_decode(
        &dest_path_32,
        "decode_32(opcode: u32)",
        "decode_UDF_t2(opcode)",
        &instructions_thumb32,
        32,
    )?;
    generate_decode(
        &dest_path_16,
        "decode_16(opcode: u16)",
        "decode_undefined(opcode)",
        &instructions_thumb16,
        16,
    )?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
