"""
MIPS Encodings.
"""
from __future__ import absolute_import
from base import instructions as base
from base.immediates import intcc
from .defs import MIPS32, MIPS64
from .recipes import OP, OPF
from .recipes import R, I
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

MIPS32.enc(base.iadd.i32,     R, OPF(0b000000, 0b100001)) # ADDU
MIPS32.enc(base.iadd_imm.i32, I, OP (0b001001))           # ADDIU
MIPS32.enc(base.isub.i32,     R, OPF(0b000000, 0b100011)) # SUBU

MIPS32.enc(base.band.i32, R, OPF(0b000000, 0b100100))  # AND
MIPS64.enc(base.band.i64, R, OPF(0b000000, 0b100100))
MIPS32.enc(base.bor.i32,  R, OPF(0b000000, 0b100101))  # OR
MIPS64.enc(base.bor.i64,  R, OPF(0b000000, 0b100101))
MIPS32.enc(base.bxor.i32, R, OPF(0b000000, 0b100110))  # XOR
MIPS64.enc(base.bxor.i64, R, OPF(0b000000, 0b100110))

MIPS64.enc(base.iadd.i64,     R, OPF(0b000000, 0b101101))  # DADDU
MIPS64.enc(base.iadd_imm.i64, I, OP (0b011001))            # DADDIU
MIPS64.enc(base.isub.i64,     R, OPF(0b000000, 0b101111))  # DSUBU
