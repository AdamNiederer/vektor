#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Counts the leading most significant zero bits.
///
/// When the operand is zero, it returns its size in bits.
#[inline]
#[target_feature(enable = "lzcnt")]
#[cfg_attr(test, assert_instr(lzcnt))]
pub unsafe fn _lzcnt_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_lzcnt_u32(::mem::transmute(x)))
}

/// Counts the bits that are set.
#[inline]
#[target_feature(enable = "popcnt")]
#[cfg_attr(test, assert_instr(popcnt))]
pub unsafe fn _popcnt32(x: i32) -> i32 {
    ::mem::transmute(::myarch::_popcnt32(::mem::transmute(x)))
}

