use ::arch::x86_64::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsd_si64(a: __m128d) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_cvtsd_si64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsd_si64x(a: __m128d) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_cvtsd_si64x(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvttsd_si64(a: __m128d) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_cvttsd_si64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvttsd_si64x(a: __m128d) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_cvttsd_si64x(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi64_si128(a: i64) -> __m128i {
    ::mem::transmute(::arch::x86_64::_mm_cvtsi64_si128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi64x_si128(a: i64) -> __m128i {
    ::mem::transmute(::arch::x86_64::_mm_cvtsi64x_si128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi128_si64(a: __m128i) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_cvtsi128_si64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi128_si64x(a: __m128i) -> i64 {
    ::mem::transmute(::arch::x86_64::_mm_cvtsi128_si64x(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi64_sd(a: __m128d, b: i64) -> f64x2 {
    ::mem::transmute(::arch::x86_64::_mm_cvtsi64_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi64x_sd(a: __m128d, b: i64) -> f64x2 {
    ::mem::transmute(::arch::x86_64::_mm_cvtsi64x_sd(::mem::transmute(a), ::mem::transmute(b)))
}

