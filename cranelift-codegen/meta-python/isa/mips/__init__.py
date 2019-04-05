"""
MIPS Target
-------------

Work-in-progress MIPS backend with support for Loongson instruction set
extensions.
"""
from __future__ import absolute_import
from . import defs
from . import encodings, settings, registers  # noqa
from cdsl.isa import TargetISA  # noqa

# Re-export the primary target ISA definition.
ISA = defs.ISA.finish()  # type: TargetISA
