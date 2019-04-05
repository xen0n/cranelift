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

# Encbits for the 32-bit recipes are (shamt << 12) | (funct << 6) | opcode
# The functions below encode the encbits.


def OP(opcode):
    # type: (int, int) -> int
    return OPSF(opcode, 0, 0)


def OPF(opcode, funct):
    # type: (int, int) -> int
    return OPSF(opcode, 0, funct)


def OPSF(opcode, shamt, funct):
    # type: (int, int, int) -> int
    assert opcode <= 0b111111
    assert shamt <= 0b11111
    assert funct <= 0b111111
    return (shamt << 12) | (funct << 6) | opcode


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

J = EncRecipe(
        'J', Jump, base_size=4, ins=(), outs=(), branch_range=(0, 26),
        emit='unimplemented!();')

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
