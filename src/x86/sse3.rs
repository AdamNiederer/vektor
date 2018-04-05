use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_addsub_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_addsub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_addsub_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_addsub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_hadd_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_hadd_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_hadd_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_hadd_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_hsub_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_hsub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_hsub_ps(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_hsub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_lddqu_si128(mem_addr: *const __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_lddqu_si128(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_movedup_pd(a: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_movedup_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_loaddup_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_loaddup_pd(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_movehdup_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_movehdup_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse3")]
pub unsafe fn _mm_moveldup_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_moveldup_ps(::mem::transmute(a)))
}

