"""
MIPS settings.
"""
from __future__ import absolute_import
from cdsl.settings import SettingGroup, BoolSetting
from cdsl.predicates import And
import base.settings as shared
from .defs import ISA

ISA.settings = SettingGroup('mips', parent=shared.group)

supports_lext = BoolSetting("CPU supports the Loongson EXT ASE")

enable_lext = BoolSetting(
        "Enable the use of Loongson EXT instructions if available",
        default=True)

use_lext = And(supports_lext, enable_lext)

ISA.settings.close(globals())
