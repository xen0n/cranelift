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
/// Encoding bits: `(funct << 6) | opcode`.
fn decompose_bits(bits: u16) -> (u32, u32) {
    let opcode = bits & 0x3f;
    let funct = (bits >> 6) & 0x3f;

    (opcode as u32, funct as u32)
}

/// Decompose encoding bits for REGIMM insns into individual fields.
///
/// This is currently identical to normal encoding bits scheme.
fn decompose_bits_regimm(bits: u16) -> (u32, u32) {
    decompose_bits(bits)
}

/// NOP helper.
fn put_nop<CS: CodeSink + ?Sized>(sink: &mut CS) {
    sink.put4(0);
}

/// R-type instructions with constant zero shamt.
fn put_r<CS: CodeSink + ?Sized>(
    bits: u16,
    rs: RegUnit,
    rt: RegUnit,
    rd: RegUnit,
    sink: &mut CS,
) {
    let (opcode, funct) = decompose_bits(bits);
    let rs = u32::from(rs) & 0x1f;
    let rt = u32::from(rt) & 0x1f;
    let rd = u32::from(rd) & 0x1f;

    internal_put_r(opcode, 0, funct, rs, rt, rd, sink);
}

/// R-type instructions with dynamic shamt.
fn put_rshamt<CS: CodeSink + ?Sized>(
    bits: u16,
    rt: RegUnit,
    rd: RegUnit,
    shamt: i64,
    sink: &mut CS,
) {
    let (opcode, funct) = decompose_bits(bits);
    let shamt = (shamt as u32) & 0x1f;
    let rt = u32::from(rt) & 0x1f;
    let rd = u32::from(rd) & 0x1f;

    internal_put_r(opcode, shamt, funct, 0, rt, rd, sink);
}

/// R-type instructions.
///
///   31     25  20  15  10    5
///   opcode rs  rt  rd  shamt funct
///       26  21  16  11     6     0
fn internal_put_r<CS: CodeSink + ?Sized>(
    opcode: u32,
    shamt: u32,
    funct: u32,
    rs: u32,
    rt: u32,
    rd: u32,
    sink: &mut CS,
) {
    let mut i = funct;
    i |= shamt << 6;
    i |= rd << 11;
    i |= rt << 16;
    i |= rs << 21;
    i |= opcode << 26;

    sink.put4(i);
}

/// I-type instructions.
fn put_i<CS: CodeSink + ?Sized>(bits: u16, rs: RegUnit, rt: RegUnit, imm: i64, sink: &mut CS) {
    let (opcode, _funct) = decompose_bits(bits);
    let rs = u32::from(rs) & 0x1f;
    let rt = u32::from(rt) & 0x1f;

    internal_put_i(opcode, rs, rt, imm, sink);
}

/// I-type REGIMM instructions.
fn put_i_regimm<CS: CodeSink + ?Sized>(bits: u16, rs: RegUnit, imm: i64, sink: &mut CS) {
    let (opcode, rt) = decompose_bits_regimm(bits);
    let rs = u32::from(rs) & 0x1f;

    internal_put_i(opcode, rs, rt, imm, sink);
}

/// I-type instructions.
///
///   31     25  20  15
///   opcode rs  rt  immediate
///       26  21  16         0
fn internal_put_i<CS: CodeSink + ?Sized>(
    opcode: u32,
    rs: u32,
    rt: u32,
    imm: i64,
    sink: &mut CS,
) {
    debug_assert!(is_signed_int(imm, 16, 0), "IMM out of range {:#x}", imm);
    let imm = (imm & 0xffff) as u32;

    let mut i = imm;
    i |= rt << 16;
    i |= rs << 21;
    i |= opcode << 26;

    sink.put4(i);
}

/// J-type instructions.
///
///   31     25
///   opcode address
///       26       0
fn put_j<CS: CodeSink + ?Sized>(bits: u16, imm: i64, sink: &mut CS) {
    let (opcode, _funct) = decompose_bits(bits);

    debug_assert!(is_signed_int(imm, 28, 2), "IMM out of range {:#x}", imm);
    let imm = imm as u32;

    let mut i = imm;
    i |= opcode << 26;

    sink.put4(i);
}
