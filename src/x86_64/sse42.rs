#![allow(unused_imports)]
use crate::myarch::*;
use crate::simd::*;

/// Starting with the initial value in `crc`, return the accumulated
/// CRC32 value for unsigned 64-bit integer `v`.
#[inline]
#[target_feature(enable = "sse4.2")]
#[cfg_attr(test, assert_instr(crc32))]
pub unsafe fn _mm_crc32_u64(crc: u64, v: u64) -> u64 {
    crate::mem::transmute(crate::myarch::_mm_crc32_u64(crate::mem::transmute(crc), crate::mem::transmute(v)))
}

