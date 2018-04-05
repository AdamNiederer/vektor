use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_add_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_add_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_add_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_add_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_and_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_and_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_and_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_and_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_or_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_or_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_or_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_or_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_shuffle_pd(a: __m256d, b: __m256d, imm8: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_shuffle_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_shuffle_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_shuffle_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_andnot_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_andnot_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_andnot_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_andnot_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_max_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_max_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_max_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_max_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_min_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_min_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_min_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_min_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_mul_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_mul_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_mul_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_mul_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_addsub_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_addsub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_addsub_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_addsub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_sub_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_sub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_sub_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_sub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_div_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_div_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_div_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_div_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_round_pd(a: __m256d, b: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_round_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_ceil_pd(a: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_ceil_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_floor_pd(a: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_floor_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_round_ps(a: f32x8, b: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_round_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_ceil_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_ceil_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_floor_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_floor_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_sqrt_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_sqrt_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_sqrt_pd(a: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_sqrt_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_blend_pd(a: __m256d, b: __m256d, imm8: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_blend_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_blend_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_blend_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_blendv_pd(a: __m256d, b: __m256d, c: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_blendv_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_blendv_ps(a: f32x8, b: f32x8, c: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_blendv_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_dp_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_dp_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_hadd_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_hadd_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_hadd_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_hadd_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_hsub_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_hsub_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_hsub_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_hsub_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_xor_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_xor_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_xor_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_xor_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx,sse2")]
pub unsafe fn _mm_cmp_pd(a: __m128d, b: __m128d, imm8: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmp_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cmp_pd(a: __m256d, b: __m256d, imm8: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_cmp_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx,sse")]
pub unsafe fn _mm_cmp_ps(a: f32x4, b: f32x4, imm8: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmp_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cmp_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_cmp_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx,sse2")]
pub unsafe fn _mm_cmp_sd(a: __m128d, b: __m128d, imm8: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_cmp_sd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx,sse")]
pub unsafe fn _mm_cmp_ss(a: f32x4, b: f32x4, imm8: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_cmp_ss(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvtepi32_pd(a: __m128i) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtepi32_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvtepi32_ps(a: __m256i) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_cvtepi32_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvtpd_ps(a: __m256d) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtpd_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvtps_epi32(a: f32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_cvtps_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvtps_pd(a: f32x4) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtps_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvttpd_epi32(a: __m256d) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm256_cvttpd_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvtpd_epi32(a: __m256d) -> i32x4 {
    ::mem::transmute(::arch::x86::_mm256_cvtpd_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvttps_epi32(a: f32x8) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_cvttps_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_extractf128_ps(a: f32x8, imm8: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm256_extractf128_ps(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_extractf128_pd(a: __m256d, imm8: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm256_extractf128_pd(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_extractf128_si256(a: __m256i, imm8: i32) -> __m128i {
    ::mem::transmute(::arch::x86::_mm256_extractf128_si256(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_permutevar_ps(a: f32x8, b: __m256i) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_permutevar_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_permutevar_ps(a: f32x4, b: __m128i) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_permutevar_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_permute_ps(a: f32x8, imm8: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_permute_ps(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx,sse")]
pub unsafe fn _mm_permute_ps(a: f32x4, imm8: i32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_permute_ps(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_permutevar_pd(a: __m256d, b: __m256i) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_permutevar_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_permutevar_pd(a: __m128d, b: __m128i) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_permutevar_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_permute_pd(a: __m256d, imm8: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_permute_pd(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx,sse2")]
pub unsafe fn _mm_permute_pd(a: __m128d, imm8: i32) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_permute_pd(::mem::transmute(a), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_permute2f128_ps(a: f32x8, b: f32x8, imm8: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_permute2f128_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_permute2f128_pd(a: __m256d, b: __m256d, imm8: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_permute2f128_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_permute2f128_si256(a: __m256i, b: __m256i, imm8: i32) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_permute2f128_si256(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_broadcast_ss(f: &f32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_broadcast_ss(::mem::transmute(f)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_broadcast_ss(f: &f32) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_broadcast_ss(::mem::transmute(f)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_broadcast_sd(f: &f64) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_broadcast_sd(::mem::transmute(f)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_broadcast_ps(a: &f32x4) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_broadcast_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_broadcast_pd(a: &__m128d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_broadcast_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_insertf128_ps(a: f32x8, b: f32x4, imm8: i32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_insertf128_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_insertf128_pd(a: __m256d, b: __m128d, imm8: i32) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_insertf128_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_insertf128_si256(a: __m256i, b: __m128i, imm8: i32) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_insertf128_si256(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(imm8)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_insert_epi8(a: i8x32, i: i8, index: i32) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_insert_epi8(::mem::transmute(a), ::mem::transmute(i), ::mem::transmute(index)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_insert_epi16(a: i16x16, i: i16, index: i32) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_insert_epi16(::mem::transmute(a), ::mem::transmute(i), ::mem::transmute(index)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_insert_epi32(a: i32x8, i: i32, index: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_insert_epi32(::mem::transmute(a), ::mem::transmute(i), ::mem::transmute(index)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_load_pd(mem_addr: *const f64) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_load_pd(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_load_ps(mem_addr: *const f32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_load_ps(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_loadu_pd(mem_addr: *const f64) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_loadu_pd(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_loadu_ps(mem_addr: *const f32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_loadu_ps(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_load_si256(mem_addr: *const __m256i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_load_si256(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_loadu_si256(mem_addr: *const __m256i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_loadu_si256(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_maskload_pd(mem_addr: *const f64, mask: __m256i) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_maskload_pd(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_maskload_pd(mem_addr: *const f64, mask: __m128i) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm_maskload_pd(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_maskload_ps(mem_addr: *const f32, mask: __m256i) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_maskload_ps(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_maskload_ps(mem_addr: *const f32, mask: __m128i) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm_maskload_ps(::mem::transmute(mem_addr), ::mem::transmute(mask)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_movehdup_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_movehdup_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_moveldup_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_moveldup_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_movedup_pd(a: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_movedup_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_lddqu_si256(mem_addr: *const __m256i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_lddqu_si256(::mem::transmute(mem_addr)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_rcp_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_rcp_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_rsqrt_ps(a: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_rsqrt_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_unpackhi_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_unpackhi_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_unpackhi_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_unpackhi_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_unpacklo_pd(a: __m256d, b: __m256d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_unpacklo_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_unpacklo_ps(a: f32x8, b: f32x8) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_unpacklo_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testz_si256(a: __m256i, b: __m256i) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testz_si256(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testc_si256(a: __m256i, b: __m256i) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testc_si256(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testnzc_si256(a: __m256i, b: __m256i) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testnzc_si256(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testz_pd(a: __m256d, b: __m256d) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testz_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testc_pd(a: __m256d, b: __m256d) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testc_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testnzc_pd(a: __m256d, b: __m256d) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testnzc_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_testz_pd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testz_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_testc_pd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testc_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_testnzc_pd(a: __m128d, b: __m128d) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testnzc_pd(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testz_ps(a: f32x8, b: f32x8) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testz_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testc_ps(a: f32x8, b: f32x8) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testc_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_testnzc_ps(a: f32x8, b: f32x8) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_testnzc_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_testz_ps(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testz_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_testc_ps(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testc_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm_testnzc_ps(a: f32x4, b: f32x4) -> i32 {
    ::mem::transmute(::arch::x86::_mm_testnzc_ps(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_movemask_pd(a: __m256d) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_movemask_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_movemask_ps(a: f32x8) -> i32 {
    ::mem::transmute(::arch::x86::_mm256_movemask_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setzero_pd() -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_setzero_pd())
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setzero_ps() -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_setzero_ps())
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setzero_si256() -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_setzero_si256())
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_pd(a: f64, b: f64, c: f64, d: f64) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_set_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_ps(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_set_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d), ::mem::transmute(e), ::mem::transmute(f), ::mem::transmute(g), ::mem::transmute(h)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_epi8(e00: i8, e01: i8, e02: i8, e03: i8, e04: i8, e05: i8, e06: i8, e07: i8, e08: i8, e09: i8, e10: i8, e11: i8, e12: i8, e13: i8, e14: i8, e15: i8, e16: i8, e17: i8, e18: i8, e19: i8, e20: i8, e21: i8, e22: i8, e23: i8, e24: i8, e25: i8, e26: i8, e27: i8, e28: i8, e29: i8, e30: i8, e31: i8) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_set_epi8(::mem::transmute(e00), ::mem::transmute(e01), ::mem::transmute(e02), ::mem::transmute(e03), ::mem::transmute(e04), ::mem::transmute(e05), ::mem::transmute(e06), ::mem::transmute(e07), ::mem::transmute(e08), ::mem::transmute(e09), ::mem::transmute(e10), ::mem::transmute(e11), ::mem::transmute(e12), ::mem::transmute(e13), ::mem::transmute(e14), ::mem::transmute(e15), ::mem::transmute(e16), ::mem::transmute(e17), ::mem::transmute(e18), ::mem::transmute(e19), ::mem::transmute(e20), ::mem::transmute(e21), ::mem::transmute(e22), ::mem::transmute(e23), ::mem::transmute(e24), ::mem::transmute(e25), ::mem::transmute(e26), ::mem::transmute(e27), ::mem::transmute(e28), ::mem::transmute(e29), ::mem::transmute(e30), ::mem::transmute(e31)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_epi16(e00: i16, e01: i16, e02: i16, e03: i16, e04: i16, e05: i16, e06: i16, e07: i16, e08: i16, e09: i16, e10: i16, e11: i16, e12: i16, e13: i16, e14: i16, e15: i16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_set_epi16(::mem::transmute(e00), ::mem::transmute(e01), ::mem::transmute(e02), ::mem::transmute(e03), ::mem::transmute(e04), ::mem::transmute(e05), ::mem::transmute(e06), ::mem::transmute(e07), ::mem::transmute(e08), ::mem::transmute(e09), ::mem::transmute(e10), ::mem::transmute(e11), ::mem::transmute(e12), ::mem::transmute(e13), ::mem::transmute(e14), ::mem::transmute(e15)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_epi32(e0: i32, e1: i32, e2: i32, e3: i32, e4: i32, e5: i32, e6: i32, e7: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_set_epi32(::mem::transmute(e0), ::mem::transmute(e1), ::mem::transmute(e2), ::mem::transmute(e3), ::mem::transmute(e4), ::mem::transmute(e5), ::mem::transmute(e6), ::mem::transmute(e7)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_epi64x(a: i64, b: i64, c: i64, d: i64) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_set_epi64x(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_pd(a: f64, b: f64, c: f64, d: f64) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_setr_pd(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_ps(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_setr_ps(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d), ::mem::transmute(e), ::mem::transmute(f), ::mem::transmute(g), ::mem::transmute(h)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_epi8(e00: i8, e01: i8, e02: i8, e03: i8, e04: i8, e05: i8, e06: i8, e07: i8, e08: i8, e09: i8, e10: i8, e11: i8, e12: i8, e13: i8, e14: i8, e15: i8, e16: i8, e17: i8, e18: i8, e19: i8, e20: i8, e21: i8, e22: i8, e23: i8, e24: i8, e25: i8, e26: i8, e27: i8, e28: i8, e29: i8, e30: i8, e31: i8) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_setr_epi8(::mem::transmute(e00), ::mem::transmute(e01), ::mem::transmute(e02), ::mem::transmute(e03), ::mem::transmute(e04), ::mem::transmute(e05), ::mem::transmute(e06), ::mem::transmute(e07), ::mem::transmute(e08), ::mem::transmute(e09), ::mem::transmute(e10), ::mem::transmute(e11), ::mem::transmute(e12), ::mem::transmute(e13), ::mem::transmute(e14), ::mem::transmute(e15), ::mem::transmute(e16), ::mem::transmute(e17), ::mem::transmute(e18), ::mem::transmute(e19), ::mem::transmute(e20), ::mem::transmute(e21), ::mem::transmute(e22), ::mem::transmute(e23), ::mem::transmute(e24), ::mem::transmute(e25), ::mem::transmute(e26), ::mem::transmute(e27), ::mem::transmute(e28), ::mem::transmute(e29), ::mem::transmute(e30), ::mem::transmute(e31)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_epi16(e00: i16, e01: i16, e02: i16, e03: i16, e04: i16, e05: i16, e06: i16, e07: i16, e08: i16, e09: i16, e10: i16, e11: i16, e12: i16, e13: i16, e14: i16, e15: i16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_setr_epi16(::mem::transmute(e00), ::mem::transmute(e01), ::mem::transmute(e02), ::mem::transmute(e03), ::mem::transmute(e04), ::mem::transmute(e05), ::mem::transmute(e06), ::mem::transmute(e07), ::mem::transmute(e08), ::mem::transmute(e09), ::mem::transmute(e10), ::mem::transmute(e11), ::mem::transmute(e12), ::mem::transmute(e13), ::mem::transmute(e14), ::mem::transmute(e15)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_epi32(e0: i32, e1: i32, e2: i32, e3: i32, e4: i32, e5: i32, e6: i32, e7: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_setr_epi32(::mem::transmute(e0), ::mem::transmute(e1), ::mem::transmute(e2), ::mem::transmute(e3), ::mem::transmute(e4), ::mem::transmute(e5), ::mem::transmute(e6), ::mem::transmute(e7)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_epi64x(a: i64, b: i64, c: i64, d: i64) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_setr_epi64x(::mem::transmute(a), ::mem::transmute(b), ::mem::transmute(c), ::mem::transmute(d)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set1_pd(a: f64) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_set1_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set1_ps(a: f32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_set1_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set1_epi8(a: i8) -> i8x32 {
    ::mem::transmute(::arch::x86::_mm256_set1_epi8(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set1_epi16(a: i16) -> i16x16 {
    ::mem::transmute(::arch::x86::_mm256_set1_epi16(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set1_epi32(a: i32) -> i32x8 {
    ::mem::transmute(::arch::x86::_mm256_set1_epi32(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set1_epi64x(a: i64) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_set1_epi64x(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castpd_ps(a: __m256d) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_castpd_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castps_pd(a: f32x8) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_castps_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castps_si256(a: f32x8) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_castps_si256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castsi256_ps(a: __m256i) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_castsi256_ps(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castpd_si256(a: __m256d) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_castpd_si256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castsi256_pd(a: __m256i) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_castsi256_pd(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castps256_ps128(a: f32x8) -> f32x4 {
    ::mem::transmute(::arch::x86::_mm256_castps256_ps128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castpd256_pd128(a: __m256d) -> f64x2 {
    ::mem::transmute(::arch::x86::_mm256_castpd256_pd128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castsi256_si128(a: __m256i) -> __m128i {
    ::mem::transmute(::arch::x86::_mm256_castsi256_si128(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castps128_ps256(a: f32x4) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_castps128_ps256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castpd128_pd256(a: __m128d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_castpd128_pd256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_castsi128_si256(a: __m128i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_castsi128_si256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx,sse")]
pub unsafe fn _mm256_zextps128_ps256(a: f32x4) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_zextps128_ps256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx,sse2")]
pub unsafe fn _mm256_zextsi128_si256(a: __m128i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_zextsi128_si256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx,sse2")]
pub unsafe fn _mm256_zextpd128_pd256(a: __m128d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_zextpd128_pd256(::mem::transmute(a)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_undefined_ps() -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_undefined_ps())
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_undefined_pd() -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_undefined_pd())
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_undefined_si256() -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_undefined_si256())
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_m128(hi: f32x4, lo: f32x4) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_set_m128(::mem::transmute(hi), ::mem::transmute(lo)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_m128d(hi: __m128d, lo: __m128d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_set_m128d(::mem::transmute(hi), ::mem::transmute(lo)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_set_m128i(hi: __m128i, lo: __m128i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_set_m128i(::mem::transmute(hi), ::mem::transmute(lo)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_m128(lo: f32x4, hi: f32x4) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_setr_m128(::mem::transmute(lo), ::mem::transmute(hi)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_m128d(lo: __m128d, hi: __m128d) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_setr_m128d(::mem::transmute(lo), ::mem::transmute(hi)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_setr_m128i(lo: __m128i, hi: __m128i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_setr_m128i(::mem::transmute(lo), ::mem::transmute(hi)))
}

#[inline]
#[target_feature(enable = "avx,sse")]
pub unsafe fn _mm256_loadu2_m128(hiaddr: *const f32, loaddr: *const f32) -> f32x8 {
    ::mem::transmute(::arch::x86::_mm256_loadu2_m128(::mem::transmute(hiaddr), ::mem::transmute(loaddr)))
}

#[inline]
#[target_feature(enable = "avx,sse2")]
pub unsafe fn _mm256_loadu2_m128d(hiaddr: *const f64, loaddr: *const f64) -> f64x4 {
    ::mem::transmute(::arch::x86::_mm256_loadu2_m128d(::mem::transmute(hiaddr), ::mem::transmute(loaddr)))
}

#[inline]
#[target_feature(enable = "avx,sse2")]
pub unsafe fn _mm256_loadu2_m128i(hiaddr: *const __m128i, loaddr: *const __m128i) -> __m256i {
    ::mem::transmute(::arch::x86::_mm256_loadu2_m128i(::mem::transmute(hiaddr), ::mem::transmute(loaddr)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn _mm256_cvtss_f32(a: f32x8) -> f32 {
    ::mem::transmute(::arch::x86::_mm256_cvtss_f32(::mem::transmute(a)))
}

