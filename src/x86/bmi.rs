// Autogenerated by `scrape.py`.
// See https://github.com/AdamNiederer/vektor-gen

#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Extracts bits in range [`start`, `start` + `length`) from `a` into
/// the least significant bits of the result.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(bextr))]
pub unsafe fn _bextr_u32(a: u32, start: u32, len: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_bextr_u32(crate::mem::transmute(a), crate::mem::transmute(start), crate::mem::transmute(len)))
}

/// Extracts bits of `a` specified by `control` into
/// the least significant bits of the result.
///
/// Bits [7,0] of `control` specify the index to the first bit in the range to
/// be extracted, and bits [15,8] specify the length of the range.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(bextr))]
pub unsafe fn _bextr2_u32(a: u32, control: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_bextr2_u32(crate::mem::transmute(a), crate::mem::transmute(control)))
}

/// Bitwise logical `AND` of inverted `a` with `b`.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(andn))]
pub unsafe fn _andn_u32(a: u32, b: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_andn_u32(crate::mem::transmute(a), crate::mem::transmute(b)))
}

/// Extract lowest set isolated bit.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(blsi))]
pub unsafe fn _blsi_u32(x: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_blsi_u32(crate::mem::transmute(x)))
}

/// Get mask up to lowest set bit.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(blsmsk))]
pub unsafe fn _blsmsk_u32(x: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_blsmsk_u32(crate::mem::transmute(x)))
}

/// Resets the lowest set bit of `x`.
///
/// If `x` is sets CF.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(blsr))]
pub unsafe fn _blsr_u32(x: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_blsr_u32(crate::mem::transmute(x)))
}

/// Counts the number of trailing least significant zero bits.
///
/// When the source operand is 0, it returns its size in bits.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(tzcnt))]
pub unsafe fn _tzcnt_u32(x: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_tzcnt_u32(crate::mem::transmute(x)))
}

/// Counts the number of trailing least significant zero bits.
///
/// When the source operand is 0, it returns its size in bits.
#[inline]
#[target_feature(enable = "bmi1")]
#[cfg_attr(test, assert_instr(tzcnt))]
pub unsafe fn _mm_tzcnt_32(x: u32) -> i32 {
    crate::mem::transmute(crate::myarch::_mm_tzcnt_32(crate::mem::transmute(x)))
}

