use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_blendv_epi8(a: u8x16, b: u8x16, mask: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_blendv_epi8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_blend_epi16(a: i16x8, b: i16x8, imm8: i32) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_blend_epi16(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_blendv_pd(a: __m128d, b: __m128d, mask: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_blendv_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_blendv_ps(a: f32x4, b: f32x4, mask: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_blendv_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_blend_pd(a: __m128d, b: __m128d, imm2: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_blend_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm2)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_blend_ps(a: f32x4, b: f32x4, imm4: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_blend_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm4)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_extract_ps(a: f32x4, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_extract_ps(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_extract_epi8(a: u8x16, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_extract_epi8(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_extract_epi32(a: i32x4, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_extract_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_insert_ps(a: f32x4, b: f32x4, imm8: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_insert_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_insert_epi8(a: u8x16, i: i32, imm8: i32) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_insert_epi8(::mem::transmute(a), ::mem::transmute(i), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_insert_epi32(a: i32x4, i: i32, imm8: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_insert_epi32(::mem::transmute(a), ::mem::transmute(i), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_max_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_max_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_max_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::arch::x86::_mm_max_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_max_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_max_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_max_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_max_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_min_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_min_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_min_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::arch::x86::_mm_min_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_min_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_min_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_min_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_min_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_packus_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_packus_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cmpeq_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpeq_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepi8_epi16(a: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_cvtepi8_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepi8_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtepi8_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepi8_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtepi8_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepi16_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtepi16_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepi16_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtepi16_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepi32_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtepi32_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepu8_epi16(a: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_cvtepu8_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepu8_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtepu8_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepu8_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtepu8_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepu16_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtepu16_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepu16_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtepu16_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_cvtepu32_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtepu32_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_dp_pd(a: __m128d, b: __m128d, imm8: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_dp_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_dp_ps(a: f32x4, b: f32x4, imm8: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_dp_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_floor_pd(a: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_floor_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_floor_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_floor_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_floor_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_floor_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_floor_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_floor_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_ceil_pd(a: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_ceil_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_ceil_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_ceil_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_ceil_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_ceil_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_ceil_ss(a: f32x4, b: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_ceil_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_round_pd(a: __m128d, rounding: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_round_pd(::mem::transmute(a), ::mem::transmute(rounding)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_round_ps(a: f32x4, rounding: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_round_ps(::mem::transmute(a), ::mem::transmute(rounding)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_round_sd(a: __m128d, b: __m128d, rounding: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_round_sd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(rounding)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_round_ss(a: f32x4, b: f32x4, rounding: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_round_ss(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(rounding)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_minpos_epu16(a: u16x8) -> u16x8 {
    ::mem::transmute(::arch::x86::_mm_minpos_epu16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_mul_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_mul_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_mullo_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_mullo_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_mpsadbw_epu8(a: u8x16, b: u8x16, imm8: i32) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_mpsadbw_epu8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_testz_si128(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testz_si128(::mem::transmute(a), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_testc_si128(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testc_si128(::mem::transmute(a), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_testnzc_si128(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testnzc_si128(::mem::transmute(a), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_test_all_zeros(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::arch::x86::_mm_test_all_zeros(::mem::transmute(a), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_test_all_ones(a: __m128i) -> i32 {
    ::mem::transmute(::arch::x86::_mm_test_all_ones(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse4.1")]
pub unsafe fn _mm_test_mix_ones_zeros(a: __m128i, mask: __m128i) -> i32 {
    ::mem::transmute(::arch::x86::_mm_test_mix_ones_zeros(::mem::transmute(a), ::mem::transmute(mask)))
}

