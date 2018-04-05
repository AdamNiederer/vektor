use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_abs_epi32(a: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_abs_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_abs_epi16(a: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_abs_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_abs_epi8(a: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_abs_epi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_add_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_add_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_add_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_add_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_add_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_add_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_add_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_add_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_adds_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_adds_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_adds_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_adds_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_adds_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::arch::x86::_mm256_adds_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_adds_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::arch::x86::_mm256_adds_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_alignr_epi8(a: i8x32, b: i8x32, n: i32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_alignr_epi8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(n)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_and_si256(a: __m256i, b: __m256i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_and_si256(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_andnot_si256(a: __m256i, b: __m256i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_andnot_si256(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_avg_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::arch::x86::_mm256_avg_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_avg_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::arch::x86::_mm256_avg_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_blend_epi32(a: i32x4, b: i32x4, imm8: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_blend_epi32(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_blend_epi32(a: i32x8, b: i32x8, imm8: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_blend_epi32(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_blend_epi16(a: i16x16, b: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_blend_epi16(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_blendv_epi8(a: i8x32, b: i8x32, mask: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_blendv_epi8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_broadcastb_epi8(a: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_broadcastb_epi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_broadcastb_epi8(a: u8x16) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_broadcastb_epi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_broadcastd_epi32(a: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_broadcastd_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_broadcastd_epi32(a: i32x4) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_broadcastd_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_broadcastq_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_broadcastq_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_broadcastq_epi64(a: i64x2) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_broadcastq_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_broadcastsd_pd(a: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_broadcastsd_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_broadcastsd_pd(a: __m128d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_broadcastsd_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_broadcastsi128_si256(a: __m128i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_broadcastsi128_si256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_broadcastss_ps(a: f32x4) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_broadcastss_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_broadcastss_ps(a: f32x4) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_broadcastss_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_broadcastw_epi16(a: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_broadcastw_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_broadcastw_epi16(a: i16x8) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_broadcastw_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cmpeq_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_cmpeq_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cmpeq_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_cmpeq_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cmpeq_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_cmpeq_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cmpeq_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_cmpeq_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cmpgt_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_cmpgt_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cmpgt_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_cmpgt_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cmpgt_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_cmpgt_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cmpgt_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_cmpgt_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepi16_epi32(a: i32x4) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_cvtepi16_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepi16_epi64(a: i64x2) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtepi16_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepi32_epi64(a: i64x2) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtepi32_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepi8_epi16(a: i16x8) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_cvtepi8_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepi8_epi32(a: i32x4) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_cvtepi8_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepi8_epi64(a: i64x2) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtepi8_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepu16_epi32(a: i32x4) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_cvtepu16_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepu16_epi64(a: i64x2) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtepu16_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepu32_epi64(a: i64x2) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtepu32_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepu8_epi16(a: i16x8) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_cvtepu8_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepu8_epi32(a: i32x4) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_cvtepu8_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtepu8_epi64(a: i64x2) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtepu8_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_extracti128_si256(a: __m256i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm256_extracti128_si256(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_hadd_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_hadd_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_hadd_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_hadd_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_hadds_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_hadds_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_hsub_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_hsub_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_hsub_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_hsub_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_hsubs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_hsubs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_i32gather_epi32(slice: *const i32, offsets: __m128i, scale: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_i32gather_epi32(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_mask_i32gather_epi32(src: __m128i, slice: *const i32, offsets: __m128i, mask: __m128i, scale: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_mask_i32gather_epi32(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_i32gather_epi32(slice: *const i32, offsets: __m256i, scale: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_i32gather_epi32(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mask_i32gather_epi32(src: __m256i, slice: *const i32, offsets: __m256i, mask: __m256i, scale: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_mask_i32gather_epi32(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_i32gather_ps(slice: *const f32, offsets: __m128i, scale: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_i32gather_ps(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_mask_i32gather_ps(src: f32x4, slice: *const f32, offsets: __m128i, mask: f32x4, scale: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_mask_i32gather_ps(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_i32gather_ps(slice: *const f32, offsets: __m256i, scale: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_i32gather_ps(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mask_i32gather_ps(src: f32x8, slice: *const f32, offsets: __m256i, mask: f32x8, scale: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_mask_i32gather_ps(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_i32gather_epi64(slice: *const i64, offsets: __m128i, scale: i32) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_i32gather_epi64(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_mask_i32gather_epi64(src: __m128i, slice: *const i64, offsets: __m128i, mask: __m128i, scale: i32) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_mask_i32gather_epi64(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_i32gather_epi64(slice: *const i64, offsets: __m128i, scale: i32) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_i32gather_epi64(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mask_i32gather_epi64(src: __m256i, slice: *const i64, offsets: __m128i, mask: __m256i, scale: i32) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_mask_i32gather_epi64(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_i32gather_pd(slice: *const f64, offsets: __m128i, scale: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_i32gather_pd(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_mask_i32gather_pd(src: __m128d, slice: *const f64, offsets: __m128i, mask: __m128d, scale: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_mask_i32gather_pd(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_i32gather_pd(slice: *const f64, offsets: __m128i, scale: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_i32gather_pd(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mask_i32gather_pd(src: __m256d, slice: *const f64, offsets: __m128i, mask: __m256d, scale: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_mask_i32gather_pd(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_i64gather_epi32(slice: *const i32, offsets: __m128i, scale: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_i64gather_epi32(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_mask_i64gather_epi32(src: __m128i, slice: *const i32, offsets: __m128i, mask: __m128i, scale: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_mask_i64gather_epi32(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_i64gather_epi32(slice: *const i32, offsets: __m256i, scale: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm256_i64gather_epi32(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mask_i64gather_epi32(src: __m128i, slice: *const i32, offsets: __m256i, mask: __m128i, scale: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm256_mask_i64gather_epi32(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_i64gather_ps(slice: *const f32, offsets: __m128i, scale: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_i64gather_ps(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_mask_i64gather_ps(src: f32x4, slice: *const f32, offsets: __m128i, mask: f32x4, scale: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_mask_i64gather_ps(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_i64gather_ps(slice: *const f32, offsets: __m256i, scale: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm256_i64gather_ps(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mask_i64gather_ps(src: f32x4, slice: *const f32, offsets: __m256i, mask: f32x4, scale: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm256_mask_i64gather_ps(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_i64gather_epi64(slice: *const i64, offsets: __m128i, scale: i32) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_i64gather_epi64(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_mask_i64gather_epi64(src: __m128i, slice: *const i64, offsets: __m128i, mask: __m128i, scale: i32) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_mask_i64gather_epi64(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_i64gather_epi64(slice: *const i64, offsets: __m256i, scale: i32) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_i64gather_epi64(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mask_i64gather_epi64(src: __m256i, slice: *const i64, offsets: __m256i, mask: __m256i, scale: i32) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_mask_i64gather_epi64(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_i64gather_pd(slice: *const f64, offsets: __m128i, scale: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_i64gather_pd(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_mask_i64gather_pd(src: __m128d, slice: *const f64, offsets: __m128i, mask: __m128d, scale: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_mask_i64gather_pd(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_i64gather_pd(slice: *const f64, offsets: __m256i, scale: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_i64gather_pd(::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mask_i64gather_pd(src: __m256d, slice: *const f64, offsets: __m256i, mask: __m256d, scale: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_mask_i64gather_pd(::mem::transmute(src), ::mem::transmute(slice), ::mem::transmute(offsets), ::mem::transmute(mask), ::mem::transmute(scale)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_inserti128_si256(a: __m256i, b: __m128i, imm8: i32) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_inserti128_si256(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_madd_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_madd_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_maddubs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_maddubs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_maskload_epi32(mem_addr: *const i32, mask: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_maskload_epi32(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_maskload_epi32(mem_addr: *const i32, mask: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_maskload_epi32(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_maskload_epi64(mem_addr: *const i64, mask: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_maskload_epi64(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_maskload_epi64(mem_addr: *const i64, mask: i64x4) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_maskload_epi64(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_max_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_max_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_max_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_max_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_max_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_max_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_max_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::arch::x86::_mm256_max_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_max_epu32(a: u32x8, b: u32x8) -> u32x8 {
    ::mem::transmute(::arch::x86::_mm256_max_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_max_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::arch::x86::_mm256_max_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_min_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_min_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_min_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_min_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_min_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_min_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_min_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::arch::x86::_mm256_min_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_min_epu32(a: u32x8, b: u32x8) -> u32x8 {
    ::mem::transmute(::arch::x86::_mm256_min_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_min_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::arch::x86::_mm256_min_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_movemask_epi8(a: i8x32) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_movemask_epi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mpsadbw_epu8(a: u8x32, b: u8x32, imm8: i32) -> u8x32 {
    ::mem::transmute(::arch::x86::_mm256_mpsadbw_epu8(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mul_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_mul_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mul_epu32(a: u32x8, b: u32x8) -> u32x8 {
    ::mem::transmute(::arch::x86::_mm256_mul_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mulhi_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_mulhi_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mulhi_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::arch::x86::_mm256_mulhi_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mullo_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_mullo_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mullo_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_mullo_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_mulhrs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_mulhrs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_or_si256(a: __m256i, b: __m256i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_or_si256(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_packs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_packs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_packs_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_packs_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_packus_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_packus_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_packus_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_packus_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_permutevar8x32_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_permutevar8x32_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_permute4x64_epi64(a: i64x4, imm8: i32) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_permute4x64_epi64(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_permute2x128_si256(a: __m256i, b: __m256i, imm8: i32) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_permute2x128_si256(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_permute4x64_pd(a: __m256d, imm8: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_permute4x64_pd(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_permutevar8x32_ps(a: f32x8, idx: __m256i) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_permutevar8x32_ps(::mem::transmute(a), ::mem::transmute(idx)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sad_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::arch::x86::_mm256_sad_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_shuffle_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_shuffle_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_shuffle_epi32(a: i32x8, imm8: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_shuffle_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_shufflehi_epi16(a: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_shufflehi_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_shufflelo_epi16(a: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_shufflelo_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sign_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_sign_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sign_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_sign_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sign_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_sign_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sll_epi16(a: i16x16, count: __m128i) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_sll_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sll_epi32(a: i32x8, count: __m128i) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_sll_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sll_epi64(a: i64x4, count: __m128i) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_sll_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_slli_epi16(a: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_slli_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_slli_epi32(a: i32x8, imm8: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_slli_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_slli_epi64(a: i64x4, imm8: i32) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_slli_epi64(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_slli_si256(a: __m256i, imm8: i32) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_slli_si256(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_bslli_epi128(a: __m256i, imm8: i32) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_bslli_epi128(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_sllv_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_sllv_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sllv_epi32(a: i32x8, count: __m256i) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_sllv_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_sllv_epi64(a: i64x2, count: __m128i) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_sllv_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sllv_epi64(a: i64x4, count: __m256i) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_sllv_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sra_epi16(a: i16x16, count: __m128i) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_sra_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sra_epi32(a: i32x8, count: __m128i) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_sra_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srai_epi16(a: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_srai_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srai_epi32(a: i32x8, imm8: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_srai_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_srav_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_srav_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srav_epi32(a: i32x8, count: __m256i) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_srav_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srli_si256(a: __m256i, imm8: i32) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_srli_si256(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_bsrli_epi128(a: __m256i, imm8: i32) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_bsrli_epi128(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srl_epi16(a: i16x16, count: __m128i) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_srl_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srl_epi32(a: i32x8, count: __m128i) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_srl_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srl_epi64(a: i64x4, count: __m128i) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_srl_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srli_epi16(a: i16x16, imm8: i32) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_srli_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srli_epi32(a: i32x8, imm8: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_srli_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srli_epi64(a: i64x4, imm8: i32) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_srli_epi64(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_srlv_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_srlv_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srlv_epi32(a: i32x8, count: __m256i) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_srlv_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm_srlv_epi64(a: i64x2, count: __m128i) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_srlv_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_srlv_epi64(a: i64x4, count: __m256i) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_srlv_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sub_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_sub_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sub_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_sub_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sub_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_sub_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_sub_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_sub_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_subs_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_subs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_subs_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_subs_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_subs_epu16(a: u16x16, b: u16x16) -> u16x16 {
    ::mem::transmute(::arch::x86::_mm256_subs_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_subs_epu8(a: u8x32, b: u8x32) -> u8x32 {
    ::mem::transmute(::arch::x86::_mm256_subs_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_unpackhi_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_unpackhi_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_unpacklo_epi8(a: i8x32, b: i8x32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_unpacklo_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_unpackhi_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_unpackhi_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_unpacklo_epi16(a: i16x16, b: i16x16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_unpacklo_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_unpackhi_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_unpackhi_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_unpacklo_epi32(a: i32x8, b: i32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_unpacklo_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_unpackhi_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_unpackhi_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_unpacklo_epi64(a: i64x4, b: i64x4) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_unpacklo_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_xor_si256(a: __m256i, b: __m256i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_xor_si256(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_extract_epi8(a: i8x32, imm8: i32) -> i8 {
    ::mem::transmute(::arch::x86::_mm256_extract_epi8(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_extract_epi16(a: i16x16, imm8: i32) -> i16 {
    ::mem::transmute(::arch::x86::_mm256_extract_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_extract_epi32(a: i32x8, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_extract_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtsd_f64(a: __m256d) -> f64 {
    ::mem::transmute(::arch::x86::_mm256_cvtsd_f64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_cvtsi256_si32(a: __m256i) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_cvtsi256_si32(::mem::transmute(a)))
}

