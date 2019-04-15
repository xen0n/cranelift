//! MIPS Instruction Set Architecture.

mod abi;
mod binemit;
mod enc_tables;
mod registers;
pub mod settings;

use super::super::settings as shared_settings;
#[cfg(feature = "testing_hooks")]
use crate::binemit::CodeSink;
use crate::binemit::{emit_function, MemoryCodeSink};
use crate::cursor::{Cursor, EncCursor, FuncCursor};
use crate::ir;
use crate::isa::enc_tables::{self as shared_enc_tables, lookup_enclist, Encodings};
use crate::isa::Builder as IsaBuilder;
use crate::isa::{EncInfo, RegClass, RegInfo, TargetIsa};
use crate::regalloc;
use core::fmt;
use std::boxed::Box;
use target_lexicon::{PointerWidth, Triple};

#[allow(dead_code)]
struct Isa {
    triple: Triple,
    shared_flags: shared_settings::Flags,
    isa_flags: settings::Flags,
    cpumode: &'static [shared_enc_tables::Level1Entry<u16>],
}

/// Get an ISA builder for creating MIPS targets.
pub fn isa_builder(triple: Triple) -> IsaBuilder {
    IsaBuilder {
        triple,
        setup: settings::builder(),
        constructor: isa_constructor,
    }
}

fn isa_constructor(
    triple: Triple,
    shared_flags: shared_settings::Flags,
    builder: shared_settings::Builder,
) -> Box<TargetIsa> {
    let level1 = match triple.pointer_width().unwrap() {
        PointerWidth::U16 => panic!("16-bit MIPS unrecognized"),
        PointerWidth::U32 => &enc_tables::LEVEL1_MIPS32[..],
        PointerWidth::U64 => &enc_tables::LEVEL1_MIPS64[..],
    };
    Box::new(Isa {
        triple,
        isa_flags: settings::Flags::new(&shared_flags, builder),
        shared_flags,
        cpumode: level1,
    })
}

impl TargetIsa for Isa {
    fn name(&self) -> &'static str {
        "mips"
    }

    fn triple(&self) -> &Triple {
        &self.triple
    }

    fn has_delay_slot(&self) -> bool {
        true
    }

    fn flags(&self) -> &shared_settings::Flags {
        &self.shared_flags
    }

    fn register_info(&self) -> RegInfo {
        registers::INFO.clone()
    }

    fn encoding_info(&self) -> EncInfo {
        enc_tables::INFO.clone()
    }

    fn legal_encodings<'a>(
        &'a self,
        func: &'a ir::Function,
        inst: &'a ir::InstructionData,
        ctrl_typevar: ir::Type,
    ) -> Encodings<'a> {
        lookup_enclist(
            ctrl_typevar,
            inst,
            func,
            self.cpumode,
            &enc_tables::LEVEL2[..],
            &enc_tables::ENCLISTS[..],
            &enc_tables::LEGALIZE_ACTIONS[..],
            &enc_tables::RECIPE_PREDICATES[..],
            &enc_tables::INST_PREDICATES[..],
            self.isa_flags.predicate_view(),
        )
    }

    fn legalize_signature(&self, sig: &mut ir::Signature, current: bool) {
        abi::legalize_signature(sig, &self.triple, &self.isa_flags, current)
    }

    fn regclass_for_abi_type(&self, ty: ir::Type) -> RegClass {
        abi::regclass_for_abi_type(ty)
    }

    fn allocatable_registers(&self, func: &ir::Function) -> regalloc::RegisterSet {
        abi::allocatable_registers(func, &self.isa_flags)
    }

    #[cfg(feature = "testing_hooks")]
    fn emit_inst(
        &self,
        func: &ir::Function,
        inst: ir::Inst,
        divert: &mut regalloc::RegDiversions,
        sink: &mut CodeSink,
    ) {
        binemit::emit_inst(func, inst, divert, sink)
    }

    fn emit_function_to_memory(&self, func: &ir::Function, sink: &mut MemoryCodeSink) {
        emit_function(func, binemit::emit_inst, sink)
    }

    fn fill_delay_slot_for_inst(
        &self,
        cur: &mut FuncCursor,
        _divert: &regalloc::RegDiversions,
        encinfo: &EncInfo,
    ) {
        let inst = cur.current_inst().unwrap();
        eprintln!(
            "Filling delay slot for inst [{}] {}",
            encinfo.display(cur.func.encodings[inst]),
            cur.func.dfg.display_inst(inst, self as &dyn TargetIsa),
        );

        use crate::ir::InstBuilder;

        let mut cur = EncCursor::new(cur.func, self).after_inst(inst);
        // TODO: do some real re-ordering here
        cur.ins().nop();
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::{immediates, types};
    use crate::ir::{Function, InstructionData, Opcode};
    use crate::isa;
    use crate::settings::{self, Configurable};
    use core::str::FromStr;
    use std::string::{String, ToString};
    use target_lexicon::triple;

    fn encstr(isa: &isa::TargetIsa, enc: Result<isa::Encoding, isa::Legalize>) -> String {
        match enc {
            Ok(e) => isa.encoding_info().display(e).to_string(),
            Err(_) => "no encoding".to_string(),
        }
    }

    #[test]
    fn test_64bitenc() {
        let shared_builder = settings::builder();
        let shared_flags = settings::Flags::new(shared_builder);
        let isa = isa::lookup(triple!("mips64el"))
            .unwrap()
            .finish(shared_flags);

        let mut func = Function::new();
        let ebb = func.dfg.make_ebb();
        let arg64 = func.dfg.append_ebb_param(ebb, types::I64);
        let arg32 = func.dfg.append_ebb_param(ebb, types::I32);

        // Try to encode iadd_imm.i64 v1, -10.
        let inst64 = InstructionData::BinaryImm {
            opcode: Opcode::IaddImm,
            arg: arg64,
            imm: immediates::Imm64::new(-10),
        };

        // DADDIU is I/0b011001
        assert_eq!(
            encstr(&*isa, isa.encode(&func, &inst64, types::I64)),
            "I#19"
        );

        // Try to encode iadd_imm.i64 v1, -32769.
        let inst64_large = InstructionData::BinaryImm {
            opcode: Opcode::IaddImm,
            arg: arg64,
            imm: immediates::Imm64::new(-32769),
        };

        // Immediate is out of range for DADDIU.
        assert!(isa.encode(&func, &inst64_large, types::I64).is_err());

        // Create an iadd_imm.i32 which is encodable in MIPS64.
        let inst32 = InstructionData::BinaryImm {
            opcode: Opcode::IaddImm,
            arg: arg32,
            imm: immediates::Imm64::new(10),
        };

        // ADDIU is I/0b001001
        assert_eq!(
            encstr(&*isa, isa.encode(&func, &inst32, types::I32)),
            "I#09"
        );

    }
}

impl fmt::Display for Isa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.shared_flags, self.isa_flags)
    }
}