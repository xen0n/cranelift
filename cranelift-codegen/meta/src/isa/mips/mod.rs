use crate::cdsl::cpu_modes::CpuMode;
use crate::cdsl::inst::InstructionGroup;
use crate::cdsl::isa::TargetIsa;
use crate::cdsl::regs::{IsaRegs, IsaRegsBuilder, RegBankBuilder, RegClassBuilder};
use crate::cdsl::settings::{PredicateNode, SettingGroup, SettingGroupBuilder};

use crate::shared::types::Float::{F32, F64};
use crate::shared::types::Int::{I32, I64};
use crate::shared::Definitions as SharedDefinitions;

fn define_settings(shared: &SettingGroup) -> SettingGroup {
    let mut setting = SettingGroupBuilder::new("mips");

    let supports_lext = setting.add_bool(
        "supports_lext",
        "CPU supports the Loongson EXT ASE",
        false,
    );

    let enable_lext = setting.add_bool(
        "enable_lext",
        "Enable the use of Loongson EXT instructions if available",
        true,
    );

    setting.add_predicate("use_lext", predicate!(supports_lext && enable_lext));

    setting.finish()
}

fn define_registers() -> IsaRegs {
    let mut regs = IsaRegsBuilder::new();

    let builder = RegBankBuilder::new("IntRegs", "")
        .units(32)
        .names(vec![
               "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3",
               "a4", "a5", "a6", "a7", "t4", "t5", "t6", "t7",
               "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
               "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
        ])
        .track_pressure(true);
    let int_regs = regs.add_bank(builder);

    let builder = RegBankBuilder::new("FloatRegs", "f")
        .units(32)
        .track_pressure(true);
    let float_regs = regs.add_bank(builder);

    let builder = RegBankBuilder::new("HiLoRegs", "")
        .units(2)
        .names(vec!["hi", "lo"])
        .track_pressure(false);
    let hilo_regs = regs.add_bank(builder);

    let builder = RegClassBuilder::new_toplevel("GPR", int_regs);
    regs.add_class(builder);

    let builder = RegClassBuilder::new_toplevel("FPR", float_regs);
    regs.add_class(builder);

    let builder = RegClassBuilder::new_toplevel("HILO", hilo_regs);
    regs.add_class(builder);

    regs.finish()
}

pub fn define(shared_defs: &mut SharedDefinitions) -> TargetIsa {
    let settings = define_settings(&shared_defs.settings);
    let regs = define_registers();

    let inst_group = InstructionGroup::new("mips", "mips specific instruction set");

    // CPU modes for 32-bit and 64-bit operation.
    let mut mips_32 = CpuMode::new("MIPS32");
    let mut mips_64 = CpuMode::new("MIPS64");

    let expand = shared_defs.transform_groups.by_name("expand");
    let narrow = shared_defs.transform_groups.by_name("narrow");
    mips_32.legalize_monomorphic(expand);
    mips_32.legalize_default(narrow);
    mips_32.legalize_type(I32, expand);
    mips_32.legalize_type(F32, expand);
    mips_32.legalize_type(F64, expand);

    mips_64.legalize_monomorphic(expand);
    mips_64.legalize_default(narrow);
    mips_64.legalize_type(I32, expand);
    mips_64.legalize_type(I64, expand);
    mips_64.legalize_type(F32, expand);
    mips_64.legalize_type(F64, expand);

    let cpu_modes = vec![mips_32, mips_64];

    TargetIsa::new("mips", inst_group, settings, regs, cpu_modes)
}
