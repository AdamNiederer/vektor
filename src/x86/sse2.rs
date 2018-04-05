use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_add_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_add_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_add_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_add_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_add_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_add_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_add_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_add_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_adds_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_adds_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_adds_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_adds_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_adds_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_adds_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_adds_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::arch::x86::_mm_adds_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_avg_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_avg_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_avg_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::arch::x86::_mm_avg_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_madd_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_madd_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_max_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_max_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_max_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_max_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_min_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_min_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_min_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_min_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_mulhi_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_mulhi_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_mulhi_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::arch::x86::_mm_mulhi_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_mullo_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_mullo_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_mul_epu32(a: u32x4, b: u32x4) -> u32x4 {
    ::mem::transmute(::arch::x86::_mm_mul_epu32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sad_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_sad_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sub_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_sub_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sub_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_sub_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sub_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_sub_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sub_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_sub_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_subs_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_subs_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_subs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_subs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_subs_epu8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_subs_epu8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_subs_epu16(a: u16x8, b: u16x8) -> u16x8 {
    ::mem::transmute(::arch::x86::_mm_subs_epu16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_slli_si128(a: __m128i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_slli_si128(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_bslli_si128(a: __m128i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_bslli_si128(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_bsrli_si128(a: __m128i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_bsrli_si128(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_slli_epi16(a: i16x8, imm8: i32) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_slli_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sll_epi16(a: i16x8, count: __m128i) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_sll_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_slli_epi32(a: i32x4, imm8: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_slli_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sll_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_sll_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_slli_epi64(a: i64x2, imm8: i32) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_slli_epi64(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sll_epi64(a: i64x2, count: __m128i) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_sll_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srai_epi16(a: i16x8, imm8: i32) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_srai_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sra_epi16(a: i16x8, count: __m128i) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_sra_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srai_epi32(a: i32x4, imm8: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_srai_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sra_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_sra_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srli_si128(a: __m128i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_srli_si128(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srli_epi16(a: i16x8, imm8: i32) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_srli_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srl_epi16(a: i16x8, count: __m128i) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_srl_epi16(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srli_epi32(a: i32x4, imm8: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_srli_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srl_epi32(a: i32x4, count: __m128i) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_srl_epi32(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srli_epi64(a: i64x2, imm8: i32) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_srli_epi64(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_srl_epi64(a: i64x2, count: __m128i) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_srl_epi64(::mem::transmute(a), ::mem::transmute(count)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_and_si128(a: __m128i, b: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_and_si128(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_andnot_si128(a: __m128i, b: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_andnot_si128(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_or_si128(a: __m128i, b: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_or_si128(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_xor_si128(a: __m128i, b: __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_xor_si128(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpeq_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_cmpeq_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpeq_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_cmpeq_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpeq_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpeq_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpgt_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpgt_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpgt_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmplt_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_cmplt_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmplt_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_cmplt_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmplt_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cmplt_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtepi32_pd(a: __m128i) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtepi32_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi32_sd(a: __m128d, b: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtsi32_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtepi32_ps(a: __m128i) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtepi32_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtps_epi32(a: f32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtps_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi32_si128(a: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_cvtsi32_si128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsi128_si32(a: __m128i) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cvtsi128_si32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_epi64x(e1: i64, e0: i64) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_set_epi64x(::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_set_epi32(::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_epi16(e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_set_epi16(::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_epi8(e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_set_epi8(::mem::transmute(e15), ::mem::transmute(e14), ::mem::transmute(e13), ::mem::transmute(e12), ::mem::transmute(e11), ::mem::transmute(e10), ::mem::transmute(e9), ::mem::transmute(e8), ::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set1_epi64x(a: i64) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_set1_epi64x(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set1_epi32(a: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_set1_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set1_epi16(a: i16) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_set1_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set1_epi8(a: i8) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_set1_epi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_setr_epi32(e3: i32, e2: i32, e1: i32, e0: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_setr_epi32(::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_setr_epi16(e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_setr_epi16(::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_setr_epi8(e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_setr_epi8(::mem::transmute(e15), ::mem::transmute(e14), ::mem::transmute(e13), ::mem::transmute(e12), ::mem::transmute(e11), ::mem::transmute(e10), ::mem::transmute(e9), ::mem::transmute(e8), ::mem::transmute(e7), ::mem::transmute(e6), ::mem::transmute(e5), ::mem::transmute(e4), ::mem::transmute(e3), ::mem::transmute(e2), ::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_setzero_si128() -> __m128i {
    ::mem::transmute(::arch::x86::_mm_setzero_si128())
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_loadl_epi64(mem_addr: *const __m128i) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_loadl_epi64(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_load_si128(mem_addr: *const __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_load_si128(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_loadu_si128(mem_addr: *const __m128i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_loadu_si128(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_move_epi64(a: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_move_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_packs_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_packs_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_packs_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_packs_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_packus_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_packus_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_extract_epi16(a: i16x8, imm8: i32) -> i32 {
    ::mem::transmute(::arch::x86::_mm_extract_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_insert_epi16(a: i16x8, i: i32, imm8: i32) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_insert_epi16(::mem::transmute(a), ::mem::transmute(i), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_movemask_epi8(a: u8x16) -> i32 {
    ::mem::transmute(::arch::x86::_mm_movemask_epi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_shuffle_epi32(a: i32x4, imm8: i32) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_shuffle_epi32(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_shufflehi_epi16(a: i16x8, imm8: i32) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_shufflehi_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_shufflelo_epi16(a: i16x8, imm8: i32) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_shufflelo_epi16(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpackhi_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpackhi_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpackhi_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpackhi_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpacklo_epi8(a: u8x16, b: u8x16) -> u8x16 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_epi8(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpacklo_epi16(a: i16x8, b: i16x8) -> i16x8 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_epi16(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpacklo_epi32(a: i32x4, b: i32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_epi32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpacklo_epi64(a: i64x2, b: i64x2) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_epi64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_add_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_add_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_add_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_add_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_div_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_div_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_div_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_div_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_max_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_max_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_max_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_max_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_min_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_min_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_min_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_min_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_mul_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_mul_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_mul_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_mul_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sqrt_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_sqrt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sqrt_pd(a: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_sqrt_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sub_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_sub_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_sub_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_sub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_and_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_and_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_andnot_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_andnot_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_or_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_or_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_xor_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_xor_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpeq_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpeq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmplt_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmplt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmple_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmple_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpgt_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpge_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpge_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpord_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpord_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpunord_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpunord_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpneq_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpneq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpnlt_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpnlt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpnle_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpnle_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpngt_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpngt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpnge_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpnge_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpeq_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpeq_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmplt_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmplt_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmple_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmple_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpgt_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpgt_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpge_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpge_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpord_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpord_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpunord_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpunord_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpneq_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpneq_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpnlt_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpnlt_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpnle_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpnle_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpngt_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpngt_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cmpnge_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmpnge_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_comieq_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comieq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_comilt_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comilt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_comile_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comile_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_comigt_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comigt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_comige_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comige_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_comineq_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_comineq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_ucomieq_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomieq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_ucomilt_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomilt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_ucomile_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomile_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_ucomigt_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomigt_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_ucomige_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomige_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_ucomineq_sd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_ucomineq_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtpd_ps(a: __m128d) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtpd_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtps_pd(a: f32x4) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtps_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtpd_epi32(a: __m128d) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtpd_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsd_si32(a: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cvtsd_si32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsd_ss(a: f32x4, b: __m128d) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cvtsd_ss(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtsd_f64(a: __m128d) -> f64 {
    ::mem::transmute(::arch::x86::_mm_cvtsd_f64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvtss_sd(a: __m128d, b: f32x4) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtss_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvttpd_epi32(a: __m128d) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cvttpd_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvttsd_si32(a: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_cvttsd_si32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_cvttps_epi32(a: f32x4) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm_cvttps_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_sd(a: f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_set_sd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set1_pd(a: f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_set1_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_pd1(a: f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_set_pd1(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_set_pd(a: f64, b: f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_set_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_setr_pd(a: f64, b: f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_setr_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_setzero_pd() -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_setzero_pd())
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_movemask_pd(a: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_movemask_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_load_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_load_pd(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_load_sd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_load_sd(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_loadh_pd(a: __m128d, mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_loadh_pd(::mem::transmute(a), ::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_loadl_pd(a: __m128d, mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_loadl_pd(::mem::transmute(a), ::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_load1_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_load1_pd(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_load_pd1(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_load_pd1(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_loadr_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_loadr_pd(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_loadu_pd(mem_addr: *const f64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_loadu_pd(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_shuffle_pd(a: __m128d, b: __m128d, imm8: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_shuffle_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_move_sd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_move_sd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castpd_ps(a: __m128d) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_castpd_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castpd_si128(a: __m128d) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_castpd_si128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castps_pd(a: f32x4) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_castps_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castps_si128(a: f32x4) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_castps_si128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castsi128_pd(a: __m128i) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_castsi128_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_castsi128_ps(a: __m128i) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_castsi128_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_undefined_pd() -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_undefined_pd())
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_undefined_si128() -> __m128i {
    ::mem::transmute(::arch::x86::_mm_undefined_si128())
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpackhi_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_unpackhi_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_unpacklo_pd(a: __m128d, b: __m128d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_unpacklo_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_add_si64(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_add_si64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_mul_su32(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_mul_su32(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_sub_si64(a: __m64, b: __m64) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_sub_si64(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_cvtpi32_pd(a: __m64) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cvtpi32_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_set_epi64(e1: __m64, e0: __m64) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_set_epi64(::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_set1_epi64(a: __m64) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_set1_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_setr_epi64(e1: __m64, e0: __m64) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_setr_epi64(::mem::transmute(e1), ::mem::transmute(e0)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_movepi64_pi64(a: __m128i) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_movepi64_pi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_movpi64_epi64(a: __m64) -> i64x2 {
    ::mem::transmute(::arch::x86::_mm_movpi64_epi64(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_cvtpd_pi32(a: __m128d) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cvtpd_pi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "sse2,mmx")]
pub unsafe fn _mm_cvttpd_pi32(a: __m128d) -> __m64 {
    ::mem::transmute(::arch::x86::_mm_cvttpd_pi32(::mem::transmute(a)))
}

