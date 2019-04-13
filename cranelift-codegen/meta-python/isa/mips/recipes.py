"""
MIPS Encoding recipes.
"""
from __future__ import absolute_import
from cdsl.isa import EncRecipe
from cdsl.predicates import IsSignedInt
from cdsl.registers import Stack
from base.formats import Binary, BinaryImm, MultiAry, IntCompare, IntCompareImm
from base.formats import Unary, UnaryImm, BranchIcmp, Branch, Jump
from base.formats import Call, CallIndirect, RegMove
from .registers import GPR

# Encbits for the 32-bit recipes are (funct << 6) | opcode
# The functions below encode the encbits.


def OP(opcode):
    # type: (int, int) -> int
    return OPF(opcode, 0)


# Shorthand for REGIMM instruction encoding bits.
def OPRI(funct):
    # type: (int) -> int
    return OPF(0b000001, funct)


def OPF(opcode, funct):
    # type: (int, int) -> int
    assert opcode <= 0b111111
    assert funct <= 0b111111
    return (funct << 6) | opcode


# R-type with constant shamt.
R = EncRecipe(
        'R', Binary, base_size=4, ins=(GPR, GPR), outs=GPR,
        emit='put_r(bits, in_reg0, in_reg1, out_reg0, sink);')

# R-type for variable shifts where the two input registers should be swapped in position.
Rshift = EncRecipe(
        'Rshift', Binary, base_size=4, ins=(GPR, GPR), outs=GPR,
        emit='put_r(bits, in_reg1, in_reg0, out_reg0, sink);')

# R-type with an immediate shamt.
Rshamt = EncRecipe(
        'Rshamt', BinaryImm, base_size=4, ins=GPR, outs=GPR,
        emit='put_rshamt(bits, in_reg0, out_reg0, imm.into(), sink);')

# R-type encoding of an integer comparison.
Ricmp = EncRecipe(
        'Ricmp', IntCompare, base_size=4, ins=(GPR, GPR), outs=GPR,
        emit='put_r(bits, in_reg0, in_reg1, out_reg0, sink);')

# R-type encoding for `jr` as a return instruction.
Rret = EncRecipe(
        'Rret', MultiAry, base_size=4, ins=(), outs=(),
        emit='''
        // Return instructions are always a jalr to %x1.
        // The return address is provided as a special-purpose link argument.
        put_r(
            bits,
            31, // rs = %ra
            0,  // rt = unused
            0,  // rd = unused
            sink,
        );
        put_nop(sink);
        ''')

I = EncRecipe(
        'I', BinaryImm, base_size=4, ins=GPR, outs=GPR,
        instp=IsSignedInt(BinaryImm.imm, 16),
        emit='put_i(bits, in_reg0, out_reg0, imm.into(), sink);')

# I-type encoding of an integer comparison.
Iicmp = EncRecipe(
        'Iicmp', IntCompareImm, base_size=4, ins=GPR, outs=GPR,
        instp=IsSignedInt(IntCompareImm.imm, 16),
        emit='put_i(bits, in_reg0, out_reg0, imm.into(), sink);')

# I-type branch instructions with 18-bit offset.
Ic = EncRecipe(
        'Ic', BranchIcmp, base_size=4,
        ins=(GPR, GPR), outs=(),
        branch_range=(0, 18),
        emit='''
        let dest = i64::from(func.offsets[destination]);
        let disp = dest - i64::from(sink.offset());
        put_i(bits, in_reg0, in_reg1, disp, sink);
        put_nop(sink);
        ''')

# Ic implemented with REGIMM instructions.
Icr = EncRecipe(
        'Icr', Branch, base_size=4,
        ins=(GPR), outs=(),
        branch_range=(0, 18),
        emit='''
        let dest = i64::from(func.offsets[destination]);
        let disp = dest - i64::from(sink.offset());
        put_i_regimm(bits, in_reg0, disp, sink);
        put_nop(sink);
        ''')

# Icr for unconditional branches (rs=zero).
Icrz = EncRecipe(
        'Icrz', Jump, base_size=4,
        ins=(), outs=(),
        branch_range=(0, 18),
        emit='''
        let dest = i64::from(func.offsets[destination]);
        let disp = dest - i64::from(sink.offset());
        put_i_regimm(bits, 0, disp, sink);
        put_nop(sink);
        ''')

J = EncRecipe(
        'J', Jump, base_size=4, ins=(), outs=(), branch_range=(0, 26),
        emit='''
        let dest = i64::from(func.offsets[destination]);
        let disp = dest - i64::from(sink.offset());
        put_j(bits, disp, sink);
        put_nop(sink);
        ''')

# Spill of a GPR.
GPsp = EncRecipe(
        'GPsp', Unary, base_size=4,
        ins=GPR, outs=Stack(GPR),
        emit='unimplemented!();')

# Fill of a GPR.
GPfi = EncRecipe(
        'GPfi', Unary, base_size=4,
        ins=Stack(GPR), outs=GPR,
        emit='unimplemented!();')
