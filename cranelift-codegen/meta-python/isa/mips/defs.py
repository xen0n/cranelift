"""
MIPS definitions.

Commonly used definitions.
"""
from __future__ import absolute_import
from cdsl.isa import TargetISA, CPUMode
import base.instructions

ISA = TargetISA('mips', [base.instructions.GROUP])  # type: TargetISA

# CPU modes for 32-bit and 64-bit operation.
MIPS32 = CPUMode('MIPS32', ISA)
MIPS64 = CPUMode('MIPS64', ISA)
