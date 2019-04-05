//! Emitting binary MIPS machine code.

use crate::binemit::{bad_encoding, CodeSink, Reloc};
use crate::ir::{Function, Inst, InstructionData};
use crate::isa::{RegUnit, StackBaseMask, StackRef};
use crate::predicates::is_signed_int;
use crate::regalloc::RegDiversions;
use core::u32;

include!(concat!(env!("OUT_DIR"), "/binemit-mips.rs"));

/// Decompose encoding bits into individual fields.
///
/// Encoding bits: `(shamt << 12) | (funct << 6) | opcode`.
fn decompose_bits(bits: u16) -> (u8, u8, u8) {
    let opcode = bits & 0x3f;
    let funct = (bits >> 6) & 0x1f;
    let shamt = (bits >> 12) & 0x1f;

    (opcode as u8, shamt as u8, funct as u8)
}

/// R-type instructions with constant opcode, shamt and funct.
fn put_r<CS: CodeSink + ?Sized>(
    bits: u16,
    rs: RegUnit,
    rt: RegUnit,
    rd: RegUnit,
    sink: &mut CS,
) {
    let (opcode, shamt, funct) = decompose_bits(bits);

    internal_put_r(opcode, shamt, funct, rs, rt, rd, sink);
}

/// R-type instructions with dynamic shamt.
fn put_rshamt<CS: CodeSink + ?Sized>(
    bits: u16,
    shamt: i64,
    rs: RegUnit,
    rt: RegUnit,
    rd: RegUnit,
    sink: &mut CS,
) {
    let (opcode, _shamt, funct) = decompose_bits(bits);
    let shamt = ((shamt as u32) & 0x1f) as u8;

    internal_put_r(opcode, shamt, funct, rs, rt, rd, sink);
}

/// R-type instructions.
///
///   31     25  20  15  10    5
///   opcode rs  rt  rd  shamt funct
///       26  21  16  11     6     0
fn internal_put_r<CS: CodeSink + ?Sized>(
    opcode: u8,
    shamt: u8,
    funct: u8,
    rs: RegUnit,
    rt: RegUnit,
    rd: RegUnit,
    sink: &mut CS,
) {
    let rs = u32::from(rs) & 0x1f;
    let rt = u32::from(rt) & 0x1f;
    let rd = u32::from(rd) & 0x1f;

    let mut i = funct as u32;
    i |= (shamt as u32) << 6;
    i |= rd << 11;
    i |= rt << 16;
    i |= rs << 21;
    i |= (opcode as u32) << 26;

    sink.put4(i);
}

/// I-type instructions.
///
///   31     25  20  15
///   opcode rs  rt  immediate
///       26  21  16         0
fn put_i<CS: CodeSink + ?Sized>(bits: u16, rs: RegUnit, rt: RegUnit, imm: i64, sink: &mut CS) {
    let (opcode, _shamt, _funct) = decompose_bits(bits);
    let rs = u32::from(rs) & 0x1f;
    let rt = u32::from(rt) & 0x1f;

    debug_assert!(is_signed_int(imm, 16, 0), "IMM out of range {:#x}", imm);
    let imm = (imm & 0xffff) as u32;

    let mut i = imm;
    i |= rt << 16;
    i |= rs << 21;
    i |= (opcode as u32) << 26;

    sink.put4(i);
}

/// J-type instructions.
///
///   31     25
///   opcode address
///       26       0
fn put_j<CS: CodeSink + ?Sized>(bits: u16, imm: i64, sink: &mut CS) {
    let (opcode, _shamt, _funct) = decompose_bits(bits);

    debug_assert!(is_signed_int(imm, 28, 2), "IMM out of range {:#x}", imm);
    let imm = imm as u32;

    let mut i = imm;
    i |= (opcode as u32) << 26;

    sink.put4(i);
}
