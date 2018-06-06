#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Return an integer with the reversed byte order of x
#[inline]
#[cfg_attr(test, assert_instr(bswap))]
pub unsafe fn _bswap(x: i32) -> i32 {
    ::mem::transmute(::myarch::_bswap(::mem::transmute(x)))
}

