use ::myarch::*;
use ::simd::*;

// #[target_feature(enable = "sse2")]
// pub unsafe fn get_m128d(a: __m128d, idx: usize) -> f64 {
//     ::mem::transmute(::myarch::get_m128d(::mem::transmute(a), ::mem::transmute(idx)))
// }

// #[target_feature(enable = "sse")]
// pub unsafe fn get_m128(a: f32x4, idx: usize) -> f32 {
//     ::mem::transmute(::myarch::get_m128(::mem::transmute(a), ::mem::transmute(idx)))
// }

// // not actually an intrinsic but useful in various tests as we proted from
// // `i64x2::new` which is backwards from `_mm_set_epi64x`
// #[target_feature(enable = "sse2")]
// pub unsafe fn _mm_setr_epi64x(a: i64, b: i64) -> __m128i {
//     ::mem::transmute(::myarch::_mm_setr_epi64x(::mem::transmute(a), ::mem::transmute(b)))
// }

// #[target_feature(enable = "avx")]
// pub unsafe fn get_m256d(a: __m256d, idx: usize) -> f64 {
//     ::mem::transmute(::myarch::get_m256d(::mem::transmute(a), ::mem::transmute(idx)))
// }

// #[target_feature(enable = "avx")]
// pub unsafe fn get_m256(a: f32x8, idx: usize) -> f32 {
//     ::mem::transmute(::myarch::get_m256(::mem::transmute(a), ::mem::transmute(idx)))
// }
