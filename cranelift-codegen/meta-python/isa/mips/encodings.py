"""
MIPS Encodings.
"""
from __future__ import absolute_import
from base import instructions as base
from base.immediates import intcc
from cdsl.ast import Var
from .defs import MIPS32, MIPS64
from .recipes import OP, OPF, OPRI
from .recipes import R, Ricmp, Rshift, Rshamt, Rret, I, Iicmp, Ic, Icz, Icr, Icrz, J
from base.legalize import narrow, expand


MIPS32.legalize_monomorphic(expand)
MIPS32.legalize_type(
        default=narrow,
        i32=expand,
        f32=expand,
        f64=expand)

MIPS64.legalize_monomorphic(expand)
MIPS64.legalize_type(
        default=narrow,
        i32=expand,
        i64=expand,
        f32=expand,
        f64=expand)

# Dummies for instruction predicates.
x = Var('x')
y = Var('y')
dest = Var('dest')
args = Var('args')

MIPS32.enc(base.iadd.i32,     R, OPF(0b000000, 0b100001))  # ADDU
MIPS64.enc(base.iadd.i64,     R, OPF(0b000000, 0b101101))  # DADDU
MIPS32.enc(base.iadd_imm.i32, I, OP (0b001001))            # ADDIU
MIPS64.enc(base.iadd_imm.i32, I, OP (0b001001))
MIPS64.enc(base.iadd_imm.i64, I, OP (0b011001))            # DADDIU
MIPS32.enc(base.isub.i32,     R, OPF(0b000000, 0b100011))  # SUBU
MIPS64.enc(base.isub.i64,     R, OPF(0b000000, 0b101111))  # DSUBU

MIPS32.enc(base.band.i32,     R, OPF(0b000000, 0b100100))  # AND
MIPS64.enc(base.band.i64,     R, OPF(0b000000, 0b100100))
MIPS32.enc(base.band_imm.i32, I, OP (0b001100))            # ANDI
MIPS64.enc(base.band_imm.i64, I, OP (0b001100))
MIPS32.enc(base.bor.i32,      R, OPF(0b000000, 0b100101))  # OR
MIPS64.enc(base.bor.i64,      R, OPF(0b000000, 0b100101))
MIPS32.enc(base.bor_imm.i32,  I, OP (0b001101))            # ORI
MIPS64.enc(base.bor_imm.i64,  I, OP (0b001101))
MIPS32.enc(base.bor_not.i32,  R, OPF(0b000000, 0b100111))  # NOR
MIPS64.enc(base.bor_not.i64,  R, OPF(0b000000, 0b100111))
MIPS32.enc(base.bxor.i32,     R, OPF(0b000000, 0b100110))  # XOR
MIPS64.enc(base.bxor.i64,     R, OPF(0b000000, 0b100110))
MIPS32.enc(base.bxor_imm.i32, I, OP (0b001110))            # XORI
MIPS64.enc(base.bxor_imm.i64, I, OP (0b001110))

# Dynamic shifts have the same masking semantics as the clif base instructions.
MIPS32.enc(base.ishl.i32.i32, Rshift, OPF(0b000000, 0b000100))  # SLLV
MIPS64.enc(base.ishl.i32.i32, Rshift, OPF(0b000000, 0b000100))
MIPS64.enc(base.ishl.i64.i64, Rshift, OPF(0b000000, 0b010100))  # DSLLV
MIPS32.enc(base.ishl_imm.i32, Rshamt, OPF(0b000000, 0b000000))  # SLL
MIPS64.enc(base.ishl_imm.i32, Rshamt, OPF(0b000000, 0b000000))
MIPS64.enc(base.ishl_imm.i64, Rshamt, OPF(0b000000, 0b111000))  # DSLL
MIPS32.enc(base.ushr.i32.i32, Rshift, OPF(0b000000, 0b000110))  # SRLV
MIPS64.enc(base.ushr.i32.i32, Rshift, OPF(0b000000, 0b000110))
MIPS64.enc(base.ushr.i64.i64, Rshift, OPF(0b000000, 0b010110))  # DSRLV
MIPS32.enc(base.ushr_imm.i32, Rshamt, OPF(0b000000, 0b000010))  # SRL
MIPS64.enc(base.ushr_imm.i32, Rshamt, OPF(0b000000, 0b000010))
MIPS64.enc(base.ushr_imm.i64, Rshamt, OPF(0b000000, 0b111010))  # DSRL
MIPS32.enc(base.sshr.i32.i32, Rshift, OPF(0b000000, 0b000111))  # SRAV
MIPS64.enc(base.sshr.i32.i32, Rshift, OPF(0b000000, 0b000111))
MIPS64.enc(base.sshr.i64.i64, Rshift, OPF(0b000000, 0b010111))  # DSRAV
MIPS32.enc(base.sshr_imm.i32, Rshamt, OPF(0b000000, 0b000011))  # SRA
MIPS64.enc(base.sshr_imm.i32, Rshamt, OPF(0b000000, 0b000011))
MIPS64.enc(base.sshr_imm.i64, Rshamt, OPF(0b000000, 0b111011))  # DSRA

