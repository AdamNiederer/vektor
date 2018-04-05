use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse4a")]
pub unsafe fn _mm_extract_si64(x: __m128i, y: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_extract_si64(::mem::transmute(x), ::mem::transmute(y)))
}

#[inline]
#[target_feature(enable = "sse4a")]
pub unsafe fn _mm_insert_si64(x: __m128i, y: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_insert_si64(::mem::transmute(x), ::mem::transmute(y)))
}

