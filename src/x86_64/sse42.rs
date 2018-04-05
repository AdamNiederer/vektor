use ::arch::x86_64::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse4.2")]
pub unsafe fn _mm_crc32_u64(crc: u64, v: u64) -> u64 {
    ::mem::transmute(::arch::x86_64::_mm_crc32_u64(::mem::transmute(crc), ::mem::transmute(v)))
}

