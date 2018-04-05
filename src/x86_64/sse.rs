use ::arch::x86_64::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvtss_si64(a: f32x4) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_cvtss_si64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvttss_si64(a: f32x4) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_cvttss_si64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn _mm_cvtsi64_ss(a: f32x4, b: i64) -> f32x4 {
    ::mem::transmute(::arch::x86_64::_mm_cvtsi64_ss(::mem::transmute(a), ::mem::transmute(b)))
}

