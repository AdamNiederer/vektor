use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "pclmulqdq")]
pub unsafe fn _mm_clmulepi64_si128(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_clmulepi64_si128(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

