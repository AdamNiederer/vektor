use ::arch::x86_64::*;
use ::simd::*;

// #[inline]
// #[target_feature(enable = "avx")]
// pub unsafe fn _mm256_insert_epi64(a: i64x4, i: i64, index: i32) -> i64x4 {
//     ::mem::transmute(::arch::x86_64::_mm256_insert_epi64(::mem::transmute(a), ::mem::transmute(i), ::mem::transmute(index)))
// }
