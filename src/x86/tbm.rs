#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Clears all bits below the least significant zero bit of `x`.
///
/// If there is no zero bit in `x`, it returns zero.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(blcfill))]
pub unsafe fn _blcfill_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_blcfill_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blcfill_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blcfill_u64(::mem::transmute(x)))
}

/// Sets all bits of `x` to 1 except for the least significant zero bit.
///
/// If there is no zero bit in `x`, it sets all bits.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(blci))]
pub unsafe fn _blci_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_blci_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blci_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blci_u64(::mem::transmute(x)))
}

/// Sets the least significant zero bit of `x` and clears all other bits.
///
/// If there is no zero bit in `x`, it returns zero.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(blcic))]
pub unsafe fn _blcic_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_blcic_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blcic_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blcic_u64(::mem::transmute(x)))
}

/// Sets the least significant zero bit of `x` and clears all bits above
/// that bit.
///
/// If there is no zero bit in `x`, it sets all the bits.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(blcmsk))]
pub unsafe fn _blcmsk_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_blcmsk_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blcmsk_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blcmsk_u64(::mem::transmute(x)))
}

/// Sets the least significant zero bit of `x`.
///
/// If there is no zero bit in `x`, it returns `x`.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(blcs))]
pub unsafe fn _blcs_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_blcs_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blcs_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blcs_u64(::mem::transmute(x)))
}

/// Sets all bits of `x` below the least significant one.
///
/// If there is no set bit in `x`, it sets all the bits.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(blsfill))]
pub unsafe fn _blsfill_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_blsfill_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blsfill_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blsfill_u64(::mem::transmute(x)))
}

/// Clears least significant bit and sets all other bits.
///
/// If there is no set bit in `x`, it sets all the bits.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(blsic))]
pub unsafe fn _blsic_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_blsic_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _blsic_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_blsic_u64(::mem::transmute(x)))
}

/// Clears all bits below the least significant zero of `x` and sets all other
/// bits.
///
/// If the least significant bit of `x` is 0, it sets all bits.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(t1mskc))]
pub unsafe fn _t1mskc_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_t1mskc_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _t1mskc_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_t1mskc_u64(::mem::transmute(x)))
}

/// Sets all bits below the least significant one of `x` and clears all other
/// bits.
///
/// If the least significant bit of `x` is 1, it returns zero.
#[inline]
#[target_feature(enable = "tbm")]
#[cfg_attr(test, assert_instr(tzmsk))]
pub unsafe fn _tzmsk_u32(x: u32) -> u32 {
    ::mem::transmute(::myarch::_tzmsk_u32(::mem::transmute(x)))
}

 // generates lots of instructions
pub unsafe fn _tzmsk_u64(x: u64) -> u64 {
    ::mem::transmute(::myarch::_tzmsk_u64(::mem::transmute(x)))
}

