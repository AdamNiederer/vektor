use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "aes")]
pub unsafe fn _mm_aesdec_si128(a: __m128i, round_key: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_aesdec_si128(::mem::transmute(a), ::mem::transmute(round_key)))
}

#[inline]
#[target_feature(enable = "aes")]
pub unsafe fn _mm_aesdeclast_si128(a: __m128i, round_key: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_aesdeclast_si128(::mem::transmute(a), ::mem::transmute(round_key)))
}

#[inline]
#[target_feature(enable = "aes")]
pub unsafe fn _mm_aesenc_si128(a: __m128i, round_key: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_aesenc_si128(::mem::transmute(a), ::mem::transmute(round_key)))
}

#[inline]
#[target_feature(enable = "aes")]
pub unsafe fn _mm_aesenclast_si128(a: __m128i, round_key: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_aesenclast_si128(::mem::transmute(a), ::mem::transmute(round_key)))
}

#[inline]
#[target_feature(enable = "aes")]
pub unsafe fn _mm_aesimc_si128(a: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_aesimc_si128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "aes")]
pub unsafe fn _mm_aeskeygenassist_si128(a: __m128i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_aeskeygenassist_si128(::mem::transmute(a), ::mem::transmute(imm8)))
}

