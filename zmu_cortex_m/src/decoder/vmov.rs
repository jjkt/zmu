use crate::core::{
    bits::Bits,
    instruction::{
        Instruction, VMovCr2DpParams, VMovCrSpParams, VMovRegParamsf32, VMovRegParamsf64,
    },
    register::{DoubleReg, Reg, SingleReg},
};

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VMOV_imm(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VMOV_reg(opcode: u32) -> Instruction {
    let sz = opcode.get_bit(8);

    let vm = opcode.get_bits(0..4) as u8;
    let M = opcode.get_bit(5) as u8;
    let vd = opcode.get_bits(12..16) as u8;
    let D = opcode.get_bit(22) as u8;

    let dp_operation = sz;

    let d = if dp_operation {
        D << 4 | vd
    } else {
        vd << 1 | D
    };

    let m = if dp_operation {
        M << 4 | vm
    } else {
        vm << 1 | M
    };

    if dp_operation {
        return Instruction::VMOV_reg_f64 {
            params: VMovRegParamsf64 {
                dm: DoubleReg::from(m),
                dd: DoubleReg::from(d),
            },
        };
    }
    Instruction::VMOV_reg_f32 {
        params: VMovRegParamsf32 {
            sd: SingleReg::from(d),
            sm: SingleReg::from(m),
        },
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VMOV_cr_scalar(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VMOV_scalar_cr(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VMOV_cr_sp(opcode: u32) -> Instruction {
    let rt = Reg::from(opcode.get_bits(12..16) as u8);

    // rt 15 and 13 are unpredictable:
    if rt == Reg::PC || rt == Reg::SP {
        return Instruction::UDF {
            opcode: opcode.into(),
            thumb32: true,
            imm32: 0,
        };
    }

    let N = opcode.get_bit(7);
    let vn = (opcode.get_bits(16..20) as u8) << 1;
    let sn = SingleReg::from(vn | (N as u8));
    Instruction::VMOV_cr_sp {
        params: VMovCrSpParams {
            to_arm_register: opcode.get_bit(20),
            rt,
            sn,
        },
    }
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VMOV_cr2_sp2(_opcode: u32) -> Instruction {
    unimplemented!()
}

#[allow(non_snake_case)]
#[inline(always)]
pub fn decode_VMOV_cr2_dp(opcode: u32) -> Instruction {
    let rt = Reg::from(opcode.get_bits(12..16) as u8);
    let rt2 = Reg::from(opcode.get_bits(16..20) as u8);

    // rt&rt2 15 and 13 are unpredictable:
    if rt == Reg::PC || rt == Reg::SP {
        return Instruction::UDF {
            opcode: opcode.into(),
            thumb32: true,
            imm32: 0,
        };
    }

    if rt2 == Reg::PC || rt2 == Reg::SP {
        return Instruction::UDF {
            opcode: opcode.into(),
            thumb32: true,
            imm32: 0,
        };
    }
    let op = opcode.get_bit(20);
    if op && rt == rt2 {
        return Instruction::UDF {
            opcode: opcode.into(),
            thumb32: true,
            imm32: 0,
        };
    }

    let M = opcode.get_bit(5);
    let vm = opcode.get_bits(0..4) as u8;
    let m = ((M as u8) << 4) + vm;

    let dm = DoubleReg::from(m);

    Instruction::VMOV_cr2_dp {
        params: VMovCr2DpParams {
            to_arm_registers: op,
            rt,
            rt2,
            dm,
        },
    }
}
