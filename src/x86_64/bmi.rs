#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Extracts bits in range [`start`, `start` + `length`) from `a` into
/// the least significant bits of the result.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(bextr))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _bextr_u64(a: u64, start: u32, len: u32) -> u64 {
    ::mem::transmute(::myarch::_bextr_u64(::mem::transmute(a), ::mem::transmute(start), ::mem::transmute(len)))
}

/// Extracts bits of `a` specified by `control` into
/// the least significant bits of the result.
///
/// Bits [7,0] of `control` specify the index to the first bit in the range to
/// be extracted, and bits [15,8] specify the length of the range.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(bextr))]
#[cfg(not(target_arch = "x86"))]
pub unsafe fn _bextr2_u64(a: u64, control: u64) -> u64 {
    ::mem::transmute(::myarch::_bextr2_u64(::mem::transmute(a), ::mem::transmute(control)))
}

/// Bitwise logical `AND` of inverted `a` with `b`.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(andn))]
pub unsafe fn _andn_u64(a: u64, b: u64) -> u64 {
    ::mem::transmute(::myarch::_andn_u64(::mem::transmute(a), ::mem::transmute(b)))
}

 // generates lots of instructions
pub unsafe fn _blsi_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blsi_u64(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blsmsk_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blsmsk_u64(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blsr_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blsr_u64(::mem::transmute(x)))
}

/// Counts the number of trailing least significant zero bits.
///
/// When the source operand is 0, it returns its size in bits.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(tzcnt))]
pub unsafe fn _tzcnt_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_tzcnt_u64(::mem::transmute(x)))
}

/// Counts the number of trailing least significant zero bits.
///
/// When the source operand is 0, it returns its size in bits.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(tzcnt))]
pub unsafe fn _mm_tzcnt_64(x: u64) -> i64 {
    ::mem::transmute(::myarch::_mm_tzcnt_64(::mem::transmute(x)))
}

