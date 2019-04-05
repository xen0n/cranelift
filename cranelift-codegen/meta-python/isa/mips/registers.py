"""
MIPS register banks.
"""
from __future__ import absolute_import
from cdsl.registers import RegBank, RegClass
from .defs import ISA


# We include `zero` in the register bank. It will be reserved.
IntRegs = RegBank(
        'IntRegs', ISA,
        'General purpose registers',
        units=32,
        names='zero at v0 v1 a0 a1 a2 a3 a4 a5 a6 a7 t4 t5 t6 t7 s0 s1 s2 s3 s4 s5 s6 s7 t8 t9 k0 k1 gp sp s8 ra'.split())

FloatRegs = RegBank(
        'FloatRegs', ISA,
        'Floating point registers',
        units=32, prefix='f')

HiLoRegs = RegBank(
        'HiLoRegs', ISA,
        'HI/LO registers',
        units=2,
        names='hi lo'.split(),
        pressure_tracking=False)

GPR = RegClass(IntRegs)
FPR = RegClass(FloatRegs)
HILO = RegClass(HiLoRegs)

RegClass.extract_names(globals())
