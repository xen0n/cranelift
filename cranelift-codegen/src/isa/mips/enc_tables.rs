//! Encoding tables for MIPS.

use super::registers::*;
use crate::ir;
use crate::isa;
use crate::isa::constraints::*;
use crate::isa::enc_tables::*;
use crate::isa::encoding::{base_size, RecipeSizing};

include!(concat!(env!("OUT_DIR"), "/encoding-mips.rs"));
include!(concat!(env!("OUT_DIR"), "/legalize-mips.rs"));
