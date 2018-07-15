// Autogenerated by `scrape.py`.
// See https://github.com/AdamNiederer/vektor-gen

#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Reverse the order of the bytes.
#[inline]
#[cfg_attr(test, assert_instr(rev))]
pub unsafe fn _rev_u64(x: u64) -> u64 {
    crate::mem::transmute(crate::myarch::_rev_u64(crate::mem::transmute(x)))
}

/// Count Leading Zeros.
#[inline]
#[cfg_attr(test, assert_instr(clz))]
pub unsafe fn _clz_u64(x: u64) -> u64 {
    crate::mem::transmute(crate::myarch::_clz_u64(crate::mem::transmute(x)))
}

/// Reverse the bit order.
#[inline]
#[cfg_attr(test, assert_instr(rbit))]
pub unsafe fn _rbit_u64(x: u64) -> u64 {
    crate::mem::transmute(crate::myarch::_rbit_u64(crate::mem::transmute(x)))
}

/// Counts the leading most significant bits set.
///
/// When all bits of the operand are set it returns the size of the operand in
/// bits.
#[inline]
#[cfg_attr(test, assert_instr(cls))]
pub unsafe fn _cls_u32(x: u32) -> u32 {
    crate::mem::transmute(crate::myarch::_cls_u32(crate::mem::transmute(x)))
}

/// Counts the leading most significant bits set.
///
/// When all bits of the operand are set it returns the size of the operand in
/// bits.
#[inline]
#[cfg_attr(test, assert_instr(cls))]
pub unsafe fn _cls_u64(x: u64) -> u64 {
    crate::mem::transmute(crate::myarch::_cls_u64(crate::mem::transmute(x)))
}