# Signed and unsigned integer 'less than'. There are no 'w' variants for
# comparing 32-bit numbers in MIPS64.
MIPS32.enc(base.icmp.i32(intcc.slt, x, y),     Ricmp, OPF(0b000000, 0b101010))  # SLT
MIPS64.enc(base.icmp.i64(intcc.slt, x, y),     Ricmp, OPF(0b000000, 0b101010))
MIPS32.enc(base.icmp.i32(intcc.ult, x, y),     Ricmp, OPF(0b000000, 0b000000))  # SLTU
MIPS64.enc(base.icmp.i64(intcc.ult, x, y),     Ricmp, OPF(0b000000, 0b000000))

MIPS32.enc(base.icmp_imm.i32(intcc.slt, x, y), Iicmp, OP (0b001010))  # SLTI
MIPS64.enc(base.icmp_imm.i64(intcc.slt, x, y), Iicmp, OP (0b001010))
MIPS32.enc(base.icmp_imm.i32(intcc.ult, x, y), Iicmp, OP (0b001011))  # SLTIU
MIPS64.enc(base.icmp_imm.i64(intcc.ult, x, y), Iicmp, OP (0b001011))

# Control flow.

# Unconditional branches.
MIPS32.enc(base.jump, Icrz, OPRI(0b00000))   # B
MIPS64.enc(base.jump, Icrz, OPRI(0b00000))
MIPS32.enc(base.jump, J,    OP  (0b000010))  # J
MIPS64.enc(base.jump, J,    OP  (0b000010))
# TODO
# MIPS32.enc(base.call, Icrz, OPRI(0b10001))   # BAL = BGEZAL(rs=0)
# MIPS64.enc(base.call, Icrz, OPRI(0b10001))
# MIPS32.enc(base.call, J,    OP  (0b000011))  # JAL
# MIPS64.enc(base.call, J,    OP  (0b000011))

# Conditional branches.
MIPS32.enc(base.br_icmp.i32(intcc.eq, x, y, dest, args), Ic, OP(0b000100))  # BEQ
MIPS64.enc(base.br_icmp.i64(intcc.eq, x, y, dest, args), Ic, OP(0b000100))
MIPS32.enc(base.br_icmp.i32(intcc.ne, x, y, dest, args), Ic, OP(0b000101))  # BNE
MIPS64.enc(base.br_icmp.i64(intcc.ne, x, y, dest, args), Ic, OP(0b000101))

for inst,           op in [
        (base.brz,  0b000100),  # BEQ rs, zero
        (base.brnz, 0b000101),  # BNE rs, zero
        ]:
    MIPS32.enc(inst.i32, Icz, OP(op))
    MIPS64.enc(inst.i64, Icz, OP(op))
    MIPS32.enc(inst.b1, Icz, OP(op))
    MIPS64.enc(inst.b1, Icz, OP(op))

# Returns are just `jr $ra`'s in MIPS.
# The return address is provided by a special-purpose `link` return value that
# is added by legalize_signature().
MIPS32.enc(base.x_return,          Rret,  OPF(0b000000, 0b001000))  # JR
MIPS64.enc(base.x_return,          Rret,  OPF(0b000000, 0b001000))
#MIPS32.enc(base.call_indirect.i32, Rcall, OPF(0b000000, 0b001000))
#MIPS64.enc(base.call_indirect.i64, Rcall, OPF(0b000000, 0b001000))
