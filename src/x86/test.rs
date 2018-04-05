use ::arch::x86::*;
use ::simd::*;

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn get_m128d(a: __m128d, idx: usize) -> f64 {
    ::mem::transmute(::arch::x86::get_m128d(::mem::transmute(a), ::mem::transmute(idx)))
}

#[inline]
#[target_feature(enable = "sse")]
pub unsafe fn get_m128(a: f32x4, idx: usize) -> f32 {
    ::mem::transmute(::arch::x86::get_m128(::mem::transmute(a), ::mem::transmute(idx)))
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn _mm_setr_epi64x(a: i64, b: i64) -> __m128i {
    ::mem::transmute(::arch::x86::_mm_setr_epi64x(::mem::transmute(a), ::mem::transmute(b)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn get_m256d(a: __m256d, idx: usize) -> f64 {
    ::mem::transmute(::arch::x86::get_m256d(::mem::transmute(a), ::mem::transmute(idx)))
}

#[inline]
#[target_feature(enable = "avx")]
pub unsafe fn get_m256(a: f32x8, idx: usize) -> f32 {
    ::mem::transmute(::arch::x86::get_m256(::mem::transmute(a), ::mem::transmute(idx)))
}

#[inline]
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_insert_epi64(a: i64x4, val: i64, idx: i32) -> i64x4 {
    ::mem::transmute(::arch::x86::_mm256_insert_epi64(::mem::transmute(a), ::mem::transmute(val), ::mem::transmute(idx)))
}

