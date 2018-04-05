use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sha")]
pub unsafe fn _mm_sha1msg1_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_sha1msg1_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sha")]
pub unsafe fn _mm_sha1msg2_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_sha1msg2_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sha")]
pub unsafe fn _mm_sha1nexte_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_sha1nexte_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sha")]
pub unsafe fn _mm_sha1rnds4_epu32(a: u32x4, b: u32x4, func: i32) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_sha1rnds4_epu32(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(func)))
}

#[inline]
#[target_feature(enable = "sha")]
pub unsafe fn _mm_sha256msg1_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_sha256msg1_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sha")]
pub unsafe fn _mm_sha256msg2_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_sha256msg2_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sha")]
pub unsafe fn _mm_sha256rnds2_epu32(a: u32x4, b: u32x4, k: __m128i) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_sha256rnds2_epu32(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(k)))
}

