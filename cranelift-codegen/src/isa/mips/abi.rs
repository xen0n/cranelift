//! MIPS ABI implementation.
//!
//! This module implements the MIPS calling convention through the primary `legalize_signature()`
//! entry point. Only n64 is supported for now. Largely based on the RISC-V implementation.
//!
//! This doesn't support the soft-float ABI at the moment.

use super::registers::{FPR, GPR};
use super::settings;
use crate::abi::{legalize_args, ArgAction, ArgAssigner, ValueConversion};
use crate::ir::{self, AbiParam, ArgumentExtension, ArgumentLoc, ArgumentPurpose, Type};
use crate::isa::RegClass;
use crate::regalloc::RegisterSet;
use core::i32;
use target_lexicon::Triple;

struct Args {
    pointer_bits: u8,
    pointer_bytes: u8,
    pointer_type: Type,
    is_for_return: bool,
    regs: u32,
    reg_limit: u32,
    offset: u32,
}

impl Args {
    fn new(bits: u8, is_for_return: bool) -> Self {
        Self {
            pointer_bits: bits,
            pointer_bytes: bits / 8,
            pointer_type: Type::int(u16::from(bits)).unwrap(),
            is_for_return: is_for_return,
            regs: 0,
            reg_limit: if is_for_return { 2 } else { 8 },
            offset: 0,
        }
    }
}

impl ArgAssigner for Args {
    fn assign(&mut self, arg: &AbiParam) -> ArgAction {
        fn align(value: u32, to: u32) -> u32 {
            (value + to - 1) & !(to - 1)
        }

        let ty = arg.value_type;

        // Check for a legal type.
        if ty.is_vector() {
            // TODO: SIMD
            return ValueConversion::VectorSplit.into();
        }

        // Large integers and booleans are broken down to fit in a register.
        if !ty.is_float() && ty.bits() > u16::from(self.pointer_bits) {
            // Align registers and stack to a multiple of two pointers.
            self.regs = align(self.regs, 2);
            self.offset = align(self.offset, 2 * u32::from(self.pointer_bytes));
            return ValueConversion::IntSplit.into();
        }

        // Small integers are extended to the size of a pointer register.
        if ty.is_int() && ty.bits() < u16::from(self.pointer_bits) {
            match arg.extension {
                ArgumentExtension::None => {}
                ArgumentExtension::Uext => return ValueConversion::Uext(self.pointer_type).into(),
                ArgumentExtension::Sext => return ValueConversion::Sext(self.pointer_type).into(),
            }
        }

        if self.regs < self.reg_limit {
            // Assign to a register.
            let (bank, reg_initial_idx, increment) = match (ty.is_float(), self.is_for_return) {
                (false, false) => (GPR, 4, 1),
                (true, false) => (FPR, 12, 1),
                (false, true) => (GPR, 2, 1),
                (true, true) => (FPR, 0, 2),
            };

            let reg = bank.unit(reg_initial_idx + self.regs as usize);
            self.regs += increment;
            ArgumentLoc::Reg(reg).into()
        } else {
            // Assign a stack location.
            let loc = ArgumentLoc::Stack(self.offset as i32);
            self.offset += u32::from(self.pointer_bytes);
            debug_assert!(self.offset <= i32::MAX as u32);
            loc.into()
        }
    }
}

/// Legalize `sig` for MIPS.
pub fn legalize_signature(
    sig: &mut ir::Signature,
    triple: &Triple,
    isa_flags: &settings::Flags,
    current: bool,
) {
    let bits = triple.pointer_width().unwrap().bits();

    let mut args = Args::new(bits, false);
    legalize_args(&mut sig.params, &mut args);

    let mut rets = Args::new(bits, true);
    legalize_args(&mut sig.returns, &mut rets);

    if current {
        let ptr = Type::int(u16::from(bits)).unwrap();

        // Add the link register as an argument and return value.
        let link = AbiParam::special_reg(ptr, ArgumentPurpose::Link, GPR.unit(31));
        sig.params.push(link);
        sig.returns.push(link);
    }
}

/// Get register class for a type appearing in a legalized signature.
pub fn regclass_for_abi_type(ty: Type) -> RegClass {
    if ty.is_float() {
        FPR
    } else {
        GPR
    }
}

pub fn allocatable_registers(_func: &ir::Function, _isa_flags: &settings::Flags) -> RegisterSet {
    let mut regs = RegisterSet::new();
    regs.take(GPR, GPR.unit(0));  // Hard-wired 0.
    regs.take(GPR, GPR.unit(1));  // at is usually reserved.
    regs.take(GPR, GPR.unit(26)); // k0.
    regs.take(GPR, GPR.unit(27)); // k1.
    regs.take(GPR, GPR.unit(28)); // Global pointer.
    regs.take(GPR, GPR.unit(29)); // Stack pointer.
                                  // %ra is the link register which is available for allocation.
                                  // TODO: $30 is the frame pointer. Reserve it?

    // TODO: no-odd-spreg for FPR

    regs
}
