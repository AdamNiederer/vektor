#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Counts the leading most significant zero bits.
///
/// When the operand is zero, it returns its size in bits.
#[inline]
#[target_feature(enable = "lzcnt")]
#[cfg_attr(test, assert_instr(lzcnt))]
pub unsafe fn _lzcnt_u64(x: u64) -> u64 {
    crate::mem::transmute(crate::myarch::_lzcnt_u64(crate::mem::transmute(x)))
}

/// Counts the bits that are set.
#[inline]
#[target_feature(enable = "popcnt")]
#[cfg_attr(test, assert_instr(popcnt))]
pub unsafe fn _popcnt64(x: i64) -> i32 {
    crate::mem::transmute(crate::myarch::_popcnt64(crate::mem::transmute(x)))
}

