#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Return an integer with the reversed byte order of x
#[inline]
#[cfg_attr(test, assert_instr(bswap))]
pub unsafe fn _bswap64(x: i64) -> i64 {
    ::mem::transmute(::myarch::_bswap64(::mem::transmute(x)))
}

