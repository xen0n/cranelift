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

# R-type with an immediate shamt.
Rshamt = EncRecipe(
        'Rshamt', BinaryImm, base_size=4, ins=GPR, outs=GPR,
        emit='put_rshamt(bits, in_reg0, out_reg0, imm.into(), sink);')

I = EncRecipe(
        'I', BinaryImm, base_size=4, ins=GPR, outs=GPR,
        instp=IsSignedInt(BinaryImm.imm, 16),
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
        ''')

J = EncRecipe(
        'J', Jump, base_size=4, ins=(), outs=(), branch_range=(0, 26),
        emit='''
        let dest = i64::from(func.offsets[destination]);
        let disp = dest - i64::from(sink.offset());
        put_j(bits, disp, sink);
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
